use crate::{error::DecodeError, register::Register};
use std::str::FromStr;

/// Represents a register or an immediate value.
///
/// This type is used when an argument of an instruction
/// could be either a register, or an immediately defined value.
#[derive(Debug, PartialEq)]
pub enum Operand<Size> {
    Register(Register),
    Immediate(Size),
}

fn is_possible_reg(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_lowercase())
}

fn is_possible_immediate_value(s: &str) -> bool {
    (s.starts_with('-') && s[1..].chars().all(|c| c.is_ascii_digit()))
        || s.chars().all(|c| c.is_ascii_digit())
}

impl<T: FromStr> TryFrom<&str> for Operand<T> {
    type Error = DecodeError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if is_possible_reg(s) {
            let register = Register::try_from(s)?;

            Ok(Operand::Register(register))
        } else if is_possible_immediate_value(s) {
            if let Ok(number) = s.parse::<T>() {
                Ok(Operand::Immediate(number))
            } else {
                Err(DecodeError::InvalidImmediateValue(s.to_string()))
            }
        } else {
            Err(DecodeError::InvalidOperand(s.to_string()))
        }
    }
}

#[cfg(test)]
mod is_possible_reg {
    use crate::operand::is_possible_reg;

    #[test]
    fn actual_register() {
        let input = "ra";

        assert!(is_possible_reg(input));
    }

    #[test]
    fn not_register_but_correct_format() {
        let input = "hello";
        assert!(is_possible_reg(input));
    }

    #[test]
    fn invalid_format() {
        let input = "ra10";
        assert!(!is_possible_reg(input));
    }
}

#[cfg(test)]
mod is_possible_immediate_value {
    use crate::operand::is_possible_immediate_value;

    #[test]
    fn positive_number() {
        let input = "1000";
        assert!(is_possible_immediate_value(input));
    }

    #[test]
    fn negative_number() {
        let input = "-1000";
        assert!(is_possible_immediate_value(input));
    }

    #[test]
    fn invalid_immediate_value() {
        let input = "1000u32";
        assert!(!is_possible_immediate_value(input));
    }
}

#[cfg(test)]
mod try_from_specific_types {
    use super::Operand;
    use crate::{
        constant::{Byte, Half, Quarter, Word},
        error::DecodeError,
    };

    #[test]
    fn invalid_immediate_value_error_byte() {
        let input = "-1";
        let expected = Err(DecodeError::InvalidImmediateValue(input.to_string()));

        let actual: Result<Operand<Byte>, DecodeError> = Operand::try_from(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn invalid_immediate_value_error_quarter() {
        let input = "-1";
        let expected = Err(DecodeError::InvalidImmediateValue(input.to_string()));

        let actual: Result<Operand<Quarter>, DecodeError> = Operand::try_from(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn invalid_immediate_value_error_half() {
        let input = "-1";
        let expected = Err(DecodeError::InvalidImmediateValue(input.to_string()));

        let actual: Result<Operand<Half>, DecodeError> = Operand::try_from(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn invalid_immediate_value_error_word() {
        let input = "-1";
        let expected = Err(DecodeError::InvalidImmediateValue(input.to_string()));

        let actual: Result<Operand<Word>, DecodeError> = Operand::try_from(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_byte() {
        let input = format!("{}", Byte::MAX);
        let expected = Ok(Operand::Immediate(Byte::MAX));

        let actual = Operand::try_from(input.as_ref());

        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_quarter() {
        let input = format!("{}", Quarter::MAX);
        let expected = Ok(Operand::Immediate(Quarter::MAX));

        let actual = Operand::try_from(input.as_ref());

        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_half() {
        let input = format!("{}", Half::MAX);
        let expected = Ok(Operand::Immediate(Half::MAX));

        let actual = Operand::try_from(input.as_ref());

        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_word() {
        let input = format!("{}", Word::MAX);
        let expected = Ok(Operand::Immediate(Word::MAX));

        let actual = Operand::try_from(input.as_ref());

        assert_eq!(actual, expected);
    }
}
