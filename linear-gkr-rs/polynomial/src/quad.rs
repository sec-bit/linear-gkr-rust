//! Lightweight polynomial structs (degree 1 → 5) used by the verifier / prover.
//! No fancy FFTs—just coefficient storage and evaluation.

use field::FieldElement;
use std::ops::{Add, AddAssign, Mul};

/// a  x + b
#[derive(Clone, Copy, Debug, Default)]
pub struct LinearPoly {
    pub a: FieldElement,
    pub b: FieldElement,
}

impl LinearPoly {
    #[inline]
    pub fn new(a: FieldElement, b: FieldElement) -> Self {
        Self { a, b }
    }
    #[inline]
    pub fn eval(&self, x: FieldElement) -> FieldElement {
        self.a * x + self.b
    }
}
impl Add for LinearPoly {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.a + rhs.a, self.b + rhs.b)
    }
}
impl AddAssign for LinearPoly {
    fn add_assign(&mut self, rhs: Self) {
        self.a += rhs.a;
        self.b += rhs.b;
    }
}
impl Mul for LinearPoly {
    type Output = QuadraticPoly;
    fn mul(self, rhs: Self) -> Self::Output {
        // (a₁x+b₁)(a₂x+b₂) = (a₁a₂)x² + (a₁b₂+a₂b₁)x + b₁b₂
        QuadraticPoly::new(
            self.a * rhs.a,
            self.a * rhs.b + rhs.a * self.b,
            self.b * rhs.b,
        )
    }
}

/// a  x² + b  x + c
#[derive(Clone, Copy, Debug, Default)]
pub struct QuadraticPoly {
    pub a: FieldElement,
    pub b: FieldElement,
    pub c: FieldElement,
}

impl QuadraticPoly {
    #[inline]
    pub const fn new(a: FieldElement, b: FieldElement, c: FieldElement) -> Self {
        Self { a, b, c }
    }
    #[inline]
    pub fn eval(&self, x: FieldElement) -> FieldElement {
        (self.a * x + self.b) * x + self.c
    }
}
impl Add for QuadraticPoly {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.a + rhs.a, self.b + rhs.b, self.c + rhs.c)
    }
}
impl AddAssign for QuadraticPoly {
    fn add_assign(&mut self, rhs: Self) {
        self.a += rhs.a;
        self.b += rhs.b;
        self.c += rhs.c;
    }
}

/// a  x⁵ + b  x⁴ + c  x³ + d  x² + e  x + f
#[derive(Clone, Copy, Debug, Default)]
pub struct QuintuplePoly {
    pub a: FieldElement,
    pub b: FieldElement,
    pub c: FieldElement,
    pub d: FieldElement,
    pub e: FieldElement,
    pub f: FieldElement,
}

impl QuintuplePoly {
    #[inline]
    pub const fn new(
        a: FieldElement,
        b: FieldElement,
        c: FieldElement,
        d: FieldElement,
        e: FieldElement,
        f: FieldElement,
    ) -> Self {
        Self { a, b, c, d, e, f }
    }
    #[inline]
    pub fn eval(&self, x: FieldElement) -> FieldElement {
        ((((self.a * x + self.b) * x + self.c) * x + self.d) * x + self.e) * x + self.f
    }
}
impl Add for QuintuplePoly {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.a + rhs.a,
            self.b + rhs.b,
            self.c + rhs.c,
            self.d + rhs.d,
            self.e + rhs.e,
            self.f + rhs.f,
        )
    }
}
impl AddAssign for QuintuplePoly {
    fn add_assign(&mut self, rhs: Self) {
        self.a += rhs.a;
        self.b += rhs.b;
        self.c += rhs.c;
        self.d += rhs.d;
        self.e += rhs.e;
        self.f += rhs.f;
    }
}
