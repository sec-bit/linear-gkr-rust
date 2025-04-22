use ark_bn254::Fr as Fp;
use ark_ff::{Field, PrimeField};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Re‑export the underlying prime field type.
pub type FieldElement = Fp;

/// Convenience trait to convert integers / strings into `FieldElement` just like the C++ ctor.
pub trait IntoField {
    fn fe(self) -> FieldElement;
}

impl IntoField for u64 {
    #[inline] fn fe(self) -> FieldElement { FieldElement::from(self) }
}
impl<'a> IntoField for &'a str {
    #[inline] fn fe(self) -> FieldElement {
        FieldElement::from_str(self).expect("invalid field literal")
    }
}

// --- helpers --------------------------------------------------------------

/// Extract bits least‑significant‐first (128 bits) – needed by the verifier to build r‑bit streams.
#[inline]
pub fn bit_stream(fe: &FieldElement, bits: usize) -> Vec<bool> {
    assert!(bits <= 256);
    let mut out = Vec::with_capacity(bits);
    // `ark_ff` exposes `into_bigint` → little‑endian limbs (u64 each)
    let limbs = fe.into_bigint().to_bits_le();
    for i in 0..bits { out.push(limbs[i]); }
    out
}