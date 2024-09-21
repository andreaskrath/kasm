use super::ParameterDecoderHelper;
use crate::{
    constant::Parameters,
    error::DecodeError,
    instruction::{Instruction, Xor},
};

pub struct XorParameterDecoder;

impl XorParameterDecoder {
    pub fn byte(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let (register, operand) = ParameterDecoderHelper::try_register_and_operand(parameters)?;
        let instruction = Xor::Byte(register, operand);

        Ok(Instruction::Xor(instruction))
    }

    pub fn quarter(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let (register, operand) = ParameterDecoderHelper::try_register_and_operand(parameters)?;
        let instruction = Xor::Quarter(register, operand);

        Ok(Instruction::Xor(instruction))
    }

    pub fn half(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let (register, operand) = ParameterDecoderHelper::try_register_and_operand(parameters)?;
        let instruction = Xor::Half(register, operand);

        Ok(Instruction::Xor(instruction))
    }

    pub fn word(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let (register, operand) = ParameterDecoderHelper::try_register_and_operand(parameters)?;
        let instruction = Xor::Word(register, operand);

        Ok(Instruction::Xor(instruction))
    }
}

#[cfg(test)]
mod regression {
    mod byte {
        use crate::{
            decode::decode,
            error::DecodeError,
            instruction::{Instruction, Xor},
            operand::Operand,
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let instruction = "xorb";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let instruction = "xorb ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "xorb rx 1";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_operand() {
            let instruction = "xorb ra rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let instruction = "xorb ra -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let instruction = "xorb ra 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn second_param_register() -> Result<(), DecodeError> {
            let instruction = "xorb ra rb";
            let expected = Instruction::Xor(Xor::Byte(Register::A, Operand::Register(Register::B)));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn second_param_immediate() -> Result<(), DecodeError> {
            let instruction = "xorb ra 20";
            let expected = Instruction::Xor(Xor::Byte(Register::A, Operand::Immediate(20)));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod quarter {
        use crate::{
            decode::decode,
            error::DecodeError,
            instruction::{Instruction, Xor},
            operand::Operand,
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let instruction = "xorq";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let instruction = "xorq ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "xorq rx 1";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_operand() {
            let instruction = "xorq ra rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let instruction = "xorq ra -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let instruction = "xorq ra 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn second_param_register() -> Result<(), DecodeError> {
            let instruction = "xorq ra rb";
            let expected =
                Instruction::Xor(Xor::Quarter(Register::A, Operand::Register(Register::B)));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn second_param_immediate() -> Result<(), DecodeError> {
            let instruction = "xorq ra 20";
            let expected = Instruction::Xor(Xor::Quarter(Register::A, Operand::Immediate(20)));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod half {
        use crate::{
            decode::decode,
            error::DecodeError,
            instruction::{Instruction, Xor},
            operand::Operand,
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let instruction = "xorh";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let instruction = "xorh ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "xorh rx 1";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_operand() {
            let instruction = "xorh ra rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let instruction = "xorh ra -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let instruction = "xorh ra 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn second_param_register() -> Result<(), DecodeError> {
            let instruction = "xorh ra rb";
            let expected = Instruction::Xor(Xor::Half(Register::A, Operand::Register(Register::B)));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn second_param_immediate() -> Result<(), DecodeError> {
            let instruction = "xorh ra 20";
            let expected = Instruction::Xor(Xor::Half(Register::A, Operand::Immediate(20)));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod word {
        use crate::{
            decode::decode,
            error::DecodeError,
            instruction::{Instruction, Xor},
            operand::Operand,
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let instruction = "xorw";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let instruction = "xorw ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "xorw rx 1";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_operand() {
            let instruction = "xorw ra rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let instruction = "xorw ra -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let instruction = "xorw ra 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn second_param_register() -> Result<(), DecodeError> {
            let instruction = "xorw ra rb";
            let expected = Instruction::Xor(Xor::Word(Register::A, Operand::Register(Register::B)));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn second_param_immediate() -> Result<(), DecodeError> {
            let instruction = "xorw ra 20";
            let expected = Instruction::Xor(Xor::Word(Register::A, Operand::Immediate(20)));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }
}
