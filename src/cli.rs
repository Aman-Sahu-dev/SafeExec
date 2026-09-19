use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "safeexec")]
#[command(about = "A lighweight linux container runtime and sandbox")]

pub struct Args {
    #[arg(long, short = 'e')]
    pub exec: PathBuf,

    #[arg(long, short = 'a', num_args = 0..)]
    pub args: Vec<String>,

    #[arg(long, default_value = "64MB")]
    pub max_memory: String,

    #[arg(long, default_value = "20")]
    pub max_pids: u64,

    #[arg(long)]
    pub cpu_max: Option<String>,

    #[arg(long, short = 't', default_value = "30s")]
    pub timeout: String,

    #[arg(long, short = 'i')]
    pub input: Option<PathBuf>,

    #[arg(long, short = 'o')]
    pub output: Option<PathBuf>,

    #[arg(long, alias = "theatermode", value_enum, default_value = "narrative")]
    pub theater_mode: TheaterMode,

    #[arg(long, alias = "quite_telemtry")]
    pub quiet_telemetry: bool,

    #[arg(long)]
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum TheaterMode {
    #[value(name = "narrative", alias = "narretive")]
    Narretive,

    Technical,

    Silent,
}

impl TheaterMode {
    #[allow(non_upper_case_globals)]
    pub const Narrative: Self = Self::Narretive;
}
