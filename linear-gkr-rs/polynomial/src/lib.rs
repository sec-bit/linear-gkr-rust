//! Port of `polynomial.cpp` – multivariate polynomials of degree ≤ 5 used by the GKR prover.

use field::FieldElement as F;
use std::ops::{Add, AddAssign};

/// a + b·x
#[derive(Clone, Copy, Debug, Default)]
pub struct Linear {
    pub a: F,
    pub b: F,
}
impl Linear {
    #[inline]
    pub fn eval(&self, x: F) -> F {
        self.a + self.b * x
    }
}
impl Add for Linear {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}
impl AddAssign for Linear {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

/// a + b·x + c·x²
#[derive(Clone, Copy, Debug, Default)]
pub struct Quadratic {
    pub a: F,
    pub b: F,
    pub c: F,
}
impl Quadratic {
    #[inline]
    pub fn eval(&self, x: F) -> F {
        self.a + x * (self.b + self.c * x)
    }
}
impl Add for Quadratic {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
            c: self.c + rhs.c,
        }
    }
}
impl AddAssign for Quadratic {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

/// cubic: a + b·x + c·x² + d·x³
#[derive(Clone, Copy, Debug, Default)]
pub struct Cubic {
    pub a: F,
    pub b: F,
    pub c: F,
    pub d: F,
}
impl Cubic {
    #[inline]
    pub fn eval(&self, x: F) -> F {
        // Horner
        (((self.d * x) + self.c) * x + self.b) * x + self.a
    }
}
impl Add for Cubic {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
            c: self.c + rhs.c,
            d: self.d + rhs.d,
        }
    }
}
impl AddAssign for Cubic {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

/// quartic: a + b·x + c·x² + d·x³ + e·x⁴
#[derive(Clone, Copy, Debug, Default)]
pub struct Quadruple {
    pub a: F,
    pub b: F,
    pub c: F,
    pub d: F,
    pub e: F,
}
impl Quadruple {
    #[inline]
    pub fn eval(&self, x: F) -> F {
        ((((self.e * x) + self.d) * x + self.c) * x + self.b) * x + self.a
    }
}
impl Add for Quadruple {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
            c: self.c + rhs.c,
            d: self.d + rhs.d,
            e: self.e + rhs.e,
        }
    }
}
impl AddAssign for Quadruple {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

/// quintic: a + b·x + c·x² + d·x³ + e·x⁴ + f·x⁵
#[derive(Clone, Copy, Debug, Default)]
pub struct Quintuple {
    pub a: F,
    pub b: F,
    pub c: F,
    pub d: F,
    pub e: F,
    pub f: F,
}
impl Quintuple {
    #[inline]
    pub fn eval(&self, x: F) -> F {
        (((((self.f * x) + self.e) * x + self.d) * x + self.c) * x + self.b) * x + self.a
    }
}
impl Add for Quintuple {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
            c: self.c + rhs.c,
            d: self.d + rhs.d,
            e: self.e + rhs.e,
            f: self.f + rhs.f,
        }
    }
}
impl AddAssign for Quintuple {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
