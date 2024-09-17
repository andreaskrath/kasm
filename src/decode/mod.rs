use crate::{
    constant::{DecodeTable, Parameters},
    error::DecodeError,
    instruction::Instruction,
    operand::Operand,
    register::Register,
    Interpreter,
};
use addition::AdditionParameterDecoder;
use and::AndParameterDecoder;
use compare::CompareParameterDecoder;
use division::DivisionParameterDecoder;
use jump::JumpParameterDecoder;
use multiplication::MultiplicationParameterDecoder;
use not::NotParameterDecoder;
use or::OrParameterDecoder;
use phf::phf_map;
use pop::PopParameterDecoder;
use print_register::PrintRegisterParameterDecoder;
use print_stack::PrintStackParameterDecoder;
use push::PushParameterDecoder;
use remainder::RemainderParameterDecoder;
use set::SetParameterDecoder;
use std::str::FromStr;
use subtraction::SubtractionParameterDecoder;
use test::TestParameterDecoder;
use xor::XorParameterDecoder;

mod addition;
mod and;
mod compare;
mod division;
mod jump;
mod multiplication;
mod not;
mod or;
mod pop;
mod print_register;
mod print_stack;
mod push;
mod remainder;
mod set;
mod subtraction;
mod test;
mod xor;

impl Interpreter {
    /// Decodes an instruction in string representation into an [`Instruction`].
    ///
    /// # Errors
    /// An error is returned if something went wrong during decoding.
    ///
    /// This includes, but is not limited to issues with the instruction itself, or issues in decoding instruction parameters.
    pub(super) fn decode(&mut self, s: &str) -> Result<Instruction, DecodeError> {
        let mut s_iter = s.split_ascii_whitespace();
        let instruction = s_iter.next().ok_or(DecodeError::EmptyLine)?;
        let decoder = DECODE_TABLE
            .get(instruction)
            .ok_or(DecodeError::UnknownInstruction(instruction.to_string()))?;
        let instruction = decoder(s_iter)?;

        Ok(instruction)
    }
}

const DECODE_TABLE: DecodeTable = phf_map! {
    "stop" => Instruction::stop,
    "setb" => SetParameterDecoder::byte,
    "setq" => SetParameterDecoder::quarter,
    "seth" => SetParameterDecoder::half,
    "setw" => SetParameterDecoder::word,
    "addb" => AdditionParameterDecoder::byte,
    "addq" => AdditionParameterDecoder::quarter,
    "addh" => AdditionParameterDecoder::half,
    "addw" => AdditionParameterDecoder::word,
    "subb" => SubtractionParameterDecoder::byte,
    "subq" => SubtractionParameterDecoder::quarter,
    "subh" => SubtractionParameterDecoder::half,
    "subw" => SubtractionParameterDecoder::word,
    "mulb" => MultiplicationParameterDecoder::byte,
    "mulq" => MultiplicationParameterDecoder::quarter,
    "mulh" => MultiplicationParameterDecoder::half,
    "mulw" => MultiplicationParameterDecoder::word,
    "divb" => DivisionParameterDecoder::byte,
    "divq" => DivisionParameterDecoder::quarter,
    "divh" => DivisionParameterDecoder::half,
    "divw" => DivisionParameterDecoder::word,
    "remb" => RemainderParameterDecoder::byte,
    "remq" => RemainderParameterDecoder::quarter,
    "remh" => RemainderParameterDecoder::half,
    "remw" => RemainderParameterDecoder::word,
    "pshb" => PushParameterDecoder::byte,
    "pshq" => PushParameterDecoder::quarter,
    "pshh" => PushParameterDecoder::half,
    "pshw" => PushParameterDecoder::word,
    "popb" => PopParameterDecoder::byte,
    "popq" => PopParameterDecoder::quarter,
    "poph" => PopParameterDecoder::half,
    "popw" => PopParameterDecoder::word,
    "call" => Instruction::call,
    "ret" => Instruction::ret,
    "andb" => AndParameterDecoder::byte,
    "andq" => AndParameterDecoder::quarter,
    "andh" => AndParameterDecoder::half,
    "andw" => AndParameterDecoder::word,
    "orb" => OrParameterDecoder::byte,
    "orq" => OrParameterDecoder::quarter,
    "orh" => OrParameterDecoder::half,
    "orw" => OrParameterDecoder::word,
    "xorb" => XorParameterDecoder::byte,
    "xorq" => XorParameterDecoder::quarter,
    "xorh" => XorParameterDecoder::half,
    "xorw" => XorParameterDecoder::word,
    "notb" => NotParameterDecoder::byte,
    "notq" => NotParameterDecoder::quarter,
    "noth" => NotParameterDecoder::half,
    "notw" => NotParameterDecoder::word,
    "tstb" => TestParameterDecoder::byte,
    "tstq" => TestParameterDecoder::quarter,
    "tsth" => TestParameterDecoder::half,
    "tstw" => TestParameterDecoder::word,
    "cmpb" => CompareParameterDecoder::byte,
    "cmpq" => CompareParameterDecoder::quarter,
    "cmph" => CompareParameterDecoder::half,
    "cmpw" => CompareParameterDecoder::word,
    "jmp" => JumpParameterDecoder::unconditional,
    "jiz" => JumpParameterDecoder::if_zero,
    "jnz" => JumpParameterDecoder::if_not_zero,
    "jis" => JumpParameterDecoder::if_sign,
    "jns" => JumpParameterDecoder::if_not_sign,
    "jio" => JumpParameterDecoder::if_overflow,
    "jno" => JumpParameterDecoder::if_not_overflow,
    "jig" => JumpParameterDecoder::if_greater,
    "jil" => JumpParameterDecoder::if_lesser,
    "jge" => JumpParameterDecoder::if_greater_or_equal,
    "jle" => JumpParameterDecoder::if_lesser_or_equal,
    "prrb" => PrintRegisterParameterDecoder::byte,
    "prrq" => PrintRegisterParameterDecoder::quarter,
    "prrh" => PrintRegisterParameterDecoder::half,
    "prrw" => PrintRegisterParameterDecoder::word,
    "prsb" => PrintStackParameterDecoder::byte,
    "prsq" => PrintStackParameterDecoder::quarter,
    "prsh" => PrintStackParameterDecoder::half,
    "prsw" => PrintStackParameterDecoder::word,
    "prss" => PrintStackParameterDecoder::str,
};

