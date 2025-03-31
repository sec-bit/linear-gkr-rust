use crate::linear_gkr::field::PrimeFieldElement;

#[derive(Clone)]
pub enum GateType {
    Add, Mul, Input, Dummy, // Add others like Xor, Not as needed
}

pub struct Gate {
    ty: GateType,
    id: usize,
    inputs: (usize, usize), // Indices to previous layer
}

pub struct Circuit {
    layers: Vec<Vec<Gate>>,
}

impl Circuit {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }
    pub fn add_layer(&mut self, gates: Vec<Gate>) {
        self.layers.push(gates);
    }
}