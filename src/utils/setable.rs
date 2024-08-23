use crate::constant::{Byte, Half, Quarter, Word};

pub trait Setable {
    fn is_zero(&self) -> bool;
    fn is_signed(&self) -> bool;
}

impl Setable for Byte {
    fn is_zero(&self) -> bool {
        *self == 0
    }

    fn is_signed(&self) -> bool {
        (*self >> (Byte::BITS - 1)) == 1
    }
}

impl Setable for Quarter {
    fn is_zero(&self) -> bool {
        *self == 0
    }

    fn is_signed(&self) -> bool {
        (*self >> (Quarter::BITS - 1)) == 1
    }
}

impl Setable for Half {
    fn is_zero(&self) -> bool {
        *self == 0
    }

    fn is_signed(&self) -> bool {
        (*self >> (Half::BITS - 1)) == 1
    }
}

impl Setable for Word {
    fn is_zero(&self) -> bool {
        *self == 0
    }

    fn is_signed(&self) -> bool {
        (*self >> (Word::BITS - 1)) == 1
    }
}
