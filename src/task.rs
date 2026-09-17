use crate::cgroup::CgroupV2Manager;
use crate::error::{Result, SafeExecError};
use crate::mount::VfsManager;
use crate::namespace::{NamespaceController, allocate_clone_stack};
use crate::sync_primitives::SyncBarrier;
use nix::sys::prctl;
use nix::sys::signal::Signal;
use nix::sys::wait::{WaitPidFlag, waitpid};
use nix::unistd::{self, Gid, Pid, Uid, geteuid, getgid};
use std::ffi::CString;
use std::path::{self, Path};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

pub struct TaskLauncher<'a> {
    pub binary: &'a Path,
    pub args: &'a [String],
    pub timeout: Duration,
    pub hostname: String,
    pub cgroup: Option<&'a CgroupV2Manager>,
    pub input: Option<&'a Path>,
    pub output: Option<&'a Path>,
}
impl<'a> TaskLauncher<'a> {
    pub fn new(binary: &'a Path, args: &'a [String], timeout: Duration, hostname: String) -> Self {
        Self {
            binary,
            args,
            timeout,
            hostname,
            cgroup: None,
            input: None,
            output: None,
        }
    }
    pub fn with_cgroup(mut self, cg: &'a CgroupV2Manager) -> Self {
        self.cgroup = Some(cg);
        self
    }
    pub fn with_input(mut self, path: &'a Path) -> Self {
        self.input = Some(path);
        self
    }
    pub fn with_output(mut self, path: &'a Path) -> Self {
        self.output = Some(path);
        self
    }
    pub fn run(&self) -> Result<i32> {
        let flags = NamespaceController::build_clone_flags();
        let mut stack = allocate_clone_stack(1024 * 1024);
        let sync = SyncBarrier::new();

        let vfs = VfsManager::new();
        let workspace = vfs.allocate_wokspace("Phase 4")?;
        let workspace_path = workspace.path().to_path_buf();
        vfs.setup_minimal_root(&workspace_path)?;
        if let Some(input) = self.input {
            vfs.bind_mount_into(&input, &workspace_path.join("in"))?;
        }
        if let Some(output) = self.output {
            vfs.bind_mount_output(&output, &workspace_path.join("out"))?;
        }
        let args_cstr: Vec<CString> = std::iter::once(binary_cstr.clone())
            .chain(
                self.args
                    .iter()
                    .map(|s| CString::new(s.as_bytes()).unwrap()),
            )
            .collect();

        let envp: Vec<CString> = std::env::vars()
            .map(|(k, v)| CString::new(format!("{}={}", k, v)).unwrap())
            .collect();

        let hostname_cstr = CString::new(self.hostname.as_bytes()).unwrap();
        let wp = workspace_path.clone();

        let child_barrier = sync.child_view();
        let parent_barrier = sync.parent_view();
        let child_callback = move || {
            if let Err(e) = child_barrier.close_parent_descriptors() {
                eprintln!("[child] close_parent_descriptors failed: {}", e);
                return 1;
            }

            if let Err(e) = prctl::set_pdeathsig(Signal::SIGKILL) {
                eprintln!("[child] prctl(PR_SET_PDEATHSIG) failed: {}", e);
                return 1;
            }

            if let Err(e) = unistd::setuid(Uid::from_raw(0)) {
                eprintln!("[child] setuid(0) failed: {}", e);
                return 1;
            }
            if let Err(e) = unistd::setgid(Gid::from_raw(0)) {
                eprintln!("[child] setgid(0) failed: {}", e);
                return 1;
            }

            if let Err(e) = child_barrier.signal_ready() {
                eprintln!("[child] signal_ready failed: {}", e);
                return 1;
            }

            if let Err(e) = child_barrier.wait_for_parent_continue() {
                eprintln!("[child] wait_for_parent_continue failed: {}", e);
                return 1;
            }

            if let Err(e) = unistd::sethostname(&hostname_cstr) {
                eprintln!("[child] sethostname failed: {}", e);
                return 1;
            }

            let vfs = VfsManager::new();
            if let Err(e) = vfs.pivot_root_into(&wp) {
                eprintln!("[child] pivot_root failed: {}", e);
                return 1;
            }
            if let Err(e) = vfs.setup_proc() {
                eprintln!("[child] setup_proc failed: {}", e);
                return 1;
            }
            if let Err(e) = vfs.setup_tmpfs() {
                eprintln!("[child] setup_tmpfs failed: {}", e);
                return 1;
            }

            if let Err(e) = unistd::execve(&binary_cstr, &args_cstr, &envp) {
                eprintln!("[child] execve failed: {}", e);
                return 127;
            }

            unreachable!("execve should never return on success")
            let ns = NamespaceController::new();
            let child_pid = unsafe { ns.spawn_container_init(flags, &mut stack, child_callback)? };

            parent_barrier.close_child_descriptors()?;
             parent_barrier.wait_for_child_ready()?;

            ns.write_uid_gid_map(child_pid, getuid().as_raw(), getgid().as_raw())?;

            if let Some(cg) = self.cgroup {
            cg.attach_pid(child_pid)?;
            }

            parent_barrier.signal_continue()?;
        };

    }
}
