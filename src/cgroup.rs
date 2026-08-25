use crate::error::{Result, SafeExecError};
use nix::unistd::Pid;
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

/// Manager for cgroups v2 resource limits and metric queries.
///
/// Creates a slice at `/sys/fs/cgroup/safeexec_<session_id>/` and provides
/// RAII cleanup via Drop.
pub struct CgroupV2Manager {
    cgroup_path: PathBuf,
}

impl CgroupV2Manager {
    /// Create a new cgroup slice. Cleans up stale slices with the same name.
    pub fn new(session_id: &str) -> Result<Self> {
        let path = PathBuf::from(format!("/sys/fs/cgroup/safeexec_{}", session_id));

        // Stale cleanup: if a previous run crashed, the directory may persist.
        if path.exists() {
            Self::force_remove(&path);
        }

        fs::create_dir(&path).map_err(|e| {
            SafeExecError::Cgroup(format!(
                "failed to create cgroup {} (are you root?): {}",
                path.display(),
                e
            ))
        })?;

        Ok(Self { cgroup_path: path })
    }

    /// Hard memory limit in bytes. Writes to `memory.max`.
    pub fn set_memory_max(&self, bytes: u64) -> Result<()> {
        self.write_control("memory.max", &bytes.to_string())
    }

    /// CPU quota/period. Writes raw string to `cpu.max`.
    /// Example: `set_cpu_max("50000 100000")` for 0.5 cores.
    pub fn set_cpu_max(&self, value: &str) -> Result<()> {
        self.write_control("cpu.max", value)
    }

    /// Maximum number of tasks (PIDs). Fork-bomb protection.
    pub fn set_pids_max(&self, max: u64) -> Result<()> {
        self.write_control("pids.max", &max.to_string())
    }

    /// Attach a running process to this cgroup.
    pub fn attach_pid(&self, pid: Pid) -> Result<()> {
        self.write_control("cgroup.procs", &pid.as_raw().to_string())
    }

    /// Current memory usage in bytes (from `memory.current`).
    pub fn memory_current(&self) -> Result<u64> {
        self.read_control_u64("memory.current")
    }

    /// Current number of tasks in the cgroup (from `pids.current`).
    pub fn pids_current(&self) -> Result<u64> {
        self.read_control_u64("pids.current")
    }

    /// CPU usage in microseconds (from `cpu.stat`).
    pub fn cpu_stat_usage_usec(&self) -> Result<u64> {
        let path = self.cgroup_path.join("cpu.stat");
        let content = fs::read_to_string(&path)
            .map_err(|e| SafeExecError::Cgroup(format!("failed to read cpu.stat: {}", e)))?;
        for line in content.lines() {
            if let Some(v) = line.strip_prefix("usage_usec ") {
                return v
                    .parse()
                    .map_err(|_| SafeExecError::Cgroup("invalid usage_usec".into()));
            }
        }
        Ok(0)
    }

    /// Read a specific event counter from `memory.events`.
    /// Common events: `oom_kill`, `oom_group_kill`, `high`, `max`.
    pub fn read_memory_event(&self, event: &str) -> Result<u64> {
        let path = self.cgroup_path.join("memory.events");
        let content = fs::read_to_string(&path)
            .map_err(|e| SafeExecError::Cgroup(format!("failed to read memory.events: {}", e)))?;
        for line in content.lines() {
            let mut parts = line.split_whitespace();
            if let Some(key) = parts.next() {
                if key == event {
                    if let Some(val) = parts.next() {
                        return val.parse().map_err(|_| {
                            SafeExecError::Cgroup(format!("invalid memory.events line: {}", line))
                        });
                    }
                }
            }
        }
        Ok(0)
    }

    /// Expose the cgroup path for external tools or debugging.
    pub fn path(&self) -> &Path {
        &self.cgroup_path
    }

    // -----------------------------------------------------------------
    // Internal helpers
    // -----------------------------------------------------------------

    fn write_control(&self, file: &str, value: &str) -> Result<()> {
        let path = self.cgroup_path.join(file);
        fs::write(&path, value).map_err(|e| {
            SafeExecError::Cgroup(format!(
                "failed to write '{}' to {}: {}",
                value,
                path.display(),
                e
            ))
        })
    }

    fn read_control_u64(&self, file: &str) -> Result<u64> {
        let path = self.cgroup_path.join(file);
        let s = fs::read_to_string(&path).map_err(|e| {
            SafeExecError::Cgroup(format!("failed to read {}: {}", path.display(), e))
        })?;
        s.trim()
            .parse()
            .map_err(|_| SafeExecError::Cgroup(format!("invalid number in {}", path.display())))
    }

    /// Best-effort forced removal of a stale cgroup directory.
    fn force_remove(path: &Path) {
        // 1. Try cgroup.kill (kernel 5.14+)
        let _ = fs::write(path.join("cgroup.kill"), "1");

        // 2. Fallback: read cgroup.procs and SIGKILL each PID
        if let Ok(content) = fs::read_to_string(path.join("cgroup.procs")) {
            for line in content.lines() {
                if let Ok(pid) = line.parse::<i32>() {
                    let _ = nix::sys::signal::kill(
                        Pid::from_raw(pid),
                        nix::sys::signal::Signal::SIGKILL,
                    );
                }
            }
        }

        // 3. Retry rmdir with backoff
        for _ in 0..20 {
            if fs::remove_dir(path).is_ok() {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
    }
}

impl Drop for CgroupV2Manager {
    fn drop(&mut self) {
        Self::force_remove(&self.cgroup_path);
    }
}
