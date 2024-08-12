use super::{
    constant::{Word, REGISTER_AMOUNT},
    error::ParseError,
};
use std::ops::{Index, IndexMut};
use variant_count::VariantCount;

/// The X and Y registers are private registers for interal use in the processor.
///
/// They are inaccessible through assembly.
#[derive(Clone, Copy, Debug, PartialEq, VariantCount)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    X,
    Y,
}

impl Register {
    const REG_A: &'static str = "ra";
    const REG_B: &'static str = "rb";
    const REG_C: &'static str = "rc";
    const REG_D: &'static str = "rd";
    const REG_E: &'static str = "re";
    const REG_F: &'static str = "rf";
    const REG_G: &'static str = "rg";
    const REG_H: &'static str = "rh";
    const REG_X: &'static str = "rx";
    const REG_Y: &'static str = "ry";

    pub fn as_str(self) -> &'static str {
        match self {
            Register::A => Register::REG_A,
            Register::B => Register::REG_B,
            Register::C => Register::REG_C,
            Register::D => Register::REG_D,
            Register::E => Register::REG_E,
            Register::F => Register::REG_F,
            Register::G => Register::REG_G,
            Register::H => Register::REG_H,
            Register::X => Register::REG_X,
            Register::Y => Register::REG_Y,
        }
    }
}

impl<'a> TryFrom<&'a str> for Register {
    type Error = ParseError<'a>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        use Register::*;

        match s {
            Register::REG_A => Ok(A),
            Register::REG_B => Ok(B),
            Register::REG_C => Ok(C),
            Register::REG_D => Ok(D),
            Register::REG_E => Ok(E),
            Register::REG_F => Ok(F),
            Register::REG_G => Ok(G),
            Register::REG_H => Ok(H),
            unknown => Err(ParseError::InvalidRegister(unknown)),
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
        let s_reg = "rx";
        let expected = Err(ParseError::InvalidRegister(s_reg));
        let actual = Register::try_from(s_reg);
        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_register_a_parse() {
        let s_reg = "ra";
        let expected = Ok(Register::A);
        let actual = Register::try_from(s_reg);
        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_register_b_parse() {
        let s_reg = "rb";
        let expected = Ok(Register::B);
        let actual = Register::try_from(s_reg);
        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_register_c_parse() {
        let s_reg = "rc";
        let expected = Ok(Register::C);
        let actual = Register::try_from(s_reg);
        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_register_d_parse() {
        let s_reg = "rd";
        let expected = Ok(Register::D);
        let actual = Register::try_from(s_reg);
        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_register_e_parse() {
        let s_reg = "re";
        let expected = Ok(Register::E);
        let actual = Register::try_from(s_reg);
        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_register_f_parse() {
        let s_reg = "rf";
        let expected = Ok(Register::F);
        let actual = Register::try_from(s_reg);
        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_register_g_parse() {
        let s_reg = "rg";
        let expected = Ok(Register::G);
        let actual = Register::try_from(s_reg);
        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_register_h_parse() {
        let s_reg = "rh";
        let expected = Ok(Register::H);
        let actual = Register::try_from(s_reg);
        assert_eq!(actual, expected);
    }
}
