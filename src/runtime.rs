use std::fmt::Result;

use crate::cli::Args;
use crate::error::Result;

pub struct RuntimeOrchesterator {
    pub args: Args,
    pub session_id: String,
}

impl RuntimeOrchesterator {
    pub fn new(args: Args) -> Result<Self> {
        let session_id = args
            .session_id
            .clone()
            .unwrap_or_else(|| nanoid::nanoid!(10));
        Ok(Self { args, session_id })
    }
}
pub fn run(&self) -> Result<()> {
    todo!("RuntimeOrchesterator::run() - integrate later");
}
