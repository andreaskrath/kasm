use crate::{
    constant::Parameters,
    error::DecodeError,
    instruction::{Instruction, Jump, Relative},
    operand::Operand,
};

pub struct JumpParameterDecoder;

impl JumpParameterDecoder {
    pub fn unconditional(mut parameters: Parameters) -> Result<Instruction, DecodeError> {
        let mut param = parameters
            .next()
            .ok_or(DecodeError::IncompleteInstruction)?
            .to_string();

        let relative = is_relative(&mut param);
        let operand = Operand::try_from(param.as_str())?;
        let instruction = Jump::Unconditional;

        Ok(Instruction::Jump(instruction, operand, relative))
    }

    pub fn if_zero(mut parameters: Parameters) -> Result<Instruction, DecodeError> {
        let mut param = parameters
            .next()
            .ok_or(DecodeError::IncompleteInstruction)?
            .to_string();

        let relative = is_relative(&mut param);
        let operand = Operand::try_from(param.as_str())?;
        let instruction = Jump::IfZero;

        Ok(Instruction::Jump(instruction, operand, relative))
    }

    pub fn if_not_zero(mut parameters: Parameters) -> Result<Instruction, DecodeError> {
        let mut param = parameters
            .next()
            .ok_or(DecodeError::IncompleteInstruction)?
            .to_string();

        let relative = is_relative(&mut param);
        let operand = Operand::try_from(param.as_str())?;
        let instruction = Jump::IfNotZero;

        Ok(Instruction::Jump(instruction, operand, relative))
    }

    pub fn if_sign(mut parameters: Parameters) -> Result<Instruction, DecodeError> {
        let mut param = parameters
            .next()
            .ok_or(DecodeError::IncompleteInstruction)?
            .to_string();

        let relative = is_relative(&mut param);
        let operand = Operand::try_from(param.as_str())?;
        let instruction = Jump::IfSign;

        Ok(Instruction::Jump(instruction, operand, relative))
    }

    pub fn if_not_sign(mut parameters: Parameters) -> Result<Instruction, DecodeError> {
        let mut param = parameters
            .next()
            .ok_or(DecodeError::IncompleteInstruction)?
            .to_string();

        let relative = is_relative(&mut param);
        let operand = Operand::try_from(param.as_str())?;
        let instruction = Jump::IfNotSign;

        Ok(Instruction::Jump(instruction, operand, relative))
    }

    pub fn if_overflow(mut parameters: Parameters) -> Result<Instruction, DecodeError> {
        let mut param = parameters
            .next()
            .ok_or(DecodeError::IncompleteInstruction)?
            .to_string();

        let relative = is_relative(&mut param);
        let operand = Operand::try_from(param.as_str())?;
        let instruction = Jump::IfOverflow;

        Ok(Instruction::Jump(instruction, operand, relative))
    }

    pub fn if_not_overflow(mut parameters: Parameters) -> Result<Instruction, DecodeError> {
        let mut param = parameters
            .next()
            .ok_or(DecodeError::IncompleteInstruction)?
            .to_string();

        let relative = is_relative(&mut param);
        let operand = Operand::try_from(param.as_str())?;
        let instruction = Jump::IfNotOverflow;

        Ok(Instruction::Jump(instruction, operand, relative))
    }

    pub fn if_greater(mut parameters: Parameters) -> Result<Instruction, DecodeError> {
        let mut param = parameters
            .next()
            .ok_or(DecodeError::IncompleteInstruction)?
            .to_string();

        let relative = is_relative(&mut param);
        let operand = Operand::try_from(param.as_str())?;
        let instruction = Jump::IfGreater;

        Ok(Instruction::Jump(instruction, operand, relative))
    }

    pub fn if_lesser(mut parameters: Parameters) -> Result<Instruction, DecodeError> {
        let mut param = parameters
            .next()
            .ok_or(DecodeError::IncompleteInstruction)?
            .to_string();

        let relative = is_relative(&mut param);
        let operand = Operand::try_from(param.as_str())?;
        let instruction = Jump::IfLesser;

        Ok(Instruction::Jump(instruction, operand, relative))
    }

    pub fn if_greater_or_equal(mut parameters: Parameters) -> Result<Instruction, DecodeError> {
        let mut param = parameters
            .next()
            .ok_or(DecodeError::IncompleteInstruction)?
            .to_string();

        let relative = is_relative(&mut param);
        let operand = Operand::try_from(param.as_str())?;
        let instruction = Jump::IfGreaterOrEqual;

        Ok(Instruction::Jump(instruction, operand, relative))
    }

