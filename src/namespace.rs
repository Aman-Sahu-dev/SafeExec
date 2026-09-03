use crate::error::{Result, SafeExecError};
use nix::sched::{CloneFlags, clone};
use nix::unistd::Pid;

pub struct NamespaceController;

impl NamespaceController {
    pub fn new() -> Self {
        Self
    }
}
