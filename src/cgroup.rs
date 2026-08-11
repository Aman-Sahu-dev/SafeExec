use crate::error::{Result, SafeExecError};
use nix::unistd::Pid;
use std::{
    fmt::Result,
    path::{Path, PathBuf},
};

pub struct CgrpupV2Manager {
    cgroup_path: PathBuf,
}

impl CgrpupV2Manager {
    pub fn new(session_id: &str) -> Result<Self> {
        let cgroup_path = PathBuf::from(format!("/sys/fs/cgroup/safeexec{}", session_id));
        Ok(Self { cgroup_path })
    }
    pub fn set_memory_max(&self, _bytes: u64) -> Result<()> {
        todo!("CgrpupV2Manager set memory max imlement in phase 3");
    }

    pub fn set_cpu_max(&self, _quota: u64, _period: u64) -> Result<()> {
        // TODO(Phase 3): write cpu.max
        todo!("CgroupV2Manager::set_cpu_max() — implement in Phase 3")
    }

    pub fn set_pids_max(&self, _max: u64) -> Result<()> {
        // TODO(Phase 3): write pids.max
        todo!("CgroupV2Manager::set_pids_max() — implement in Phase 3")
    }

    pub fn attach_pid(&self, _pid: Pid) -> Result<()> {
        // TODO(Phase 3): write pid to cgroup.procs
        todo!("CgroupV2Manager::attach_pid() — implement in Phase 3")
    }
}
impl Drop for CgrpupV2Manager {
    fn drop(&mut self) {
        let _ = &self.cgroup_path;
    }
}
