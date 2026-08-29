use crate::cgroup::CgroupV2Manager;
use crate::error::{Result, SafeExecError};
use crate::namespace::{ChildContext, NamespaceController, allocate_clone_stack};
use crate::sync_primitives::SyncBarrier;
use nix::sys::wait::{WaitStatus, waitpid};
use nix::unistd::{Pid, getgid, getuid};
use std::ffi::CString;
use std::path::Path;
use std::time::Duration;

pub struct TaskLauncher<'a> {
    pub binary: &'a Path,
    pub args: &'a [String],
    pub timeout: Duration,
    pub hostname: String,
    pub cgroup: Option<&'a CgroupV2Manager>,
}

impl<'a> TaskLauncher<'a> {
    pub fn new(binary: &'a Path, args: &'a [String], timeout: Duration, hostname: String) -> Self {
        Self {
            binary,
            args,
            timeout,
            hostname,
            cgroup: None,
        }
    }

    pub fn with_cgroup(mut self, cg: &'a CgroupV2Manager) -> Self {
        self.cgroup = Some(cg);
        self
    }

    pub fn run(&self) -> Result<i32> {
        let flags = NamespaceController::build_clone_flags();
        let mut stack = allocate_clone_stack(1024 * 1024);

        let sync = SyncBarrier::new()?;

        let binary = CString::new(self.binary.as_os_str().as_encoded_bytes())
            .map_err(|_| SafeExecError::InvalidArgument("binary path contains null".into()))?;

        let args: Vec<CString> = std::iter::once(binary.clone())
            .chain(
                self.args
                    .iter()
                    .map(|s| CString::new(s.as_bytes()).unwrap()),
            )
            .collect();

        let envp: Vec<CString> = std::env::vars()
            .map(|(k, v)| CString::new(format!("{}={}", k, v)).unwrap())
            .collect();

        let ctx = ChildContext {
            binary,
            args,
            envp,
            hostname: Some(CString::new(self.hostname.as_bytes()).unwrap()),
            sync,
        };

        let ns = NamespaceController::new();
        ctx.sync.close_child_descriptors_in_parent()?;

        let child_pid = unsafe { ns.spawn_container_init(flags, &mut stack, ctx)? };

        ctx.sync.wait_for_child_ready()?;
        ns.write_uid_gid_map(child_pid, getuid().as_raw(), getgid().as_raw())?;

        if let Some(cg) = self.cgroup {
            cg.attach_pid(child_pid)?;
        }

        ctx.sync.signal_continue()?;

        let status = waitpid(child_pid, None).map_err(SafeExecError::Syscall)?;

        match status {
            WaitStatus::Exited(_, code) => Ok(code),
            WaitStatus::Signaled(_, sig, _) => Ok(128 + sig as i32),
            _ => Err(SafeExecError::Task(format!(
                "unexpected wait status: {:?}",
                status
            ))),
        }
    }
}
