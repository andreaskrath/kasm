use super::ParameterDecoderHelper;
use crate::{
    constant::Parameters,
    error::DecodeError,
    instruction::{Instruction, Not},
};

pub struct NotParameterDecoder;

impl NotParameterDecoder {
    pub fn byte(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let register = ParameterDecoderHelper::try_register(parameters)?;
        let instruction = Not::Byte(register);

        Ok(Instruction::Not(instruction))
    }

    pub fn quarter(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let register = ParameterDecoderHelper::try_register(parameters)?;
        let instruction = Not::Quarter(register);

        Ok(Instruction::Not(instruction))
    }

    pub fn half(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let register = ParameterDecoderHelper::try_register(parameters)?;
        let instruction = Not::Half(register);

        Ok(Instruction::Not(instruction))
    }

    pub fn word(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let register = ParameterDecoderHelper::try_register(parameters)?;
        let instruction = Not::Word(register);

        Ok(Instruction::Not(instruction))
    }
}

#[cfg(test)]
mod regression {
    mod byte {
        use crate::{
            decode::decode,
            error::DecodeError,
            instruction::{Instruction, Not},
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let instruction = "notb";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "notb rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn valid_register() -> Result<(), DecodeError> {
            let instruction = "notb ra";
            let expected = Instruction::Not(Not::Byte(Register::A));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod quarter {
        use crate::{
            decode::decode,
            error::DecodeError,
            instruction::{Instruction, Not},
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let instruction = "notq";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "notq rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn valid_register() -> Result<(), DecodeError> {
            let instruction = "notq ra";
            let expected = Instruction::Not(Not::Quarter(Register::A));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod half {
        use crate::{
            decode::decode,
            error::DecodeError,
            instruction::{Instruction, Not},
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let instruction = "noth";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "noth rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn valid_register() -> Result<(), DecodeError> {
            let instruction = "noth ra";
            let expected = Instruction::Not(Not::Half(Register::A));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod word {
        use crate::{
            decode::decode,
            error::DecodeError,
            instruction::{Instruction, Not},
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let instruction = "notw";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "notw rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn valid_register() -> Result<(), DecodeError> {
            let instruction = "notw ra";
            let expected = Instruction::Not(Not::Word(Register::A));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }
}