    pub fn if_lesser_or_equal(mut parameters: Parameters) -> Result<Instruction, DecodeError> {
        let mut param = parameters
            .next()
            .ok_or(DecodeError::IncompleteInstruction)?
            .to_string();

        let relative = is_relative(&mut param);
        let operand = Operand::try_from(param.as_str())?;
        let instruction = Jump::IfLesserOrEqual;

        Ok(Instruction::Jump(instruction, operand, relative))
    }
}

fn is_relative(s: &mut String) -> Option<Relative> {
    if let Some(stripped_s) = s.strip_prefix('-') {
        *s = stripped_s.to_string();
        Some(Relative::Negative)
    } else if let Some(stripped_s) = s.strip_prefix('+') {
        *s = stripped_s.to_string();
        Some(Relative::Positive)
    } else {
        None
    }
}

#[cfg(test)]
mod regression {
    mod unconditional {
        use crate::{
            constant::Word,
            decode::decode,
            error::DecodeError,
            instruction::{Instruction, Jump, Relative},
            operand::Operand,
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let instruction = "jmp";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "jmp rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let instruction = format!("jmp {}0", Word::MAX);
            let expected = Err(DecodeError::InvalidImmediateValue(format!(
                "{}0",
                Word::MAX
            )));

            let actual = decode(&instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let instruction = "jmp 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let instruction = "jmp ra";
            let expected =
                Instruction::Jump(Jump::Unconditional, Operand::Register(Register::A), None);

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let instruction = "jmp 10";
            let expected = Instruction::Jump(Jump::Unconditional, Operand::Immediate(10), None);

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod if_zero {
        use crate::{
            constant::Word,
            decode::decode,
            error::DecodeError,
            instruction::{Instruction, Jump},
            operand::Operand,
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let instruction = "jiz";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "jiz rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let instruction = format!("jiz {}0", Word::MAX);
            let expected = Err(DecodeError::InvalidImmediateValue(format!(
                "{}0",
                Word::MAX
            )));

            let actual = decode(&instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let instruction = "jiz 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let instruction = "jiz ra";
            let expected = Instruction::Jump(Jump::IfZero, Operand::Register(Register::A), None);

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let instruction = "jiz 10";
            let expected = Instruction::Jump(Jump::IfZero, Operand::Immediate(10), None);

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod if_not_zero {
        use crate::{
            constant::Word,
            decode::decode,
            error::DecodeError,
            instruction::{Instruction, Jump},
            operand::Operand,
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let instruction = "jnz";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "jnz rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let instruction = format!("jnz {}0", Word::MAX);
            let expected = Err(DecodeError::InvalidImmediateValue(format!(
                "{}0",
                Word::MAX
            )));

            let actual = decode(&instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let instruction = "jnz 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let instruction = "jnz ra";
            let expected = Instruction::Jump(Jump::IfNotZero, Operand::Register(Register::A), None);

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let instruction = "jnz 10";
            let expected = Instruction::Jump(Jump::IfNotZero, Operand::Immediate(10), None);

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod if_sign {
        use crate::{
            constant::Word,
            decode::decode,
            error::DecodeError,
            instruction::{Instruction, Jump},
            operand::Operand,
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let instruction = "jis";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "jis rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let instruction = format!("jis {}0", Word::MAX);
            let expected = Err(DecodeError::InvalidImmediateValue(format!(
                "{}0",
                Word::MAX
            )));

            let actual = decode(&instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let instruction = "jis 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let instruction = "jis ra";
            let expected = Instruction::Jump(Jump::IfSign, Operand::Register(Register::A), None);

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let instruction = "jis 10";
            let expected = Instruction::Jump(Jump::IfSign, Operand::Immediate(10), None);

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod if_not_sign {
        use crate::{
            constant::Word,
            decode::decode,
            error::DecodeError,
            instruction::{Instruction, Jump},
            operand::Operand,
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let instruction = "jns";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "jns rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let instruction = format!("jns {}0", Word::MAX);
            let expected = Err(DecodeError::InvalidImmediateValue(format!(
                "{}0",
                Word::MAX
            )));

            let actual = decode(&instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let instruction = "jns 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let instruction = "jns ra";
            let expected = Instruction::Jump(Jump::IfNotSign, Operand::Register(Register::A), None);

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let instruction = "jns 10";
            let expected = Instruction::Jump(Jump::IfNotSign, Operand::Immediate(10), None);

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod if_overflow {
        use crate::{
            constant::Word,
            decode::decode,
            error::DecodeError,
            instruction::{Instruction, Jump},
            operand::Operand,
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let instruction = "jio";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "jio rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let instruction = format!("jio {}0", Word::MAX);
            let expected = Err(DecodeError::InvalidImmediateValue(format!(
                "{}0",
                Word::MAX
            )));

            let actual = decode(&instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let instruction = "jio 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let instruction = "jio ra";
            let expected =
                Instruction::Jump(Jump::IfOverflow, Operand::Register(Register::A), None);

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let instruction = "jio 10";
            let expected = Instruction::Jump(Jump::IfOverflow, Operand::Immediate(10), None);

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod if_not_overflow {
        use crate::{
            constant::Word,
            decode::decode,
            error::DecodeError,
            instruction::{Instruction, Jump},
            operand::Operand,
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let instruction = "jno";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "jno rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let instruction = format!("jno {}0", Word::MAX);
            let expected = Err(DecodeError::InvalidImmediateValue(format!(
                "{}0",
                Word::MAX
            )));

            let actual = decode(&instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let instruction = "jno 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let instruction = "jno ra";
            let expected =
                Instruction::Jump(Jump::IfNotOverflow, Operand::Register(Register::A), None);

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let instruction = "jno 10";
            let expected = Instruction::Jump(Jump::IfNotOverflow, Operand::Immediate(10), None);

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod if_greater {
        use crate::{
            constant::Word,
            decode::decode,
            error::DecodeError,
            instruction::{Instruction, Jump},
            operand::Operand,
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let instruction = "jig";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "jig rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let instruction = format!("jig {}0", Word::MAX);
            let expected = Err(DecodeError::InvalidImmediateValue(format!(
                "{}0",
                Word::MAX
            )));

            let actual = decode(&instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let instruction = "jig 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let instruction = "jig ra";
            let expected = Instruction::Jump(Jump::IfGreater, Operand::Register(Register::A), None);

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let instruction = "jig 10";
            let expected = Instruction::Jump(Jump::IfGreater, Operand::Immediate(10), None);

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod if_lesser {
        use crate::{
            constant::Word,
            decode::decode,
            error::DecodeError,
            instruction::{Instruction, Jump},
            operand::Operand,
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let instruction = "jil";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "jil rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let instruction = format!("jil {}0", Word::MAX);
            let expected = Err(DecodeError::InvalidImmediateValue(format!(
                "{}0",
                Word::MAX
            )));

            let actual = decode(&instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let instruction = "jil 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let instruction = "jil ra";
            let expected = Instruction::Jump(Jump::IfLesser, Operand::Register(Register::A), None);

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let instruction = "jil 10";
            let expected = Instruction::Jump(Jump::IfLesser, Operand::Immediate(10), None);

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod if_greater_or_equal {
        use crate::{
            constant::Word,
            decode::decode,
            error::DecodeError,
            instruction::{Instruction, Jump},
            operand::Operand,
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let instruction = "jge";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "jge rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let instruction = format!("jge {}0", Word::MAX);
            let expected = Err(DecodeError::InvalidImmediateValue(format!(
                "{}0",
                Word::MAX
            )));

            let actual = decode(&instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let instruction = "jge 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let instruction = "jge ra";
            let expected =
                Instruction::Jump(Jump::IfGreaterOrEqual, Operand::Register(Register::A), None);

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let instruction = "jge 10";
            let expected = Instruction::Jump(Jump::IfGreaterOrEqual, Operand::Immediate(10), None);

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }

    mod if_lesser_or_equal {
        use crate::{
            constant::Word,
            decode::decode,
            error::DecodeError,
            instruction::{Instruction, Jump},
            operand::Operand,
            register::Register,
        };

        #[test]
        fn incomplete_instruction_error_missing_param() {
            let instruction = "jle";
            let expected = Err(DecodeError::IncompleteInstruction);

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_register_error() {
            let instruction = "jle rx";
            let expected = Err(DecodeError::InvalidRegister("rx".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value_error() {
            let instruction = format!("jle {}0", Word::MAX);
            let expected = Err(DecodeError::InvalidImmediateValue(format!(
                "{}0",
                Word::MAX
            )));

            let actual = decode(&instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand_error() {
            let instruction = "jle 200u8";
            let expected = Err(DecodeError::InvalidOperand("200u8".to_string()));

            let actual = decode(instruction);

            assert_eq!(actual, expected);
        }

        #[test]
        fn register_in_operand() -> Result<(), DecodeError> {
            let instruction = "jle ra";
            let expected =
                Instruction::Jump(Jump::IfLesserOrEqual, Operand::Register(Register::A), None);

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }

        #[test]
        fn immediate_value_in_operand() -> Result<(), DecodeError> {
            let instruction = "jle 10";
            let expected = Instruction::Jump(Jump::IfLesserOrEqual, Operand::Immediate(10), None);

            let actual = decode(instruction)?;

            assert_eq!(actual, expected);

            Ok(())
        }
    }
}
