use super::DecoderHelper;
use crate::{
    error::DecodeError,
    instruction::{Instruction, PrintRegister},
};
use std::str::SplitWhitespace;

pub struct PrintRegisterDecoder;

impl PrintRegisterDecoder {
    pub fn print_register_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let register = DecoderHelper::try_register(iter)?;
        let instruction = PrintRegister::Byte(register);

        Ok(Instruction::PrintRegister(instruction))
    }

    pub fn print_register_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let register = DecoderHelper::try_register(iter)?;
        let instruction = PrintRegister::Quarter(register);

        Ok(Instruction::PrintRegister(instruction))
    }

    pub fn print_register_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let register = DecoderHelper::try_register(iter)?;
        let instruction = PrintRegister::Half(register);

        Ok(Instruction::PrintRegister(instruction))
    }

    pub fn print_register_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let register = DecoderHelper::try_register(iter)?;
        let instruction = PrintRegister::Word(register);

        Ok(Instruction::PrintRegister(instruction))
    }
}