/// Attempts to get one parameter from the instruction arguments.
///
/// # Errors
/// Will return [`DecodeError::IncompleteInstruction`] if no parameter could be found.
fn try_get_first_parameter_str(mut parameters: Parameters) -> Result<&str, DecodeError> {
    match parameters.next() {
        Some(s_operand) => Ok(s_operand),
        None => Err(DecodeError::IncompleteInstruction),
    }
}

/// Attempts to get two parameters from the instruction arguments.
///
/// # Errors
/// Will return [`DecodeError::IncompleteInstruction`] if either parameter could be found.
fn try_get_both_parameters_str(mut parameters: Parameters) -> Result<(&str, &str), DecodeError> {
    let (Some(s_register), Some(s_operand)) = (parameters.next(), parameters.next()) else {
        return Err(DecodeError::IncompleteInstruction);
    };

    Ok((s_register, s_operand))
}

struct ParameterDecoderHelper;

impl ParameterDecoderHelper {
    fn try_register(parameters: Parameters) -> Result<Register, DecodeError> {
        let s_register = try_get_first_parameter_str(parameters)?;
        let register = Register::try_from(s_register)?;

        Ok(register)
    }

    fn try_operand<T>(parameters: Parameters) -> Result<Operand<T>, DecodeError>
    where
        T: FromStr,
    {
        let s_operand = try_get_first_parameter_str(parameters)?;
        let operand = Operand::try_from(s_operand)?;

        Ok(operand)
    }

    fn try_register_and_operand<T>(
        parameters: Parameters,
    ) -> Result<(Register, Operand<T>), DecodeError>
    where
        T: FromStr,
    {
        let (s_register, s_operand) = try_get_both_parameters_str(parameters)?;
        let register = Register::try_from(s_register)?;
        let operand = Operand::try_from(s_operand)?;

        Ok((register, operand))
    }

