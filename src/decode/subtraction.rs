use super::DecoderHelper;
use crate::{
    error::DecodeError,
    instruction::{Instruction, Subtraction},
};
use std::str::SplitWhitespace;

pub struct SubDecoder;

impl SubDecoder {
    pub fn sub_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_reg_and_byte_operand(iter)?;
        let instruction = Subtraction::Byte(register, operand);

        Ok(Instruction::Subtraction(instruction))
    }

    pub fn sub_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_reg_and_quarter_operand(iter)?;
        let instruction = Subtraction::Quarter(register, operand);

        Ok(Instruction::Subtraction(instruction))
    }

    pub fn sub_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_reg_and_half_operand(iter)?;
        let instruction = Subtraction::Half(register, operand);

        Ok(Instruction::Subtraction(instruction))
    }

    pub fn sub_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_reg_and_word_operand(iter)?;
        let instruction = Subtraction::Word(register, operand);

        Ok(Instruction::Subtraction(instruction))
    }
}
