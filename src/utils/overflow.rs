use crate::constant::{Byte, Half, Quarter, Word};

pub trait Overflow {
    fn overflow_add(self, rhs: Self) -> (Self, bool)
    where
        Self: Sized;
}

impl Overflow for Byte {
    fn overflow_add(self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }
}

impl Overflow for Quarter {
    fn overflow_add(self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }
}

impl Overflow for Half {
    fn overflow_add(self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }
}

impl Overflow for Word {
    fn overflow_add(self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }
}
