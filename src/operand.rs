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

fn is_possible_register(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_lowercase())
}

fn is_possible_immediate_value(s: &str) -> bool {
    (s.starts_with('-') && s[1..].chars().all(|c| c.is_ascii_digit()))
        || s.chars().all(|c| c.is_ascii_digit())
}

impl<T: FromStr> TryFrom<&str> for Operand<T> {
    type Error = DecodeError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if is_possible_register(s) {
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
mod is_possible_register {
    use crate::operand::is_possible_register;

    #[test]
    fn actual_register() {
        let input = "ra";

        assert!(is_possible_register(input));
    }

    #[test]
    fn not_register_but_correct_format() {
        let input = "hello";
        assert!(is_possible_register(input));
    }

    #[test]
    fn invalid_format() {
        let input = "ra10";
        assert!(!is_possible_register(input));
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
mod try_from {
    mod byte {
        use crate::{constant::Byte, error::DecodeError, operand::Operand, register::Register};

        #[test]
        fn invalid_register() {
            let input = "rx";
            let expected = Err(DecodeError::InvalidRegister(input.to_string()));
            let actual: Result<Operand<Byte>, DecodeError> = Operand::try_from(input);
            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value() {
            let input = "-1";
            let expected = Err(DecodeError::InvalidImmediateValue(input.to_string()));
            let actual: Result<Operand<Byte>, DecodeError> = Operand::try_from(input);
            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand() {
            let input = "200u8";
            let expected = Err(DecodeError::InvalidOperand(input.to_string()));
            let actual: Result<Operand<Byte>, DecodeError> = Operand::try_from(input);
            assert_eq!(actual, expected);
        }

        #[test]
        fn valid_register() {
            let input = "ra";
            let expected = Ok(Operand::Register(Register::A));
            let actual: Result<Operand<Byte>, DecodeError> = Operand::try_from(input);
            assert_eq!(actual, expected);
        }

        #[test]
        fn valid_immediate_value() {
            let input = format!("{}", Byte::MAX);
            let expected = Ok(Operand::Immediate(Byte::MAX));
            let actual = Operand::try_from(input.as_ref());
            assert_eq!(actual, expected);
        }
    }

    mod quarter {
        use crate::{constant::Quarter, error::DecodeError, operand::Operand, register::Register};

        #[test]
        fn invalid_register() {
            let input = "rx";
            let expected = Err(DecodeError::InvalidRegister(input.to_string()));
            let actual: Result<Operand<Quarter>, DecodeError> = Operand::try_from(input);
            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value() {
            let input = "-1";
            let expected = Err(DecodeError::InvalidImmediateValue(input.to_string()));
            let actual: Result<Operand<Quarter>, DecodeError> = Operand::try_from(input);
            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand() {
            let input = "200u8";
            let expected = Err(DecodeError::InvalidOperand(input.to_string()));
            let actual: Result<Operand<Quarter>, DecodeError> = Operand::try_from(input);
            assert_eq!(actual, expected);
        }

        #[test]
        fn valid_register() {
            let input = "ra";
            let expected = Ok(Operand::Register(Register::A));
            let actual: Result<Operand<Quarter>, DecodeError> = Operand::try_from(input);
            assert_eq!(actual, expected);
        }

        #[test]
        fn valid_immediate_value() {
            let input = format!("{}", Quarter::MAX);
            let expected = Ok(Operand::Immediate(Quarter::MAX));
            let actual = Operand::try_from(input.as_ref());
            assert_eq!(actual, expected);
        }
    }

    mod half {
        use crate::{constant::Half, error::DecodeError, operand::Operand, register::Register};

        #[test]
        fn invalid_register() {
            let input = "rx";
            let expected = Err(DecodeError::InvalidRegister(input.to_string()));
            let actual: Result<Operand<Half>, DecodeError> = Operand::try_from(input);
            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value() {
            let input = "-1";
            let expected = Err(DecodeError::InvalidImmediateValue(input.to_string()));
            let actual: Result<Operand<Half>, DecodeError> = Operand::try_from(input);
            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand() {
            let input = "200u8";
            let expected = Err(DecodeError::InvalidOperand(input.to_string()));
            let actual: Result<Operand<Half>, DecodeError> = Operand::try_from(input);
            assert_eq!(actual, expected);
        }

        #[test]
        fn valid_register() {
            let input = "ra";
            let expected = Ok(Operand::Register(Register::A));
            let actual: Result<Operand<Half>, DecodeError> = Operand::try_from(input);
            assert_eq!(actual, expected);
        }

        #[test]
        fn valid_immediate_value() {
            let input = format!("{}", Half::MAX);
            let expected = Ok(Operand::Immediate(Half::MAX));
            let actual = Operand::try_from(input.as_ref());
            assert_eq!(actual, expected);
        }
    }

    mod word {
        use crate::{constant::Word, error::DecodeError, operand::Operand, register::Register};

        #[test]
        fn invalid_register() {
            let input = "rx";
            let expected = Err(DecodeError::InvalidRegister(input.to_string()));
            let actual: Result<Operand<Word>, DecodeError> = Operand::try_from(input);
            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_immediate_value() {
            let input = "-1";
            let expected = Err(DecodeError::InvalidImmediateValue(input.to_string()));
            let actual: Result<Operand<Word>, DecodeError> = Operand::try_from(input);
            assert_eq!(actual, expected);
        }

        #[test]
        fn invalid_operand() {
            let input = "200u8";
            let expected = Err(DecodeError::InvalidOperand(input.to_string()));
            let actual: Result<Operand<Word>, DecodeError> = Operand::try_from(input);
            assert_eq!(actual, expected);
        }

        #[test]
        fn valid_register() {
            let input = "ra";
            let expected = Ok(Operand::Register(Register::A));
            let actual: Result<Operand<Word>, DecodeError> = Operand::try_from(input);
            assert_eq!(actual, expected);
        }

        #[test]
        fn valid_immediate_value() {
            let input = format!("{}", Word::MAX);
            let expected = Ok(Operand::Immediate(Word::MAX));
            let actual = Operand::try_from(input.as_ref());
            assert_eq!(actual, expected);
        }
    }
}
