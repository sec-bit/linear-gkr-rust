use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Read};

use crate::gate::{Gate, GateType};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("parse error: {0}")]
    Parse(&'static str),
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("int error")]
    Int(#[from] std::num::ParseIntError),
}

/// split on ASCII‑white and yield successive tokens
fn tokens<R: Read>(r: R) -> impl Iterator<Item = String> {
    BufReader::new(r)
        .lines()
        .filter_map(|l| l.ok())
        .flat_map(|l| l.split_whitespace().map(str::to_owned).collect::<Vec<_>>())
}

pub struct Circuit {
    pub layers: Vec<Layer>,
}

pub struct Layer {
    pub gates: BTreeMap<usize, Gate>,
    pub bit_length: usize,
}

pub fn load_from_path<P: AsRef<std::path::Path>>(p: P) -> Result<Circuit, Error> {
    let file = std::fs::File::open(p)?;
    let mut tok = tokens(file);

    let depth: usize = tok.next().ok_or(Error::Parse("missing depth"))?.parse()?;

    let mut layers = Vec::with_capacity(depth);

    for _ in 0..depth {
        let n: usize = tok
            .next()
            .ok_or(Error::Parse("layer header missing"))?
            .parse()?;

        let mut gates = BTreeMap::<usize, Gate>::new();
        let mut max_id = 0usize;
        for _ in 0..n {
            // each gate = 4 ints
            let ty: i32 = tok.next().ok_or(Error::Parse("ty"))?.parse()?;
            let g: usize = tok.next().ok_or(Error::Parse("g"))?.parse()?;
            let u: usize = tok.next().ok_or(Error::Parse("u"))?.parse()?;
            let v: usize = tok.next().ok_or(Error::Parse("v"))?.parse()?;

            let gate_type = match ty {
                0 => GateType::Input,
                1 => GateType::Add,
                2 => GateType::Mul,
                3 => GateType::Dummy,
                _ => return Err(Error::Parse("unknown gate type")),
            };

            gates.insert(
                g,
                Gate {
                    ty: gate_type,
                    u,
                    v,
                },
            );
            max_id = max_id.max(g);
        }

        // same rule as C++: bit_length = ⌈log₂(max_id+1)⌉
        let bit_length = if n == 1 {
            1 // will pad with dummy later
        } else {
            let bl = (max_id.next_power_of_two()).trailing_zeros() as usize;
            if bl == 0 {
                1
            } else {
                bl
            }
        };

        layers.push(Layer { gates, bit_length });
    }

    Ok(Circuit { layers })
}
