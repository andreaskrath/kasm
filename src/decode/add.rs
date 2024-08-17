use super::{
    decode_reg_and_byte_operand, decode_reg_and_half_operand, decode_reg_and_quarter_operand,
    decode_reg_and_word_operand,
};
use crate::{
    error::DecodeError,
    instruction::{AddInstruction, Instruction},
};
use std::str::SplitWhitespace;

pub struct AddDecoder;

impl AddDecoder {
    pub fn decode_add_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_byte_operand(iter)?;
        let instruction = AddInstruction::Byte(register, operand);

        Ok(Instruction::Add(instruction))
    }

    pub fn decode_add_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_quarter_operand(iter)?;
        let instruction = AddInstruction::Quarter(register, operand);

        Ok(Instruction::Add(instruction))
    }

    pub fn decode_add_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_half_operand(iter)?;
        let instruction = AddInstruction::Half(register, operand);

        Ok(Instruction::Add(instruction))
    }

    pub fn decode_add_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_word_operand(iter)?;
        let instruction = AddInstruction::Word(register, operand);

        Ok(Instruction::Add(instruction))
    }
}
