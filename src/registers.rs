use crate::{constant::Word, register::Register, utils::FromBytes};

pub type Registers = [Word; Register::VARIANT_COUNT];

pub trait RegisterOperations {
    fn get<T: FromBytes>(&self, register: Register) -> T;
}

impl RegisterOperations for Registers {
    fn get<T: FromBytes>(&self, register: Register) -> T {
        let slice = &self[register].to_le_bytes()[0..size_of::<T>()];
        T::from_bytes(slice)
    }
}
