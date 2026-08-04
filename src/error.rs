use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SafeExecError {
    #[error("syscall failed: {0}")]
    Syscall(#[from] nix::Error),

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("cgroup operation failed: {0}")]
    Cgroup(String),

    #[error("namespace creation failed: {0}")]
    Namespace(String),

    #[error("mount operation failed: {0}")]
    Mount(String),

    #[error("network setup failed: {0}")]
    Network(String),

    #[error("task execution failed: {0}")]
    Task(String),

    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    #[error("privilege error: {0}")]
    Privilege(String),

    #[error("timeout: execution exceeded {0:?}")]
    Timeout(std::time::Duration),
}

pub type Result<T> = std::result::Result<T, SafeExecError>;
