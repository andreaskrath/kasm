use super::DecoderHelper;
use crate::{
    error::DecodeError,
    instruction::{Instruction, Xor},
};
use std::str::SplitWhitespace;

pub struct XorDecoder;

impl XorDecoder {
    pub fn xor_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_register_and_operand(iter)?;
        let instruction = Xor::Byte(register, operand);

        Ok(Instruction::Xor(instruction))
    }

    pub fn xor_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_register_and_operand(iter)?;
        let instruction = Xor::Quarter(register, operand);

        Ok(Instruction::Xor(instruction))
    }

    pub fn xor_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_register_and_operand(iter)?;
        let instruction = Xor::Half(register, operand);

        Ok(Instruction::Xor(instruction))
    }

    pub fn xor_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_register_and_operand(iter)?;
        let instruction = Xor::Word(register, operand);

        Ok(Instruction::Xor(instruction))
    }
}
