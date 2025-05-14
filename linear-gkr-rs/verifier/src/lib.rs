//! Extremely thin "always‑accept" verifier for now.
//! (Sum‑check will land next, but this lets the CLI round‑trip.)

use circuit::Circuit;
use field::FieldElement;
use prover::Prover;

pub struct Verifier;

impl Verifier {
    pub fn verify(circuit: &Circuit, prover: &Prover) -> bool {
        // Compare the prover's outputs with a fresh re‑evaluation.
        let fresh = Prover::evaluate(circuit);
        prover.outputs() == fresh.outputs()
    }
}
