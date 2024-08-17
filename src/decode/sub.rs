use super::{
    decode_reg_and_byte_operand, decode_reg_and_half_operand, decode_reg_and_quarter_operand,
    decode_reg_and_word_operand,
};
use crate::{
    error::DecodeError,
    instruction::{Instruction, SubInstruction},
};
use std::str::SplitWhitespace;

pub struct SubDecoder;

impl SubDecoder {
    pub fn decode_sub_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_byte_operand(iter)?;
        let instruction = SubInstruction::Byte(register, operand);

        Ok(Instruction::Sub(instruction))
    }

    pub fn decode_sub_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_quarter_operand(iter)?;
        let instruction = SubInstruction::Quarter(register, operand);

        Ok(Instruction::Sub(instruction))
    }

    pub fn decode_sub_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_half_operand(iter)?;
        let instruction = SubInstruction::Half(register, operand);

        Ok(Instruction::Sub(instruction))
    }

    pub fn decode_sub_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_word_operand(iter)?;
        let instruction = SubInstruction::Word(register, operand);

        Ok(Instruction::Sub(instruction))
    }
}
