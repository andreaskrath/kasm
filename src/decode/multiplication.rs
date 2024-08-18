use super::{
    decode_reg_and_byte_operand, decode_reg_and_half_operand, decode_reg_and_quarter_operand,
    decode_reg_and_word_operand,
};
use crate::{
    error::DecodeError,
    instruction::{Instruction, Multiplication},
};
use std::str::SplitWhitespace;

pub struct MulDecoder;

impl MulDecoder {
    pub fn mul_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_byte_operand(iter)?;
        let instruction = Multiplication::Byte(register, operand);

        Ok(Instruction::Multiplication(instruction))
    }

    pub fn mul_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_quarter_operand(iter)?;
        let instruction = Multiplication::Quarter(register, operand);

        Ok(Instruction::Multiplication(instruction))
    }

    pub fn mul_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_half_operand(iter)?;
        let instruction = Multiplication::Half(register, operand);

        Ok(Instruction::Multiplication(instruction))
    }

    pub fn mul_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_word_operand(iter)?;
        let instruction = Multiplication::Word(register, operand);

        Ok(Instruction::Multiplication(instruction))
    }
}
