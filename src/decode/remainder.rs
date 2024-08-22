use super::DecoderHelper;
use crate::{
    error::DecodeError,
    instruction::{Instruction, Remainder},
};
use std::str::SplitWhitespace;

pub struct RemDecoder;

impl RemDecoder {
    pub fn rem_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_register_and_operand(iter)?;
        let instruction = Remainder::Byte(register, operand);

        Ok(Instruction::Remainder(instruction))
    }

    pub fn rem_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_register_and_operand(iter)?;
        let instruction = Remainder::Quarter(register, operand);

        Ok(Instruction::Remainder(instruction))
    }

    pub fn rem_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_register_and_operand(iter)?;
        let instruction = Remainder::Half(register, operand);

        Ok(Instruction::Remainder(instruction))
    }

    pub fn rem_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_register_and_operand(iter)?;
        let instruction = Remainder::Word(register, operand);

        Ok(Instruction::Remainder(instruction))
    }
}
