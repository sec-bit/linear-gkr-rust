use clap::Parser;
use std::path::PathBuf;

/// Reference (slow) prover + verifier – good for correctness tests
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Circuit description file (txt)
    #[arg(short, long)]
    circuit: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if !args.circuit.exists() {
        anyhow::bail!("circuit file {:?} does not exist", args.circuit);
    }
    println!(
        "[slow_track] would verify circuit {:?} – logic not implemented yet",
        args.circuit
    );
    Ok(())
}