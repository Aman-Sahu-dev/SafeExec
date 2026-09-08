use crate::error::{Result, SafeExecError};
use nix::mount::{MntFlags, MsFlags, mount, umount2};
use nix::unistd::chdir;
use std::path::Component::Prefix;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

pub struct VfsManager;

impl VfsManager {
    pub fn new() -> Self {
        Self
    }
    pub fn allocate_wokspace(&self, session_id: &str) -> Result<tempfile::TempDir> {
        let dir = tempfile::Builder::new()
            .prefix(&format!("safeexec {}", session_id))
            .tempdir()
            .map_err(|e| SafeExecError::Mount(format!("failed to create worksapce {}", e)))?;
        Ok(dir)
    }
    pub fn setup_minimal_root(&self, root: &Path) -> Result<()> {
        let dirs = &[
            "bin", "dev", "etc", "lib", "proc", "sys", "lib64", "tmp", "usr", "in", "out",
        ];
        for subdir in dirs {
            let path = root.join(subdir);
            std::fs::create_dir_all(&path).map_err(|e| {
                SafeExecError::Mount(format!("failed to mkdir {}: {}", path.display(), e))
            })?;
        }


}
