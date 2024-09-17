use super::ParameterDecoderHelper;
use crate::{
    error::DecodeError,
    instruction::{Instruction, Push},
};
use std::str::SplitWhitespace;

pub struct PushParameterDecoder;

impl PushParameterDecoder {
    pub fn byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let operand = ParameterDecoderHelper::try_operand(iter)?;
        let instruction = Push::Byte(operand);

        Ok(Instruction::Push(instruction))
    }

    pub fn quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let operand = ParameterDecoderHelper::try_operand(iter)?;
        let instruction = Push::Quarter(operand);

        Ok(Instruction::Push(instruction))
    }

    pub fn half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let operand = ParameterDecoderHelper::try_operand(iter)?;
        let instruction = Push::Half(operand);

        Ok(Instruction::Push(instruction))
    }

    pub fn word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let operand = ParameterDecoderHelper::try_operand(iter)?;
        let instruction = Push::Word(operand);

        Ok(Instruction::Push(instruction))
    }
}

#[cfg(test)]
mod regression {
    mod byte {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, Push},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "pshb";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "pshb rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let mut i = Interpreter::new_test();
            let instruction = "pshb -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let mut i = Interpreter::new_test();
            let instruction = "pshb 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "pshb ra";
            let expected = Instruction::Push(Push::Byte(Operand::Register(Register::A)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "pshb 10";
            let expected = Instruction::Push(Push::Byte(Operand::Immediate(10)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod quarter {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, Push},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "pshq";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "pshq rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let mut i = Interpreter::new_test();
            let instruction = "pshq -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let mut i = Interpreter::new_test();
            let instruction = "pshq 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "pshq ra";
            let expected = Instruction::Push(Push::Quarter(Operand::Register(Register::A)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "pshq 10";
            let expected = Instruction::Push(Push::Quarter(Operand::Immediate(10)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod half {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, Push},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "pshh";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "pshh rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let mut i = Interpreter::new_test();
            let instruction = "pshh -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let mut i = Interpreter::new_test();
            let instruction = "pshh 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "pshh ra";
            let expected = Instruction::Push(Push::Half(Operand::Register(Register::A)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "pshh 10";
            let expected = Instruction::Push(Push::Half(Operand::Immediate(10)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod word {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, Push},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "pshw";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "pshw rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let mut i = Interpreter::new_test();
            let instruction = "pshw -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let mut i = Interpreter::new_test();
            let instruction = "pshw 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "pshw ra";
            let expected = Instruction::Push(Push::Word(Operand::Register(Register::A)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "pshw 10";
            let expected = Instruction::Push(Push::Word(Operand::Immediate(10)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }
}
