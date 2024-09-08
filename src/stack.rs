use crate::{
    constant::{Byte, Word},
    error::ExecuteError,
    utils::{FromBytes, ToBytes},
};

pub struct Stack {
    /// The next free byte on the stack.
    pointer: Word,
    bytes: Box<[Byte]>,
}

impl Stack {
    pub fn new(size: usize) -> Self {
        // Kind of a hack, but simply allocating an array inside a box causes a stack overflow.
        // https://github.com/rust-lang/rust/issues/53827
        let bytes = vec![0; size].into_boxed_slice();

        Self { bytes, pointer: 0 }
    }

    pub fn push<T>(&mut self, value: T) -> Result<(), ExecuteError>
    where
        T: ToBytes,
    {
        if self.sp() + size_of::<T>() > self.bytes.len() {
            return Err(ExecuteError::StackOverflow);
        }

        for (index, byte) in value.to_bytes().iter().enumerate() {
            self.bytes[self.sp() + index] = *byte;
        }

        self.pointer += size_of::<T>() as Word;

        Ok(())
    }

    pub fn pop<T>(&mut self) -> Result<T, ExecuteError>
    where
        T: FromBytes,
    {
        let type_size = size_of::<T>();
        let lower_bound = self
            .sp()
            .checked_sub(type_size)
            .ok_or(ExecuteError::StackUnderflow)?;

        let bytes = &self.bytes[lower_bound..self.sp()];
        let value = T::from_bytes(bytes);

        self.pointer -= type_size as Word;

        Ok(value)
    }

    pub fn slice<T>(&self, amount: Word) -> Result<Box<[T]>, ExecuteError>
    where
        T: FromBytes,
    {
        let index_offset = amount as usize * size_of::<T>();
        let lower_bound = self
            .sp()
            .checked_sub(index_offset)
            .ok_or(ExecuteError::StackUnderflow)?;

        let bytes = &self.bytes[lower_bound..self.sp()];
        let slice: Box<[T]> = bytes
            .chunks_exact(size_of::<T>())
            .map(|chunk| T::from_bytes(chunk))
            .collect();

        Ok(slice)
    }

    fn sp(&self) -> usize {
        self.pointer as usize
    }
}
