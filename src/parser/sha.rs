use crate::linear_gkr::circuit::{Circuit, Gate, GateType};
use regex::Regex;

pub fn parse_sha(file: &str) -> Circuit {
    let mut circuit = Circuit::new();
    // Read file, parse gates using Regex, build layers
    circuit
}