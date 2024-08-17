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

// It does not make sense to test `overflow` flag and `zero` flag as they are trivially set.
//
// As such, only the `sign` flag has test cases.

#[cfg(test)]
mod set_from_byte_sign_flag {
    use super::Flags;

    #[test]
    fn min_value_not_set() {
        let mut f = Flags::new();
        f.set_from_byte(0b0000_0000, false);
        assert!(!f.sign);
    }

    #[test]
    fn min_value_that_sets() {
        let mut f = Flags::new();
        f.set_from_byte(0b1000_0000, false);
        assert!(f.sign);
    }

    #[test]
    fn max_value_that_sets() {
        let mut f = Flags::new();
        f.set_from_byte(0b1111_1111, false);
        assert!(f.sign);
    }
}

#[cfg(test)]
mod set_from_quarter_sign_flag {
    use super::Flags;

    #[test]
    fn min_value_not_set() {
        let mut f = Flags::new();
        f.set_from_quarter(0b0000_0000_0000_0000, false);
        assert!(!f.sign);
    }

    #[test]
    fn min_value_that_sets() {
        let mut f = Flags::new();
        f.set_from_quarter(0b1000_0000_0000_0000, false);
        assert!(f.sign);
    }

    #[test]
    fn max_value_that_sets() {
        let mut f = Flags::new();
        f.set_from_quarter(0b1111_1111_1111_1111, false);
        assert!(f.sign);
    }
}

#[cfg(test)]
mod set_from_half_sign_flag {
    use super::Flags;

    #[test]
    fn min_value_not_set() {
        let mut f = Flags::new();
        f.set_from_half(0b00000000_00000000_00000000_00000000, false);
        assert!(!f.sign);
    }

    #[test]
    fn min_value_that_sets() {
        let mut f = Flags::new();
        f.set_from_half(0b10000000_00000000_00000000_00000000, false);
        assert!(f.sign);
    }

    #[test]
    fn max_value_that_sets() {
        let mut f = Flags::new();
        f.set_from_half(0b11111111_11111111_11111111_11111111, false);
        assert!(f.sign);
    }
}

#[cfg(test)]
mod set_from_word_sign_flag {
    use super::Flags;

    #[test]
    fn min_value_not_set() {
        let mut f = Flags::new();
        f.set_from_word(
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            false,
        );
        assert!(!f.sign);
    }

    #[test]
    fn min_value_that_sets() {
        let mut f = Flags::new();
        f.set_from_word(
            0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            false,
        );
        assert!(f.sign);
    }

    #[test]
    fn max_value_that_sets() {
        let mut f = Flags::new();
        f.set_from_word(
            0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111,
            false,
        );
        assert!(f.sign);
    }
}
