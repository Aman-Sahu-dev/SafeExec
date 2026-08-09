use crate::error::{Result,SafeExecError};
use nix::sched::CloneFlags;
use nix::unistd::Pid;

pub struct NamespaceController;

impl NamespaceController {
    pub fn new()->Self{
        Self
    }

    pub fn build_clone_flags()->CloneFlags {
        CloneFlags::CLONE_NEWPID
            CloneFlags::CLONE_NEWNS
            CloneFlags::CLONE_NEWUTS
            CloneFlags::CLONE_NEWUSER
            CloneFlags::CLONE_NEWNET
    }
}
