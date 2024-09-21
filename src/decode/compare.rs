use super::ParameterDecoderHelper;
use crate::{
    constant::Parameters,
    error::DecodeError,
    instruction::{Compare, Instruction},
};

pub struct CompareParameterDecoder;

impl CompareParameterDecoder {
    pub fn byte(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let (operand1, operand2) = ParameterDecoderHelper::try_double_operand(parameters)?;
        let instruction = Compare::Byte(operand1, operand2);

        Ok(Instruction::Compare(instruction))
    }

    pub fn quarter(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let (operand1, operand2) = ParameterDecoderHelper::try_double_operand(parameters)?;
        let instruction = Compare::Quarter(operand1, operand2);

        Ok(Instruction::Compare(instruction))
    }

    pub fn half(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let (operand1, operand2) = ParameterDecoderHelper::try_double_operand(parameters)?;
        let instruction = Compare::Half(operand1, operand2);

        Ok(Instruction::Compare(instruction))
    }

    pub fn word(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let (operand1, operand2) = ParameterDecoderHelper::try_double_operand(parameters)?;
        let instruction = Compare::Word(operand1, operand2);

        Ok(Instruction::Compare(instruction))
    }
}

#[cfg(test)]
mod regression {
    mod byte {
        use crate::{
            decode::decode,
            error::DecodeError,
            instruction::{Compare, Instruction},
            operand::Operand,
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let instruction = "cmpb";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let instruction = "cmpb ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_first_operand() {
            let instruction = "cmpb rx ra";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error_in_first_operand() {
            let instruction = "cmpb -1 ra";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error_in_first_operand() {
            let instruction = "cmpb 200u8 ra";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_second_operand() {
            let instruction = "cmpb ra rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error_in_second_operand() {
            let instruction = "cmpb ra -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error_in_second_operand() {
            let instruction = "cmpb ra 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn first_param_register_and_second_param_register() -> Result<(), DecodeError> {
            let instruction = "cmpb ra rb";
            let expected = Instruction::Compare(Compare::Byte(
                Operand::Register(Register::A),
                Operand::Register(Register::B),
            ));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn first_param_immediate_and_second_param_immediate() -> Result<(), DecodeError> {
            let instruction = "cmpb 10 20";
            let expected = Instruction::Compare(Compare::Byte(
                Operand::Immediate(10),
                Operand::Immediate(20),
            ));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn first_param_register_and_second_param_immediate() -> Result<(), DecodeError> {
            let instruction = "cmpb ra 20";
            let expected = Instruction::Compare(Compare::Byte(
                Operand::Register(Register::A),
                Operand::Immediate(20),
            ));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn first_param_immediate_and_second_param_register() -> Result<(), DecodeError> {
            let instruction = "cmpb 10 ra";
            let expected = Instruction::Compare(Compare::Byte(
                Operand::Immediate(10),
                Operand::Register(Register::A),
            ));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod quarter {
        use crate::{
            decode::decode,
            error::DecodeError,
            instruction::{Compare, Instruction},
            operand::Operand,
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let instruction = "cmpq";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let instruction = "cmpq ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_first_operand() {
            let instruction = "cmpq rx ra";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error_in_first_operand() {
            let instruction = "cmpq -1 ra";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error_in_first_operand() {
            let instruction = "cmpq 200u8 ra";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_second_operand() {
            let instruction = "cmpq ra rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error_in_second_operand() {
            let instruction = "cmpq ra -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error_in_second_operand() {
            let instruction = "cmpq ra 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn first_param_register_and_second_param_register() -> Result<(), DecodeError> {
            let instruction = "cmpq ra rb";
            let expected = Instruction::Compare(Compare::Quarter(
                Operand::Register(Register::A),
                Operand::Register(Register::B),
            ));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn first_param_immediate_and_second_param_immediate() -> Result<(), DecodeError> {
            let instruction = "cmpq 10 20";
            let expected = Instruction::Compare(Compare::Quarter(
                Operand::Immediate(10),
                Operand::Immediate(20),
            ));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn first_param_register_and_second_param_immediate() -> Result<(), DecodeError> {
            let instruction = "cmpq ra 20";
            let expected = Instruction::Compare(Compare::Quarter(
                Operand::Register(Register::A),
                Operand::Immediate(20),
            ));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn first_param_immediate_and_second_param_register() -> Result<(), DecodeError> {
            let instruction = "cmpq 10 ra";
            let expected = Instruction::Compare(Compare::Quarter(
                Operand::Immediate(10),
                Operand::Register(Register::A),
            ));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod half {
        use crate::{
            decode::decode,
            error::DecodeError,
            instruction::{Compare, Instruction},
            operand::Operand,
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let instruction = "cmph";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let instruction = "cmph ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_first_operand() {
            let instruction = "cmph rx ra";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error_in_first_operand() {
            let instruction = "cmph -1 ra";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error_in_first_operand() {
            let instruction = "cmph 200u8 ra";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_second_operand() {
            let instruction = "cmph ra rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error_in_second_operand() {
            let instruction = "cmph ra -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error_in_second_operand() {
            let instruction = "cmph ra 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn first_param_register_and_second_param_register() -> Result<(), DecodeError> {
            let instruction = "cmph ra rb";
            let expected = Instruction::Compare(Compare::Half(
                Operand::Register(Register::A),
                Operand::Register(Register::B),
            ));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn first_param_immediate_and_second_param_immediate() -> Result<(), DecodeError> {
            let instruction = "cmph 10 20";
            let expected = Instruction::Compare(Compare::Half(
                Operand::Immediate(10),
                Operand::Immediate(20),
            ));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn first_param_register_and_second_param_immediate() -> Result<(), DecodeError> {
            let instruction = "cmph ra 20";
            let expected = Instruction::Compare(Compare::Half(
                Operand::Register(Register::A),
                Operand::Immediate(20),
            ));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn first_param_immediate_and_second_param_register() -> Result<(), DecodeError> {
            let instruction = "cmph 10 ra";
            let expected = Instruction::Compare(Compare::Half(
                Operand::Immediate(10),
                Operand::Register(Register::A),
            ));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod word {
        use crate::{
            decode::decode,
            error::DecodeError,
            instruction::{Compare, Instruction},
            operand::Operand,
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let instruction = "cmpw";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let instruction = "cmpw ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_first_operand() {
            let instruction = "cmpw rx ra";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error_in_first_operand() {
            let instruction = "cmpw -1 ra";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error_in_first_operand() {
            let instruction = "cmpw 200u8 ra";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_second_operand() {
            let instruction = "cmpw ra rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error_in_second_operand() {
            let instruction = "cmpw ra -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error_in_second_operand() {
            let instruction = "cmpw ra 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn first_param_register_and_second_param_register() -> Result<(), DecodeError> {
            let instruction = "cmpw ra rb";
            let expected = Instruction::Compare(Compare::Word(
                Operand::Register(Register::A),
                Operand::Register(Register::B),
            ));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn first_param_immediate_and_second_param_immediate() -> Result<(), DecodeError> {
            let instruction = "cmpw 10 20";
            let expected = Instruction::Compare(Compare::Word(
                Operand::Immediate(10),
                Operand::Immediate(20),
            ));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn first_param_register_and_second_param_immediate() -> Result<(), DecodeError> {
            let instruction = "cmpw ra 20";
            let expected = Instruction::Compare(Compare::Word(
                Operand::Register(Register::A),
                Operand::Immediate(20),
            ));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn first_param_immediate_and_second_param_register() -> Result<(), DecodeError> {
            let instruction = "cmpw 10 ra";
            let expected = Instruction::Compare(Compare::Word(
                Operand::Immediate(10),
                Operand::Register(Register::A),
            ));

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }
}
