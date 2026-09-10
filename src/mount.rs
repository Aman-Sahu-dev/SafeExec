use crate::error::{Result, SafeExecError};
use nix::libc::{SYS_rmdir, rmdir};
use nix::mount::{MntFlags, MsFlags, mount, umount2};
use nix::unistd::chdir;
use std::fmt::Result;
use std::os::unix::fs::PermissionsExt;
use std::path::Component::Prefix;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

pub struct VfsManager;

impl VfsManager {
    pub fn new() -> Self {
        Self
    }
    pub fn allocate_wokspace(&self,on_id: &str) -> Result<tempfile::TempDir> {
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
    
    pub fn bind_mount_output(&self,src: &Path,dst: &Path) -> Result<()>{
        std::fs::create_dir_all(dst).ok();
        self.bind_mount_rw(src,dst)
    } 
    pub fn pivot_root_into(&self, new_root: &Path) -> Result<()>{
        let put_old = new_root.join(".old_root");
        std::fs::create_dir_all(&put_old).map_err(|e| {
            SafeExecError::Mount(fromat!("failed to create put_old: {}"),e)
        })?;

        mount(
            Some(new_root),
            new_root,
            None::<&str>,
            MsFlags::MS_BIND | MsFlags::MS_REC,
            None<&str>,
            ).map_err(|e| SafeExecError::Mount(format!("self-bind-mount failed: {}"),e))?;
        Mount(
            None::<&str>,
            Path::new("/"),
            None::<&str>,
            MsFlags::MS_PRIVATE | MsFlags::MS_REC,
            None<&str>,
            ).map_err(|e| SafeExecError::Mount(format!("make private failed: {}"),e))?;
        chdir("/").map_err(SafeExecError::Mount(format!("chdir / failed : {}",e)))?;
        umount2(".old_root", MntFlags::MNT_DETACH).map_err(SafeExecError::Mount(format!("umount2 failed: {}",e))?;
        std::fs::remove_dir(".old_root").map_err(SafeExecError::Mount(format!("remove_dir failed: {}",e)))?;
        Ok(())
    }
}
