use super::DecoderHelper;
use crate::{
    error::DecodeError,
    instruction::{Compare, Instruction},
};
use std::str::SplitWhitespace;

pub struct CompareDecoder;

impl CompareDecoder {
    pub fn compare_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (operand1, operand2) = DecoderHelper::try_double_operand(iter)?;
        let instruction = Compare::Byte(operand1, operand2);

        Ok(Instruction::Compare(instruction))
    }

    pub fn compare_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (operand1, operand2) = DecoderHelper::try_double_operand(iter)?;
        let instruction = Compare::Quarter(operand1, operand2);

        Ok(Instruction::Compare(instruction))
    }

    pub fn compare_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (operand1, operand2) = DecoderHelper::try_double_operand(iter)?;
        let instruction = Compare::Half(operand1, operand2);

        Ok(Instruction::Compare(instruction))
    }

    pub fn compare_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (operand1, operand2) = DecoderHelper::try_double_operand(iter)?;
        let instruction = Compare::Word(operand1, operand2);

        Ok(Instruction::Compare(instruction))
    }
}

#[cfg(test)]
mod regression {
    mod byte {
        use crate::{
            error::DecodeError,
            instruction::{Compare, Instruction},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let mut i = Interpreter::new_test();
            let instruction = "cmpb";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let mut i = Interpreter::new_test();
            let instruction = "cmpb ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_first_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "cmpb rx ra";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error_in_first_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "cmpb -1 ra";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error_in_first_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "cmpb 200u8 ra";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_second_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "cmpb ra rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error_in_second_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "cmpb ra -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error_in_second_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "cmpb ra 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn first_param_register_and_second_param_register() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "cmpb ra rb";
            let expected = Instruction::Compare(Compare::Byte(
                Operand::Register(Register::A),
                Operand::Register(Register::B),
            ));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn first_param_immediate_and_second_param_immediate() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "cmpb 10 20";
            let expected = Instruction::Compare(Compare::Byte(
                Operand::Immediate(10),
                Operand::Immediate(20),
            ));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn first_param_register_and_second_param_immediate() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "cmpb ra 20";
            let expected = Instruction::Compare(Compare::Byte(
                Operand::Register(Register::A),
                Operand::Immediate(20),
            ));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn first_param_immediate_and_second_param_register() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "cmpb 10 ra";
            let expected = Instruction::Compare(Compare::Byte(
                Operand::Immediate(10),
                Operand::Register(Register::A),
            ));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod quarter {
        use crate::{
            error::DecodeError,
            instruction::{Compare, Instruction},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let mut i = Interpreter::new_test();
            let instruction = "cmpq";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let mut i = Interpreter::new_test();
            let instruction = "cmpq ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_first_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "cmpq rx ra";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error_in_first_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "cmpq -1 ra";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error_in_first_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "cmpq 200u8 ra";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_second_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "cmpq ra rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error_in_second_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "cmpq ra -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error_in_second_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "cmpq ra 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn first_param_register_and_second_param_register() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "cmpq ra rb";
            let expected = Instruction::Compare(Compare::Quarter(
                Operand::Register(Register::A),
                Operand::Register(Register::B),
            ));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn first_param_immediate_and_second_param_immediate() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "cmpq 10 20";
            let expected = Instruction::Compare(Compare::Quarter(
                Operand::Immediate(10),
                Operand::Immediate(20),
            ));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn first_param_register_and_second_param_immediate() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "cmpq ra 20";
            let expected = Instruction::Compare(Compare::Quarter(
                Operand::Register(Register::A),
                Operand::Immediate(20),
            ));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn first_param_immediate_and_second_param_register() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "cmpq 10 ra";
            let expected = Instruction::Compare(Compare::Quarter(
                Operand::Immediate(10),
                Operand::Register(Register::A),
            ));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod half {
        use crate::{
            error::DecodeError,
            instruction::{Compare, Instruction},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let mut i = Interpreter::new_test();
            let instruction = "cmph";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let mut i = Interpreter::new_test();
            let instruction = "cmph ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_first_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "cmph rx ra";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error_in_first_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "cmph -1 ra";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error_in_first_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "cmph 200u8 ra";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_second_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "cmph ra rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error_in_second_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "cmph ra -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error_in_second_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "cmph ra 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn first_param_register_and_second_param_register() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "cmph ra rb";
            let expected = Instruction::Compare(Compare::Half(
                Operand::Register(Register::A),
                Operand::Register(Register::B),
            ));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn first_param_immediate_and_second_param_immediate() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "cmph 10 20";
            let expected = Instruction::Compare(Compare::Half(
                Operand::Immediate(10),
                Operand::Immediate(20),
            ));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn first_param_register_and_second_param_immediate() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "cmph ra 20";
            let expected = Instruction::Compare(Compare::Half(
                Operand::Register(Register::A),
                Operand::Immediate(20),
            ));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn first_param_immediate_and_second_param_register() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "cmph 10 ra";
            let expected = Instruction::Compare(Compare::Half(
                Operand::Immediate(10),
                Operand::Register(Register::A),
            ));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod word {
        use crate::{
            error::DecodeError,
            instruction::{Compare, Instruction},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let mut i = Interpreter::new_test();
            let instruction = "cmpw";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let mut i = Interpreter::new_test();
            let instruction = "cmpw ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_first_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "cmpw rx ra";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error_in_first_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "cmpw -1 ra";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error_in_first_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "cmpw 200u8 ra";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_second_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "cmpw ra rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error_in_second_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "cmpw ra -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error_in_second_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "cmpw ra 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn first_param_register_and_second_param_register() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "cmpw ra rb";
            let expected = Instruction::Compare(Compare::Word(
                Operand::Register(Register::A),
                Operand::Register(Register::B),
            ));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn first_param_immediate_and_second_param_immediate() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "cmpw 10 20";
            let expected = Instruction::Compare(Compare::Word(
                Operand::Immediate(10),
                Operand::Immediate(20),
            ));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn first_param_register_and_second_param_immediate() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "cmpw ra 20";
            let expected = Instruction::Compare(Compare::Word(
                Operand::Register(Register::A),
                Operand::Immediate(20),
            ));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn first_param_immediate_and_second_param_register() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "cmpw 10 ra";
            let expected = Instruction::Compare(Compare::Word(
                Operand::Immediate(10),
                Operand::Register(Register::A),
            ));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }
}
