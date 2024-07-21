use crate::constant::Word;

#[derive(Debug, PartialEq)]
pub struct Flags {
    pub sign: bool,
    pub overflow: bool,
    pub zero: bool
}

impl Flags {
    pub fn new() -> Self {
        Self { sign: false, overflow: false, zero: false }
    }

    pub fn set(&mut self, result: Word, overflow: bool) {
        self.overflow = overflow;
        self.zero = result == 0;
        self.sign = (result >> 31) == 1;
    }
}

#[cfg(test)]
mod set {
    use super::Flags;

    #[test]
    fn only_zero_flag() {
        let mut f = Flags::new();
        let expected = Flags { sign: false, overflow: false, zero: true };
        f.set(0, false);
        assert_eq!(f, expected);
    }

    #[test]
    fn only_overflow_flag() {
        let mut f = Flags::new();
        let expected = Flags { sign: false, overflow: true, zero: false };
        f.set(1, true);
        assert_eq!(f, expected);
    }

    #[test]
    fn only_sign_flag() {
        let mut f = Flags::new();
        let expected = Flags { sign: true, overflow: false, zero: false };
        f.set(0b10000000_00000000_00000000_00000000, false);
        assert_eq!(f, expected);
    }
}
