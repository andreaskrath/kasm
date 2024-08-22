use super::DecoderHelper;
use crate::{
    error::DecodeError,
    instruction::{Instruction, Multiplication},
};
use std::str::SplitWhitespace;

pub struct MulDecoder;

impl MulDecoder {
    pub fn mul_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_register_and_operand(iter)?;
        let instruction = Multiplication::Byte(register, operand);

        Ok(Instruction::Multiplication(instruction))
    }

    pub fn mul_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_register_and_operand(iter)?;
        let instruction = Multiplication::Quarter(register, operand);

        Ok(Instruction::Multiplication(instruction))
    }

    pub fn mul_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_register_and_operand(iter)?;
        let instruction = Multiplication::Half(register, operand);

        Ok(Instruction::Multiplication(instruction))
    }

    pub fn mul_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_register_and_operand(iter)?;
        let instruction = Multiplication::Word(register, operand);

        Ok(Instruction::Multiplication(instruction))
    }
}
