use clap::Parser;
use nix::libc;
use safeexec::cli::Args;
use safeexec::error::Result;
use safeexec::runtime::RuntimeOrchestrator;

fn main() -> Result<()> {
    let args = Args::parse();

    if unsafe { libc::geteuid() } != 0 {
        eprintln!("warning: safeexec requires root or CAP_SYS_ADMIN for namespace operations.");
    }

    let orchestrator = RuntimeOrchestrator::new(args)?;

    match orchestrator.run() {
        Ok(()) => {
            println!("Safeexec completed successfully");
            Ok(())
        }
        Err(e) => {
            eprintln!("x SafeExec failed: {}", e);
            Err(e)
        }
    }
}
