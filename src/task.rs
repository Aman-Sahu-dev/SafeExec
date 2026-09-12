use crate::error::{SafeExecError,Result};
use crate::mount::VfsManager;
use crate::cgroup::CgroupV2Manager;
use crate::namespace::{NamespaceController,allocate_clone_stack};
use crate::sync_primitives::SyncBarrier;
use nix::sys::prctl;
use nix::sys::signal::Signal;
use nix::sys::wait::{waitpid,WaitPidFlag};
use nix::unistd::{self,getgid,geteuid,Gid,Pid,Uid};
use std::ffi::CString;
use std::path::{self, Path};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool,Ordering}
use std::thread;
use std::time::Duration;

pub struct TaskLauncher{
    pub binary: &'a Path,
    pub args: &'a [String],
    pub timeout: Duration,
    pub hostname: String,
    pub cgroup: Option<&'a CgroupV2Manager>,
    pub input: Option<&'a Path>,
    pub output: Option<&'a Path>,
}
impl <&'a> TaskLauncher <&'a> {
    pub fn new(binary: &'a Path,args: &'a [String],timeout: Duration,hostname: String) -> Self{
     Self{
        binary,
        args,
        timeout,
        hostname,
        cgroup: None,
        input:None,
        output:None
     }
    }
    pub fn with_cgroup(mut self,cg: &'a CgroupV2Manager)-> Self{
        self.cgroup = Some(cg);
        Self
    }
    pub fn with_input(mut self, path: &'a Path) -> Self{
        self.input = Some(path);
        Self 
    }
    pub fn with_output(mut self, path: &'a Path) -> Self{
        self.output = Some(path);
        Self 
    }
}
