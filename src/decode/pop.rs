use crate::{
    error::DecodeError,
    instruction::{Instruction, Pop},
};
use std::str::SplitWhitespace;

use super::DecoderHelper;

pub struct PopDecoder;

impl PopDecoder {
    pub fn pop_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let register = DecoderHelper::try_register(iter)?;
        let instruction = Pop::Byte(register);

        Ok(Instruction::Pop(instruction))
    }

    pub fn pop_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let register = DecoderHelper::try_register(iter)?;
        let instruction = Pop::Quarter(register);

        Ok(Instruction::Pop(instruction))
    }

    pub fn pop_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let register = DecoderHelper::try_register(iter)?;
        let instruction = Pop::Half(register);

        Ok(Instruction::Pop(instruction))
    }

    pub fn pop_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let register = DecoderHelper::try_register(iter)?;
        let instruction = Pop::Word(register);

        Ok(Instruction::Pop(instruction))
    }
}
