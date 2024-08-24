use crate::constant::{Byte, Half, Quarter, Word};

pub trait ToWord {
    fn to_word(self) -> Word;
}

impl ToWord for Byte {
    fn to_word(self) -> Word {
        self as Word
    }
}

impl ToWord for Quarter {
    fn to_word(self) -> Word {
        self as Word
    }
}

impl ToWord for Half {
    fn to_word(self) -> Word {
        self as Word
    }
}

impl ToWord for Word {
    fn to_word(self) -> Word {
        self as Word
    }
}
