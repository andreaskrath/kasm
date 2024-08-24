use super::DecoderHelper;
use crate::{
    error::DecodeError,
    instruction::{Instruction, Test},
};
use std::str::SplitWhitespace;

pub struct TestDecoder;

impl TestDecoder {
    pub fn test_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (operand1, operand2) = DecoderHelper::try_double_operand(iter)?;
        let instruction = Test::Byte(operand1, operand2);

        Ok(Instruction::Test(instruction))
    }

    pub fn test_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (operand1, operand2) = DecoderHelper::try_double_operand(iter)?;
        let instruction = Test::Quarter(operand1, operand2);

        Ok(Instruction::Test(instruction))
    }

    pub fn test_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (operand1, operand2) = DecoderHelper::try_double_operand(iter)?;
        let instruction = Test::Half(operand1, operand2);

        Ok(Instruction::Test(instruction))
    }

    pub fn test_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (operand1, operand2) = DecoderHelper::try_double_operand(iter)?;
        let instruction = Test::Word(operand1, operand2);

        Ok(Instruction::Test(instruction))
    }
}
