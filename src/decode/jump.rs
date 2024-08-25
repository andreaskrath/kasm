use super::DecoderHelper;
use crate::{
    error::DecodeError,
    instruction::{Instruction, Jump},
};
use std::str::SplitWhitespace;

pub struct JumpDecoder;

impl JumpDecoder {
    pub fn jump_unconditional(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let operand = DecoderHelper::try_operand(iter)?;
        let instruction = Jump::Unconditional(operand);

        Ok(Instruction::Jump(instruction))
    }

    pub fn jump_if_zero(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let operand = DecoderHelper::try_operand(iter)?;
        let instruction = Jump::IfZero(operand);

        Ok(Instruction::Jump(instruction))
    }

    pub fn jump_if_not_zero(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let operand = DecoderHelper::try_operand(iter)?;
        let instruction = Jump::IfNotZero(operand);

        Ok(Instruction::Jump(instruction))
    }

    pub fn jump_if_sign(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let operand = DecoderHelper::try_operand(iter)?;
        let instruction = Jump::IfSign(operand);

        Ok(Instruction::Jump(instruction))
    }

    pub fn jump_if_not_sign(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let operand = DecoderHelper::try_operand(iter)?;
        let instruction = Jump::IfNotSign(operand);

        Ok(Instruction::Jump(instruction))
    }

    pub fn jump_if_overflow(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let operand = DecoderHelper::try_operand(iter)?;
        let instruction = Jump::IfOverflow(operand);

        Ok(Instruction::Jump(instruction))
    }

    pub fn jump_if_not_overflow(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let operand = DecoderHelper::try_operand(iter)?;
        let instruction = Jump::IfNotOverflow(operand);

        Ok(Instruction::Jump(instruction))
    }

    pub fn jump_if_greater(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let operand = DecoderHelper::try_operand(iter)?;
        let instruction = Jump::IfGreater(operand);

        Ok(Instruction::Jump(instruction))
    }

    pub fn jump_if_lesser(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let operand = DecoderHelper::try_operand(iter)?;
        let instruction = Jump::IfLesser(operand);

        Ok(Instruction::Jump(instruction))
    }

    pub fn jump_if_greater_or_equal(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let operand = DecoderHelper::try_operand(iter)?;
        let instruction = Jump::IfGreaterOrEqual(operand);

        Ok(Instruction::Jump(instruction))
    }

    pub fn jump_if_lesser_or_equal(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let operand = DecoderHelper::try_operand(iter)?;
        let instruction = Jump::IfLesserOrEqual(operand);

        Ok(Instruction::Jump(instruction))
    }
}
