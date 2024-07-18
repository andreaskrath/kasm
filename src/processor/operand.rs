use super::{constant::Word, error::DecodeError, register::Register};

/// Represents a register or a constant value.
///
/// This type is used when an argument of an instruction
/// could be either a register, or a constant value.
#[derive(Debug, PartialEq)]
pub enum Operand {
    Register(Register),
    Constant(Word),
}

impl Operand {
    pub fn parse(s: &str) -> Result<Operand, DecodeError> {
        match (s.parse::<Word>(), Register::parse(s)) {
            (Ok(_), Ok(_)) => unreachable!("impossible to parse as both number and register"),
            (Ok(num), Err(_)) => Ok(Operand::Constant(num)),
            (Err(_), Ok(reg)) => Ok(Operand::Register(reg)),
            (Err(_), Err(_)) => Err(DecodeError::InvalidOperand(s))
        }
    }
}
