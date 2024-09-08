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
    #[expect(
        clippy::indexing_slicing,
        reason = "
            index type is an enum that is compile time asserted to be a safe index

            additionally, the slicing is safe as a register is 64 bits, while
            the types that implement FromBytes, which is necessary for this method
            are all 64 bit or less, meaning no slice would ever be invalid
        "
    )]
    fn get<T: FromBytes>(&self, register: Register) -> T {
        let slice = &self[register].to_le_bytes()[0..size_of::<T>()];
        T::from_bytes(slice)
    }

    #[expect(
        clippy::indexing_slicing,
        reason = "index type is an enum that is compile time asserted to be a safe index"
    )]
    fn set<T: ToWord>(&mut self, register: Register, value: T) {
        self[register] = value.to_word();
    }
}
