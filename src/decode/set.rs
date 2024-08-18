use super::{
    decode_reg_and_byte_operand, decode_reg_and_half_operand, decode_reg_and_quarter_operand,
    decode_reg_and_word_operand,
};
use crate::{
    error::DecodeError,
    instruction::{Instruction, Set},
};
use std::str::SplitWhitespace;

pub struct SetDecoder;

impl SetDecoder {
    pub fn set_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_byte_operand(iter)?;
        let instruction = Set::Byte(register, operand);

        Ok(Instruction::Set(instruction))
    }

    pub fn set_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_quarter_operand(iter)?;
        let instruction = Set::Quarter(register, operand);

        Ok(Instruction::Set(instruction))
    }

    pub fn set_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_half_operand(iter)?;
        let instruction = Set::Half(register, operand);

        Ok(Instruction::Set(instruction))
    }

    pub fn set_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_word_operand(iter)?;
        let instruction = Set::Word(register, operand);

        Ok(Instruction::Set(instruction))
    }
}
