use crate::error::Result;
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
    pub fn write_uid_gid_map(&self, pid: Pid, host_uid: u32, host_gid: u32) {}
}
