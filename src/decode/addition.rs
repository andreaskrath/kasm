use super::DecoderHelper;
use crate::{
    error::DecodeError,
    instruction::{Addition, Instruction},
};
use std::str::SplitWhitespace;

pub struct AddDecoder;

impl AddDecoder {
    pub fn add_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_reg_and_byte_operand(iter)?;
        let instruction = Addition::Byte(register, operand);

        Ok(Instruction::Addition(instruction))
    }

    pub fn add_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_reg_and_quarter_operand(iter)?;
        let instruction = Addition::Quarter(register, operand);

        Ok(Instruction::Addition(instruction))
    }

    pub fn add_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_reg_and_half_operand(iter)?;
        let instruction = Addition::Half(register, operand);

        Ok(Instruction::Addition(instruction))
    }

    pub fn add_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = DecoderHelper::try_reg_and_word_operand(iter)?;
        let instruction = Addition::Word(register, operand);

        Ok(Instruction::Addition(instruction))
    }
}
