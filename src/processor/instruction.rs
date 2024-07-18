use super::{error::InstructionError, operand::Operand, register::Register};

/// The instruction set of the virtual processor.
#[derive(Debug, PartialEq)]
pub enum Instruction {
    /// Stops the execution of the program.
    Stop,
    /// Assigns an operand to a specified register.
    Set(Register, Operand),
    /// Pushes an operand onto the stack.
    Push(Operand),
    /// Pops the top-most value on the stack into a specified register.
    Pop(Register),
    /// Performs addition on two specified registers.
    ///
    /// The first specified register is also the destination.
    Add(Register, Operand),
    /// Performs subtraction on two specified registers.
    ///
    /// The first specified register is also the destination.
    Sub(Register, Operand),
    /// Performs addition on two specified registers.
    ///
    /// The first multiplication register is also the destination.
    Mul(Register, Operand),
    /// Performs division on two specified registers.
    ///
    /// The first specified register is also the destination.
    Div(Register, Operand),
}

impl Instruction {
    const STOP: &'static str = "stp";
    const SET: &'static str = "set";
    const PUSH: &'static str = "psh";
    const POP: &'static str = "pop";
    pub fn decode(s: &str) -> Result<Instruction, InstructionError> {
        use InstructionError as IE;

        let mut s_iter = s.split_whitespace();
        let Some(instruction) = s_iter.next() else {
            return Err(IE::EmptyLine);
        };

        match instruction.trim() {
            Instruction::STOP => Ok(Instruction::Stop),
            Instruction::SET => {
                let (Some(s_reg), Some(s_operand)) = (s_iter.next(), s_iter.next()) else {
                    return Err(IE::IncompleteInstruction(s));
                };

                let reg = Register::parse(s_reg)?;
                let operand = Operand::parse(s_operand)?;

                Ok(Instruction::Set(reg, operand))
            }
            Instruction::PUSH => {
                let Some(s_operand) = s_iter.next() else {
                    return Err(IE::IncompleteInstruction(s));
                };

                let operand = Operand::parse(s_operand)?;

                Ok(Instruction::Push(operand))
            }
            Instruction::POP => {
                let Some(s_reg) = s_iter.next() else {
                    return Err(IE::IncompleteInstruction(s));
                };

                let reg = Register::parse(s_reg)?;

                Ok(Instruction::Pop(reg))
            }
            unknown => Err(IE::UnknownInstruction(unknown)),
        }
    }
}

#[cfg(test)]
mod decode {
    use super::Instruction;
    use crate::processor::instruction::InstructionError as IE;

    #[test]
    fn empty_line_error() {
        let instruction = "";
        let expected = Err(IE::EmptyLine);
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn unknown_instruction_error() {
        let instruction = "hello world!";
        let expected = Err(IE::UnknownInstruction("hello"));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod decode_set {
    use crate::processor::{
        error::InstructionError as IE, instruction::Instruction, operand::Operand,
        register::Register,
    };

    #[test]
    fn valid_set_instruction() {
        let instruction = "set ra 255";
        let expected = Ok(Instruction::Set(Register::A, Operand::Constant(255)));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn incomplete_instruction_error_one_missing_param() {
        let instruction = "set ra";
        let expected = Err(IE::IncompleteInstruction(instruction));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn incomplete_instruction_error_two_missing_param() {
        let instruction = "set";
        let expected = Err(IE::IncompleteInstruction(instruction));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn invalid_register_error() {
        let instruction = "set re 200";
        let expected = Err(IE::InvalidRegister("re"));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn invalid_operand_error_bad_register() {
        let instruction = "set ra re";
        let expected = Err(IE::InvalidOperand("re"));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn invalid_operand_error_bad_constant() {
        let instruction = "set ra 18u32";
        let expected = Err(IE::InvalidOperand("18u32"));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod decode_push {
    use crate::processor::{error::InstructionError, operand::Operand};

    use super::Instruction;

    #[test]
    fn valid_push_instruction() {
        let instruction = "psh 250";
        let expected = Ok(Instruction::Push(Operand::Constant(250)));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn incomplete_instruction_error() {
        let instruction = "psh";
        let expected = Err(InstructionError::IncompleteInstruction(instruction));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn invalid_operand_error_bad_register() {
        let instruction = "psh re";
        let expected = Err(InstructionError::InvalidOperand("re"));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn invalid_operand_error_bad_constant() {
        let instruction = "psh 18u32";
        let expected = Err(InstructionError::InvalidOperand("18u32"));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod decode_pop {
    use crate::processor::{error::InstructionError, register::Register};

    use super::Instruction;

    #[test]
    fn valid_pop_instruction() {
        let instruction = "pop ra";
        let expected = Ok(Instruction::Pop(Register::A));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn incomplete_instruction_error() {
        let instruction = "pop";
        let expected = Err(InstructionError::IncompleteInstruction(instruction));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn invalid_register_error() {
        let instruction = "pop re";
        let expected = Err(InstructionError::InvalidRegister("re"));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }
}

