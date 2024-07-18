use std::str::SplitWhitespace;
use super::{error::DecodeError, operand::Operand, register::Register};

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

    /// A helper that parses a register and operand.
    fn parse_register_and_operand(mut iter: SplitWhitespace) -> Result<(Register, Operand), DecodeError> {
        let (Some(s_reg), Some(s_operand)) = (iter.next(), iter.next()) else {
            return Err(DecodeError::IncompleteInstruction);
        };

        let reg = Register::parse(s_reg)?;
        let operand = Operand::parse(s_operand)?;

        Ok((reg, operand))
    }

    /// Decodes an instruction.
    ///
    /// # Errors
    /// Returns and error when there is a problem with the decoding process.
    ///
    /// Look at [`crate::processor::error::DecodeError`] for more details.
    pub fn decode(s: &str) -> Result<Instruction, DecodeError> {
        use DecodeError as DE;

        let mut s_iter = s.split_whitespace();
        let Some(instruction) = s_iter.next() else {
            return Err(DE::EmptyLine);
        };

        match instruction.trim() {
            Instruction::STOP => Ok(Instruction::Stop),
            Instruction::SET => {
                let (reg, operand) = Instruction::parse_register_and_operand(s_iter)?;

                Ok(Instruction::Set(reg, operand))
            }
            Instruction::PUSH => {
                let Some(s_operand) = s_iter.next() else {
                    return Err(DE::IncompleteInstruction);
                };

                let operand = Operand::parse(s_operand)?;

                Ok(Instruction::Push(operand))
            }
            Instruction::POP => {
                let Some(s_reg) = s_iter.next() else {
                    return Err(DE::IncompleteInstruction);
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
            unknown => Err(DE::UnknownInstruction(unknown)),
        }
    }
}

#[cfg(test)]
mod parse_helper {
    use crate::error::DecodeError;
    use super::Instruction;

    #[test]
    fn incomplete_instruction_error_missing_one_param() {
        let instruction = "set ra";
        let expected = Err(DecodeError::IncompleteInstruction);
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);

    }

    #[test]
    fn incomplete_instruction_error_missing_two_param() {
        let instruction = "set";
        let expected = Err(DecodeError::IncompleteInstruction);
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod decode {
    use crate::error::DecodeError;
    use super::Instruction;

    #[test]
    fn empty_line_error() {
        let instruction = "";
        let expected = Err(DecodeError::EmptyLine);
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn unknown_instruction_error() {
        let instruction = "hello world!";
        let expected = Err(DecodeError::UnknownInstruction("hello"));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod decode_set {
    use crate::{operand::Operand, register::Register};
    use super::Instruction;

    #[test]
    fn valid_set_instruction_one_register_one_constant() {
        let instruction = "set ra 255";
        let expected = Ok(Instruction::Set(Register::A, Operand::Constant(255)));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_set_instruction_double_register() {
        let instruction = "set ra rb";
        let expected = Ok(Instruction::Set(Register::A, Operand::Register(Register::B)));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod decode_push {
    use crate::{error::DecodeError, operand::Operand};
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
        let expected = Err(DecodeError::IncompleteInstruction);
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn invalid_operand_error_bad_register() {
        let instruction = "psh re";
        let expected = Err(DecodeError::InvalidOperand("re"));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn invalid_operand_error_bad_constant() {
        let instruction = "psh 18u32";
        let expected = Err(DecodeError::InvalidOperand("18u32"));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod decode_pop {
    use crate::{error::DecodeError, register::Register};
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
        let expected = Err(DecodeError::IncompleteInstruction);
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn invalid_register_error() {
        let instruction = "pop re";
        let expected = Err(DecodeError::InvalidRegister("re"));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod decode_add {
    use crate::{operand::Operand, register::Register};
    use super::Instruction;

    #[test]
    fn valid_add_instruction_double_register() {
        let instruction = "add ra rb";
        let expected = Ok(Instruction::Add(
            Register::A,
            Operand::Register(Register::B),
        ));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_add_instruction_register_constant() {
        let instruction = "add ra 200";
        let expected = Ok(Instruction::Add(Register::A, Operand::Constant(200)));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod decode_sub {
    use crate::{operand::Operand, register::Register};
    use super::Instruction;

    #[test]
    fn valid_sub_instruction_double_register() {
        let instruction = "sub ra rb";
        let expected = Ok(Instruction::Sub(
            Register::A,
            Operand::Register(Register::B),
        ));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_sub_instruction_register_constant() {
        let instruction = "sub ra 200";
        let expected = Ok(Instruction::Sub(Register::A, Operand::Constant(200)));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod decode_mul {
    use crate::{operand::Operand, register::Register};
    use super::Instruction;

    #[test]
    fn valid_mul_instruction_double_register() {
        let instruction = "mul ra rb";
        let expected = Ok(Instruction::Mul(
            Register::A,
            Operand::Register(Register::B),
        ));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_mul_instruction_register_constant() {
        let instruction = "mul ra 200";
        let expected = Ok(Instruction::Mul(Register::A, Operand::Constant(200)));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod decode_div {
    use crate::{operand::Operand, register::Register};
    use super::Instruction;

    #[test]
    fn valid_div_instruction_double_register() {
        let instruction = "div ra rb";
        let expected = Ok(Instruction::Div(
            Register::A,
            Operand::Register(Register::B),
        ));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_div_instruction_register_constant() {
        let instruction = "div ra 200";
        let expected = Ok(Instruction::Div(Register::A, Operand::Constant(200)));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }
}
