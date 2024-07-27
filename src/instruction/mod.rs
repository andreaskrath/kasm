use super::{error::ParseError, operand::Operand, register::Register};
use std::str::SplitWhitespace;

mod jump;
// Re-exporting Jump enum.
pub use jump::Jump;

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
    /// Performs addition on two specified parameters.
    ///
    /// The first specified parameter is also the destination.
    Add(Register, Operand),
    /// Performs subtraction on two specified parameters.
    ///
    /// The first specified parameter is also the destination.
    Sub(Register, Operand),
    /// Performs addition on two specified parameters.
    ///
    /// The first multiplication parameter is also the destination.
    Mul(Register, Operand),
    /// Performs division on two specified parameters.
    ///
    /// The first specified parameter is also the destination.
    Div(Register, Operand),
    /// Performs a jump to another line in the source program.
    ///
    /// The jump target can either be a constant or the value found in a register.
    Jump(Jump, Operand),
    /// Calls another line and pushes a return address onto the stack.
    ///
    /// The return address is the line below the call.
    Call(Operand),
    /// Pops the top value off the stack and jumps to there.
    Return,
    /// Performs a bitwise AND operation, discarding the result, but setting appropriate flags.
    Test(Operand, Operand),
}

impl Instruction {
    const STOP: &'static str = "stp";
    const SET: &'static str = "set";
    const PUSH: &'static str = "psh";
    const POP: &'static str = "pop";
    const ADD: &'static str = "add";
    const SUB: &'static str = "sub";
    const MUL: &'static str = "mul";
    const DIV: &'static str = "div";
    const JUMP_PREFIX: char = 'j';
    const CALL: &'static str = "cll";
    const RETURN: &'static str = "ret";
    const TEST: &'static str = "tst";

    /// A helper that parses a register and operand.
    fn parse_register_and_operand(
        mut iter: SplitWhitespace,
    ) -> Result<(Register, Operand), ParseError> {
        let (Some(s_reg), Some(s_operand)) = (iter.next(), iter.next()) else {
            return Err(ParseError::IncompleteInstruction);
        };

        let reg = Register::parse(s_reg)?;
        let operand = Operand::parse(s_operand)?;

        Ok((reg, operand))
    }

    /// Parses an instruction.
    ///
    /// # Errors
    /// Returns and error when there is a problem with the parsing process.
    ///
    /// Look at [`crate::processor::error::ParseError`] for more details.
    pub fn parse(s: &str) -> Result<Instruction, ParseError> {
        use ParseError as PE;

        let mut s_iter = s.split_whitespace();
        let Some(instruction) = s_iter.next() else {
            return Err(PE::EmptyLine);
        };

        match instruction.trim() {
            Instruction::STOP => Ok(Instruction::Stop),
            Instruction::SET => {
                let (reg, operand) = Instruction::parse_register_and_operand(s_iter)?;

                Ok(Instruction::Set(reg, operand))
            }
            Instruction::PUSH => {
                let Some(s_operand) = s_iter.next() else {
                    return Err(PE::IncompleteInstruction);
                };

                let operand = Operand::parse(s_operand)?;

                Ok(Instruction::Push(operand))
            }
            Instruction::POP => {
                let Some(s_reg) = s_iter.next() else {
                    return Err(PE::IncompleteInstruction);
                };

                let reg = Register::parse(s_reg)?;

                Ok(Instruction::Pop(reg))
            }
            Instruction::ADD => {
                let (reg, operand) = Instruction::parse_register_and_operand(s_iter)?;

                Ok(Instruction::Add(reg, operand))
            }
            Instruction::SUB => {
                let (reg, operand) = Instruction::parse_register_and_operand(s_iter)?;

                Ok(Instruction::Sub(reg, operand))
            }
            Instruction::MUL => {
                let (reg, operand) = Instruction::parse_register_and_operand(s_iter)?;

                Ok(Instruction::Mul(reg, operand))
            }
            Instruction::DIV => {
                let (reg, operand) = Instruction::parse_register_and_operand(s_iter)?;

                Ok(Instruction::Div(reg, operand))
            }
            s_jump if s_jump.starts_with(Instruction::JUMP_PREFIX) => {
                let Some(s_operand) = s_iter.next() else {
                    return Err(PE::IncompleteInstruction);
                };

                let jump = Jump::parse(s_jump)?;
                let operand = Operand::parse(s_operand)?;

                Ok(Instruction::Jump(jump, operand))
            }
            Instruction::CALL => {
                let Some(s_operand) = s_iter.next() else {
                    return Err(PE::IncompleteInstruction);
                };

                let operand = Operand::parse(s_operand)?;

                Ok(Instruction::Call(operand))
            }
            Instruction::RETURN => Ok(Instruction::Return),
            Instruction::TEST => {
                let (Some(s_operand_a), Some(s_operand_b)) = (s_iter.next(), s_iter.next()) else {
                    return Err(PE::IncompleteInstruction);
                };

                let operand_a = Operand::parse(s_operand_a)?;
                let operand_b = Operand::parse(s_operand_b)?;

                Ok(Instruction::Test(operand_a, operand_b))
            }
            unknown => Err(PE::UnknownInstruction(unknown)),
        }
    }
}

