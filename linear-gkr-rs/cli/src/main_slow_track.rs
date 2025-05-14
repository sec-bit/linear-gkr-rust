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

    println!("Loading circuit from: {}", opts.circuit);
    let circuit = match load_from_path(&opts.circuit) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to load circuit: {}", e);
            std::process::exit(1);
        }
    };
    println!("Circuit loaded successfully:");
    println!("  - Number of layers: {}", circuit.layers.len());
    for (i, layer) in circuit.layers.iter().enumerate() {
        println!(
            "  - Layer {}: {} gates, bit_length = {}",
            i,
            layer.gates.len(),
            layer.bit_length
        );
    }

    println!("\nStarting prover evaluation...");
    let prover = Prover::evaluate(&circuit);
    println!("Prover evaluation complete:");
    println!("  - Number of layers evaluated: {}", prover.values.len());
    for (i, layer) in prover.values.iter().enumerate() {
        println!("  - Layer {}: {} values", i, layer.len());
    }

    println!("\nStarting verifier...");
    let ok = Verifier::verify(&circuit, &prover);
    println!(
        "Verification {}",
        if ok { "successful! ✅" } else { "failed! ❌" }
    );

    if !ok {
        std::process::exit(1);
    }
    Ok(())
}
