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
    AddByte,
    AddQuarter,
    AddHalf,
    AddWord,
    SubByte,
    SubQuarter,
    SubHalf,
    SubWord,
    MulByte,
    MulQuarter,
}

impl Index<Instruction> for ExecuteTable {
    type Output = ExecuteFn;

    fn index(&self, index: Instruction) -> &Self::Output {
        &self[index as usize]
    }
}
