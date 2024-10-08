use crate::constant::{Byte, Half, Quarter, Word};

pub trait Overflow {
    fn overflow_add(self, rhs: Self) -> (Self, bool)
    where
        Self: Sized;

    fn overflow_sub(self, rhs: Self) -> (Self, bool)
    where
        Self: Sized;

    fn overflow_mul(self, rhs: Self) -> (Self, bool)
    where
        Self: Sized;

    fn overflow_div(self, rhs: Self) -> (Self, bool)
    where
        Self: Sized;

    fn overflow_rem(self, rhs: Self) -> (Self, bool)
    where
        Self: Sized;
}

impl Overflow for Byte {
    fn overflow_add(self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }

    fn overflow_sub(self, rhs: Self) -> (Self, bool) {
        self.overflowing_sub(rhs)
    }

    fn overflow_mul(self, rhs: Self) -> (Self, bool) {
        self.overflowing_mul(rhs)
    }

    fn overflow_div(self, rhs: Self) -> (Self, bool) {
        self.overflowing_div(rhs)
    }

    fn overflow_rem(self, rhs: Self) -> (Self, bool) {
        self.overflowing_rem(rhs)
    }
}

impl Overflow for Quarter {
    fn overflow_add(self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }

    fn overflow_sub(self, rhs: Self) -> (Self, bool) {
        self.overflowing_sub(rhs)
    }

    fn overflow_mul(self, rhs: Self) -> (Self, bool) {
        self.overflowing_mul(rhs)
    }

    fn overflow_div(self, rhs: Self) -> (Self, bool) {
        self.overflowing_div(rhs)
    }

    fn overflow_rem(self, rhs: Self) -> (Self, bool) {
        self.overflowing_rem(rhs)
    }
}

impl Overflow for Half {
    fn overflow_add(self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }

    fn overflow_sub(self, rhs: Self) -> (Self, bool) {
        self.overflowing_sub(rhs)
    }

    fn overflow_mul(self, rhs: Self) -> (Self, bool) {
        self.overflowing_mul(rhs)
    }

    fn overflow_div(self, rhs: Self) -> (Self, bool) {
        self.overflowing_div(rhs)
    }

    fn overflow_rem(self, rhs: Self) -> (Self, bool) {
        self.overflowing_rem(rhs)
    }
}

impl Overflow for Word {
    fn overflow_add(self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }

    fn overflow_sub(self, rhs: Self) -> (Self, bool) {
        self.overflowing_sub(rhs)
    }

    fn overflow_mul(self, rhs: Self) -> (Self, bool) {
        self.overflowing_mul(rhs)
    }

    fn overflow_div(self, rhs: Self) -> (Self, bool) {
        self.overflowing_div(rhs)
    }

    fn overflow_rem(self, rhs: Self) -> (Self, bool) {
        self.overflowing_rem(rhs)
    }
}
