use std::ops::{Index, IndexMut};
use variant_count::VariantCount;
use super::{constant::{Word, REGISTER_AMOUNT}, error::DecodeError};

#[derive(Debug, PartialEq, VariantCount)]
pub enum Register {
    A,
    B,
    C,
    D,
}

impl Register {
    pub fn parse(s: &str) -> Result<Register, DecodeError> {
        use Register::*;

        match s {
            "ra" => Ok(A),
            "rb" => Ok(B),
            "rc" => Ok(C),
            "rd" => Ok(D),
            unknown => Err(DecodeError::InvalidRegister(unknown)),
        }
    }
}

impl Index<Register> for [Word; REGISTER_AMOUNT] {
    type Output = Word;

    fn index(&self, index: Register) -> &Self::Output {
        &self[index as usize]
    }
}

impl IndexMut<Register> for [Word; REGISTER_AMOUNT] {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

#[cfg(test)]
mod parse {
    use crate::processor::error::DecodeError;

    use super::Register;

    #[test]
    fn invalid_register_error() {
        let s_reg = "hello";
        let expected = Err(DecodeError::InvalidRegister(s_reg));
        let actual = Register::parse(s_reg);
        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_register_a_parse() {
        let s_reg = "ra";
        let expected = Ok(Register::A);
        let actual = Register::parse(s_reg);
        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_register_b_parse() {
        let s_reg = "rb";
        let expected = Ok(Register::B);
        let actual = Register::parse(s_reg);
        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_register_c_parse() {
        let s_reg = "rc";
        let expected = Ok(Register::C);
        let actual = Register::parse(s_reg);
        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_register_d_parse() {
        let s_reg = "rd";
        let expected = Ok(Register::D);
        let actual = Register::parse(s_reg);
        assert_eq!(actual, expected);
    }
}
