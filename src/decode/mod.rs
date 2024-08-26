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
    "prb" => PrintRegisterDecoder::print_register_byte,
    "prq" => PrintRegisterDecoder::print_register_quarter,
    "prh" => PrintRegisterDecoder::print_register_half,
    "prw" => PrintRegisterDecoder::print_register_word,
    "psb" => PrintStackDecoder::print_stack_byte,
    "psq" => PrintStackDecoder::print_stack_quarter,
    "psh" => PrintStackDecoder::print_stack_half,
    "psw" => PrintStackDecoder::print_stack_word,
    "pss" => PrintStackDecoder::print_stack_str,
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

#[cfg(test)]
mod ungrouped_regression {
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
}

#[cfg(test)]
mod set_regression {
    mod byte {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, Set},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let mut i = Interpreter::new_test();
            let instruction = "setb";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let mut i = Interpreter::new_test();
            let instruction = "setb ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn second_param_register() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "setb ra rb";
            let expected = Instruction::Set(Set::Byte(Register::A, Operand::Register(Register::B)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn second_param_immediate() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "setb ra 20";
            let expected = Instruction::Set(Set::Byte(Register::A, Operand::Immediate(20)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod quarter {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, Set},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let mut i = Interpreter::new_test();
            let instruction = "setq";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let mut i = Interpreter::new_test();
            let instruction = "setq ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn second_param_register() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "setq ra rb";
            let expected =
                Instruction::Set(Set::Quarter(Register::A, Operand::Register(Register::B)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn second_param_immediate() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "setq ra 20";
            let expected = Instruction::Set(Set::Quarter(Register::A, Operand::Immediate(20)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod half {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, Set},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let mut i = Interpreter::new_test();
            let instruction = "seth";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let mut i = Interpreter::new_test();
            let instruction = "seth ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn second_param_register() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "seth ra rb";
            let expected = Instruction::Set(Set::Half(Register::A, Operand::Register(Register::B)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn second_param_immediate() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "seth ra 20";
            let expected = Instruction::Set(Set::Half(Register::A, Operand::Immediate(20)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod word {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, Set},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let mut i = Interpreter::new_test();
            let instruction = "setw";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let mut i = Interpreter::new_test();
            let instruction = "setw ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn second_param_register() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "setw ra rb";
            let expected = Instruction::Set(Set::Word(Register::A, Operand::Register(Register::B)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn second_param_immediate() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "setw ra 20";
            let expected = Instruction::Set(Set::Word(Register::A, Operand::Immediate(20)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }
}

#[cfg(test)]
mod addition_regression {
    mod byte {
        use crate::{
            error::DecodeError,
            instruction::{Addition, Instruction},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let mut i = Interpreter::new_test();
            let instruction = "addb";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let mut i = Interpreter::new_test();
            let instruction = "addb ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn second_param_register() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "addb ra rb";
            let expected =
                Instruction::Addition(Addition::Byte(Register::A, Operand::Register(Register::B)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn second_param_immediate() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "addb ra 20";
            let expected =
                Instruction::Addition(Addition::Byte(Register::A, Operand::Immediate(20)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod quarter {
        use crate::{
            error::DecodeError,
            instruction::{Addition, Instruction},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let mut i = Interpreter::new_test();
            let instruction = "addb";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let mut i = Interpreter::new_test();
            let instruction = "addq ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn second_param_register() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "addq ra rb";
            let expected = Instruction::Addition(Addition::Quarter(
                Register::A,
                Operand::Register(Register::B),
            ));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn second_param_immediate() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "addq ra 20";
            let expected =
                Instruction::Addition(Addition::Quarter(Register::A, Operand::Immediate(20)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod half {
        use crate::{
            error::DecodeError,
            instruction::{Addition, Instruction},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let mut i = Interpreter::new_test();
            let instruction = "addh";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let mut i = Interpreter::new_test();
            let instruction = "addh ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn second_param_register() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "addh ra rb";
            let expected =
                Instruction::Addition(Addition::Half(Register::A, Operand::Register(Register::B)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn second_param_immediate() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "addh ra 20";
            let expected =
                Instruction::Addition(Addition::Half(Register::A, Operand::Immediate(20)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod word {
        use crate::{
            error::DecodeError,
            instruction::{Addition, Instruction},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let mut i = Interpreter::new_test();
            let instruction = "addw";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let mut i = Interpreter::new_test();
            let instruction = "addw ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn second_param_register() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "addw ra rb";
            let expected =
                Instruction::Addition(Addition::Word(Register::A, Operand::Register(Register::B)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn second_param_immediate() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "addw ra 20";
            let expected =
                Instruction::Addition(Addition::Word(Register::A, Operand::Immediate(20)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }
}