    fn try_double_operand<T>(
        parameters: Parameters,
    ) -> Result<(Operand<T>, Operand<T>), DecodeError>
    where
        T: FromStr,
    {
        let (s_operand1, s_operand2) = try_get_both_parameters_str(parameters)?;
        let operand1 = Operand::try_from(s_operand1)?;
        let operand2 = Operand::try_from(s_operand2)?;

        Ok((operand1, operand2))
    }
}

/// TODO: move implementation from Instruction to something else.
impl Instruction {
    pub fn stop(_parameters: Parameters) -> Result<Instruction, DecodeError> {
        Ok(Instruction::Stop)
    }

    pub fn call(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let operand = ParameterDecoderHelper::try_operand(parameters)?;

        Ok(Instruction::Call(operand))
    }

    pub fn ret(_parameters: Parameters) -> Result<Instruction, DecodeError> {
        Ok(Instruction::Return)
    }
}

#[cfg(test)]
mod try_get_both_parameters_str {
    use super::try_get_both_parameters_str;
    use crate::error::DecodeError;

    #[test]
    fn empty_parameters() {
        let iter = "".split_ascii_whitespace();
        let expected = Err(DecodeError::IncompleteInstruction);
        let actual = try_get_both_parameters_str(iter);
        assert_eq!(actual, expected);
    }

    #[test]
    fn missing_second_parameter() {
        let iter = "ra".split_ascii_whitespace();
        let expected = Err(DecodeError::IncompleteInstruction);
        let actual = try_get_both_parameters_str(iter);
        assert_eq!(actual, expected);
    }

