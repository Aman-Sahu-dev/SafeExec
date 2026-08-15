use crate::cli::Args;
use crate::error::Result;
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

        let launcher = TaskLauncher::new(
            &self.args.exec,
            &self.args.args,
            timeout,
            format!("safeexec-{}", self.session_id),
        );

        let exit_code = launcher.run()?;
        println!("Task exited with code: {}", exit_code);

        Ok(())
    }
}

fn parse_duration(s: &str) -> Result<Duration> {
    // Simple parser: "2s", "5m", "100ms"
    let s = s.trim();
    if let Some(v) = s.strip_suffix("ms") {
        let ms: u64 = v.parse().map_err(|_| {
            crate::error::SafeExecError::InvalidArgument(format!("invalid duration: {}", s))
        })?;
        return Ok(Duration::from_millis(ms));
    }
    if let Some(v) = s.strip_suffix('s') {
        let sec: u64 = v.parse().map_err(|_| {
            crate::error::SafeExecError::InvalidArgument(format!("invalid duration: {}", s))
        })?;
        return Ok(Duration::from_secs(sec));
    }
    if let Some(v) = s.strip_suffix('m') {
        let min: u64 = v.parse().map_err(|_| {
            crate::error::SafeExecError::InvalidArgument(format!("invalid duration: {}", s))
        })?;
        return Ok(Duration::from_secs(min * 60));
    }
    // Default: treat as seconds
    let sec: u64 = s.parse().map_err(|_| {
        crate::error::SafeExecError::InvalidArgument(format!("invalid duration: {}", s))
    })?;
    Ok(Duration::from_secs(sec))
}
