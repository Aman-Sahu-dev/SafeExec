use crate::error::{Result, SafeExecError};
use nix::unistd::{close, pipe, read, write};

pub struct SyncPipe {
    read_fd: i32,
    write_fd: i32,
}

impl SyncPipe {
    pub fn signal_ready(&self) -> Result<()> {
        let _ = write(self.write_fd, &[1])?;
        Ok(())
    }

    pub fn wait_for_signal(&self) -> Result<()> {
        let mut buf = [0u8; 1];
        let _ = read(self.read_fd, &mut buf)?;
        Ok(())
    }

    pub fn close_write_in_parent(&self) -> Result<()> {
        close(self.write_fd)?;
        Ok(())
    }

    pub fn close_read_in_child(&self) -> Result<()> {
        close(self.read_fd)?;
        Ok(())
    }
}

impl Drop for SyncPipe {
    fn drop(&mut self) {
        let _ = close(self.read_fd);
        let _ = close(self.write_fd);
    }
}
