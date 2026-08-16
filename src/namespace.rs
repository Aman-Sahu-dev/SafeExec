use crate::error::{Result, SafeExecError};
use crate::sync_primitives::SyncBarrier;
use nix::sched::{CloneFlags, clone};
use nix::sys::prctl;
use nix::sys::signal::Signal;
use nix::sys::wait::{WaitStatus, waitpid};
use nix::unistd::{self, Gid, Pid, Uid};
use std::ffi::CString;
use std::path::Path;

/// All data the child needs to execute after clone.
///
/// Packaged into a struct so the closure can move it cleanly into the
/// child's address space without borrow checker lifetime issues.
pub struct ChildContext {
    pub binary: CString,
    pub args: Vec<CString>,
    pub envp: Vec<CString>,
    pub hostname: Option<CString>,
    pub sync: SyncBarrier,
}

impl ChildContext {
    /// The child entry point. Never returns on success (execve takes over).
    fn run(&self) -> isize {
        // 1. If parent dies, we die — prevents orphan cgroups and zombie namespaces.
        if let Err(e) = prctl::set_pdeathsig(Signal::SIGKILL) {
            eprintln!("[child] prctl(PR_SET_PDEATHSIG) failed: {}", e);
            return 1;
        }

        // 2. Close parent-side pipe descriptors.
        if let Err(e) = self.sync.close_parent_descriptors_in_child() {
            eprintln!("[child] close_parent_descriptors_in_child failed: {}", e);
            return 1;
        }

        // 3. Inside the new user namespace we have full capabilities.
        //    Become root in the namespace so later phases (pivot_root) succeed.
        if let Err(e) = unistd::setuid(Uid::from_raw(0)) {
            eprintln!("[child] setuid(0) failed: {}", e);
            return 1;
        }
        if let Err(e) = unistd::setgid(Gid::from_raw(0)) {
            eprintln!("[child] setgid(0) failed: {}", e);
            return 1;
        }

        // 4. Signal parent that we are ready for uid_map / gid_map.
        if let Err(e) = self.sync.signal_ready() {
            eprintln!("[child] signal_ready failed: {}", e);
            return 1;
        }

        // 5. Block until parent has written uid_map / gid_map.
        if let Err(e) = self.sync.wait_for_parent_continue() {
            eprintln!("[child] wait_for_parent_continue failed: {}", e);
            return 1;
        }

        // 6. Set hostname if UTS namespace was requested.
        if let Some(ref h) = self.hostname {
            if let Err(e) = unistd::sethostname(h) {
                eprintln!("[child] sethostname failed: {}", e);
                return 1;
            }
        }

        // 7. Replace process image with target binary.
        //    execve only returns on failure.
        if let Err(e) = unistd::execve(&self.binary, &self.args, &self.envp) {
            eprintln!("[child] execve failed: {}", e);
            return 127;
        }

        unreachable!("execve should never return on success")
    }
}

/// Controller for Linux namespace creation.
pub struct NamespaceController;

impl NamespaceController {
    pub fn new() -> Self {
        Self
    }

    /// Build the clone flags for full container isolation.
    pub fn build_clone_flags() -> CloneFlags {
        CloneFlags::CLONE_NEWPID
            | CloneFlags::CLONE_NEWNS
            | CloneFlags::CLONE_NEWUTS
            | CloneFlags::CLONE_NEWUSER
            | CloneFlags::CLONE_NEWNET
    }

    /// Spawn a new container init process.
    ///
    /// # Safety
    /// This calls the raw `clone(2)` syscall. The callback runs in the child
    /// address space on the provided stack. All captured data is moved into
    /// the child's copy-on-write memory image.
    pub unsafe fn spawn_container_init(
        &self,
        flags: CloneFlags,
        stack: &mut [u8],
        ctx: ChildContext,
    ) -> Result<Pid> {
        let callback = move || ctx.run();

        let pid = clone(callback, stack, flags, Some(Signal::SIGCHLD))?;

        Ok(pid)
    }

    /// Write UID and GID maps so that container root (0) maps to the host UID.
    pub fn write_uid_gid_map(&self, pid: Pid, host_uid: u32, host_gid: u32) -> Result<()> {
        let uid_map = format!("0 {} 1\n", host_uid);
        let gid_map = format!("0 {} 1\n", host_gid);

        let uid_path = format!("/proc/{}/uid_map", pid);
        let gid_path = format!("/proc/{}/gid_map", pid);

        std::fs::write(&uid_path, uid_map).map_err(|e| {
            SafeExecError::Namespace(format!("failed to write uid_map to {}: {}", uid_path, e))
        })?;

        // Writing gid_map requires disabling setgroups first.
        let setgroups_path = format!("/proc/{}/setgroups", pid);
        let _ = std::fs::write(&setgroups_path, "deny");

        std::fs::write(&gid_path, gid_map).map_err(|e| {
            SafeExecError::Namespace(format!("failed to write gid_map to {}: {}", gid_path, e))
        })?;

        Ok(())
    }
}

/// Allocate a stack suitable for `clone(2)`.
///
/// Returns a Vec<u8> backed by anonymous mmap with MAP_STACK hint.
pub fn allocate_clone_stack(size: usize) -> Vec<u8> {
    // Vec<u8> with exact capacity acts as our stack buffer.
    // The kernel will use the highest address as the initial stack pointer.
    // For production, you might prefer `libc::mmap` directly to get MAP_STACK,
    // but a large Vec is sufficient for PBL and avoids unsafe mmap setup.
    vec![0u8; size]
}