#[cfg(test)]
mod parse_helper {
    use super::Instruction;
    use crate::error::ParseError;

    #[test]
    fn incomplete_instruction_error_missing_one_param() {
        let instruction = "set ra";
        let expected = Err(ParseError::IncompleteInstruction);
        let actual = Instruction::parse(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn incomplete_instruction_error_missing_two_param() {
        let instruction = "set";
        let expected = Err(ParseError::IncompleteInstruction);
        let actual = Instruction::parse(instruction);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod decode {
    use super::Instruction;
    use crate::error::ParseError;

    #[test]
    fn empty_line_error() {
        let instruction = "";
        let expected = Err(ParseError::EmptyLine);
        let actual = Instruction::parse(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn unknown_instruction_error() {
        let instruction = "hello world!";
        let expected = Err(ParseError::UnknownInstruction("hello"));
        let actual = Instruction::parse(instruction);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod decode_set {
    use super::Instruction;
    use crate::{operand::Operand, register::Register};

    #[test]
    fn valid_set_instruction_one_register_one_constant() {
        let instruction = "set ra 255";
        let expected = Ok(Instruction::Set(Register::A, Operand::Constant(255)));
        let actual = Instruction::parse(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_set_instruction_double_register() {
        let instruction = "set ra rb";
        let expected = Ok(Instruction::Set(
            Register::A,
            Operand::Register(Register::B),
        ));
        let actual = Instruction::parse(instruction);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod decode_push {
    use super::Instruction;
    use crate::{error::ParseError, operand::Operand};

    #[test]
    fn valid_push_instruction() {
        let instruction = "psh 250";
        let expected = Ok(Instruction::Push(Operand::Constant(250)));
        let actual = Instruction::parse(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn incomplete_instruction_error() {
        let instruction = "psh";
        let expected = Err(ParseError::IncompleteInstruction);
        let actual = Instruction::parse(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn invalid_operand_error_bad_register() {
        let instruction = "psh re";
        let expected = Err(ParseError::InvalidOperand("re"));
        let actual = Instruction::parse(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn invalid_operand_error_bad_constant() {
        let instruction = "psh 18u32";
        let expected = Err(ParseError::InvalidOperand("18u32"));
        let actual = Instruction::parse(instruction);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod decode_pop {
    use super::Instruction;
    use crate::{error::ParseError, register::Register};

    #[test]
    fn valid_pop_instruction() {
        let instruction = "pop ra";
        let expected = Ok(Instruction::Pop(Register::A));
        let actual = Instruction::parse(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn incomplete_instruction_error() {
        let instruction = "pop";
        let expected = Err(ParseError::IncompleteInstruction);
        let actual = Instruction::parse(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn invalid_register_error() {
        let instruction = "pop re";
        let expected = Err(ParseError::InvalidRegister("re"));
        let actual = Instruction::parse(instruction);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod decode_add {
    use super::Instruction;
    use crate::{operand::Operand, register::Register};

    #[test]
    fn valid_add_instruction_double_register() {
        let instruction = "add ra rb";
        let expected = Ok(Instruction::Add(
            Register::A,
            Operand::Register(Register::B),
        ));
        let actual = Instruction::parse(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_add_instruction_register_constant() {
        let instruction = "add ra 200";
        let expected = Ok(Instruction::Add(Register::A, Operand::Constant(200)));
        let actual = Instruction::parse(instruction);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod decode_sub {
    use super::Instruction;
    use crate::{operand::Operand, register::Register};

    #[test]
    fn valid_sub_instruction_double_register() {
        let instruction = "sub ra rb";
        let expected = Ok(Instruction::Sub(
            Register::A,
            Operand::Register(Register::B),
        ));
        let actual = Instruction::parse(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_sub_instruction_register_constant() {
        let instruction = "sub ra 200";
        let expected = Ok(Instruction::Sub(Register::A, Operand::Constant(200)));
        let actual = Instruction::parse(instruction);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod decode_mul {
    use super::Instruction;
    use crate::{operand::Operand, register::Register};

    #[test]
    fn valid_mul_instruction_double_register() {
        let instruction = "mul ra rb";
        let expected = Ok(Instruction::Mul(
            Register::A,
            Operand::Register(Register::B),
        ));
        let actual = Instruction::parse(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_mul_instruction_register_constant() {
        let instruction = "mul ra 200";
        let expected = Ok(Instruction::Mul(Register::A, Operand::Constant(200)));
        let actual = Instruction::parse(instruction);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod decode_div {
    use super::Instruction;
    use crate::{operand::Operand, register::Register};

    #[test]
    fn valid_div_instruction_double_register() {
        let instruction = "div ra rb";
        let expected = Ok(Instruction::Div(
            Register::A,
            Operand::Register(Register::B),
        ));
        let actual = Instruction::parse(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_div_instruction_register_constant() {
        let instruction = "div ra 200";
        let expected = Ok(Instruction::Div(Register::A, Operand::Constant(200)));
        let actual = Instruction::parse(instruction);
        assert_eq!(actual, expected);
    }
}
