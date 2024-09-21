use super::ParameterDecoderHelper;
use crate::{
    constant::Parameters,
    error::DecodeError,
    instruction::{Instruction, Subtraction},
};

pub struct SubtractionParameterDecoder;

impl SubtractionParameterDecoder {
    pub fn byte(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let (register, operand) = ParameterDecoderHelper::try_register_and_operand(parameters)?;
        let instruction = Subtraction::Byte(register, operand);

        Ok(Instruction::Subtraction(instruction))
    }

    pub fn quarter(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let (register, operand) = ParameterDecoderHelper::try_register_and_operand(parameters)?;
        let instruction = Subtraction::Quarter(register, operand);

        Ok(Instruction::Subtraction(instruction))
    }

    pub fn half(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let (register, operand) = ParameterDecoderHelper::try_register_and_operand(parameters)?;
        let instruction = Subtraction::Half(register, operand);

        Ok(Instruction::Subtraction(instruction))
    }

    pub fn word(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let (register, operand) = ParameterDecoderHelper::try_register_and_operand(parameters)?;
        let instruction = Subtraction::Word(register, operand);

        Ok(Instruction::Subtraction(instruction))
    }
}

#[cfg(test)]
mod regression {
    mod byte {
        use crate::{
            decode::decode,
            error::DecodeError,
            instruction::{Instruction, Subtraction},
            operand::Operand,
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let instruction = "subb";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let instruction = "subb ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "subb rx 1";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_operand() {
            let instruction = "subb ra rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let instruction = "subb ra -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let instruction = "subb ra 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn second_param_register() -> Result<(), DecodeError> {
            let instruction = "subb ra rb";
            let expected = Instruction::Subtraction(Subtraction::Byte(
                Register::A,
                Operand::Register(Register::B),
            ));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn second_param_immediate() -> Result<(), DecodeError> {
            let instruction = "subb ra 20";
            let expected =
                Instruction::Subtraction(Subtraction::Byte(Register::A, Operand::Immediate(20)));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod quarter {
        use crate::{
            decode::decode,
            error::DecodeError,
            instruction::{Instruction, Subtraction},
            operand::Operand,
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let instruction = "subq";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let instruction = "subq ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "subq rx 1";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_operand() {
            let instruction = "subq ra rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let instruction = "subq ra -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let instruction = "subq ra 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn second_param_register() -> Result<(), DecodeError> {
            let instruction = "subq ra rb";
            let expected = Instruction::Subtraction(Subtraction::Quarter(
                Register::A,
                Operand::Register(Register::B),
            ));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn second_param_immediate() -> Result<(), DecodeError> {
            let instruction = "subq ra 20";
            let expected =
                Instruction::Subtraction(Subtraction::Quarter(Register::A, Operand::Immediate(20)));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod half {
        use crate::{
            decode::decode,
            error::DecodeError,
            instruction::{Instruction, Subtraction},
            operand::Operand,
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let instruction = "subh";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let instruction = "subh ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "subh rx 1";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_operand() {
            let instruction = "subh ra rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let instruction = "subh ra -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let instruction = "subh ra 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn second_param_register() -> Result<(), DecodeError> {
            let instruction = "subh ra rb";
            let expected = Instruction::Subtraction(Subtraction::Half(
                Register::A,
                Operand::Register(Register::B),
            ));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn second_param_immediate() -> Result<(), DecodeError> {
            let instruction = "subh ra 20";
            let expected =
                Instruction::Subtraction(Subtraction::Half(Register::A, Operand::Immediate(20)));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod word {
        use crate::{
            decode::decode,
            error::DecodeError,
            instruction::{Instruction, Subtraction},
            operand::Operand,
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let instruction = "subw";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let instruction = "subw ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "subw rx 1";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_operand() {
            let instruction = "subw ra rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let instruction = "subw ra -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let instruction = "subw ra 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn second_param_register() -> Result<(), DecodeError> {
            let instruction = "subw ra rb";
            let expected = Instruction::Subtraction(Subtraction::Word(
                Register::A,
                Operand::Register(Register::B),
            ));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn second_param_immediate() -> Result<(), DecodeError> {
            let instruction = "subw ra 20";
            let expected =
                Instruction::Subtraction(Subtraction::Word(Register::A, Operand::Immediate(20)));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }
}
