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
    pub fn child_view(&self) -> ChildBarrier {
        ChildBarrier {
            c2p_read: self.c2p[0],
            c2p_write: self.c2p[1],
            p2c_read: self.p2c[0],
            p2c_write: self.p2c[1],
        }
    }
    pub fn parent_view(&self) -> ParentBarrier {
        ParentBarrier {
            c2p_read: self.c2p[0],
            c2p_write: self.c2p[1],
            p2c_read: self.p2c[0],
            p2c_write: self.p2c[1],
        }
    }
}
pub struct ChildBarrier {
    c2p_read: OwnedFd,
    c2p_write: OwnedFd,
    p2c_read: OwnedFd,
    p2c_write: OwnedFd,
}
impl ChildBarrier {
    pub fn signal_ready(&self) -> Result<()> {
        let _ = write(self.c2p_write, &[1])?;
        Ok(())
    }
    pub fn wait_for_parent_continue(&self) -> Result<()> {
        let mut buf = [0u8; 1];
        let _ = read(self.p2c_read, &mut buf)?;
        Ok(())
    }
    pub fn close_parent_descriptors(&self) -> Result<()> {
        close(self.c2p_read)?;
        close(self.p2c_write)?;
        Ok(())
    }
}
pub struct ParentBarrier {
    c2p_read: OwnedFd,
    c2p_write: OwnedFd,
    p2c_read: OwnedFd,
    p2c_write: OwnedFd,
}
