//! Public API for circuits + parsing utilities.

mod gate;
mod parse;

pub use gate::{Gate, GateType};
pub use parse::{load_from_path, Circuit, Layer};

/// Top‑level error type for the crate.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(String),
}
