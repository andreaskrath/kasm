use crate::{
    constant::{Byte, DecodeTable, Half, Quarter, Word},
    error::DecodeError,
    operand::Operand,
    register::Register,
};
use phf::phf_map;
use std::str::SplitWhitespace;
use variant_count::VariantCount;

const _: () = assert!(DECODE_TABLE.len() == Instruction::VARIANT_COUNT);

pub const DECODE_TABLE: DecodeTable = phf_map! {
    "stop" => Instruction::decode_stop,
    "setb" => Instruction::decode_set_byte,
    "setq" => Instruction::decode_set_quarter,
    "seth" => Instruction::decode_set_half,
    "setw" => Instruction::decode_set_word,
    "addb" => Instruction::decode_add_byte,
    "addq" => Instruction::decode_add_quarter,
    "addh" => Instruction::decode_add_half,
    "addw" => Instruction::decode_add_word,
    "subb" => Instruction::decode_sub_byte,
    "subq" => Instruction::decode_sub_quarter,
    "subh" => Instruction::decode_sub_half,
    "subw" => Instruction::decode_sub_word,
    "mulb" => Instruction::decode_mul_byte,
    "mulq" => Instruction::decode_mul_quarter,
    "mulh" => Instruction::decode_mul_half,
    "mulw" => Instruction::decode_mul_word,
    "divb" => Instruction::decode_div_byte,
    "divq" => Instruction::decode_div_quarter,
    "divh" => Instruction::decode_div_half,
    "divw" => Instruction::decode_div_word,
};

#[derive(Debug, PartialEq, VariantCount)]
pub enum Instruction {
    Stop,
    SetByte(Register, Operand<Byte>),
    SetQuarter(Register, Operand<Quarter>),
    SetHalf(Register, Operand<Half>),
    SetWord(Register, Operand<Word>),
    AddByte(Register, Operand<Byte>),
    AddQuarter(Register, Operand<Quarter>),
    AddHalf(Register, Operand<Half>),
    AddWord(Register, Operand<Word>),
    SubByte(Register, Operand<Byte>),
    SubQuarter(Register, Operand<Quarter>),
    SubHalf(Register, Operand<Half>),
    SubWord(Register, Operand<Word>),
    MulByte(Register, Operand<Byte>),
    MulQuarter(Register, Operand<Quarter>),
    MulHalf(Register, Operand<Half>),
    MulWord(Register, Operand<Word>),
    DivByte(Register, Operand<Byte>),
    DivQuarter(Register, Operand<Quarter>),
    DivHalf(Register, Operand<Half>),
    DivWord(Register, Operand<Word>),
}

fn get_reg_and_operand_str(mut iter: SplitWhitespace) -> Result<(&str, &str), DecodeError> {
    let (Some(s_register), Some(s_operand)) = (iter.next(), iter.next()) else {
        return Err(DecodeError::IncompleteInstruction);
    };

    Ok((s_register, s_operand))
}

fn decode_reg_and_byte_operand(
    iter: SplitWhitespace,
) -> Result<(Register, Operand<Byte>), DecodeError> {
    let (s_register, s_operand) = get_reg_and_operand_str(iter)?;
    let register = Register::try_from(s_register)?;
    let operand = Operand::try_from(s_operand)?;

    Ok((register, operand))
}

fn decode_reg_and_quarter_operand(
    iter: SplitWhitespace,
) -> Result<(Register, Operand<Quarter>), DecodeError> {
    let (s_register, s_operand) = get_reg_and_operand_str(iter)?;
    let register = Register::try_from(s_register)?;
    let operand = Operand::try_from(s_operand)?;

    Ok((register, operand))
}

fn decode_reg_and_half_operand(
    iter: SplitWhitespace,
) -> Result<(Register, Operand<Half>), DecodeError> {
    let (s_register, s_operand) = get_reg_and_operand_str(iter)?;
    let register = Register::try_from(s_register)?;
    let operand = Operand::try_from(s_operand)?;

    Ok((register, operand))
}

