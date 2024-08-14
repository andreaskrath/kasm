use crate::constant::{Byte, Half, Quarter, Word};

#[derive(Debug, PartialEq)]
pub struct Flags {
    pub sign: bool,
    pub overflow: bool,
    pub zero: bool,
}

impl Flags {
    pub fn new() -> Self {
        Self {
            sign: false,
            overflow: false,
            zero: false,
        }
    }

    pub fn set_from_byte(&mut self, result: Byte, overflow: bool) {
        self.overflow = overflow;
        self.zero = result == 0;
        self.sign = (result >> (Byte::BITS - 1)) == 1;
    }

    pub fn set_from_quarter(&mut self, result: Quarter, overflow: bool) {
        self.overflow = overflow;
        self.zero = result == 0;
        self.sign = (result >> (Quarter::BITS - 1)) == 1;
    }

    pub fn set_from_half(&mut self, result: Half, overflow: bool) {
        self.overflow = overflow;
        self.zero = result == 0;
        self.sign = (result >> (Half::BITS - 1)) == 1;
    }

    pub fn set_from_word(&mut self, result: Word, overflow: bool) {
        self.overflow = overflow;
        self.zero = result == 0;
        self.sign = (result >> (Word::BITS - 1)) == 1;
    }
}

