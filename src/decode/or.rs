use super::DecoderHelper;
use crate::{
    error::DecodeError,
    instruction::{Instruction, Or},
};
use std::str::SplitWhitespace;

pub struct OrDecoder;

impl OrDecoder {
    pub fn or_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_register_and_operand(iter)?;
        let instruction = Or::Byte(register, operand);

        Ok(Instruction::Or(instruction))
    }

    pub fn or_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_register_and_operand(iter)?;
        let instruction = Or::Quarter(register, operand);

        Ok(Instruction::Or(instruction))
    }

    pub fn or_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_register_and_operand(iter)?;
        let instruction = Or::Half(register, operand);

        Ok(Instruction::Or(instruction))
    }

    pub fn or_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_register_and_operand(iter)?;
        let instruction = Or::Word(register, operand);

        Ok(Instruction::Or(instruction))
    }
}
