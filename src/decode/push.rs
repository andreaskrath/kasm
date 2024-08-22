use super::DecoderHelper;
use crate::{
    error::DecodeError,
    instruction::{Instruction, Push},
};
use std::str::SplitWhitespace;

pub struct PushDecoder;

impl PushDecoder {
    pub fn push_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let operand = DecoderHelper::try_operand(iter)?;
        let instruction = Push::Byte(operand);

        Ok(Instruction::Push(instruction))
    }

    pub fn push_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let operand = DecoderHelper::try_operand(iter)?;
        let instruction = Push::Quarter(operand);

        Ok(Instruction::Push(instruction))
    }

    pub fn push_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let operand = DecoderHelper::try_operand(iter)?;
        let instruction = Push::Half(operand);

        Ok(Instruction::Push(instruction))
    }

    pub fn push_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let operand = DecoderHelper::try_operand(iter)?;
        let instruction = Push::Word(operand);

        Ok(Instruction::Push(instruction))
    }
}
