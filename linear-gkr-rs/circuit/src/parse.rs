//! Tiny parser for the text format described in README.md.

use super::{Error, Gate, GateType};
use std::{fs::File, io::{BufRead, BufReader}, path::Path};

#[derive(Clone, Debug)]
pub struct Layer {
    pub gates: Vec<Gate>,
    pub bit_length: usize, // as defined in C++ implementation
}

#[derive(Clone, Debug)]
pub struct Circuit {
    pub layers: Vec<Layer>,
}

/// Public helper – load from a file path.
pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Circuit, Error> {
    let f = File::open(path)?;
    load_from_reader(BufReader::new(f))
}

/// Core parser – consumes any `BufRead`.
pub fn load_from_reader<R: BufRead>(mut reader: R) -> Result<Circuit, Error> {
    let mut line = String::new();
    // first integer – number of layers
    reader.read_line(&mut line)?;
    let d: usize = line.trim().parse().map_err(|e| Error::Parse(format!("invalid depth: {e}")))?;
    let mut layers = Vec::with_capacity(d);
    line.clear();

    for layer_idx in 0..d {
        reader.read_line(&mut line)?;
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().ok_or_else(|| Error::Parse("missing gate count".into()))?
            .parse().map_err(|e| Error::Parse(format!("invalid gate count: {e}")))?;

        let mut gates = Vec::with_capacity(n.max(1));
        // Each gate has 4 integers: ty g u v
        let expected_ints = 1 + 4 * n; // 1 already consumed (n)
        let mut ints: Vec<i64> = Vec::with_capacity(expected_ints - 1);
        // we already consumed parts of current line; push remaining numbers then read more lines if necessary
        for p in parts { ints.push(p.parse().map_err(|e| Error::Parse(format!("{e}")))?); }
        while ints.len() < 4 * n {
            line.clear();
            let read = reader.read_line(&mut line)?;
            if read == 0 { break; }
            ints.extend(line.split_whitespace().map(|s| s.parse::<i64>()
                .map_err(|e| Error::Parse(format!("{e}"))))
                .collect::<Result<Vec<_>, _>>()?);
        }
        if ints.len() != 4 * n {
            return Err(Error::Parse(format!("layer {layer_idx}: expected {n} gates ({} ints) but found {}", n*4, ints.len())));
        }
        for i in 0..n {
            let ty = GateType::try_from(ints[4*i] as u8).map_err(Error::Parse)?;
            let g  = ints[4*i+1] as usize; // index – must equal i normally but we store in order.
            let u  = ints[4*i+2] as usize;
            let v  = ints[4*i+3] as usize;
            if g != i { // enforce contiguous order like C++ parser
                return Err(Error::Parse(format!("layer {layer_idx}: gate IDs must be sorted & dense (got {g}, expected {i})")));
            }
            gates.push(Gate { ty, u, v });
        }
        // determine bit length exactly like C++ (next pow2 then log2)
        let max_gate = if gates.len() == 1 { 1 } else { gates.len() - 1 };
        let bit_len = (usize::BITS - (max_gate as u32).leading_zeros()) as usize;
        layers.push(Layer { gates, bit_length: bit_len });
        line.clear();
    }
    Ok(Circuit { layers })
}