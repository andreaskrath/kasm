use std::str::SplitWhitespace;
use phf::phf_map;
use crate::{constant::DecodeTable, error::DecodeError, instruction::Instruction, operand::Operand, register::Register, Processor};

pub const DECODE_TABLE: DecodeTable = phf_map! {
    "stop" => Processor::decode_stop,
};

impl Processor {
    fn decode_stop(&mut self, _iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        Ok(Instruction::Stop)
    }
}
