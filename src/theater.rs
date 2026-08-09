use std::path::PathBuf;

use crate::cli::TheaterMode;
use crate::error::Result;
use crate::telemetry::TelemetryEvent;

pub struct TheatrerEngine {
    mode: TheaterMode,
}
impl TheatrerEngine {
    pub fn new(mode: TheaterMode) -> Self {
        Self { mode }
    }

    pub fn render(&self, event: &TelemetryEvent) -> Result<Option<String>> {
        if matches!(self.mode, TheaterMode::Silent) {
            return Ok(None);
        }
        let line = match self.mode {
            TheaterMode::Narretive => self.narrative(event),
            TheaterMode::Technical => self.technical(event),
            TheaterMode::Silent => unreachable!(),
        };
        Ok(line)
    }
    fn narrative(&self, event: &TelemetryEvent) -> Option<String> {
        use TelemetryEvent::*;
        match event {
            EngineInit { session_id, .. } => Some(format!(
                "🎭 [PROLOGUE]  The SafeExec stage opens for session {}...",
                session_id
            )),
            WorkspaceAllocated { path } => Some(format!(
                "📁 [VFS]       A temporary world is carved at {}",
                path
            )),
            CgroupConfigured {
                memory_max,
                pid_max,
            } => {
                let mem = byte_unit::Byte::from_bytes(*memory_max);
                Some(format!(
                    "🛡️ [CGROUP]    The memory cage is locked: {} maximum. Fork-bomb fuse: {} threads.",
                    mem.get_appropriate_unit(false),
                    pid_max
                ))
            }
            NamespaceCreated { flags } => Some(format!(
                "🔒 [NS_INIT]   Four veils descend — PID, Mount, UTS, User — the host disappears. (flags: {})",
                flags
            )),
            NetworkIsolated => Some(
                "🌐 [NETNS]     The network stage is stripped bare. Only loopback echoes remain."
                    .to_string(),
            ),
            ProcessSpawned { pid, binary } => Some(format!(
                "⚡ [ACT I]     The actor steps through the clone gate. PID {} in a universe of one. Script: {}",
                pid, binary
            )),
            MetricUpdate {
                ram_mb,
                cpu_pct,
                tasks,
            } => Some(format!(
                "📊 [METRICS]   RAM: {:.1} MB | CPU: {:.0}% | Tasks: {}",
                ram_mb, cpu_pct, tasks
            )),
            ProcessExited { code: Some(c), .. } => Some(format!(
                "✅ [EPILOGUE]  The actor exits with grace. Status Code {}.",
                c
            )),
            ProcessExited {
                code: None,
                signal: Some(s),
            } => Some(format!(
                "💀 [EPILOGUE]  The actor was struck down by signal {}.",
                s
            )),
            CleanupCompleted => Some(
                "🧹 [CLEANUP]   The temporary world dissolves. Cgroup slice erased.".to_string(),
            ),
            TimeoutReached => {
                Some("⏱️ [EPILOGUE]  The curtain falls early — execution timed out.".to_string())
            }
            OomKilled => Some(
                "💥 [EPILOGUE]  The memory cage broke the actor. OOM kill triggered.".to_string(),
            ),
        }
    }

    fn technical(&self, event: &TelemetryEvent) -> Option<String> {
        // TODO(Phase 8): Raw syscall traces and technical details
        Some(format!("[TECH] {:?}", event))
    }
}
