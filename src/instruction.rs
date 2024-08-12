use std::ops::Index;
use variant_count::VariantCount;
use crate::constant::{ExecuteFn, ExecuteTable};

/// The instruction set of the virtual processor.
#[derive(Debug, PartialEq, VariantCount)]
pub enum Instruction {
    Stop,
    SetByte,
    SetQuarter,
    SetHalf,
    SetWord,
}
