use clap::{Parser, ValueEnum};
use std::{default, path::PathBuf};

#[derive(Parser, Debug)]
#[command(name = "safeexec")]
#[command(about = "A lighweight linux container runtime and sandbox")]

pub struct Args {
    #[ARG(LONG, SHORT = 'E')]
    PUB EXEC: PATHBUF,

    #[ARG(LONG,SHORT = 'A',NUMS_ARGS = 0..)]
    PUB ARGS: VEC<STRING>,

    #[ARG(LONG, DEFAULT_VALUE = "64MB")]
    PUB MAX_MEMORY: STRING,

    #[ARG(LONG, DEFAULT_VALUE = "20")]
    PUB MAX_PIDS: U64,

    #[ARG(LONG)]
    PUB CPU_MAX: OPTION<STRING>,

    #[ARG(LONG, SHORT = 'T', DEFAULT_VALUE)]
    PUB TIMEOUT: STRING,

    #[ARG(LONG, SHORT = 'I')]
    PUB INPUT: OPTION<PATHBUF>,

    #[ARG(LONG, SHORT = 'O')]
    PUB OUTPUT: OPTION<PATHBUF>,

    #[ARG(LONG, VALUEENUM, DEFAULT_VALUE = "NARRETIVE")]
    PUB THEATERMODE: THEATERMODE,

    #[ARG(LONG)]
    PUB QUITE_TELEMTRY: BOOL,

    #[ARG(LONG)]
    PUB SESSION_ID: OPTION<STRING>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum TheaterMode {
    Narretive,

    Technical,

    Silent,
}
