use super::ParameterDecoderHelper;
use crate::{
    constant::Parameters,
    error::DecodeError,
    instruction::{Instruction, Pop},
};

pub struct PopParameterDecoder;

impl PopParameterDecoder {
    pub fn byte(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let register = ParameterDecoderHelper::try_register(parameters)?;
        let instruction = Pop::Byte(register);

        Ok(Instruction::Pop(instruction))
    }

    pub fn quarter(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let register = ParameterDecoderHelper::try_register(parameters)?;
        let instruction = Pop::Quarter(register);

        Ok(Instruction::Pop(instruction))
    }

    pub fn half(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let register = ParameterDecoderHelper::try_register(parameters)?;
        let instruction = Pop::Half(register);

        Ok(Instruction::Pop(instruction))
    }

    pub fn word(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let register = ParameterDecoderHelper::try_register(parameters)?;
        let instruction = Pop::Word(register);

        Ok(Instruction::Pop(instruction))
    }
}

#[cfg(test)]
mod regression {
    mod byte {
        use crate::{
            decode::decode,
            error::DecodeError,
            instruction::{Instruction, Pop},
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let instruction = "popb";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "popb rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn valid_register() -> Result<(), DecodeError> {
            let instruction = "popb ra";
            let expected = Instruction::Pop(Pop::Byte(Register::A));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod quarter {
        use crate::{
            decode::decode,
            error::DecodeError,
            instruction::{Instruction, Pop},
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let instruction = "popq";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "popq rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn valid_register() -> Result<(), DecodeError> {
            let instruction = "popq ra";
            let expected = Instruction::Pop(Pop::Quarter(Register::A));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod half {
        use crate::{
            decode::decode,
            error::DecodeError,
            instruction::{Instruction, Pop},
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let instruction = "poph";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "poph rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn valid_register() -> Result<(), DecodeError> {
            let instruction = "poph ra";
            let expected = Instruction::Pop(Pop::Half(Register::A));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod word {
        use crate::{
            decode::decode,
            error::DecodeError,
            instruction::{Instruction, Pop},
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let instruction = "popw";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "popw rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn valid_register() -> Result<(), DecodeError> {
            let instruction = "popw ra";
            let expected = Instruction::Pop(Pop::Word(Register::A));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }
}
