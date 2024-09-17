use super::ParameterDecoderHelper;
use crate::{
    error::DecodeError,
    instruction::{Instruction, Test},
    constant::Parameters,
};

pub struct TestParameterDecoder;

impl TestParameterDecoder {
    pub fn byte(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let (operand1, operand2) = ParameterDecoderHelper::try_double_operand(parameters)?;
        let instruction = Test::Byte(operand1, operand2);

        Ok(Instruction::Test(instruction))
    }

    pub fn quarter(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let (operand1, operand2) = ParameterDecoderHelper::try_double_operand(parameters)?;
        let instruction = Test::Quarter(operand1, operand2);

        Ok(Instruction::Test(instruction))
    }

    pub fn half(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let (operand1, operand2) = ParameterDecoderHelper::try_double_operand(parameters)?;
        let instruction = Test::Half(operand1, operand2);

        Ok(Instruction::Test(instruction))
    }

    pub fn word(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let (operand1, operand2) = ParameterDecoderHelper::try_double_operand(parameters)?;
        let instruction = Test::Word(operand1, operand2);

        Ok(Instruction::Test(instruction))
    }
}

#[cfg(test)]
mod regression {
    mod byte {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, Test},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let mut i = Interpreter::new_test();
            let instruction = "tstb";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let mut i = Interpreter::new_test();
            let instruction = "tstb ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_first_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "tstb rx ra";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error_in_first_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "tstb -1 ra";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error_in_first_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "tstb 200u8 ra";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_second_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "tstb ra rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error_in_second_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "tstb ra -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error_in_second_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "tstb ra 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn first_param_register_and_second_param_register() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "tstb ra rb";
            let expected = Instruction::Test(Test::Byte(
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
            let instruction = "tstb 10 20";
            let expected =
                Instruction::Test(Test::Byte(Operand::Immediate(10), Operand::Immediate(20)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn first_param_register_and_second_param_immediate() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "tstb ra 20";
            let expected = Instruction::Test(Test::Byte(
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
            let instruction = "tstb 10 ra";
            let expected = Instruction::Test(Test::Byte(
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
            instruction::{Instruction, Test},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let mut i = Interpreter::new_test();
            let instruction = "tstq";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let mut i = Interpreter::new_test();
            let instruction = "tstq ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_first_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "tstq rx ra";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error_in_first_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "tstq -1 ra";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error_in_first_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "tstq 200u8 ra";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_second_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "tstq ra rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error_in_second_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "tstq ra -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error_in_second_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "tstq ra 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn first_param_register_and_second_param_register() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "tstq ra rb";
            let expected = Instruction::Test(Test::Quarter(
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
            let instruction = "tstq 10 20";
            let expected = Instruction::Test(Test::Quarter(
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
            let instruction = "tstq ra 20";
            let expected = Instruction::Test(Test::Quarter(
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
            let instruction = "tstq 10 ra";
            let expected = Instruction::Test(Test::Quarter(
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
            instruction::{Instruction, Test},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let mut i = Interpreter::new_test();
            let instruction = "tsth";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let mut i = Interpreter::new_test();
            let instruction = "tsth ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_first_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "tsth rx ra";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error_in_first_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "tsth -1 ra";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error_in_first_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "tsth 200u8 ra";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_second_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "tsth ra rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error_in_second_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "tsth ra -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error_in_second_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "tsth ra 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn first_param_register_and_second_param_register() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "tsth ra rb";
            let expected = Instruction::Test(Test::Half(
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
            let instruction = "tsth 10 20";
            let expected =
                Instruction::Test(Test::Half(Operand::Immediate(10), Operand::Immediate(20)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn first_param_register_and_second_param_immediate() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "tsth ra 20";
            let expected = Instruction::Test(Test::Half(
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
            let instruction = "tsth 10 ra";
            let expected = Instruction::Test(Test::Half(
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
            instruction::{Instruction, Test},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_first_param() {
            let mut i = Interpreter::new_test();
            let instruction = "tstw";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn incomplete_instruction_error_missing_second_param() {
            let mut i = Interpreter::new_test();
            let instruction = "tstw ra";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_first_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "tstw rx ra";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error_in_first_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "tstw -1 ra";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error_in_first_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "tstw 200u8 ra";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error_in_second_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "tstw ra rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error_in_second_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "tstw ra -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error_in_second_operand() {
            let mut i = Interpreter::new_test();
            let instruction = "tstw ra 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn first_param_register_and_second_param_register() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "tstw ra rb";
            let expected = Instruction::Test(Test::Word(
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
            let instruction = "tstw 10 20";
            let expected =
                Instruction::Test(Test::Word(Operand::Immediate(10), Operand::Immediate(20)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn first_param_register_and_second_param_immediate() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "tstw ra 20";
            let expected = Instruction::Test(Test::Word(
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
            let instruction = "tstw 10 ra";
            let expected = Instruction::Test(Test::Word(
                Operand::Immediate(10),
                Operand::Register(Register::A),
            ));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }
}
