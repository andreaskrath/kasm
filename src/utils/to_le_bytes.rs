use crate::constant::{Byte, Half, Quarter, Word};

pub trait ToLeBytes {
    fn to_le_bytes(self) -> Box<[u8]>;
}

impl ToLeBytes for Byte {
    fn to_le_bytes(self) -> Box<[u8]> {
        Box::new(self.to_le_bytes())
    }
}

impl ToLeBytes for Quarter {
    fn to_le_bytes(self) -> Box<[u8]> {
        Box::new(self.to_le_bytes())
    }
}

impl ToLeBytes for Half {
    fn to_le_bytes(self) -> Box<[u8]> {
        Box::new(self.to_le_bytes())
    }
}

impl ToLeBytes for Word {
    fn to_le_bytes(self) -> Box<[u8]> {
        Box::new(self.to_le_bytes())
    }
}
