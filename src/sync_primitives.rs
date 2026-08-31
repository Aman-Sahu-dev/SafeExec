use std::os::fd::OwnedFd;

use crate::error::Result;
use nix::unistd::{close, pipe, read, write};

pub struct SyncBarrier {
    c2p: [OwnedFd; 2],
    p2c: [OwnedFd; 2],
}
impl SyncBarrier {
    pub fn new() -> Result<Self> {
        let (c2p_r, c2p_w) = pipe()?;
        let (p2c_r, p2c_w) = pipe()?;
        Ok(Self {
            c2p: [c2p_r, c2p_w],
            p2c: [p2c_r, p2c_w],
        })
    }
}
