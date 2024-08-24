use super::DecoderHelper;
use crate::{
    error::DecodeError,
    instruction::{And, Instruction},
};
use std::str::SplitWhitespace;

pub struct AndDecoder;

impl AndDecoder {
    pub fn and_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_register_and_operand(iter)?;
        let instruction = And::Byte(register, operand);

        Ok(Instruction::And(instruction))
    }

    pub fn and_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_register_and_operand(iter)?;
        let instruction = And::Quarter(register, operand);

        Ok(Instruction::And(instruction))
    }

    pub fn and_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_register_and_operand(iter)?;
        let instruction = And::Half(register, operand);

        Ok(Instruction::And(instruction))
    }

    pub fn and_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_register_and_operand(iter)?;
        let instruction = And::Word(register, operand);

        Ok(Instruction::And(instruction))
    }
}
