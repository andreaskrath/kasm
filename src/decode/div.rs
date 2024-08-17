use super::{
    decode_reg_and_byte_operand, decode_reg_and_half_operand, decode_reg_and_quarter_operand,
    decode_reg_and_word_operand,
};
use crate::{
    error::DecodeError,
    instruction::{DivInstruction, Instruction},
};
use std::str::SplitWhitespace;

pub struct DivDecoder;

impl DivDecoder {
    pub fn decode_div_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_byte_operand(iter)?;
        let instruction = DivInstruction::Byte(register, operand);

        Ok(Instruction::Div(instruction))
    }

    pub fn decode_div_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_quarter_operand(iter)?;
        let instruction = DivInstruction::Quarter(register, operand);

        Ok(Instruction::Div(instruction))
    }

    pub fn decode_div_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_half_operand(iter)?;
        let instruction = DivInstruction::Half(register, operand);

        Ok(Instruction::Div(instruction))
    }

    pub fn decode_div_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_word_operand(iter)?;
        let instruction = DivInstruction::Word(register, operand);

        Ok(Instruction::Div(instruction))
    }
}
