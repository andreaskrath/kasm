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

#[cfg(test)]
mod parse {
    use crate::processor::{error::DecodeError, register::Register};

    use super::Operand;

    #[test]
    fn valid_register_parse() {
        let s_operand = "ra";
        let expected = Ok(Operand::Register(Register::A));
        let actual = Operand::parse(s_operand);
        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_constant_parse() {
        let s_operand = "200";
        let expected = Ok(Operand::Constant(200));
        let actual = Operand::parse(s_operand);
        assert_eq!(actual, expected);
    }

    #[test]
    fn invalid_operand_error_bad_register() {
        let s_operand = "re";
        let expected = Err(DecodeError::InvalidOperand(s_operand));
        let actual = Operand::parse(s_operand);
        assert_eq!(actual, expected);

    }

    #[test]
    fn invalid_operand_error_bad_constant() {
        let s_operand = "200u32";
        let expected = Err(DecodeError::InvalidOperand(s_operand));
        let actual = Operand::parse(s_operand);
        assert_eq!(actual, expected);
       
    }
}
