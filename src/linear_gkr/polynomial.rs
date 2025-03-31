use crate::linear_gkr::field::PrimeFieldElement;

pub struct Polynomial(Vec<PrimeFieldElement>);

impl Polynomial {
    pub fn new(coeffs: Vec<PrimeFieldElement>) -> Self {
        Self(coeffs)
    }
    // pub fn evaluate(&self, x: &PrimeFieldElement) -> PrimeFieldElement {
    //     self.0.iter().rev().fold(PrimeFieldElement::new(0.into()), |acc, coeff| {
    //         acc.mul(x).add(coeff)
    //     })
    // }
}