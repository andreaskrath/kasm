use crate::{
    constant::DecodeTable, error::DecodeError, instruction::Instruction, operand::Operand,
    register::Register,
};
use addition::AddDecoder;
use and::AndDecoder;
use division::DivDecoder;
use multiplication::MulDecoder;
use phf::phf_map;
use pop::PopDecoder;
use push::PushDecoder;
use remainder::RemDecoder;
use set::SetDecoder;
use std::str::{FromStr, SplitWhitespace};
use subtraction::SubDecoder;

mod addition;
mod and;
mod division;
mod multiplication;
mod pop;
mod push;
mod remainder;
mod set;
mod subtraction;

// const _: () = assert!(DECODE_TABLE.len() == Instruction::VARIANT_COUNT);

pub const DECODE_TABLE: DecodeTable = phf_map! {
    "stop" => Instruction::stop,
    "setb" => SetDecoder::set_byte,
    "setq" => SetDecoder::set_quarter,
    "seth" => SetDecoder::set_half,
    "setw" => SetDecoder::set_word,
    "addb" => AddDecoder::add_byte,
    "addq" => AddDecoder::add_quarter,
    "addh" => AddDecoder::add_half,
    "addw" => AddDecoder::add_word,
    "subb" => SubDecoder::sub_byte,
    "subq" => SubDecoder::sub_quarter,
    "subh" => SubDecoder::sub_half,
    "subw" => SubDecoder::sub_word,
    "mulb" => MulDecoder::mul_byte,
    "mulq" => MulDecoder::mul_quarter,
    "mulh" => MulDecoder::mul_half,
    "mulw" => MulDecoder::mul_word,
    "divb" => DivDecoder::div_byte,
    "divq" => DivDecoder::div_quarter,
    "divh" => DivDecoder::div_half,
    "divw" => DivDecoder::div_word,
    "remb" => RemDecoder::rem_byte,
    "remq" => RemDecoder::rem_quarter,
    "remh" => RemDecoder::rem_half,
    "remw" => RemDecoder::rem_word,
    "pshb" => PushDecoder::push_byte,
    "pshq" => PushDecoder::push_quarter,
    "pshh" => PushDecoder::push_half,
    "pshw" => PushDecoder::push_word,
    "popb" => PopDecoder::pop_byte,
    "popq" => PopDecoder::pop_quarter,
    "poph" => PopDecoder::pop_half,
    "popw" => PopDecoder::pop_word,
    "call" => Instruction::call,
    "ret" => Instruction::ret,
    "andb" => AndDecoder::and_byte,
    "andq" => AndDecoder::and_quarter,
    "andh" => AndDecoder::and_half,
    "andw" => AndDecoder::and_word,
};

fn get_both_parameters_str(mut iter: SplitWhitespace) -> Result<(&str, &str), DecodeError> {
    let (Some(s_register), Some(s_operand)) = (iter.next(), iter.next()) else {
        return Err(DecodeError::IncompleteInstruction);
    };

    Ok((s_register, s_operand))
}

fn get_first_parameter_str(mut iter: SplitWhitespace) -> Result<&str, DecodeError> {
    match iter.next() {
        Some(s_operand) => Ok(s_operand),
        None => Err(DecodeError::IncompleteInstruction),
    }
}

struct DecoderHelper;

impl DecoderHelper {
    fn try_register_and_operand<T>(
        iter: SplitWhitespace,
    ) -> Result<(Register, Operand<T>), DecodeError>
    where
        T: FromStr,
    {
        let (s_register, s_operand) = get_both_parameters_str(iter)?;
        let register = Register::try_from(s_register)?;
        let operand = Operand::try_from(s_operand)?;

        Ok((register, operand))
    }

    fn try_operand<T>(iter: SplitWhitespace) -> Result<Operand<T>, DecodeError>
    where
        T: FromStr,
    {
        let s_operand = get_first_parameter_str(iter)?;
        let operand = Operand::try_from(s_operand)?;

        Ok(operand)
    }

    fn try_register(iter: SplitWhitespace) -> Result<Register, DecodeError> {
        let s_register = get_first_parameter_str(iter)?;
        let register = Register::try_from(s_register)?;

        Ok(register)
    }
}

impl Instruction {
    pub fn stop(_iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        Ok(Instruction::Stop)
    }

    pub fn call(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let operand = DecoderHelper::try_operand(iter)?;

        Ok(Instruction::Call(operand))
    }

    pub fn ret(_iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        Ok(Instruction::Return)
    }
}

#[cfg(test)]
mod get_reg_and_operand_str {
    use super::get_both_parameters_str;
    use crate::error::DecodeError;

    #[test]
    fn empty_parameters() {
        let iter = "".split_whitespace();
        let expected = Err(DecodeError::IncompleteInstruction);
        let actual = get_both_parameters_str(iter);
        assert_eq!(actual, expected);
    }

    #[test]
    fn missing_second_parameter() {
        let iter = "ra".split_whitespace();
        let expected = Err(DecodeError::IncompleteInstruction);
        let actual = get_both_parameters_str(iter);
        assert_eq!(actual, expected);
    }

    #[test]
    fn both_parameters_defined() {
        let iter = "ra 0".split_whitespace();
        let expected = Ok(("ra", "0"));
        let actual = get_both_parameters_str(iter);
        assert_eq!(actual, expected);
    }
}
