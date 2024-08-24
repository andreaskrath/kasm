use super::DecoderHelper;
use crate::{
    error::DecodeError,
    instruction::{Instruction, Not},
};
use std::str::SplitWhitespace;

pub struct NotDecoder;

impl NotDecoder {
    pub fn not_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let register = DecoderHelper::try_register(iter)?;
        let instruction = Not::Byte(register);

        Ok(Instruction::Not(instruction))
    }

    pub fn not_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let register = DecoderHelper::try_register(iter)?;
        let instruction = Not::Quarter(register);

        Ok(Instruction::Not(instruction))
    }

    pub fn not_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let register = DecoderHelper::try_register(iter)?;
        let instruction = Not::Half(register);

        Ok(Instruction::Not(instruction))
    }

    pub fn not_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let register = DecoderHelper::try_register(iter)?;
        let instruction = Not::Word(register);

        Ok(Instruction::Not(instruction))
    }
}
