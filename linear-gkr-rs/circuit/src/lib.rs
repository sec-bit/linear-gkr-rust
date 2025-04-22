//! Public API for circuits + parsing utilities.

mod gate;
mod parse;

pub use gate::{Gate, GateType};
pub use parse::{Circuit, Layer};

/// Re‑export parsing helpers so CLI can `use circuit::load_from_path`.
pub use parse::{load_from_path, load_from_reader};

/// Top‑level error type for the crate.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(String),
}

//! Layer‑based circuit description + parsing from the `README` file format.

mod parse;
mod gate;

pub use gate::{Gate, GateType};

/// Public error type.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(String),
}