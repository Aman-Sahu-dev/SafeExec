use crate::error::{Result, SafeExecError};
use nix::sched::{CloneFlags, clone};
use nix::unistd::Pid;

pub struct NamespaceController;

impl NamespaceController {
    pub fn new() -> Self {
        Self
    }
    pub fn build_clone_flags() -> CloneFlags {
        CloneFlags::CLONE_NEWPID
            | CloneFlags::CLONE_NEWNS
            | CloneFlags::CLONE_NEWUTS
            | CloneFlags::CLONE_NEWUSER
            | CloneFlags::CLONE_NEWNET
    }
    pub unsafe fn spawn_container_init<F>(
        &self,
        flags: CloneFlags,
        stack: &mut [u8],
        callback: F,
    ) -> Result<Pid>
    where
        F: FnMut() -> isize,
    {
        let pid = unsafe { clone(Box::new(callback), stack, flags, Some(nix::libc::SIGCHLD))? };
        Ok(pid)
    }
    pub fn write_uid_gid_map(&self, pid: Pid, host_uid: u32, host_gid: u32) -> Result<()> {
        let uid_map = format!("0 {} 1\n", host_uid);
        let gid_map = format!("0 {} 1\n", host_gid);

        let uid_path = format!("/proc/{}/uid_map", pid);
        let gid_path = format!("/proc/{}/gid_map", pid);

        std::fs::write(&uid_path, uid_map)
            .map_err(|e| SafeExecError::Namespace(format!("failed to write uid map: {}", e)));
        let setgroups_path = format!("/proc/{}/setgroups", pid);
        let _ = std::fs::write(setgroups_path, "deny");

        std::fs::write(&gid_path, gid_map)
            .map_err(|e| SafeExecError::Namespace(format!("failed to write gid_map {}", e)));
        Ok(())
    }
}
