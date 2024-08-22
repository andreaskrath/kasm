use super::DecoderHelper;
use crate::{
    error::DecodeError,
    instruction::{Instruction, Set},
};
use std::str::SplitWhitespace;

pub struct SetDecoder;

impl SetDecoder {
    pub fn set_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_register_and_operand(iter)?;
        let instruction = Set::Byte(register, operand);

        Ok(Instruction::Set(instruction))
    }

    pub fn set_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_register_and_operand(iter)?;
        let instruction = Set::Quarter(register, operand);

        Ok(Instruction::Set(instruction))
    }

    pub fn set_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_register_and_operand(iter)?;
        let instruction = Set::Half(register, operand);

        Ok(Instruction::Set(instruction))
    }

    pub fn set_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_register_and_operand(iter)?;
        let instruction = Set::Word(register, operand);

        Ok(Instruction::Set(instruction))
    }
}
