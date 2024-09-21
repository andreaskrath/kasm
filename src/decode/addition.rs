use super::ParameterDecoderHelper;
use crate::{
    constant::Parameters,
    error::DecodeError,
    instruction::{Addition, Instruction},
};

pub struct AdditionParameterDecoder;

impl AdditionParameterDecoder {
    pub fn byte(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let (register, operand) = ParameterDecoderHelper::try_register_and_operand(parameters)?;
        let instruction = Addition::Byte(register, operand);

        Ok(Instruction::Addition(instruction))
    }

    pub fn quarter(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let (register, operand) = ParameterDecoderHelper::try_register_and_operand(parameters)?;
        let instruction = Addition::Quarter(register, operand);

        Ok(Instruction::Addition(instruction))
    }

    pub fn half(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let (register, operand) = ParameterDecoderHelper::try_register_and_operand(parameters)?;
        let instruction = Addition::Half(register, operand);

        Ok(Instruction::Addition(instruction))
    }

    pub fn word(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let (register, operand) = ParameterDecoderHelper::try_register_and_operand(parameters)?;
        let instruction = Addition::Word(register, operand);

        Ok(Instruction::Addition(instruction))
    }
}

#[cfg(test)]
mod regression {
    mod byte {
        use crate::{
            decode::decode,
            error::DecodeError,
            instruction::{Addition, Instruction},
            operand::Operand,
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let instruction = "addb";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let instruction = "addb ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "addb rx 1";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_operand() {
            let instruction = "addb ra rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let instruction = "addb ra -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let instruction = "addb ra 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn second_param_register() -> Result<(), DecodeError> {
            let instruction = "addb ra rb";
            let expected =
                Instruction::Addition(Addition::Byte(Register::A, Operand::Register(Register::B)));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn second_param_immediate() -> Result<(), DecodeError> {
            let instruction = "addb ra 20";
            let expected =
                Instruction::Addition(Addition::Byte(Register::A, Operand::Immediate(20)));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod quarter {
        use crate::{
            decode::decode,
            error::DecodeError,
            instruction::{Addition, Instruction},
            operand::Operand,
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let instruction = "addb";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let instruction = "addq ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "addq rx 1";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_operand() {
            let instruction = "addq ra rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let instruction = "addq ra -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let instruction = "addq ra 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn second_param_register() -> Result<(), DecodeError> {
            let instruction = "addq ra rb";
            let expected = Instruction::Addition(Addition::Quarter(
                Register::A,
                Operand::Register(Register::B),
            ));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn second_param_immediate() -> Result<(), DecodeError> {
            let instruction = "addq ra 20";
            let expected =
                Instruction::Addition(Addition::Quarter(Register::A, Operand::Immediate(20)));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod half {
        use crate::{
            decode::decode,
            error::DecodeError,
            instruction::{Addition, Instruction},
            operand::Operand,
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let instruction = "addh";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let instruction = "addh ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "addh rx 1";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_operand() {
            let instruction = "addh ra rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let instruction = "addh ra -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let instruction = "addh ra 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn second_param_register() -> Result<(), DecodeError> {
            let instruction = "addh ra rb";
            let expected =
                Instruction::Addition(Addition::Half(Register::A, Operand::Register(Register::B)));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn second_param_immediate() -> Result<(), DecodeError> {
            let instruction = "addh ra 20";
            let expected =
                Instruction::Addition(Addition::Half(Register::A, Operand::Immediate(20)));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod word {
        use crate::{
            decode::decode,
            error::DecodeError,
            instruction::{Addition, Instruction},
            operand::Operand,
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let instruction = "addw";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let instruction = "addw ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "addw rx 1";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_operand() {
            let instruction = "addw ra rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let instruction = "addw ra -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let instruction = "addw ra 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn second_param_register() -> Result<(), DecodeError> {
            let instruction = "addw ra rb";
            let expected =
                Instruction::Addition(Addition::Word(Register::A, Operand::Register(Register::B)));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn second_param_immediate() -> Result<(), DecodeError> {
            let instruction = "addw ra 20";
            let expected =
                Instruction::Addition(Addition::Word(Register::A, Operand::Immediate(20)));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }
}
