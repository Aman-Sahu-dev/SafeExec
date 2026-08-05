use clap::{Parser, ValueEnum};
use std::{default, path::PathBuf};

#[derive(Parser, Debug)]
#[command(name = "safeexec")]
#[command(about = "A lighweight linux container runtime and sandbox")]

pub struct Args {
    #[arg(long, short = 'e')]
    pub exec: PathBuf,

    #[arg(long,short = 'a',nums_args = 0..)]
    pub args: Vec<String>,

    #[arg(long, default_value = "64MB")]
    pub max_memory: String,

    #[arg(long, default_value = "20")]
    pub max_pids: u64,

    #[arg(long)]
    pub cpu_max: Option<String>,

    #[arg(long, short = 't', default_value)]
    pub timeout: String,

    #[arg(long, short = 'i')]
    pub input: Option<PathBuf>,

    #[arg(long, short = 'o')]
    pub output: Option<PathBuf>,

    #[arg(long, ValueEnum, default_value = "narretive")]
    pub theatermode: TheaterMode,

    #[arg(long)]
    pub quite_telemtry: bool,

    #[arg(long)]
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum TheaterMode {
    Narretive,

    Technical,

    Silent,
}
