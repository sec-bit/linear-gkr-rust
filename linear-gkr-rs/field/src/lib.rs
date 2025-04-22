//! Thin wrapper around ark‑bn254::Fr providing the Public API expected by the rest of linear‑gkr.
//! We extend the external type via a trait instead of an inherent impl (which Rust forbids
//! for types defined in other crates).

use ark_bn254::Fr as Fp;
use ark_ff::{PrimeField, UniformRand};
use ark_serialize::CanonicalSerialize;
use ark_std::rand::{thread_rng, RngCore};
use std::str::FromStr;
use ark_ff::BigInteger;

/// Public re‑export so the rest of the workspace can `use field::FieldElement`.
pub type FieldElement = Fp;

// -----------------------------------------------------------------------------
// Convenience conversions (mimic C++ ctor overloads)
// -----------------------------------------------------------------------------

pub trait IntoField {
    fn fe(self) -> FieldElement;
}

impl IntoField for u64 {
    #[inline]
    fn fe(self) -> FieldElement {
        FieldElement::from(self)
    }
}

impl<'a> IntoField for &'a str {
    #[inline]
    fn fe(self) -> FieldElement {
        FieldElement::from_str(self).expect("invalid field literal")
    }
}

// -----------------------------------------------------------------------------
// Extension trait – adds helpers to FieldElement safely
// -----------------------------------------------------------------------------

pub trait FieldExt: PrimeField + UniformRand + CanonicalSerialize {
    /// Cryptographically random element using thread‑local RNG.
    fn random() -> Self {
        Self::rand(&mut thread_rng())
    }

    /// Random element with caller‑supplied RNG.
    fn random_with<R: RngCore + ?Sized>(rng: &mut R) -> Self {
        Self::rand(rng)
    }

    /// Least‑significant `bits` as little‑endian booleans.
    fn to_bits_le(&self, bits: usize) -> Vec<bool> {
        bit_stream(self, bits)
    }

    /// 32‑byte canonical little‑endian representation (compressed).
    fn to_le_bytes(&self) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        self.serialize_compressed(&mut bytes[..]).unwrap();
        bytes
    }
}

impl FieldExt for Fp {}

// -----------------------------------------------------------------------------
// Standalone helper – needed by verifier, etc.
// -----------------------------------------------------------------------------

#[inline]
pub fn bit_stream<F : PrimeField>(fe: &F, bits: usize) -> Vec<bool> {
    assert!(bits <= 256);
    let mut out = Vec::with_capacity(bits);
    let limbs = fe.into_bigint().to_bits_le();
    out.extend(limbs.iter().copied().take(bits));
    out
}