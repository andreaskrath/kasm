use super::DecoderHelper;
use crate::{
    error::DecodeError,
    instruction::{Instruction, PrintStack},
};
use std::str::SplitWhitespace;

pub struct PrintStackDecoder;

impl PrintStackDecoder {
    pub fn print_stack_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let operand = DecoderHelper::try_operand(iter)?;
        let instruction = PrintStack::Byte(operand);

        Ok(Instruction::PrintStack(instruction))
    }

    pub fn print_stack_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let operand = DecoderHelper::try_operand(iter)?;
        let instruction = PrintStack::Quarter(operand);

        Ok(Instruction::PrintStack(instruction))
    }

    pub fn print_stack_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let operand = DecoderHelper::try_operand(iter)?;
        let instruction = PrintStack::Half(operand);

        Ok(Instruction::PrintStack(instruction))
    }

    pub fn print_stack_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let operand = DecoderHelper::try_operand(iter)?;
        let instruction = PrintStack::Word(operand);

        Ok(Instruction::PrintStack(instruction))
    }

    pub fn print_stack_str(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let operand = DecoderHelper::try_operand(iter)?;
        let instruction = PrintStack::Str(operand);

        Ok(Instruction::PrintStack(instruction))
    }
}
