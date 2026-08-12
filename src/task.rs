use crate::error::{Result, SafeExecError};
use nix::unistd::Pid;
use std::path::Path;
use std::time::Duration;

/// Launcher for the sandboxed child process.
pub struct TaskLauncher<'a> {
    pub binary: &'a Path,
    pub args: &'a [String],
    pub timeout: Duration,
}

impl<'a> TaskLauncher<'a> {
    pub fn new(binary: &'a Path, args: &'a [String], timeout: Duration) -> Self {
        Self {
            binary,
            args,
            timeout,
        }
    }

    /// Execute the full lifecycle: clone → setup → execve → waitpid.
    pub fn run(&self) -> Result<i32> {
        // TODO(Phase 6): Full integration of namespace, mount, cgroup, network
        todo!("TaskLauncher::run() — implement in Phase 6")
    }

    /// Child-side setup routine called inside the cloned process.
    pub fn child_setup(&self) -> Result<()> {
        // TODO(Phase 4-5): pivot_root, setup lo, drop privs
        todo!("TaskLauncher::child_setup() — implement in Phase 4-5")
    }
}
