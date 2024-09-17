use super::ParameterDecoderHelper;
use crate::{
    error::DecodeError,
    instruction::{Instruction, PrintStack},
    constant::Parameters,
};

pub struct PrintStackParameterDecoder;

impl PrintStackParameterDecoder {
    pub fn byte(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let operand = ParameterDecoderHelper::try_operand(parameters)?;
        let instruction = PrintStack::Byte(operand);

        Ok(Instruction::PrintStack(instruction))
    }

    pub fn quarter(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let operand = ParameterDecoderHelper::try_operand(parameters)?;
        let instruction = PrintStack::Quarter(operand);

        Ok(Instruction::PrintStack(instruction))
    }

    pub fn half(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let operand = ParameterDecoderHelper::try_operand(parameters)?;
        let instruction = PrintStack::Half(operand);

        Ok(Instruction::PrintStack(instruction))
    }

    pub fn word(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let operand = ParameterDecoderHelper::try_operand(parameters)?;
        let instruction = PrintStack::Word(operand);

        Ok(Instruction::PrintStack(instruction))
    }

    pub fn str(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let operand = ParameterDecoderHelper::try_operand(parameters)?;
        let instruction = PrintStack::Str(operand);

        Ok(Instruction::PrintStack(instruction))
    }
}

#[cfg(test)]
mod regression {
    mod byte {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, PrintStack},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "prsb";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "prsb rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let mut i = Interpreter::new_test();
            let instruction = "prsb -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let mut i = Interpreter::new_test();
            let instruction = "prsb 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "prsb ra";
            let expected =
                Instruction::PrintStack(PrintStack::Byte(Operand::Register(Register::A)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "prsb 10";
            let expected = Instruction::PrintStack(PrintStack::Byte(Operand::Immediate(10)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod quarter {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, PrintStack},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "prsq";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "prsq rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let mut i = Interpreter::new_test();
            let instruction = "prsq -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let mut i = Interpreter::new_test();
            let instruction = "prsq 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "prsq ra";
            let expected =
                Instruction::PrintStack(PrintStack::Quarter(Operand::Register(Register::A)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "prsq 10";
            let expected = Instruction::PrintStack(PrintStack::Quarter(Operand::Immediate(10)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod half {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, PrintStack},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "prsh";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "prsh rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let mut i = Interpreter::new_test();
            let instruction = "prsh -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let mut i = Interpreter::new_test();
            let instruction = "prsh 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "prsh ra";
            let expected =
                Instruction::PrintStack(PrintStack::Half(Operand::Register(Register::A)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "prsh 10";
            let expected = Instruction::PrintStack(PrintStack::Half(Operand::Immediate(10)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod word {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, PrintStack},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "prsw";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "prsw rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let mut i = Interpreter::new_test();
            let instruction = "prsw -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let mut i = Interpreter::new_test();
            let instruction = "prsw 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "prsw ra";
            let expected =
                Instruction::PrintStack(PrintStack::Word(Operand::Register(Register::A)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "prsw 10";
            let expected = Instruction::PrintStack(PrintStack::Word(Operand::Immediate(10)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod str {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, PrintStack},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "prss";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "prss rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let mut i = Interpreter::new_test();
            let instruction = "prss -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let mut i = Interpreter::new_test();
            let instruction = "prss 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "prss ra";
            let expected = Instruction::PrintStack(PrintStack::Str(Operand::Register(Register::A)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "prsw 10";
            let expected = Instruction::PrintStack(PrintStack::Word(Operand::Immediate(10)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }
}
