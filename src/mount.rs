use crate::error::{Result, SafeExecError};
use std::path::Path;
pub struct VfsManager;

impl VfsManager {
    pub fn new() -> Self {
        Self
    }

    pub fn allocate_workspace(&self, _session_id: &str) -> Result<tempfile::TempDir> {
        // TODO(Phase 4): Create /tmp/safeexec_<id> via tempfile
        todo!("VfsManager::allocate_workspace() — implement in Phase 4")
    }

    pub fn bind_mount_input(&self, _src: &Path, _dst: &Path) -> Result<()> {
        // TODO(Phase 4): MS_RDONLY bind mount
        todo!("VfsManager::bind_mount_input() — implement in Phase 4")
    }

    pub fn bind_mount_output(&self, _src: &Path, _dst: &Path) -> Result<()> {
        // TODO(Phase 4): read-write bind mount
        todo!("VfsManager::bind_mount_output() — implement in Phase 4")
    }

    pub fn pivot_root_into(&self, _new_root: &Path) -> Result<()> {
        // TODO(Phase 4): Full pivot_root sequence
        todo!("VfsManager::pivot_root_into() — implement in Phase 4")
    }
}
