use super::ParameterDecoderHelper;
use crate::{
    error::DecodeError,
    instruction::{Instruction, PrintRegister},
    constant::Parameters,
};

pub struct PrintRegisterParameterDecoder;

impl PrintRegisterParameterDecoder {
    pub fn byte(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let register = ParameterDecoderHelper::try_register(parameters)?;
        let instruction = PrintRegister::Byte(register);

        Ok(Instruction::PrintRegister(instruction))
    }

    pub fn quarter(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let register = ParameterDecoderHelper::try_register(parameters)?;
        let instruction = PrintRegister::Quarter(register);

        Ok(Instruction::PrintRegister(instruction))
    }

    pub fn half(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let register = ParameterDecoderHelper::try_register(parameters)?;
        let instruction = PrintRegister::Half(register);

        Ok(Instruction::PrintRegister(instruction))
    }

    pub fn word(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let register = ParameterDecoderHelper::try_register(parameters)?;
        let instruction = PrintRegister::Word(register);

        Ok(Instruction::PrintRegister(instruction))
    }
}

#[cfg(test)]
mod regression {
    mod byte {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, PrintRegister},
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "prrb";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "prrb rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn valid_register() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "prrb ra";
            let expected = Instruction::PrintRegister(PrintRegister::Byte(Register::A));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod quarter {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, PrintRegister},
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "prrq";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "prrq rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn valid_register() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "prrq ra";
            let expected = Instruction::PrintRegister(PrintRegister::Quarter(Register::A));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod half {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, PrintRegister},
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "prrh";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "prrh rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn valid_register() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "prrh ra";
            let expected = Instruction::PrintRegister(PrintRegister::Half(Register::A));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod word {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, PrintRegister},
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "prrw";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "prrw rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn valid_register() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "prrw ra";
            let expected = Instruction::PrintRegister(PrintRegister::Word(Register::A));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }
}
