use super::DecoderHelper;
use crate::{
    error::DecodeError,
    instruction::{Compare, Instruction},
};
use std::str::SplitWhitespace;

pub struct CompareDecoder;

impl CompareDecoder {
    pub fn compare_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (operand1, operand2) = DecoderHelper::try_double_operand(iter)?;
        let instruction = Compare::Byte(operand1, operand2);

        Ok(Instruction::Compare(instruction))
    }

    pub fn compare_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (operand1, operand2) = DecoderHelper::try_double_operand(iter)?;
        let instruction = Compare::Quarter(operand1, operand2);

        Ok(Instruction::Compare(instruction))
    }

    pub fn compare_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (operand1, operand2) = DecoderHelper::try_double_operand(iter)?;
        let instruction = Compare::Half(operand1, operand2);

        Ok(Instruction::Compare(instruction))
    }

    pub fn compare_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (operand1, operand2) = DecoderHelper::try_double_operand(iter)?;
        let instruction = Compare::Word(operand1, operand2);

        Ok(Instruction::Compare(instruction))
    }
}
