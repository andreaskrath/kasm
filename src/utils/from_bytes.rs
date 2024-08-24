use crate::constant::{Byte, Half, Quarter, Word};

pub trait FromBytes {
    fn from_bytes(slice: &[Byte]) -> Self;
}

impl FromBytes for Byte {
    fn from_bytes(slice: &[Byte]) -> Self {
        slice[0]
    }
}

impl FromBytes for Quarter {
    fn from_bytes(slice: &[Byte]) -> Self {
        let mut bytes = [0; size_of::<Quarter>()];
        bytes.copy_from_slice(&slice[0..size_of::<Quarter>()]);
        Quarter::from_le_bytes(bytes)
    }
}

impl FromBytes for Half {
    fn from_bytes(slice: &[Byte]) -> Self {
        let mut bytes = [0; size_of::<Half>()];
        bytes.copy_from_slice(&slice[0..size_of::<Half>()]);
        Half::from_le_bytes(bytes)
    }
}

impl FromBytes for Word {
    fn from_bytes(slice: &[Byte]) -> Self {
        let mut bytes = [0; size_of::<Word>()];
        bytes.copy_from_slice(&slice[0..size_of::<Word>()]);
        Word::from_le_bytes(bytes)
    }
}
