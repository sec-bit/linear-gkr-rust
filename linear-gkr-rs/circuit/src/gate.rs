use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum GateType {
    Add = 0,
    Mul = 1,
    Dummy = 2,
    Input = 3,
    DirectRelay = 4,
    Sum = 5,
    Not = 6,
    Minus = 7,
    Xor = 8,
    Naab = 9,
    Relay = 10,
}

impl TryFrom<u8> for GateType {
    type Error = String;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        use GateType::*;
        Ok(match v {
            0 => Add,
            1 => Mul,
            2 => Dummy,
            3 => Input,
            4 => DirectRelay,
            5 => Sum,
            6 => Not,
            7 => Minus,
            8 => Xor,
            9 => Naab,
            10 => Relay,
            _ => return Err(format!("unknown gate type {v}")),
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Gate {
    pub ty: GateType,
    /// first input (meaning depends on gate type)
    pub u: usize,
    /// second input (meaning depends on gate type)
    pub v: usize,
}
