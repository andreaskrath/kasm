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
