use crate::{
    constant::{Byte, DecodeTable, Half, Quarter, Word},
    error::DecodeError,
    instruction::Instruction,
    operand::Operand,
    register::Register,
};
use add::AddDecoder;
use div::DivDecoder;
use mul::MulDecoder;
use phf::phf_map;
use set::SetDecoder;
use std::str::SplitWhitespace;
use sub::SubDecoder;

mod add;
mod div;
mod mul;
mod set;
mod sub;

// const _: () = assert!(DECODE_TABLE.len() == Instruction::VARIANT_COUNT);

pub const DECODE_TABLE: DecodeTable = phf_map! {
    "stop" => Instruction::decode_stop,
    "setb" => SetDecoder::decode_set_byte,
    "setq" => SetDecoder::decode_set_quarter,
    "seth" => SetDecoder::decode_set_half,
    "setw" => SetDecoder::decode_set_word,
    "addb" => AddDecoder::decode_add_byte,
    "addq" => AddDecoder::decode_add_quarter,
    "addh" => AddDecoder::decode_add_half,
    "addw" => AddDecoder::decode_add_word,
    "subb" => SubDecoder::decode_sub_byte,
    "subq" => SubDecoder::decode_sub_quarter,
    "subh" => SubDecoder::decode_sub_half,
    "subw" => SubDecoder::decode_sub_word,
    "mulb" => MulDecoder::decode_mul_byte,
    "mulq" => MulDecoder::decode_mul_quarter,
    "mulh" => MulDecoder::decode_mul_half,
    "mulw" => MulDecoder::decode_mul_word,
    "divb" => DivDecoder::decode_div_byte,
    "divq" => DivDecoder::decode_div_quarter,
    "divh" => DivDecoder::decode_div_half,
    "divw" => DivDecoder::decode_div_word,
};

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
