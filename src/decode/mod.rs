use crate::{
    constant::DecodeTable, error::DecodeError, instruction::Instruction, operand::Operand,
    register::Register, Interpreter,
};
use addition::AddDecoder;
use and::AndDecoder;
use compare::CompareDecoder;
use division::DivDecoder;
use jump::JumpDecoder;
use multiplication::MulDecoder;
use not::NotDecoder;
use or::OrDecoder;
use phf::phf_map;
use pop::PopDecoder;
use print_register::PrintRegisterDecoder;
use print_stack::PrintStackDecoder;
use push::PushDecoder;
use remainder::RemDecoder;
use set::SetDecoder;
use std::str::{FromStr, SplitWhitespace};
use subtraction::SubDecoder;
use test::TestDecoder;
use xor::XorDecoder;

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
    pub fn decode(&mut self, s: &str) -> Result<Instruction, DecodeError> {
        let mut s_iter = s.split_whitespace();
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
    "orb" => OrDecoder::or_byte,
    "orq" => OrDecoder::or_quarter,
    "orh" => OrDecoder::or_half,
    "orw" => OrDecoder::or_word,
    "xorb" => XorDecoder::xor_byte,
    "xorq" => XorDecoder::xor_quarter,
    "xorh" => XorDecoder::xor_half,
    "xorw" => XorDecoder::xor_word,
    "notb" => NotDecoder::not_byte,
    "notq" => NotDecoder::not_quarter,
    "noth" => NotDecoder::not_half,
    "notw" => NotDecoder::not_word,
    "tstb" => TestDecoder::test_byte,
    "tstq" => TestDecoder::test_quarter,
    "tsth" => TestDecoder::test_half,
    "tstw" => TestDecoder::test_word,
    "cmpb" => CompareDecoder::compare_byte,
    "cmpq" => CompareDecoder::compare_quarter,
    "cmph" => CompareDecoder::compare_half,
    "cmpw" => CompareDecoder::compare_word,
    "jmp" => JumpDecoder::jump_unconditional,
    "jiz" => JumpDecoder::jump_if_zero,
    "jnz" => JumpDecoder::jump_if_not_zero,
    "jis" => JumpDecoder::jump_if_sign,
    "jns" => JumpDecoder::jump_if_not_sign,
    "jio" => JumpDecoder::jump_if_overflow,
    "jno" => JumpDecoder::jump_if_not_overflow,
    "jig" => JumpDecoder::jump_if_greater,
    "jil" => JumpDecoder::jump_if_lesser,
    "jge" => JumpDecoder::jump_if_greater_or_equal,
    "jle" => JumpDecoder::jump_if_lesser_or_equal,
    "prrb" => PrintRegisterDecoder::print_register_byte,
    "prrq" => PrintRegisterDecoder::print_register_quarter,
    "prrh" => PrintRegisterDecoder::print_register_half,
    "prrw" => PrintRegisterDecoder::print_register_word,
    "prsb" => PrintStackDecoder::print_stack_byte,
    "prsq" => PrintStackDecoder::print_stack_quarter,
    "prsh" => PrintStackDecoder::print_stack_half,
    "prsw" => PrintStackDecoder::print_stack_word,
    "prss" => PrintStackDecoder::print_stack_str,
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
    fn try_register(iter: SplitWhitespace) -> Result<Register, DecodeError> {
        let s_register = get_first_parameter_str(iter)?;
        let register = Register::try_from(s_register)?;

        Ok(register)
    }

    fn try_operand<T>(iter: SplitWhitespace) -> Result<Operand<T>, DecodeError>
    where
        T: FromStr,
    {
        let s_operand = get_first_parameter_str(iter)?;
        let operand = Operand::try_from(s_operand)?;

        Ok(operand)
    }

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

    fn try_double_operand<T>(iter: SplitWhitespace) -> Result<(Operand<T>, Operand<T>), DecodeError>
    where
        T: FromStr,
    {
        let (s_operand1, s_operand2) = get_both_parameters_str(iter)?;
        let operand1 = Operand::try_from(s_operand1)?;
        let operand2 = Operand::try_from(s_operand2)?;

        Ok((operand1, operand2))
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
mod get_both_parameters_str {
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

#[cfg(test)]
mod get_first_parameter_str {
    use super::get_first_parameter_str;
    use crate::error::DecodeError;

    #[test]
    fn empty_parameter() {
        let iter = "".split_whitespace();
        let expected = Err(DecodeError::IncompleteInstruction);
        let actual = get_first_parameter_str(iter);
        assert_eq!(actual, expected);
    }

    #[test]
    fn first_parameter_defined() {
        let iter = "ra".split_whitespace();
        let expected = Ok("ra");
        let actual = get_first_parameter_str(iter);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod decoder_helper {
    mod try_register {
        use crate::{decode::DecoderHelper, error::DecodeError, register::Register};

        #[test]
        fn missing_parameter() {
            let iter = "".split_whitespace();
            let expected = Err(DecodeError::IncompleteInstruction);
            let actual = DecoderHelper::try_register(iter);
            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register() {
            let iter = "rx".split_whitespace();
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
            let actual = DecoderHelper::try_register(iter);
            assert_eq!(actual, expected);
        }

        #[test]
        fn valid_register() {
            let iter = "ra".split_whitespace();
            let expected = Ok(Register::A);
            let actual = DecoderHelper::try_register(iter);
            assert_eq!(actual, expected);
        }
    }

    mod try_operand {
        mod byte {
            use crate::{
                constant::Byte, decode::DecoderHelper, error::DecodeError, operand::Operand,
                register::Register,
            };

            #[test]
            fn missing_parameter() {
                let iter = "".split_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = DecoderHelper::try_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_operand() {
                let iter = "rx".split_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = DecoderHelper::try_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_operand() {
                let iter = "-1".split_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = DecoderHelper::try_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error() {
                let iter = "200u8".split_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = DecoderHelper::try_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_register_in_operand() {
                let iter = "ra".split_whitespace();
                let expected = Ok(Operand::Register(Register::A));
                let actual = DecoderHelper::try_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_immediate_value_in_operand() {
                let iter = "10".split_whitespace();
                let expected = Ok(Operand::Immediate(10));
                let actual = DecoderHelper::try_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }
        }

        mod quarter {
            use crate::{
                constant::Quarter, decode::DecoderHelper, error::DecodeError, operand::Operand,
                register::Register,
            };

            #[test]
            fn missing_parameter() {
                let iter = "".split_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = DecoderHelper::try_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_operand() {
                let iter = "rx".split_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = DecoderHelper::try_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_operand() {
                let iter = "-1".split_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = DecoderHelper::try_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error() {
                let iter = "200u8".split_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = DecoderHelper::try_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_register_in_operand() {
                let iter = "ra".split_whitespace();
                let expected = Ok(Operand::Register(Register::A));
                let actual = DecoderHelper::try_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_immediate_value_in_operand() {
                let iter = "10".split_whitespace();
                let expected = Ok(Operand::Immediate(10));
                let actual = DecoderHelper::try_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }
        }

        mod half {
            use crate::{
                constant::Half, decode::DecoderHelper, error::DecodeError, operand::Operand,
                register::Register,
            };

            #[test]
            fn missing_parameter() {
                let iter = "".split_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = DecoderHelper::try_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_operand() {
                let iter = "rx".split_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = DecoderHelper::try_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_operand() {
                let iter = "-1".split_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = DecoderHelper::try_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error() {
                let iter = "200u8".split_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = DecoderHelper::try_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_register_in_operand() {
                let iter = "ra".split_whitespace();
                let expected = Ok(Operand::Register(Register::A));
                let actual = DecoderHelper::try_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_immediate_value_in_operand() {
                let iter = "10".split_whitespace();
                let expected = Ok(Operand::Immediate(10));
                let actual = DecoderHelper::try_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }
        }

        mod word {
            use crate::{
                constant::Word, decode::DecoderHelper, error::DecodeError, operand::Operand,
                register::Register,
            };

            #[test]
            fn missing_parameter() {
                let iter = "".split_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = DecoderHelper::try_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_operand() {
                let iter = "rx".split_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = DecoderHelper::try_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_operand() {
                let iter = "-1".split_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = DecoderHelper::try_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error() {
                let iter = "200u8".split_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = DecoderHelper::try_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_register_in_operand() {
                let iter = "ra".split_whitespace();
                let expected = Ok(Operand::Register(Register::A));
                let actual = DecoderHelper::try_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_immediate_value_in_operand() {
                let iter = "10".split_whitespace();
                let expected = Ok(Operand::Immediate(10));
                let actual = DecoderHelper::try_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }
        }
    }

    mod try_register_and_operand {
        mod byte {
            use crate::{
                constant::Byte, decode::DecoderHelper, error::DecodeError, operand::Operand,
                register::Register,
            };

            #[test]
            fn missing_both_parameters() {
                let iter = "".split_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = DecoderHelper::try_register_and_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn missing_second_paramter() {
                let iter = "ra".split_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = DecoderHelper::try_register_and_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_first_parameter() {
                let iter = "rx ra".split_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = DecoderHelper::try_register_and_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_operand() {
                let iter = "ra rx".split_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = DecoderHelper::try_register_and_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_operand() {
                let iter = "ra -1".split_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = DecoderHelper::try_register_and_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error() {
                let iter = "ra 200u8".split_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = DecoderHelper::try_register_and_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_register_and_register_in_operand() {
                let iter = "ra rb".split_whitespace();
                let expected = Ok((Register::A, Operand::Register(Register::B)));
                let actual = DecoderHelper::try_register_and_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_register_and_immediate_value_in_operand() {
                let iter = "ra 10".split_whitespace();
                let expected = Ok((Register::A, Operand::Immediate(10)));
                let actual = DecoderHelper::try_register_and_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }
        }

        mod quarter {
            use crate::{
                constant::Quarter, decode::DecoderHelper, error::DecodeError, operand::Operand,
                register::Register,
            };

            #[test]
            fn missing_both_parameters() {
                let iter = "".split_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = DecoderHelper::try_register_and_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn missing_second_paramter() {
                let iter = "ra".split_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = DecoderHelper::try_register_and_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_first_parameter() {
                let iter = "rx ra".split_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = DecoderHelper::try_register_and_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_operand() {
                let iter = "ra rx".split_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = DecoderHelper::try_register_and_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_operand() {
                let iter = "ra -1".split_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = DecoderHelper::try_register_and_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error() {
                let iter = "ra 200u8".split_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = DecoderHelper::try_register_and_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_register_and_register_in_operand() {
                let iter = "ra rb".split_whitespace();
                let expected = Ok((Register::A, Operand::Register(Register::B)));
                let actual = DecoderHelper::try_register_and_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_register_and_immediate_value_in_operand() {
                let iter = "ra 10".split_whitespace();
                let expected = Ok((Register::A, Operand::Immediate(10)));
                let actual = DecoderHelper::try_register_and_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }
        }

        mod half {
            use crate::{
                constant::Half, decode::DecoderHelper, error::DecodeError, operand::Operand,
                register::Register,
            };

            #[test]
            fn missing_both_parameters() {
                let iter = "".split_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = DecoderHelper::try_register_and_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn missing_second_paramter() {
                let iter = "ra".split_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = DecoderHelper::try_register_and_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_first_parameter() {
                let iter = "rx ra".split_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = DecoderHelper::try_register_and_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_operand() {
                let iter = "ra rx".split_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = DecoderHelper::try_register_and_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_operand() {
                let iter = "ra -1".split_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = DecoderHelper::try_register_and_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error() {
                let iter = "ra 200u8".split_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = DecoderHelper::try_register_and_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_register_and_register_in_operand() {
                let iter = "ra rb".split_whitespace();
                let expected = Ok((Register::A, Operand::Register(Register::B)));
                let actual = DecoderHelper::try_register_and_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_register_and_immediate_value_in_operand() {
                let iter = "ra 10".split_whitespace();
                let expected = Ok((Register::A, Operand::Immediate(10)));
                let actual = DecoderHelper::try_register_and_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }
        }

        mod word {
            use crate::{
                constant::Word, decode::DecoderHelper, error::DecodeError, operand::Operand,
                register::Register,
            };

            #[test]
            fn missing_both_parameters() {
                let iter = "".split_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = DecoderHelper::try_register_and_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn missing_second_paramter() {
                let iter = "ra".split_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = DecoderHelper::try_register_and_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_first_parameter() {
                let iter = "rx ra".split_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = DecoderHelper::try_register_and_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_operand() {
                let iter = "ra rx".split_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = DecoderHelper::try_register_and_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_operand() {
                let iter = "ra -1".split_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = DecoderHelper::try_register_and_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error() {
                let iter = "ra 200u8".split_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = DecoderHelper::try_register_and_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_register_and_register_in_operand() {
                let iter = "ra rb".split_whitespace();
                let expected = Ok((Register::A, Operand::Register(Register::B)));
                let actual = DecoderHelper::try_register_and_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn valid_register_and_immediate_value_in_operand() {
                let iter = "ra 10".split_whitespace();
                let expected = Ok((Register::A, Operand::Immediate(10)));
                let actual = DecoderHelper::try_register_and_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }
        }
    }

    mod try_double_operand {
        mod byte {
            use crate::{
                constant::Byte, decode::DecoderHelper, error::DecodeError, operand::Operand,
                register::Register,
            };

            #[test]
            fn missing_both_parameters() {
                let iter = "".split_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = DecoderHelper::try_double_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn missing_second_parameter() {
                let iter = "ra".split_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = DecoderHelper::try_double_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_first_operand() {
                let iter = "rx ra".split_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = DecoderHelper::try_double_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_first_operand() {
                let iter = "-1 ra".split_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = DecoderHelper::try_double_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error_in_first_operand() {
                let iter = "200u8 ra".split_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = DecoderHelper::try_double_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_second_operand() {
                let iter = "ra rx".split_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = DecoderHelper::try_double_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_second_operand() {
                let iter = "ra -1".split_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = DecoderHelper::try_double_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error_in_second_operand() {
                let iter = "ra 200u8".split_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = DecoderHelper::try_double_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn register_in_both_operands() {
                let iter = "ra rb".split_whitespace();
                let expected = Ok((
                    Operand::Register(Register::A),
                    Operand::Register(Register::B),
                ));
                let actual = DecoderHelper::try_double_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn immediate_value_in_both_operands() {
                let iter = "10 20".split_whitespace();
                let expected = Ok((Operand::Immediate(10), Operand::Immediate(20)));
                let actual = DecoderHelper::try_double_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn register_in_first_operand_and_immediate_value_in_second_operand() {
                let iter = "ra 20".split_whitespace();
                let expected = Ok((Operand::Register(Register::A), Operand::Immediate(20)));
                let actual = DecoderHelper::try_double_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn immediate_value_in_first_operand_and_register_in_second_operand() {
                let iter = "10 rb".split_whitespace();
                let expected = Ok((Operand::Immediate(10), Operand::Register(Register::B)));
                let actual = DecoderHelper::try_double_operand::<Byte>(iter);
                assert_eq!(actual, expected);
            }
        }

        mod quarter {
            use crate::{
                constant::Quarter, decode::DecoderHelper, error::DecodeError, operand::Operand,
                register::Register,
            };

            #[test]
            fn missing_both_parameters() {
                let iter = "".split_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = DecoderHelper::try_double_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn missing_second_parameter() {
                let iter = "ra".split_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = DecoderHelper::try_double_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_first_operand() {
                let iter = "rx ra".split_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = DecoderHelper::try_double_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_first_operand() {
                let iter = "-1 ra".split_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = DecoderHelper::try_double_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error_in_first_operand() {
                let iter = "200u8 ra".split_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = DecoderHelper::try_double_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_second_operand() {
                let iter = "ra rx".split_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = DecoderHelper::try_double_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_second_operand() {
                let iter = "ra -1".split_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = DecoderHelper::try_double_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error_in_second_operand() {
                let iter = "ra 200u8".split_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = DecoderHelper::try_double_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn register_in_both_operands() {
                let iter = "ra rb".split_whitespace();
                let expected = Ok((
                    Operand::Register(Register::A),
                    Operand::Register(Register::B),
                ));
                let actual = DecoderHelper::try_double_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn immediate_value_in_both_operands() {
                let iter = "10 20".split_whitespace();
                let expected = Ok((Operand::Immediate(10), Operand::Immediate(20)));
                let actual = DecoderHelper::try_double_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn register_in_first_operand_and_immediate_value_in_second_operand() {
                let iter = "ra 20".split_whitespace();
                let expected = Ok((Operand::Register(Register::A), Operand::Immediate(20)));
                let actual = DecoderHelper::try_double_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn immediate_value_in_first_operand_and_register_in_second_operand() {
                let iter = "10 rb".split_whitespace();
                let expected = Ok((Operand::Immediate(10), Operand::Register(Register::B)));
                let actual = DecoderHelper::try_double_operand::<Quarter>(iter);
                assert_eq!(actual, expected);
            }
        }

        mod half {
            use crate::{
                constant::Half, decode::DecoderHelper, error::DecodeError, operand::Operand,
                register::Register,
            };

            #[test]
            fn missing_both_parameters() {
                let iter = "".split_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = DecoderHelper::try_double_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn missing_second_parameter() {
                let iter = "ra".split_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = DecoderHelper::try_double_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_first_operand() {
                let iter = "rx ra".split_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = DecoderHelper::try_double_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_first_operand() {
                let iter = "-1 ra".split_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = DecoderHelper::try_double_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error_in_first_operand() {
                let iter = "200u8 ra".split_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = DecoderHelper::try_double_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_second_operand() {
                let iter = "ra rx".split_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = DecoderHelper::try_double_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_second_operand() {
                let iter = "ra -1".split_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = DecoderHelper::try_double_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error_in_second_operand() {
                let iter = "ra 200u8".split_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = DecoderHelper::try_double_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn register_in_both_operands() {
                let iter = "ra rb".split_whitespace();
                let expected = Ok((
                    Operand::Register(Register::A),
                    Operand::Register(Register::B),
                ));
                let actual = DecoderHelper::try_double_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn immediate_value_in_both_operands() {
                let iter = "10 20".split_whitespace();
                let expected = Ok((Operand::Immediate(10), Operand::Immediate(20)));
                let actual = DecoderHelper::try_double_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn register_in_first_operand_and_immediate_value_in_second_operand() {
                let iter = "ra 20".split_whitespace();
                let expected = Ok((Operand::Register(Register::A), Operand::Immediate(20)));
                let actual = DecoderHelper::try_double_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn immediate_value_in_first_operand_and_register_in_second_operand() {
                let iter = "10 rb".split_whitespace();
                let expected = Ok((Operand::Immediate(10), Operand::Register(Register::B)));
                let actual = DecoderHelper::try_double_operand::<Half>(iter);
                assert_eq!(actual, expected);
            }
        }

        mod word {
            use crate::{
                constant::Word, decode::DecoderHelper, error::DecodeError, operand::Operand,
                register::Register,
            };

            #[test]
            fn missing_both_parameters() {
                let iter = "".split_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = DecoderHelper::try_double_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn missing_second_parameter() {
                let iter = "ra".split_whitespace();
                let expected = Err(DecodeError::IncompleteInstruction);
                let actual = DecoderHelper::try_double_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_first_operand() {
                let iter = "rx ra".split_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = DecoderHelper::try_double_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_first_operand() {
                let iter = "-1 ra".split_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = DecoderHelper::try_double_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error_in_first_operand() {
                let iter = "200u8 ra".split_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = DecoderHelper::try_double_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_register_in_second_operand() {
                let iter = "ra rx".split_whitespace();
                let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
                let actual = DecoderHelper::try_double_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_immediate_value_in_second_operand() {
                let iter = "ra -1".split_whitespace();
                let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));
                let actual = DecoderHelper::try_double_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn invalid_operand_error_in_second_operand() {
                let iter = "ra 200u8".split_whitespace();
                let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));
                let actual = DecoderHelper::try_double_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn register_in_both_operands() {
                let iter = "ra rb".split_whitespace();
                let expected = Ok((
                    Operand::Register(Register::A),
                    Operand::Register(Register::B),
                ));
                let actual = DecoderHelper::try_double_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn immediate_value_in_both_operands() {
                let iter = "10 20".split_whitespace();
                let expected = Ok((Operand::Immediate(10), Operand::Immediate(20)));
                let actual = DecoderHelper::try_double_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn register_in_first_operand_and_immediate_value_in_second_operand() {
                let iter = "ra 20".split_whitespace();
                let expected = Ok((Operand::Register(Register::A), Operand::Immediate(20)));
                let actual = DecoderHelper::try_double_operand::<Word>(iter);
                assert_eq!(actual, expected);
            }

            #[test]
            fn immediate_value_in_first_operand_and_register_in_second_operand() {
                let iter = "10 rb".split_whitespace();
                let expected = Ok((Operand::Immediate(10), Operand::Register(Register::B)));
                let actual = DecoderHelper::try_double_operand::<Word>(iter);
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