    #[test]
    fn both_parameters_defined() {
        let iter = "ra 0".split_ascii_whitespace();
        let expected = Ok(("ra", "0"));
        let actual = try_get_both_parameters_str(iter);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod try_get_first_parameter_str {
    use super::try_get_first_parameter_str;
    use crate::error::DecodeError;

    #[test]
    fn empty_parameter() {
        let iter = "".split_ascii_whitespace();
        let expected = Err(DecodeError::IncompleteInstruction);
        let actual = try_get_first_parameter_str(iter);
        assert_eq!(actual, expected);
    }

    #[test]
    fn first_parameter_defined() {
        let iter = "ra".split_ascii_whitespace();
        let expected = Ok("ra");
        let actual = try_get_first_parameter_str(iter);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod decoder_helper {
    mod try_register {
        use crate::{decode::ParameterDecoderHelper, error::DecodeError, register::Register};

        #[test]
        fn missing_parameter() {
            let iter = "".split_ascii_whitespace();
            let expected = Err(DecodeError::IncompleteInstruction);
            let actual = ParameterDecoderHelper::try_register(iter);
            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register() {
            let iter = "rx".split_ascii_whitespace();
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
            let actual = ParameterDecoderHelper::try_register(iter);
            assert_eq!(actual, expected);
        }

        #[test]
        fn valid_register() {
            let iter = "ra".split_ascii_whitespace();
            let expected = Ok(Register::A);
            let actual = ParameterDecoderHelper::try_register(iter);
            assert_eq!(actual, expected);
        }
    }

    mod try_operand {
        mod byte {
            use crate::{
                constant::Byte, decode::ParameterDecoderHelper, error::DecodeError,
                operand::Operand, register::Register,
            };

            #[test]
            fn missing_parameter() {
                let iter = "".split_ascii_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = ParameterDecoderHelper::try_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_operand() {
                let iter = "rx".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = ParameterDecoderHelper::try_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_operand() {
                let iter = "-1".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = ParameterDecoderHelper::try_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error() {
                let iter = "200u8".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = ParameterDecoderHelper::try_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_register_in_operand() {
                let iter = "ra".split_ascii_whitespace();
                let expected = Ok(Operand::Register(Register::A));
                let actual = ParameterDecoderHelper::try_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_immediate_value_in_operand() {
                let iter = "10".split_ascii_whitespace();
                let expected = Ok(Operand::Immediate(10));
                let actual = ParameterDecoderHelper::try_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }
        }

        mod quarter {
            use crate::{
                constant::Quarter, decode::ParameterDecoderHelper, error::DecodeError,
                operand::Operand, register::Register,
            };

            #[test]
            fn missing_parameter() {
                let iter = "".split_ascii_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = ParameterDecoderHelper::try_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_operand() {
                let iter = "rx".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = ParameterDecoderHelper::try_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_operand() {
                let iter = "-1".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = ParameterDecoderHelper::try_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error() {
                let iter = "200u8".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = ParameterDecoderHelper::try_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_register_in_operand() {
                let iter = "ra".split_ascii_whitespace();
                let expected = Ok(Operand::Register(Register::A));
                let actual = ParameterDecoderHelper::try_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_immediate_value_in_operand() {
                let iter = "10".split_ascii_whitespace();
                let expected = Ok(Operand::Immediate(10));
                let actual = ParameterDecoderHelper::try_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }
        }

        mod half {
            use crate::{
                constant::Half, decode::ParameterDecoderHelper, error::DecodeError,
                operand::Operand, register::Register,
            };

            #[test]
            fn missing_parameter() {
                let iter = "".split_ascii_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = ParameterDecoderHelper::try_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_operand() {
                let iter = "rx".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = ParameterDecoderHelper::try_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_operand() {
                let iter = "-1".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = ParameterDecoderHelper::try_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error() {
                let iter = "200u8".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = ParameterDecoderHelper::try_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_register_in_operand() {
                let iter = "ra".split_ascii_whitespace();
                let expected = Ok(Operand::Register(Register::A));
                let actual = ParameterDecoderHelper::try_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_immediate_value_in_operand() {
                let iter = "10".split_ascii_whitespace();
                let expected = Ok(Operand::Immediate(10));
                let actual = ParameterDecoderHelper::try_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }
        }

        mod word {
            use crate::{
                constant::Word, decode::ParameterDecoderHelper, error::DecodeError,
                operand::Operand, register::Register,
            };

            #[test]
            fn missing_parameter() {
                let iter = "".split_ascii_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = ParameterDecoderHelper::try_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_operand() {
                let iter = "rx".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = ParameterDecoderHelper::try_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_operand() {
                let iter = "-1".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = ParameterDecoderHelper::try_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error() {
                let iter = "200u8".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = ParameterDecoderHelper::try_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_register_in_operand() {
                let iter = "ra".split_ascii_whitespace();
                let expected = Ok(Operand::Register(Register::A));
                let actual = ParameterDecoderHelper::try_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_immediate_value_in_operand() {
                let iter = "10".split_ascii_whitespace();
                let expected = Ok(Operand::Immediate(10));
                let actual = ParameterDecoderHelper::try_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }
        }
    }

    mod try_register_and_operand {
        mod byte {
            use crate::{
                constant::Byte, decode::ParameterDecoderHelper, error::DecodeError,
                operand::Operand, register::Register,
            };

            #[test]
            fn missing_both_parameters() {
                let iter = "".split_ascii_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = ParameterDecoderHelper::try_register_and_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn missing_second_paramter() {
                let iter = "ra".split_ascii_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = ParameterDecoderHelper::try_register_and_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_first_parameter() {
                let iter = "rx ra".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = ParameterDecoderHelper::try_register_and_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_operand() {
                let iter = "ra rx".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = ParameterDecoderHelper::try_register_and_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_operand() {
                let iter = "ra -1".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = ParameterDecoderHelper::try_register_and_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error() {
                let iter = "ra 200u8".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = ParameterDecoderHelper::try_register_and_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_register_and_register_in_operand() {
                let iter = "ra rb".split_ascii_whitespace();
                let expected = Ok((Register::A, Operand::Register(Register::B)));
                let actual = ParameterDecoderHelper::try_register_and_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_register_and_immediate_value_in_operand() {
                let iter = "ra 10".split_ascii_whitespace();
                let expected = Ok((Register::A, Operand::Immediate(10)));
                let actual = ParameterDecoderHelper::try_register_and_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }
        }

        mod quarter {
            use crate::{
                constant::Quarter, decode::ParameterDecoderHelper, error::DecodeError,
                operand::Operand, register::Register,
            };

            #[test]
            fn missing_both_parameters() {
                let iter = "".split_ascii_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = ParameterDecoderHelper::try_register_and_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn missing_second_paramter() {
                let iter = "ra".split_ascii_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = ParameterDecoderHelper::try_register_and_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_first_parameter() {
                let iter = "rx ra".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = ParameterDecoderHelper::try_register_and_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_operand() {
                let iter = "ra rx".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = ParameterDecoderHelper::try_register_and_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_operand() {
                let iter = "ra -1".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = ParameterDecoderHelper::try_register_and_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error() {
                let iter = "ra 200u8".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = ParameterDecoderHelper::try_register_and_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_register_and_register_in_operand() {
                let iter = "ra rb".split_ascii_whitespace();
                let expected = Ok((Register::A, Operand::Register(Register::B)));
                let actual = ParameterDecoderHelper::try_register_and_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_register_and_immediate_value_in_operand() {
                let iter = "ra 10".split_ascii_whitespace();
                let expected = Ok((Register::A, Operand::Immediate(10)));
                let actual = ParameterDecoderHelper::try_register_and_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }
        }

        mod half {
            use crate::{
                constant::Half, decode::ParameterDecoderHelper, error::DecodeError,
                operand::Operand, register::Register,
            };

            #[test]
            fn missing_both_parameters() {
                let iter = "".split_ascii_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = ParameterDecoderHelper::try_register_and_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn missing_second_paramter() {
                let iter = "ra".split_ascii_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = ParameterDecoderHelper::try_register_and_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_first_parameter() {
                let iter = "rx ra".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = ParameterDecoderHelper::try_register_and_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_operand() {
                let iter = "ra rx".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = ParameterDecoderHelper::try_register_and_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_operand() {
                let iter = "ra -1".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = ParameterDecoderHelper::try_register_and_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error() {
                let iter = "ra 200u8".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = ParameterDecoderHelper::try_register_and_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_register_and_register_in_operand() {
                let iter = "ra rb".split_ascii_whitespace();
                let expected = Ok((Register::A, Operand::Register(Register::B)));
                let actual = ParameterDecoderHelper::try_register_and_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_register_and_immediate_value_in_operand() {
                let iter = "ra 10".split_ascii_whitespace();
                let expected = Ok((Register::A, Operand::Immediate(10)));
                let actual = ParameterDecoderHelper::try_register_and_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }
        }

        mod word {
            use crate::{
                constant::Word, decode::ParameterDecoderHelper, error::DecodeError,
                operand::Operand, register::Register,
            };

            #[test]
            fn missing_both_parameters() {
                let iter = "".split_ascii_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = ParameterDecoderHelper::try_register_and_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn missing_second_paramter() {
                let iter = "ra".split_ascii_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = ParameterDecoderHelper::try_register_and_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_first_parameter() {
                let iter = "rx ra".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = ParameterDecoderHelper::try_register_and_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_operand() {
                let iter = "ra rx".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = ParameterDecoderHelper::try_register_and_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_operand() {
                let iter = "ra -1".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = ParameterDecoderHelper::try_register_and_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error() {
                let iter = "ra 200u8".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = ParameterDecoderHelper::try_register_and_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_register_and_register_in_operand() {
                let iter = "ra rb".split_ascii_whitespace();
                let expected = Ok((Register::A, Operand::Register(Register::B)));
                let actual = ParameterDecoderHelper::try_register_and_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_register_and_immediate_value_in_operand() {
                let iter = "ra 10".split_ascii_whitespace();
                let expected = Ok((Register::A, Operand::Immediate(10)));
                let actual = ParameterDecoderHelper::try_register_and_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }
        }
    }

    mod try_double_operand {
        mod byte {
            use crate::{
                constant::Byte, decode::ParameterDecoderHelper, error::DecodeError,
                operand::Operand, register::Register,
            };

            #[test]
            fn missing_both_parameters() {
                let iter = "".split_ascii_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = ParameterDecoderHelper::try_double_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn missing_second_parameter() {
                let iter = "ra".split_ascii_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = ParameterDecoderHelper::try_double_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_first_operand() {
                let iter = "rx ra".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = ParameterDecoderHelper::try_double_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_first_operand() {
                let iter = "-1 ra".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = ParameterDecoderHelper::try_double_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error_in_first_operand() {
                let iter = "200u8 ra".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = ParameterDecoderHelper::try_double_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_second_operand() {
                let iter = "ra rx".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = ParameterDecoderHelper::try_double_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_second_operand() {
                let iter = "ra -1".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = ParameterDecoderHelper::try_double_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error_in_second_operand() {
                let iter = "ra 200u8".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = ParameterDecoderHelper::try_double_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn register_in_both_operands() {
                let iter = "ra rb".split_ascii_whitespace();
                let expected = Ok((
                    Operand::Register(Register::A),
                    Operand::Register(Register::B),
                ));
                let actual = ParameterDecoderHelper::try_double_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn immediate_value_in_both_operands() {
                let iter = "10 20".split_ascii_whitespace();
                let expected = Ok((Operand::Immediate(10), Operand::Immediate(20)));
                let actual = ParameterDecoderHelper::try_double_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn register_in_first_operand_and_immediate_value_in_second_operand() {
                let iter = "ra 20".split_ascii_whitespace();
                let expected = Ok((Operand::Register(Register::A), Operand::Immediate(20)));
                let actual = ParameterDecoderHelper::try_double_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn immediate_value_in_first_operand_and_register_in_second_operand() {
                let iter = "10 rb".split_ascii_whitespace();
                let expected = Ok((Operand::Immediate(10), Operand::Register(Register::B)));
                let actual = ParameterDecoderHelper::try_double_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }
        }

        mod quarter {
            use crate::{
                constant::Quarter, decode::ParameterDecoderHelper, error::DecodeError,
                operand::Operand, register::Register,
            };

            #[test]
            fn missing_both_parameters() {
                let iter = "".split_ascii_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = ParameterDecoderHelper::try_double_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn missing_second_parameter() {
                let iter = "ra".split_ascii_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = ParameterDecoderHelper::try_double_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_first_operand() {
                let iter = "rx ra".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = ParameterDecoderHelper::try_double_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_first_operand() {
                let iter = "-1 ra".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = ParameterDecoderHelper::try_double_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error_in_first_operand() {
                let iter = "200u8 ra".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = ParameterDecoderHelper::try_double_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_second_operand() {
                let iter = "ra rx".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = ParameterDecoderHelper::try_double_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_second_operand() {
                let iter = "ra -1".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = ParameterDecoderHelper::try_double_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error_in_second_operand() {
                let iter = "ra 200u8".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = ParameterDecoderHelper::try_double_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn register_in_both_operands() {
                let iter = "ra rb".split_ascii_whitespace();
                let expected = Ok((
                    Operand::Register(Register::A),
                    Operand::Register(Register::B),
                ));
                let actual = ParameterDecoderHelper::try_double_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn immediate_value_in_both_operands() {
                let iter = "10 20".split_ascii_whitespace();
                let expected = Ok((Operand::Immediate(10), Operand::Immediate(20)));
                let actual = ParameterDecoderHelper::try_double_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn register_in_first_operand_and_immediate_value_in_second_operand() {
                let iter = "ra 20".split_ascii_whitespace();
                let expected = Ok((Operand::Register(Register::A), Operand::Immediate(20)));
                let actual = ParameterDecoderHelper::try_double_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn immediate_value_in_first_operand_and_register_in_second_operand() {
                let iter = "10 rb".split_ascii_whitespace();
                let expected = Ok((Operand::Immediate(10), Operand::Register(Register::B)));
                let actual = ParameterDecoderHelper::try_double_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }
        }

        mod half {
            use crate::{
                constant::Half, decode::ParameterDecoderHelper, error::DecodeError,
                operand::Operand, register::Register,
            };

            #[test]
            fn missing_both_parameters() {
                let iter = "".split_ascii_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = ParameterDecoderHelper::try_double_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn missing_second_parameter() {
                let iter = "ra".split_ascii_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = ParameterDecoderHelper::try_double_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_first_operand() {
                let iter = "rx ra".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = ParameterDecoderHelper::try_double_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_first_operand() {
                let iter = "-1 ra".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = ParameterDecoderHelper::try_double_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error_in_first_operand() {
                let iter = "200u8 ra".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = ParameterDecoderHelper::try_double_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_second_operand() {
                let iter = "ra rx".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = ParameterDecoderHelper::try_double_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_second_operand() {
                let iter = "ra -1".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = ParameterDecoderHelper::try_double_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error_in_second_operand() {
                let iter = "ra 200u8".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = ParameterDecoderHelper::try_double_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn register_in_both_operands() {
                let iter = "ra rb".split_ascii_whitespace();
                let expected = Ok((
                    Operand::Register(Register::A),
                    Operand::Register(Register::B),
                ));
                let actual = ParameterDecoderHelper::try_double_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn immediate_value_in_both_operands() {
                let iter = "10 20".split_ascii_whitespace();
                let expected = Ok((Operand::Immediate(10), Operand::Immediate(20)));
                let actual = ParameterDecoderHelper::try_double_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn register_in_first_operand_and_immediate_value_in_second_operand() {
                let iter = "ra 20".split_ascii_whitespace();
                let expected = Ok((Operand::Register(Register::A), Operand::Immediate(20)));
                let actual = ParameterDecoderHelper::try_double_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn immediate_value_in_first_operand_and_register_in_second_operand() {
                let iter = "10 rb".split_ascii_whitespace();
                let expected = Ok((Operand::Immediate(10), Operand::Register(Register::B)));
                let actual = ParameterDecoderHelper::try_double_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }
        }

        mod word {
            use crate::{
                constant::Word, decode::ParameterDecoderHelper, error::DecodeError,
                operand::Operand, register::Register,
            };

            #[test]
            fn missing_both_parameters() {
                let iter = "".split_ascii_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = ParameterDecoderHelper::try_double_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn missing_second_parameter() {
                let iter = "ra".split_ascii_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = ParameterDecoderHelper::try_double_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_first_operand() {
                let iter = "rx ra".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = ParameterDecoderHelper::try_double_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_first_operand() {
                let iter = "-1 ra".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = ParameterDecoderHelper::try_double_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error_in_first_operand() {
                let iter = "200u8 ra".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = ParameterDecoderHelper::try_double_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_second_operand() {
                let iter = "ra rx".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = ParameterDecoderHelper::try_double_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_second_operand() {
                let iter = "ra -1".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = ParameterDecoderHelper::try_double_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error_in_second_operand() {
                let iter = "ra 200u8".split_ascii_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = ParameterDecoderHelper::try_double_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn register_in_both_operands() {
                let iter = "ra rb".split_ascii_whitespace();
                let expected = Ok((
                    Operand::Register(Register::A),
                    Operand::Register(Register::B),
                ));
                let actual = ParameterDecoderHelper::try_double_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn immediate_value_in_both_operands() {
                let iter = "10 20".split_ascii_whitespace();
                let expected = Ok((Operand::Immediate(10), Operand::Immediate(20)));
                let actual = ParameterDecoderHelper::try_double_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn register_in_first_operand_and_immediate_value_in_second_operand() {
                let iter = "ra 20".split_ascii_whitespace();
                let expected = Ok((Operand::Register(Register::A), Operand::Immediate(20)));
                let actual = ParameterDecoderHelper::try_double_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn immediate_value_in_first_operand_and_register_in_second_operand() {
                let iter = "10 rb".split_ascii_whitespace();
                let expected = Ok((Operand::Immediate(10), Operand::Register(Register::B)));
                let actual = ParameterDecoderHelper::try_double_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }
        }
    }
}

#[cfg(test)]
mod regression {
    use crate::{error::DecodeError, instruction::Instruction, Interpreter};

    #[test]
    fn unknown_instruction() {
        let mut i = Interpreter::new_test();
        let instruction = "hello";
        let expected = Err(DecodeError::UnknownInstruction(instruction.to_string()));

        let actual = i.decode(instruction);

        assert_eq!(actual, expected)
    }

    #[test]
    fn stop() -> Result<(), DecodeError> {
        let mut i = Interpreter::new_test();
        let instruction = "stop";
        let expected = Instruction::Stop;

        let actual = i.decode(instruction)?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn ret() -> Result<(), DecodeError> {
        let mut i = Interpreter::new_test();
        let instruction = "ret";
        let expected = Instruction::Return;

        let actual = i.decode(instruction)?;

        assert_eq!(actual, expected);

        Ok(())
    }

    mod call_regression {
        use crate::{
            error::DecodeError, instruction::Instruction, operand::Operand, register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "call";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "call rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let mut i = Interpreter::new_test();
            let instruction = "call -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let mut i = Interpreter::new_test();
            let instruction = "call 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "call ra";
            let expected = Instruction::Call(Operand::Register(Register::A));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "call 10";
            let expected = Instruction::Call(Operand::Immediate(10));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }
}
