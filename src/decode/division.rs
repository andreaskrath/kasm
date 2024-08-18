use super::{
    decode_reg_and_byte_operand, decode_reg_and_half_operand, decode_reg_and_quarter_operand,
    decode_reg_and_word_operand,
};
use crate::{
    error::DecodeError,
    instruction::{Division, Instruction},
};
use std::str::SplitWhitespace;

pub struct DivDecoder;

impl DivDecoder {
    pub fn div_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_byte_operand(iter)?;
        let instruction = Division::Byte(register, operand);

        Ok(Instruction::Division(instruction))
    }

    pub fn div_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_quarter_operand(iter)?;
        let instruction = Division::Quarter(register, operand);

        Ok(Instruction::Division(instruction))
    }

    pub fn div_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_half_operand(iter)?;
        let instruction = Division::Half(register, operand);

        Ok(Instruction::Division(instruction))
    }

    pub fn div_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_word_operand(iter)?;
        let instruction = Division::Word(register, operand);

        Ok(Instruction::Division(instruction))
    }
}
