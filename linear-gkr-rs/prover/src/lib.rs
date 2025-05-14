//! “Slow‑track” prover: brute‑force evaluation of every gate in the circuit.

use ark_ff::{One, Zero};
use circuit::{Circuit, GateType};
use field::FieldElement;

pub struct Prover {
    /// values[layer][gate_id] → field element
    pub values: Vec<Vec<FieldElement>>,
}

impl Prover {
    /// Evaluate every gate of `c` and store intermediate values.
    pub fn evaluate(c: &Circuit) -> Self {
        let mut values: Vec<Vec<FieldElement>> = Vec::with_capacity(c.layers.len());

        // ──────────────── layer‑0 (public inputs / dummies) ────────────────
        let input_layer = &c.layers[0];
        let max_id = input_layer.gates.keys().max().copied().unwrap_or(0);
        let mut layer0 = vec![FieldElement::zero(); max_id + 1];

        for (&id, gate) in input_layer.gates.iter() {
            layer0[id] = match gate.ty {
                GateType::Input => FieldElement::from(gate.u as u64),
                GateType::Dummy => FieldElement::zero(),
                _ => panic!("only INPUT / DUMMY allowed in layer‑0"),
            };
        }
        values.push(layer0);

        // ─────────────── subsequent layers ───────────────
        for (i, layer) in c.layers.iter().enumerate().skip(1) {
            let max_id = layer.gates.keys().max().copied().unwrap_or(0);
            let mut cur = vec![FieldElement::zero(); max_id + 1];

            for (&id, gate) in layer.gates.iter() {
                let out = match gate.ty {
                    GateType::Add => values[i - 1][gate.u] + values[i - 1][gate.v],
                    GateType::Mul => values[i - 1][gate.u] * values[i - 1][gate.v],
                    GateType::Dummy => FieldElement::zero(),
                    GateType::Input => FieldElement::from(gate.u as u64),
                    GateType::DirectRelay | GateType::Relay => values[i - 1][gate.u],
                    GateType::Sum => {
                        let mut acc = FieldElement::zero();
                        for idx in gate.u..gate.v {
                            acc += &values[i - 1][idx];
                        }
                        acc
                    }
                    GateType::Not => FieldElement::one() - values[i - 1][gate.u],
                    GateType::Minus => values[i - 1][gate.u] - values[i - 1][gate.v],
                    GateType::Xor => {
                        let x = values[i - 1][gate.u];
                        let y = values[i - 1][gate.v];
                        x + y - FieldElement::from(2u64) * x * y
                    }
                    GateType::Naab => {
                        let x = values[i - 1][gate.u];
                        let y = values[i - 1][gate.v];
                        y - x * y
                    }
                };
                cur[id] = out;
            }
            values.push(cur);
        }

        Self { values }
    }

    /// Convenience accessor: returns the final layer's values.
    pub fn outputs(&self) -> &[FieldElement] {
        self.values.last().expect("at least one layer")
    }
}
