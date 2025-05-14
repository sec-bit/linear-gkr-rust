//! Sum‑check verifier – Rust port of `verifier.cpp` (slow track).

use ark_ff::{One, Zero};
use circuit::{Circuit, GateType, Layer};
use field::{FieldElement, FieldExt};
use prover::Prover;
use rand::thread_rng;

/// Generate `n` random field elements
fn rand_vec(n: usize) -> Vec<FieldElement> {
    (0..n).map(|_| FieldElement::random()).collect()
}

/// Helper: β‑coefficients for a layer (Algorithm 1 in GKR papers).
struct Betas {
    /// βᵍ( r₀ )  (alpha weight)
    g_r0: Vec<FieldElement>,
    /// βᵍ( r₁ )  (beta weight)
    g_r1: Vec<FieldElement>,
    /// βᵤ( rᵤ )
    u: Vec<FieldElement>,
    /// βᵥ( rᵥ )
    v: Vec<FieldElement>,
}

impl Betas {
    fn new(
        layer: &Layer,
        prev: &Layer,
        alpha: FieldElement,
        beta: FieldElement,
        r0: &[FieldElement],
        r1: &[FieldElement],
        ru: &[FieldElement],
        rv: &[FieldElement],
    ) -> Self {
        let mut g_r0 = vec![FieldElement::zero(); 1 << layer.bit_length];
        let mut g_r1 = vec![FieldElement::zero(); 1 << layer.bit_length];
        g_r0[0] = alpha;
        g_r1[0] = beta;

        // Horner on bits of g
        for (i, (&r0_i, &r1_i)) in r0.iter().zip(r1).enumerate() {
            for j in (0..(1 << i)).rev() {
                let base0 = g_r0[j];
                let base1 = g_r1[j];
                g_r0[j] = base0 * (FieldElement::one() - r0_i);
                g_r0[j | (1 << i)] = base0 * r0_i;
                g_r1[j] = base1 * (FieldElement::one() - r1_i);
                g_r1[j | (1 << i)] = base1 * r1_i;
            }
        }

        let mut u = vec![FieldElement::zero(); 1 << prev.bit_length];
        let mut v = vec![FieldElement::zero(); 1 << prev.bit_length];
        u[0] = FieldElement::one();
        v[0] = FieldElement::one();
        for (i, (&ru_i, &rv_i)) in ru.iter().zip(rv).enumerate() {
            for j in (0..(1 << i)).rev() {
                let bu = u[j];
                let bv = v[j];
                u[j] = bu * (FieldElement::one() - ru_i);
                u[j | (1 << i)] = bu * ru_i;
                v[j] = bv * (FieldElement::one() - rv_i);
                v[j | (1 << i)] = bv * rv_i;
            }
        }

        Self { g_r0, g_r1, u, v }
    }

    fn add_value(&self, g: usize, u: usize, v: usize) -> FieldElement {
        (self.g_r0[g] + self.g_r1[g]) * self.u[u] * self.v[v]
    }

    fn mul_value(&self, g: usize, u: usize, v: usize) -> FieldElement {
        // only for MUL gates
        (self.g_r0[g] + self.g_r1[g]) * self.u[u] * self.v[v]
    }
}

pub struct Verifier;

impl Verifier {
    pub fn verify(c: &Circuit, prover: &Prover) -> bool {
        // Initial alpha=1, beta=0 as in C++ reference
        let mut alpha = FieldElement::one();
        let mut beta = FieldElement::zero();

        // r0/r1 for output layer
        let mut rng = thread_rng();
        let mut r0 = rand_vec(c.layers.last().unwrap().bit_length);
        let mut r1 = rand_vec(c.layers.last().unwrap().bit_length);

        // The verifier holds the prover's final output vector
        let mut claim =
            alpha * Self::v_res(&r0, prover.outputs()) + beta * Self::v_res(&r1, prover.outputs());

        // work from top layer down to 1
        for depth in (1..c.layers.len()).rev() {
            let layer = &c.layers[depth];
            let prev = &c.layers[depth - 1];

            // fresh ru, rv for this round
            let ru = rand_vec(prev.bit_length);
            let rv = rand_vec(prev.bit_length);

            // compute betas
            let betas = Betas::new(layer, prev, alpha, beta, &r0, &r1, &ru, &rv);

            // direct (slow) evaluation to get mult/add values
            let add_value = Self::add_sum(layer, &betas);
            let mul_value = Self::mul_sum(layer, &betas);

            // v_u and v_v from prover's taped evaluation
            let v_u = Self::v_res(&ru, &prover.values[depth - 1]);
            let v_v = Self::v_res(&rv, &prover.values[depth - 1]);

            // verify the "final" equation
            if claim != add_value * (v_u + v_v) + mul_value * v_u * v_v {
                eprintln!("final check failed at layer {depth}");
                return false;
            }

            // update claim for next iteration
            alpha = FieldElement::random_with(&mut rng);
            beta = FieldElement::random_with(&mut rng);
            claim = alpha * v_u + beta * v_v;

            // shift randomness down for next layer
            r0 = ru;
            r1 = rv;
        }

        // base layer: verify claim equals V_input(alpha, beta)
        let input_layer = &c.layers[0];
        let mut input_vals = vec![FieldElement::zero(); 1 << input_layer.bit_length];
        for (&id, gate) in &input_layer.gates {
            if id >= input_vals.len() {
                panic!(
                    "Gate ID {} out of bounds for input_vals of length {}",
                    id,
                    input_vals.len()
                );
            }
            match gate.ty {
                GateType::Input => input_vals[id] = FieldElement::from(gate.u as u64),
                GateType::Dummy => input_vals[id] = FieldElement::zero(),
                _ => panic!("only INPUT / DUMMY allowed in layer‑0"),
            }
        }
        let res0 = Self::v_res(&r0, &input_vals);
        let res1 = Self::v_res(&r1, &input_vals);

        claim == alpha * res0 + beta * res1
    }

    /// Evaluate V_output(r) (multilinear extension) by Horner.
    fn v_res(r: &[FieldElement], vector: &[FieldElement]) -> FieldElement {
        let mut cur = vector.to_vec();
        let mut len = cur.len();
        let mut step = 0;
        while len > 1 {
            for j in 0..(len / 2) {
                cur[j] = cur[2 * j] * (FieldElement::one() - r[step]) + cur[2 * j + 1] * r[step];
            }
            len /= 2;
            step += 1;
        }
        cur[0]
    }

    /// ∑_{add gates} βg βu βv
    fn add_sum(layer: &Layer, betas: &Betas) -> FieldElement {
        let mut acc = FieldElement::zero();
        for (&g, gate) in &layer.gates {
            if gate.ty == GateType::Add {
                acc += betas.add_value(g, gate.u, gate.v);
            }
        }
        acc
    }
    /// ∑_{mul gates} βg βu βv
    fn mul_sum(layer: &Layer, betas: &Betas) -> FieldElement {
        let mut acc = FieldElement::zero();
        for (&g, gate) in &layer.gates {
            if gate.ty == GateType::Mul {
                acc += betas.mul_value(g, gate.u, gate.v);
            }
        }
        acc
    }
}
