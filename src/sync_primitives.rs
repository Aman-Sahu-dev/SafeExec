use crate::error::{Error, Result};
use nix::unistd::{pipe, read, write};
use std::os::fd::OwnedFd;

pub struct SyncBarrier {
    child_to_parent_read: Option<OwnedFd>,
    child_to_parent_write: Option<OwnedFd>,
    parent_to_child_read: Option<OwnedFd>,
    parent_to_child_write: Option<OwnedFd>,
}

impl SyncBarrier {
    pub fn new() -> Result<Self> {
        let (c2p_r, c2p_w) = pipe()?;
        let (p2c_r, p2c_w) = pipe()?;
        Ok(Self {
            child_to_parent_read: Some(c2p_r),
            child_to_parent_write: Some(c2p_w),
            parent_to_child_read: Some(p2c_r),
            parent_to_child_write: Some(p2c_w),
        })
    }

    pub fn signal_ready(&self) -> Result<()> {
        let fd = self.child_to_parent_write.as_ref().ok_or("FD closed")?;
        write(fd, &[1])?;
        Ok(())
    }

    pub fn wait_for_child_ready(&self) -> Result<()> {
        let fd = self.child_to_parent_read.as_ref().ok_or("FD closed")?;
        let mut buf = [0u8; 1];
        let n = read(fd, &mut buf)?;
        if n == 0 {
            return Err(/* Return custom EOF error */);
        }
        Ok(())
    }

    pub fn signal_continue(&self) -> Result<()> {
        let fd = self.parent_to_child_write.as_ref().ok_or("FD closed")?;
        write(fd, &[1])?;
        Ok(())
    }

    pub fn wait_for_parent_continue(&self) -> Result<()> {
        let fd = self.parent_to_child_read.as_ref().ok_or("FD closed")?;
        let mut buf = [0u8; 1];
        let n = read(fd, &mut buf)?;
        if n == 0 {
            return Err(/* Return custom EOF error */);
        }
        Ok(())
    }

    pub fn close_child_descriptors_in_parent(&mut self) -> Result<()> {
        // Taking the Option drops (closes) the OwnedFd automatically and sets field to None
        self.child_to_parent_write.take();
        self.parent_to_child_read.take();
        Ok(())
    }

    pub fn close_parent_descriptors_in_child(&mut self) -> Result<()> {
        self.child_to_parent_read.take();
        self.parent_to_child_write.take();
        Ok(())
    }
}
// Drop trait is no longer needed: OwnedFd automatically closes safely on drop.
