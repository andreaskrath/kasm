use crate::{constant::Word, error::DecodeError, registers::Registers};
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};
use variant_count::VariantCount;

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
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Register::A => Register::REG_A,
            Register::B => Register::REG_B,
            Register::C => Register::REG_C,
            Register::D => Register::REG_D,
            Register::E => Register::REG_E,
            Register::F => Register::REG_F,
            Register::G => Register::REG_G,
            Register::H => Register::REG_H,
        };

        write!(f, "{s}")
    }
}

impl TryFrom<&str> for Register {
    type Error = DecodeError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            Register::REG_A => Ok(Register::A),
            Register::REG_B => Ok(Register::B),
            Register::REG_C => Ok(Register::C),
            Register::REG_D => Ok(Register::D),
            Register::REG_E => Ok(Register::E),
            Register::REG_F => Ok(Register::F),
            Register::REG_G => Ok(Register::G),
            Register::REG_H => Ok(Register::H),
            unknown => Err(DecodeError::InvalidRegister(unknown.to_string())),
        }
    }
}

impl Index<Register> for Registers {
    type Output = Word;

    fn index(&self, index: Register) -> &Self::Output {
        &self[index as usize]
    }
}

impl IndexMut<Register> for Registers {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

#[cfg(test)]
mod try_from {
    use crate::{error::DecodeError, register::Register};

    #[test]
    fn register_is_invalid() {
        let s_reg = "rx";
        let expected = Err(DecodeError::InvalidRegister(s_reg.to_string()));
        let actual = Register::try_from(s_reg);
        assert_eq!(actual, expected);
    }

    #[test]
    fn register_is_valid_a() {
        let s_reg = "ra";
        let expected = Ok(Register::A);
        let actual = Register::try_from(s_reg);
        assert_eq!(actual, expected);
    }

    #[test]
    fn register_is_valid_b() {
        let s_reg = "rb";
        let expected = Ok(Register::B);
        let actual = Register::try_from(s_reg);
        assert_eq!(actual, expected);
    }

    #[test]
    fn register_is_valid_c() {
        let s_reg = "rc";
        let expected = Ok(Register::C);
        let actual = Register::try_from(s_reg);
        assert_eq!(actual, expected);
    }

    #[test]
    fn register_is_valid_d() {
        let s_reg = "rd";
        let expected = Ok(Register::D);
        let actual = Register::try_from(s_reg);
        assert_eq!(actual, expected);
    }

    #[test]
    fn register_is_valid_e() {
        let s_reg = "re";
        let expected = Ok(Register::E);
        let actual = Register::try_from(s_reg);
        assert_eq!(actual, expected);
    }

    #[test]
    fn register_is_valid_f() {
        let s_reg = "rf";
        let expected = Ok(Register::F);
        let actual = Register::try_from(s_reg);
        assert_eq!(actual, expected);
    }

    #[test]
    fn register_is_valid_g() {
        let s_reg = "rg";
        let expected = Ok(Register::G);
        let actual = Register::try_from(s_reg);
        assert_eq!(actual, expected);
    }

    #[test]
    fn register_is_valid_h() {
        let s_reg = "rh";
        let expected = Ok(Register::H);
        let actual = Register::try_from(s_reg);
        assert_eq!(actual, expected);
    }
}
