use super::DecoderHelper;
use crate::{
    error::DecodeError,
    instruction::{Instruction, Not},
};
use std::str::SplitWhitespace;

pub struct NotDecoder;

impl NotDecoder {
    pub fn not_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let register = DecoderHelper::try_register(iter)?;
        let instruction = Not::Byte(register);

        Ok(Instruction::Not(instruction))
    }

    pub fn not_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let register = DecoderHelper::try_register(iter)?;
        let instruction = Not::Quarter(register);

        Ok(Instruction::Not(instruction))
    }

    pub fn not_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let register = DecoderHelper::try_register(iter)?;
        let instruction = Not::Half(register);

        Ok(Instruction::Not(instruction))
    }

    pub fn not_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let register = DecoderHelper::try_register(iter)?;
        let instruction = Not::Word(register);

        Ok(Instruction::Not(instruction))
    }
}

#[cfg(test)]
mod regression {
    mod byte {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, Not},
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "notb";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "notb rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn valid_register() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "notb ra";
            let expected = Instruction::Not(Not::Byte(Register::A));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod quarter {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, Not},
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "notq";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "notq rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn valid_register() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "notq ra";
            let expected = Instruction::Not(Not::Quarter(Register::A));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod half {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, Not},
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "noth";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "noth rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn valid_register() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "noth ra";
            let expected = Instruction::Not(Not::Half(Register::A));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod word {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, Not},
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "notw";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "notw rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn valid_register() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "notw ra";
            let expected = Instruction::Not(Not::Word(Register::A));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }
}
