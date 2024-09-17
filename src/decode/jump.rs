use super::ParameterDecoderHelper;
use crate::{
    error::DecodeError,
    instruction::{Instruction, Jump},
    constant::Parameters,
};

pub struct JumpParameterDecoder;

impl JumpParameterDecoder {
    pub fn unconditional(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let operand = ParameterDecoderHelper::try_operand(parameters)?;
        let instruction = Jump::Unconditional(operand);

        Ok(Instruction::Jump(instruction))
    }

    pub fn if_zero(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let operand = ParameterDecoderHelper::try_operand(parameters)?;
        let instruction = Jump::IfZero(operand);

        Ok(Instruction::Jump(instruction))
    }

    pub fn if_not_zero(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let operand = ParameterDecoderHelper::try_operand(parameters)?;
        let instruction = Jump::IfNotZero(operand);

        Ok(Instruction::Jump(instruction))
    }

    pub fn if_sign(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let operand = ParameterDecoderHelper::try_operand(parameters)?;
        let instruction = Jump::IfSign(operand);

        Ok(Instruction::Jump(instruction))
    }

    pub fn if_not_sign(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let operand = ParameterDecoderHelper::try_operand(parameters)?;
        let instruction = Jump::IfNotSign(operand);

        Ok(Instruction::Jump(instruction))
    }

    pub fn if_overflow(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let operand = ParameterDecoderHelper::try_operand(parameters)?;
        let instruction = Jump::IfOverflow(operand);

        Ok(Instruction::Jump(instruction))
    }

    pub fn if_not_overflow(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let operand = ParameterDecoderHelper::try_operand(parameters)?;
        let instruction = Jump::IfNotOverflow(operand);

        Ok(Instruction::Jump(instruction))
    }

    pub fn if_greater(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let operand = ParameterDecoderHelper::try_operand(parameters)?;
        let instruction = Jump::IfGreater(operand);

        Ok(Instruction::Jump(instruction))
    }

    pub fn if_lesser(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let operand = ParameterDecoderHelper::try_operand(parameters)?;
        let instruction = Jump::IfLesser(operand);

        Ok(Instruction::Jump(instruction))
    }

    pub fn if_greater_or_equal(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let operand = ParameterDecoderHelper::try_operand(parameters)?;
        let instruction = Jump::IfGreaterOrEqual(operand);

        Ok(Instruction::Jump(instruction))
    }

    pub fn if_lesser_or_equal(parameters: Parameters) -> Result<Instruction, DecodeError> {
        let operand = ParameterDecoderHelper::try_operand(parameters)?;
        let instruction = Jump::IfLesserOrEqual(operand);

        Ok(Instruction::Jump(instruction))
    }
}

#[cfg(test)]
mod regression {
    mod unconditional {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, Jump},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "jmp";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jmp rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jmp -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jmp 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "jmp ra";
            let expected = Instruction::Jump(Jump::Unconditional(Operand::Register(Register::A)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "jmp 10";
            let expected = Instruction::Jump(Jump::Unconditional(Operand::Immediate(10)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod if_zero {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, Jump},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "jiz";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jiz rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jiz -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jiz 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "jiz ra";
            let expected = Instruction::Jump(Jump::IfZero(Operand::Register(Register::A)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "jiz 10";
            let expected = Instruction::Jump(Jump::IfZero(Operand::Immediate(10)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod if_not_zero {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, Jump},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "jnz";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jnz rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jnz -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jnz 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "jnz ra";
            let expected = Instruction::Jump(Jump::IfNotZero(Operand::Register(Register::A)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "jnz 10";
            let expected = Instruction::Jump(Jump::IfNotZero(Operand::Immediate(10)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod if_sign {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, Jump},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "jis";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jis rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jis -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jis 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "jis ra";
            let expected = Instruction::Jump(Jump::IfSign(Operand::Register(Register::A)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "jis 10";
            let expected = Instruction::Jump(Jump::IfSign(Operand::Immediate(10)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod if_not_sign {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, Jump},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "jns";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jns rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jns -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jns 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "jns ra";
            let expected = Instruction::Jump(Jump::IfNotSign(Operand::Register(Register::A)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "jns 10";
            let expected = Instruction::Jump(Jump::IfNotSign(Operand::Immediate(10)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod if_overflow {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, Jump},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "jio";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jio rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jio -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jio 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "jio ra";
            let expected = Instruction::Jump(Jump::IfOverflow(Operand::Register(Register::A)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "jio 10";
            let expected = Instruction::Jump(Jump::IfOverflow(Operand::Immediate(10)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod if_not_overflow {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, Jump},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "jno";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jno rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jno -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jno 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "jno ra";
            let expected = Instruction::Jump(Jump::IfNotOverflow(Operand::Register(Register::A)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "jno 10";
            let expected = Instruction::Jump(Jump::IfNotOverflow(Operand::Immediate(10)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod if_greater {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, Jump},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "jig";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jig rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jig -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jig 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "jig ra";
            let expected = Instruction::Jump(Jump::IfGreater(Operand::Register(Register::A)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "jig 10";
            let expected = Instruction::Jump(Jump::IfGreater(Operand::Immediate(10)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod if_lesser {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, Jump},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "jil";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jil rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jil -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jil 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "jil ra";
            let expected = Instruction::Jump(Jump::IfLesser(Operand::Register(Register::A)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "jil 10";
            let expected = Instruction::Jump(Jump::IfLesser(Operand::Immediate(10)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod if_greater_or_equal {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, Jump},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "jge";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jge rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jge -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jge 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "jge ra";
            let expected =
                Instruction::Jump(Jump::IfGreaterOrEqual(Operand::Register(Register::A)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "jge 10";
            let expected = Instruction::Jump(Jump::IfGreaterOrEqual(Operand::Immediate(10)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod if_lesser_or_equal {
        use crate::{
            error::DecodeError,
            instruction::{Instruction, Jump},
            operand::Operand,
            register::Register,
            Interpreter,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let mut i = Interpreter::new_test();
            let instruction = "jle";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jle rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jle -1";
            let expected = Err(DecodeError::InvalidImmediateValue("-1".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let mut i = Interpreter::new_test();
            let instruction = "jle 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = i.decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "jle ra";
            let expected = Instruction::Jump(Jump::IfLesserOrEqual(Operand::Register(Register::A)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let mut i = Interpreter::new_test();
            let instruction = "jle 10";
            let expected = Instruction::Jump(Jump::IfLesserOrEqual(Operand::Immediate(10)));

            let actual = i.decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }
}
