use std::str::SplitWhitespace;
use phf::phf_map;
use crate::{constant::DecodeTable, error::DecodeError, instruction::Instruction, operand::Operand, register::Register, Processor};

pub const DECODE_TABLE: DecodeTable = phf_map! {
    "stop" => Processor::decode_stop,
};

fn get_register_and_operand_str(mut iter: SplitWhitespace) -> Result<(&str, &str), DecodeError> {
    let (Some(s_register), Some(s_operand)) = (iter.next(), iter.next()) else {
        return Err(DecodeError::IncompleteInstruction);
    };

    Ok((s_register, s_operand))
}

impl Processor {
    fn decode_stop(&mut self, _iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        Ok(Instruction::Stop)
    }
}
