use crate::utils::Setable;

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

    pub fn set<T: Setable>(&mut self, result: T, overflow: bool) {
        self.overflow = overflow;
        self.zero = result.is_zero();
        self.sign = result.is_signed();
    }
}

// It does not make sense to test `overflow` flag and `zero` flag as they are trivially set.
//
// As such, only the `sign` flag has test cases.

#[cfg(test)]
mod byte {
    use super::Flags;
    use crate::constant::Byte;

    #[test]
    fn min_value_not_set() {
        let mut f = Flags::new();
        f.set(0b0000_0000 as Byte, false);
        assert!(!f.sign);
    }

    #[test]
    fn min_value_that_sets() {
        let mut f = Flags::new();
        f.set(0b1000_0000 as Byte, false);
        assert!(f.sign);
    }

    #[test]
    fn max_value_that_sets() {
        let mut f = Flags::new();
        f.set(0b1111_1111 as Byte, false);
        assert!(f.sign);
    }
}

#[cfg(test)]
mod quarter {
    use super::Flags;
    use crate::constant::Quarter;

    #[test]
    fn min_value_not_set() {
        let mut f = Flags::new();
        f.set(0b0000_0000_0000_0000 as Quarter, false);
        assert!(!f.sign);
    }

    #[test]
    fn min_value_that_sets() {
        let mut f = Flags::new();
        f.set(0b1000_0000_0000_0000 as Quarter, false);
        assert!(f.sign);
    }

    #[test]
    fn max_value_that_sets() {
        let mut f = Flags::new();
        f.set(0b1111_1111_1111_1111 as Quarter, false);
        assert!(f.sign);
    }
}

#[cfg(test)]
mod half {
    use crate::constant::Half;

    use super::Flags;

    #[test]
    fn min_value_not_set() {
        let mut f = Flags::new();
        f.set(0b00000000_00000000_00000000_00000000 as Half, false);
        assert!(!f.sign);
    }

    #[test]
    fn min_value_that_sets() {
        let mut f = Flags::new();
        f.set(0b10000000_00000000_00000000_00000000 as Half, false);
        assert!(f.sign);
    }

    #[test]
    fn max_value_that_sets() {
        let mut f = Flags::new();
        f.set(0b11111111_11111111_11111111_11111111 as Half, false);
        assert!(f.sign);
    }
}

#[cfg(test)]
mod word {
    use crate::constant::Word;

    use super::Flags;

    #[test]
    fn min_value_not_set() {
        let mut f = Flags::new();
        f.set(
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000 as Word,
            false,
        );
        assert!(!f.sign);
    }

    #[test]
    fn min_value_that_sets() {
        let mut f = Flags::new();
        f.set(
            0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000 as Word,
            false,
        );
        assert!(f.sign);
    }

    #[test]
    fn max_value_that_sets() {
        let mut f = Flags::new();
        f.set(
            0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111 as Word,
            false,
        );
        assert!(f.sign);
    }
}
