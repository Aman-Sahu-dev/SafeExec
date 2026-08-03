use Safeexec::cli::{Args, TheaterMode};
use Safeexec::error::Result;
use Safeexec::runtime::RuntimeOrchestrator;
use clap::Parser;
use nix::libc;

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
