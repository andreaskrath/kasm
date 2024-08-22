use super::DecoderHelper;
use crate::{
    error::DecodeError,
    instruction::{Division, Instruction},
};
use std::str::SplitWhitespace;

pub struct DivDecoder;

impl DivDecoder {
    pub fn div_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_register_and_operand(iter)?;
        let instruction = Division::Byte(register, operand);

        Ok(Instruction::Division(instruction))
    }

    pub fn div_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_register_and_operand(iter)?;
        let instruction = Division::Quarter(register, operand);

        Ok(Instruction::Division(instruction))
    }

    pub fn div_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_register_and_operand(iter)?;
        let instruction = Division::Half(register, operand);

        Ok(Instruction::Division(instruction))
    }

    pub fn div_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_register_and_operand(iter)?;
        let instruction = Division::Word(register, operand);

        Ok(Instruction::Division(instruction))
    }
}
