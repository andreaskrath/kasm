use crate::constant::{Byte, Half, Quarter, Word};

pub trait ToBytes {
    fn to_bytes(self) -> Box<[u8]>;
}

impl ToBytes for Byte {
    fn to_bytes(self) -> Box<[u8]> {
        Box::new(self.to_le_bytes())
    }
}

impl ToBytes for Quarter {
    fn to_bytes(self) -> Box<[u8]> {
        Box::new(self.to_le_bytes())
    }
}

impl ToBytes for Half {
    fn to_bytes(self) -> Box<[u8]> {
        Box::new(self.to_le_bytes())
    }
}

impl ToBytes for Word {
    fn to_bytes(self) -> Box<[u8]> {
        Box::new(self.to_le_bytes())
    }
}
