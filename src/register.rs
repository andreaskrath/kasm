use super::{
    constant::{Word, REGISTER_AMOUNT},
    error::ParseError,
};
use std::ops::{Index, IndexMut};
use variant_count::VariantCount;

#[derive(Clone, Copy, Debug, PartialEq, VariantCount)]
pub enum Register {
    A,
    B,
    C,
    D,
}

impl Register {
    const REG_A: &'static str = "ra";
    const REG_B: &'static str = "rb";
    const REG_C: &'static str = "rc";
    const REG_D: &'static str = "rd";

    pub fn parse(s: &str) -> Result<Register, ParseError> {
        use Register::*;

        match s {
            Register::REG_A => Ok(A),
            Register::REG_B => Ok(B),
            Register::REG_C => Ok(C),
            Register::REG_D => Ok(D),
            unknown => Err(ParseError::InvalidRegister(unknown)),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Register::A => Register::REG_A,
            Register::B => Register::REG_B,
            Register::C => Register::REG_C,
            Register::D => Register::REG_D,
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
    use super::Register;
    use crate::error::ParseError;

    #[test]
    fn invalid_register_error() {
        let s_reg = "hello";
        let expected = Err(ParseError::InvalidRegister(s_reg));
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
