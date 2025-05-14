use circuit::load_from_path;
use clap::Parser;
use prover::Prover;
use verifier::Verifier;

/// Replacement for C++ `main_slow_track.cpp`.
#[derive(Parser)]
struct Opts {
    /// Path to circuit text file
    #[clap(long)]
    circuit: String,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    let circuit = load_from_path(&opts.circuit)?;
    let prover = Prover::evaluate(&circuit);
    let ok = Verifier::verify(&circuit, &prover);

    println!("{}", if ok { "Pass ✅" } else { "Fail ❌" });
    Ok(())
}
