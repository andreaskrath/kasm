use crate::constant::{Byte, Half, Quarter, Word};

pub trait BitOps {
    fn bit_and(self, rhs: Self) -> Self;
    fn bit_or(self, rhs: Self) -> Self;
    fn bit_xor(self, rhs: Self) -> Self;
    fn bit_not(self) -> Self;
}

impl BitOps for Byte {
    fn bit_and(self, rhs: Self) -> Self {
        self & rhs
    }

    fn bit_or(self, rhs: Self) -> Self {
        self | rhs
    }

    fn bit_xor(self, rhs: Self) -> Self {
        self ^ rhs
    }

    fn bit_not(self) -> Self {
        !self
    }
}

impl BitOps for Quarter {
    fn bit_and(self, rhs: Self) -> Self {
        self & rhs
    }

    fn bit_or(self, rhs: Self) -> Self {
        self | rhs
    }

    fn bit_xor(self, rhs: Self) -> Self {
        self ^ rhs
    }

    fn bit_not(self) -> Self {
        !self
    }
}

impl BitOps for Half {
    fn bit_and(self, rhs: Self) -> Self {
        self & rhs
    }

    fn bit_or(self, rhs: Self) -> Self {
        self | rhs
    }

    fn bit_xor(self, rhs: Self) -> Self {
        self ^ rhs
    }

    fn bit_not(self) -> Self {
        !self
    }
}

impl BitOps for Word {
    fn bit_and(self, rhs: Self) -> Self {
        self & rhs
    }

    fn bit_or(self, rhs: Self) -> Self {
        self | rhs
    }

    fn bit_xor(self, rhs: Self) -> Self {
        self ^ rhs
    }

    fn bit_not(self) -> Self {
        !self
    }
}
