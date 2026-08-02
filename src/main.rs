use clap::Parser;
use nix::libc;
use safeexec::cli::{Args, TheaterMode};
use safeexec::error::Result;
use safeexec::runtime::RuntimeOrchestrator;

fn main() -> Result<()> {
    let args = Args::parse();

    if unsafe { libc::geteuid() } != 0 {
        eprintln!("warning:  safeexec requires root or Cap_SYS_ADMIN for namspace operations. ");

        let orchastrator = RuntimeOrchestrator::new(args)?;

        match orchastrator.run() {
            OK(()) => {
                println!("Safeexec completed successfully");
                OK(())
            }
            Err(e) => {
                eprintln!("x SafeExec failed{}", e);
                Err(e)
            }
        }
    }
}
