// Rust port of the "slow" C++ prover evaluator.

use ark_ff::{One, Zero};
use circuit::{Circuit, Gate, GateType};
use field::FieldElement;

pub struct Prover {
    pub values: Vec<Vec<FieldElement>>, // [layer][gate_id] -> field element
}

impl Prover {
    pub fn evaluate(c: &Circuit) -> Self {
        let mut values: Vec<Vec<FieldElement>> = Vec::with_capacity(c.layers.len());

        // ---------- layer 0 (inputs) ----------
        let mut layer0 = vec![FieldElement::zero(); 1 << c.layers[0].bit_length];
        for (id, gate) in c.layers[0].gates.iter().enumerate() {
            layer0[id] = match gate.ty {
                GateType::Input => FieldElement::from(gate.u as u64),
                GateType::Dummy => FieldElement::zero(),
                _ => panic!("only INPUT / DUMMY allowed in layer‑0"),
            };
        }
        values.push(layer0);

        // ---------- subsequent layers ----------
        for (i, layer) in c.layers.iter().enumerate().skip(1) {
            let mut cur = vec![FieldElement::zero(); 1 << layer.bit_length];
            for (id, gate) in layer.gates.iter().enumerate() {
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

    /// Convenience: return the output layer.
    pub fn outputs(&self) -> &[FieldElement] {
        self.values.last().unwrap()
    }
}
