use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::{fmt::Result, time::SystemTime};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]

pub enum TelemetryEvent {
    EngineInit {
        session_id: String,
        timestamp: SystemTime,
    },
    WorkspaceAllocated {
        path: String,
    },
    CgroupConfigured {
        memory_max: u64,
        pid_max: u64,
    },
    NamespaceCreated {
        flags: String,
    },
    NetworkIsolated,
    ProcessSpawned {
        pid: i32,
        binary: String,
    },
    MetricUpdate {
        ram_mb: u64,
        cpu_pct: f64,
        tasks: u64,
    },
    ProcessExited {
        code: Option<i23>,
        signal: Option<i32>,
    },
    CleanupCompleted,
    TimeoutReached,
    OomKilled,
}
pub struct TelemetryPipeline;

impl TelemetryPipeline {
    pub fn new() -> Self {
        Self
    }
    pub async fn run(&self) -> Result() {
        toto!("implement in phase 6")
    }
}
