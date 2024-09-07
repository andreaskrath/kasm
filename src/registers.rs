use crate::{
    constant::Word,
    register::Register,
    utils::{FromBytes, ToWord},
};

pub type Registers = [Word; Register::VARIANT_COUNT];

pub trait RegisterOperations {
    fn get<T: FromBytes>(&self, register: Register) -> T;
    fn set<T: ToWord>(&mut self, register: Register, value: T);
}

impl RegisterOperations for Registers {
    fn get<T: FromBytes>(&self, register: Register) -> T {
        let slice = &self[register].to_le_bytes()[0..size_of::<T>()];
        T::from_bytes(slice)
    }

    fn set<T: ToWord>(&mut self, register: Register, value: T) {
        let _ = self[register] = value.to_word();
    }
}
