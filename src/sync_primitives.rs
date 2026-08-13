use std::{fmt::Result, intrinsics::read_via_copy};

use crate::error::Result;
use nix::unistd::{close, pipe, write};

pub struct SyncBarrier {
    child_to_parent_read: i32,
    child_to_parent_write: i32,
    parent_to_child_read: i32,
    parent_to_child_write: i32,
}
impl SyncBarrier {
    pub fn new() -> Result<Self> {
        let (c2p_r, c2p_w) = pipe()?;
        let (p2c_r, p2c_w) = pipe()?;
        Ok(Self {
            child_to_parent_read: c2p_r,
            child_to_parent_write: c2p_w,
            parent_to_child_read: p2c_r,
            parent_to_child_write: p2c_w,
        })
    }

    pub fn signal_ready(&self) -> Result<()> {
        let _ = write(self.child_to_parent_write, &[1])?;
        Ok(())
    }
    pub fn wait_for_child_ready(&self) -> Result<()> {
        let mut buf = [0u8; 1];
        let _ = read(self.child_to_parent_read, &mut buf);
        Ok(())
    }

    pub fn signal_continue(&self) -> Result<()> {
        let _ = write(self.parent_to_child_write, &[1]);
        Ok(())
    }
    pub fn wait_for_parent_continue(&self) -> Result<()> {
        let mut buf = [0u8; 1];
        let _ = read(self.parent_to_child_read, &mut buf)?;
        Ok(())
    }
    pub fn close_child_descriptors_in_parent(&self) -> Result<()> {
        close(self.child_to_parent_read)?;
        close(self.child_to_parent_write)?;
        Ok(())
    }
    pub fn close_parent_descriptors_in_child(&self) -> Result<()> {
        close(self.parent_to_child_read)?;
        close(self.parent_to_child_write)?;
        Ok(())
    }
}
impl Drop for SyncBarrier {
    fn drop(&mut self) {
        let _ = close(self.child_to_parent_read);
        let _ = close(self.child_to_parent_write);
        let _ = close(self.parent_to_child_read);
        let _ = close(self.parent_to_child_write);
    }
}
