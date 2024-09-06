use crate::constant::{
    Byte, Half, Quarter, SignedByte, SignedHalf, SignedQuarter, SignedWord, Word,
};
use std::mem;

pub trait ToWord {
    fn to_word(self) -> Word;
}

impl ToWord for Byte {
    fn to_word(self) -> Word {
        Word::from(self)
    }
}

impl ToWord for Quarter {
    fn to_word(self) -> Word {
        Word::from(self)
    }
}

impl ToWord for Half {
    fn to_word(self) -> Word {
        Word::from(self)
    }
}

impl ToWord for Word {
    fn to_word(self) -> Word {
        self
    }
}

impl ToWord for SignedByte {
    fn to_word(self) -> Word {
        #[expect(
            unsafe_code,
            reason = "
                this is the only way to reinterpted the underlying bits into a new type, 
                while also maintaining the same numerical value if reinterpretted back to the original type
            "
        )]
        let byte: Byte = unsafe { mem::transmute(self) };
        Word::from(byte)
    }
}

impl ToWord for SignedQuarter {
    fn to_word(self) -> Word {
        #[expect(
            unsafe_code,
            reason = "
                this is the only way to reinterpted the underlying bits into a new type, 
                while also maintaining the same numerical value if reinterpretted back to the original type
            "
        )]
        let quarter: Quarter = unsafe { mem::transmute(self) };
        Word::from(quarter)
    }
}

impl ToWord for SignedHalf {
    fn to_word(self) -> Word {
        #[expect(
            unsafe_code,
            reason = "
                this is the only way to reinterpted the underlying bits into a new type, 
                while also maintaining the same numerical value if reinterpretted back to the original type
            "
        )]
        let half: Half = unsafe { mem::transmute(self) };
        Word::from(half)
    }
}

impl ToWord for SignedWord {
    fn to_word(self) -> Word {
        #[expect(
            unsafe_code,
            reason = "
                this is the only way to reinterpted the underlying bits into a new type, 
                while also maintaining the same numerical value if reinterpretted back to the original type
            "
        )]
        unsafe {
            mem::transmute(self)
        }
    }
}