fn decode_reg_and_word_operand(
    iter: SplitWhitespace,
) -> Result<(Register, Operand<Word>), DecodeError> {
    let (s_register, s_operand) = get_reg_and_operand_str(iter)?;
    let register = Register::try_from(s_register)?;
    let operand = Operand::try_from(s_operand)?;

    Ok((register, operand))
}

impl Instruction {
    pub fn decode_stop(_iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        Ok(Instruction::Stop)
    }

    pub fn decode_set_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_byte_operand(iter)?;

        Ok(Instruction::SetByte(register, operand))
    }

    pub fn decode_set_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_quarter_operand(iter)?;

        Ok(Instruction::SetQuarter(register, operand))
    }

    pub fn decode_set_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_half_operand(iter)?;

        Ok(Instruction::SetHalf(register, operand))
    }

    pub fn decode_set_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_word_operand(iter)?;

        Ok(Instruction::SetWord(register, operand))
    }

    pub fn decode_add_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_byte_operand(iter)?;

        Ok(Instruction::AddByte(register, operand))
    }

    pub fn decode_add_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_quarter_operand(iter)?;

        Ok(Instruction::AddQuarter(register, operand))
    }

    pub fn decode_add_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_half_operand(iter)?;

        Ok(Instruction::AddHalf(register, operand))
    }

    pub fn decode_add_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_word_operand(iter)?;

        Ok(Instruction::AddWord(register, operand))
    }

    pub fn decode_sub_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_byte_operand(iter)?;

        Ok(Instruction::SubByte(register, operand))
    }

    pub fn decode_sub_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_quarter_operand(iter)?;

        Ok(Instruction::SubQuarter(register, operand))
    }

    pub fn decode_sub_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_half_operand(iter)?;

        Ok(Instruction::SubHalf(register, operand))
    }

    pub fn decode_sub_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_word_operand(iter)?;

        Ok(Instruction::SubWord(register, operand))
    }

    pub fn decode_mul_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_byte_operand(iter)?;

        Ok(Instruction::MulByte(register, operand))
    }

    pub fn decode_mul_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_quarter_operand(iter)?;

        Ok(Instruction::MulQuarter(register, operand))
    }

    pub fn decode_mul_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_half_operand(iter)?;

        Ok(Instruction::MulHalf(register, operand))
    }

    pub fn decode_mul_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_word_operand(iter)?;

        Ok(Instruction::MulWord(register, operand))
    }

    pub fn decode_div_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_byte_operand(iter)?;

        Ok(Instruction::DivByte(register, operand))
    }

    pub fn decode_div_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_quarter_operand(iter)?;

        Ok(Instruction::DivQuarter(register, operand))
    }

    pub fn decode_div_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_half_operand(iter)?;

        Ok(Instruction::DivHalf(register, operand))
    }

    pub fn decode_div_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_word_operand(iter)?;

        Ok(Instruction::DivWord(register, operand))
    }
}

#[cfg(test)]
mod get_reg_and_operand_str {
    use crate::error::DecodeError;

    use super::get_reg_and_operand_str;

    #[test]
    fn empty_parameters() {
        let iter = "".split_whitespace();
        let expected = Err(DecodeError::IncompleteInstruction);
        let actual = get_reg_and_operand_str(iter);
        assert_eq!(actual, expected);
    }

    #[test]
    fn missing_second_parameter() {
        let iter = "ra".split_whitespace();
        let expected = Err(DecodeError::IncompleteInstruction);
        let actual = get_reg_and_operand_str(iter);
        assert_eq!(actual, expected);
    }

    #[test]
    fn both_parameters_defined() {
        let iter = "ra 0".split_whitespace();
        let expected = Ok(("ra", "0"));
        let actual = get_reg_and_operand_str(iter);
        assert_eq!(actual, expected);
    }
}
