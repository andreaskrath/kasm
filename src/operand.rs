use crate::{
    constant::{Byte, Half, Quarter, Word},
    error::ParseError,
    register::Register,
};

/// Represents a register or a constant value.
///
/// This type is used when an argument of an instruction
/// could be either a register, or a constant value.
#[derive(Debug, PartialEq)]
pub enum Operand<Size> {
    Register(Register),
    Immediate(Size),
}

impl TryFrom<&str> for Operand<Byte> {
    type Error = DecodeError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match (s.parse::<Byte>(), Register::try_from(s)) {
            (Ok(_), Ok(_)) => unreachable!("impossible to parse as both number and register"),
            (Ok(num), Err(_)) => Ok(Operand::Immediate(num)),
            (Err(_), Ok(reg)) => Ok(Operand::Register(reg)),
            (Err(_), Err(_)) => Err(DecodeError::InvalidOperand(s.to_string())),
        }
    }
}

impl TryFrom<&str> for Operand<Quarter> {
    type Error = DecodeError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match (s.parse::<Quarter>(), Register::try_from(s)) {
            (Ok(_), Ok(_)) => unreachable!("impossible to parse as both number and register"),
            (Ok(num), Err(_)) => Ok(Operand::Immediate(num)),
            (Err(_), Ok(reg)) => Ok(Operand::Register(reg)),
            (Err(_), Err(_)) => Err(DecodeError::InvalidOperand(s.to_string())),
        }
    }
}

impl TryFrom<&str> for Operand<Half> {
    type Error = DecodeError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match (s.parse::<Half>(), Register::try_from(s)) {
            (Ok(_), Ok(_)) => unreachable!("impossible to parse as both number and register"),
            (Ok(num), Err(_)) => Ok(Operand::Immediate(num)),
            (Err(_), Ok(reg)) => Ok(Operand::Register(reg)),
            (Err(_), Err(_)) => Err(DecodeError::InvalidOperand(s.to_string())),
        }
    }
}

impl TryFrom<&str> for Operand<Word> {
    type Error = DecodeError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match (s.parse::<Word>(), Register::try_from(s)) {
            (Ok(_), Ok(_)) => unreachable!("impossible to parse as both number and register"),
            (Ok(num), Err(_)) => Ok(Operand::Immediate(num)),
            (Err(_), Ok(reg)) => Ok(Operand::Register(reg)),
            (Err(_), Err(_)) => Err(DecodeError::InvalidOperand(s.to_string())),
        }
    }
}
