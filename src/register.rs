use crate::{constant::Registers, error::ExecuteError};
use super::{
    constant::Word,
    error::DecodeError,
};
use std::ops::{Index, IndexMut};
use variant_count::VariantCount;

/// The P1 and P2 registers are private registers for interal use in the processor.
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
    /// This register stores the first parameter of an instruction.
    P1,
    /// This register stores the second parameter of an instruction.
    P2,
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
    const REG_P1: &'static str = "rp1";
    const REG_P2: &'static str = "rp2";

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
            Register::P1 => Register::REG_P1,
            Register::P2 => Register::REG_P2,
        }
    }

    pub fn as_word(self) -> Word {
        self as Word
    }
}

impl TryFrom<&str> for Register {
    type Error = DecodeError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
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
            unknown => Err(DecodeError::InvalidRegister(unknown.to_string())),
        }
    }
}

impl TryFrom<Word> for Register {
    type Error = ExecuteError;

    fn try_from(w: Word) -> Result<Self, Self::Error> {
        use Register::*;

        match w {
            0 => Ok(A),
            1 => Ok(B),
            2 => Ok(C),
            3 => Ok(D),
            4 => Ok(E),
            5 => Ok(F),
            6 => Ok(G),
            7 => Ok(H),
            unknown => Err(ExecuteError::InvalidRegisterCast(unknown))
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
    use super::Register;
    use crate::error::DecodeError;

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
