use crate::cgroup::CgroupV2Manager;
use crate::cli::Args;
use crate::error::{Result, SafeExecError};
use crate::task::TaskLauncher;
use std::time::Duration;

pub struct RuntimeOrchestrator {
    pub args: Args,
    pub session_id: String,
}

impl RuntimeOrchestrator {
    pub fn new(args: Args) -> Result<Self> {
        let session_id = args
            .session_id
            .clone()
            .unwrap_or_else(|| nanoid::nanoid!(10));
        Ok(Self { args, session_id })
    }

    pub fn run(&self) -> Result<()> {
        let timeout = parse_duration(&self.args.timeout)?;

        let cgroup = CgroupV2Manager::new(&self.session_id)?;

        let memory_bytes = parse_memory(&self.args.max_memory)?;
        cgroup.set_memory_max(memory_bytes)?;
        cgroup.set_pids_max(self.args.max_pids)?;

        if let Some(ref cpu_str) = self.args.cpu_max {
            cgroup.set_cpu_max(cpu_str)?;
        }

        let launcher = TaskLauncher::new(
            &self.args.exec,
            &self.args.args,
            timeout,
            format!("safeexec-{}", self.session_id),
        )
        .with_cgroup(&cgroup);

        let exit_code = launcher.run()?;
        println!("Task exited with code: {}", exit_code);

        Ok(())
    }
}

fn parse_duration(s: &str) -> Result<Duration> {
    let s = s.trim();
    if let Some(v) = s.strip_suffix("ms") {
        let ms: u64 = v
            .parse()
            .map_err(|_| SafeExecError::InvalidArgument(format!("invalid duration: {}", s)))?;
        return Ok(Duration::from_millis(ms));
    }
    if let Some(v) = s.strip_suffix('s') {
        let sec: u64 = v
            .parse()
            .map_err(|_| SafeExecError::InvalidArgument(format!("invalid duration: {}", s)))?;
        return Ok(Duration::from_secs(sec));
    }
    if let Some(v) = s.strip_suffix('m') {
        let min: u64 = v
            .parse()
            .map_err(|_| SafeExecError::InvalidArgument(format!("invalid duration: {}", s)))?;
        return Ok(Duration::from_secs(min * 60));
    }
    let sec: u64 = s
        .parse()
        .map_err(|_| SafeExecError::InvalidArgument(format!("invalid duration: {}", s)))?;
    Ok(Duration::from_secs(sec))
}

fn parse_memory(s: &str) -> Result<u64> {
    if let Ok(byte) = byte_unit::Byte::from_str(s) {
        return Ok(byte.as_u64());
    }

    let s = s.trim().to_uppercase();
    let (num_part, mult) = if let Some(v) = s.strip_suffix("GB") {
        (v, 1024u64 * 1024 * 1024)
    } else if let Some(v) = s.strip_suffix("MB") {
        (v, 1024u64 * 1024)
    } else if let Some(v) = s.strip_suffix("KB") {
        (v, 1024u64)
    } else if let Some(v) = s.strip_suffix('B') {
        (v, 1u64)
    } else {
        (&*s, 1u64)
    };

    let n: u64 = num_part
        .trim()
        .parse()
        .map_err(|_| SafeExecError::InvalidArgument(format!("invalid memory limit: {}", s)))?;

    Ok(n * mult)
}
