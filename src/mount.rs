use crate::error::{Result, SafeExecError};
use nix::mount::{MsFlags, mount};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

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

        let tmp = root.join("tmp");
        std::fs::set_permissions(&tmp, std::fs::Permissions::from_mode(0o777)).ok();
        self.bind_mount_ro(Path::new("/bin"), &root.join("bin"))?;
        self.bind_mount_ro(Path::new("/lib"), &root.join("lib"))?;
        if Path::new("/lib64").exists() {
            self.bind_mount_ro(Path::new("/lib64"), &root.join("lib64"))?;
        }
        self.bind_mount_ro(Path::new("/usr"), &root.join("usr"))?;
        self.bind_mount_ro(Path::new("/dev"), &root.join("dev"))?;

        Ok(())
    }
    pub fn bind_mount_into(&self, src: &Path, dst: &Path) -> Result<()> {
        std::fs::create_dir_all(dst).ok();
        self.bind_mount_ro(src, dst)
    }

    pub fn bind_mount_output(&self, dst: &Path) -> Result<()> {
        std::fs::create_dir_all(dst).ok();
        Ok(())
    }
    pub fn pivot_root_into(&self, new_root: &Path) -> Result<()> {
        let put_old = new_root.join(".old_root");
        std::fs::create_dir_all(&put_old)
            .map_err(|e| SafeExecError::Mount(format!("failed to create put_old: {}", e)))?;

        mount(
            Some(new_root),
            new_root,
            None::<&str>,
            MsFlags::MS_BIND | MsFlags::MS_REC,
            None::<&str>,
        )
        .map_err(|e| SafeExecError::Mount(format!("self-bind-mount failed: {}", e)))?;
        Ok(())
    }
    fn bind_mount_ro(&self, src: &Path, dst: &Path) -> Result<()> {
        // First: recursive bind mount.
        mount(
            Some(src),
            dst,
            None::<&str>,
            MsFlags::MS_BIND | MsFlags::MS_REC,
            None::<&str>,
        )
        .map_err(|e| {
            SafeExecError::Mount(format!(
                "bind mount {} -> {} failed: {}",
                src.display(),
                dst.display(),
                e
            ))
        })?;

        // Second: remount as read-only.
        mount(
            None::<&str>,
            dst,
            None::<&str>,
            MsFlags::MS_REMOUNT | MsFlags::MS_BIND | MsFlags::MS_RDONLY | MsFlags::MS_REC,
            None::<&str>,
        )
        .map_err(|e| SafeExecError::Mount(format!("remount ro {} failed: {}", dst.display(), e)))
    }

    fn bind_mount_rw(&self, src: &Path, dst: &Path) -> Result<()> {
        mount(
            Some(src),
            dst,
            None::<&str>,
            MsFlags::MS_BIND | MsFlags::MS_REC,
            None::<&str>,
        )
        .map_err(|e| {
            SafeExecError::Mount(format!(
                "bind mount {} -> {} failed: {}",
                src.display(),
                dst.display(),
                e
            ))
        })
    }
}
