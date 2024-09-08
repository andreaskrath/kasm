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
    #[expect(
        unsafe_code,
        reason = "
                this is the only way to reinterpted the underlying bits into a new type, 
                while also maintaining the same numerical value if reinterpretted back to the original type
            "
    )]
    fn to_word(self) -> Word {
        let byte: Byte = unsafe { mem::transmute(self) };
        Word::from(byte)
    }
}

impl ToWord for SignedQuarter {
    #[expect(
        unsafe_code,
        reason = "
                this is the only way to reinterpted the underlying bits into a new type, 
                while also maintaining the same numerical value if reinterpretted back to the original type
            "
    )]
    fn to_word(self) -> Word {
        let quarter: Quarter = unsafe { mem::transmute(self) };
        Word::from(quarter)
    }
}

impl ToWord for SignedHalf {
    #[expect(
        unsafe_code,
        reason = "
                this is the only way to reinterpted the underlying bits into a new type, 
                while also maintaining the same numerical value if reinterpretted back to the original type
            "
    )]
    fn to_word(self) -> Word {
        let half: Half = unsafe { mem::transmute(self) };
        Word::from(half)
    }
}

impl ToWord for SignedWord {
    #[expect(
        unsafe_code,
        reason = "
                this is the only way to reinterpted the underlying bits into a new type, 
                while also maintaining the same numerical value if reinterpretted back to the original type
            "
    )]
    fn to_word(self) -> Word {
        unsafe { mem::transmute(self) }
    }
}

#[cfg(test)]
mod signed_byte {
    use crate::{
        constant::{Byte, SignedByte, Word},
        utils::ToWord,
    };

    #[test]
    fn within_intersected_range() {
        let input: SignedByte = 50;
        let expected = 50;

        let actual = input.to_word();

        assert_eq!(actual, expected);
    }

    #[test]
    fn outside_intersected_range() {
        let input: SignedByte = -1;
        let expected = Word::from(Byte::MAX);

        let actual = input.to_word();

        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod signed_quarter {
    use crate::{
        constant::{Quarter, SignedQuarter, Word},
        utils::ToWord,
    };

    #[test]
    fn within_intersected_range() {
        let input: SignedQuarter = 50;
        let expected = 50;

        let actual = input.to_word();

        assert_eq!(actual, expected);
    }

    #[test]
    fn outside_intersected_range() {
        let input: SignedQuarter = -1;
        let expected = Word::from(Quarter::MAX);

        let actual = input.to_word();

        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod signed_half {
    use crate::{
        constant::{Half, SignedHalf, Word},
        utils::ToWord,
    };

    #[test]
    fn within_intersected_range() {
        let input: SignedHalf = 50;
        let expected = 50;

        let actual = input.to_word();

        assert_eq!(actual, expected);
    }

    #[test]
    fn outside_intersected_range() {
        let input: SignedHalf = -1;
        let expected = Word::from(Half::MAX);

        let actual = input.to_word();

        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod signed_word {
    use crate::{
        constant::{SignedWord, Word},
        utils::ToWord,
    };

    #[test]
    fn within_intersected_range() {
        let input: SignedWord = 50;
        let expected = 50;

        let actual = input.to_word();

        assert_eq!(actual, expected);
    }

    #[test]
    fn outside_intersected_range() {
        let input: SignedWord = -1;
        let expected = Word::MAX;

        let actual = input.to_word();

        assert_eq!(actual, expected);
    }
}
