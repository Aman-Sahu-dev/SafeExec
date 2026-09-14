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
        if let Some(input) = self.input{
            vfs.bind_mount_into(&input,&workspace_path.join("in"))?;
        }
        if let Some(output) = self.output{
            vfs.bind_mount_output(&output,&workspace_path.join("out"))?;
        }
        let binary_cstr = CString::new(self.binary.as_os_str().as_encoded_bytes()).map_err(|_| SafeExecError::InvalidArgument("binary path contain null ".into()))?;
        let args_cstr: Vec<CString> = //continue
    }
}
