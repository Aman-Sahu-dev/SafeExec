use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "safeexec")]
#[command(about = "A lightweight Linux container runtime and sandbox engine")]
pub struct Args {
    /// Binary to execute inside the sandbox
    #[arg(long, short = 'e')]
    pub exec: PathBuf,

    /// Arguments to pass to the target binary
    #[arg(long, short = 'a', num_args = 0..)]
    pub args: Vec<String>,

    /// Maximum memory limit (e.g., 64MB, 1GB)
    #[arg(long, default_value = "64MB")]
    pub max_memory: String,

    /// Maximum number of PIDs (fork-bomb protection)
    #[arg(long, default_value = "20")]
    pub max_pids: u64,

    /// CPU quota per period (e.g., "100000 100000" for 1 core)
    #[arg(long)]
    pub cpu_max: Option<String>,

    /// Execution timeout (e.g., 2s, 5m)
    #[arg(long, short = 't', default_value = "30s")]
    pub timeout: String,

    /// Input directory to bind-mount as read-only
    #[arg(long, short = 'i')]
    pub input: Option<PathBuf>,

    /// Output directory to bind-mount as read-write
    #[arg(long, short = 'o')]
    pub output: Option<PathBuf>,

    /// Execution Theater rendering mode
    #[arg(long, value_enum, default_value = "narrative")]
    pub theater_mode: TheaterMode,

    /// Disable telemetry stream (Theater only)
    #[arg(long)]
    pub quiet_telemetry: bool,

    /// Session ID override (auto-generated if omitted)
    #[arg(long)]
    pub session_id: Option<String>,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum TheaterMode {
    /// Human-centric narrative of kernel events
    Narrative,
    /// Raw syscall traces and technical details
    Technical,
    /// Silent — no Theater output
    Silent,
}
