use super::{
    constant::Word,
    error::InstructionError,
    register::Register,
};

/// The instruction set of the virtual processor.
#[derive(Debug, PartialEq)]
pub enum Instruction {
    /// Stops the execution of the program.
    Stop,
    /// Assigns a value to a specified register.
    Set(Register, Word),
    /// Pushes a value onto the stack.
    Push(Word),
    /// Pops the top-most value on the stack into a specified register.
    Pop(Register),
    /// Performs addition on two specified registers.
    ///
    /// The first specified register is also the destination.
    Add,
    /// Performs subtraction on two specified registers.
    ///
    /// The first specified register is also the destination.
    Sub,
    /// Performs addition on two specified registers.
    ///
    /// The first multiplication register is also the destination.
    Mul,
    /// Performs division on two specified registers.
    ///
    /// The first specified register is also the destination.
    Div,
}

impl Instruction {
    pub fn decode(s: &str) -> Result<Instruction, InstructionError> {
        use InstructionError as IE;

        let mut s_iter = s.split_whitespace();
        let Some(instruction) = s_iter.next() else {
            return Err(IE::EmptyLine);
        };

        match instruction.trim() {
            "stp" => Ok(Instruction::Stop),
            "set" => {
                let (Some(s_reg), Some(s_val)) = (s_iter.next(), s_iter.next()) else {
                    return Err(IE::IncompleteInstruction(s));
                };

                let reg = Register::parse(s_reg)?;

                let Ok(value) = s_val.parse() else {
                    return Err(IE::InvalidValue(s_val));
                };

                Ok(Instruction::Set(reg, value))
            }
            "psh" => {
                let Some(s_val) = s_iter.next() else {
                    return Err(IE::IncompleteInstruction(s));
                };

                let Ok(value) = s_val.parse() else {
                    return Err(IE::InvalidValue(s_val)); 
                };

                Ok(Instruction::Push(value))
            },
            "pop" => {
                let Some(s_reg) = s_iter.next() else {
                    return Err(IE::IncompleteInstruction(s));
                };

                let reg = Register::parse(s_reg)?;

                Ok(Instruction::Pop(reg))
            },
            unknown => Err(InstructionError::UnknownInstruction(unknown)),
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
    use crate::processor::{error::InstructionError as IE, instruction::Instruction, register::Register};

    #[test]
    fn valid_set_instruction() {
        let instruction = "set ra 255";
        let expected = Ok(Instruction::Set(Register::A, 255));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }

    #[test]
    fn incomplete_instruction_error() {
        let instruction = "set ra";
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
    fn invalid_value_error() {
        let instruction = "set ra hello";
        let expected = Err(IE::InvalidValue("hello"));
        let actual = Instruction::decode(instruction);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod decode_push {}

#[cfg(test)]
mod decode_pop {}
