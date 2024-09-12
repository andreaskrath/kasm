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

    pub fn sp(&self) -> usize {
        self.pointer as usize
    }

    #[expect(
        clippy::indexing_slicing,
        reason = "the indexing in the for loop is guarded by the condition at the start of the function"
    )]
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

    #[expect(
        clippy::indexing_slicing,
        reason = "
            the slicing is safe due to the checked subtraction when computing lower bound
            it ensures that there are enough bytes on the stack to build the requested type
        "
    )]
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

    #[expect(
        clippy::indexing_slicing,
        reason = "
            the slicing is safe due to the checked subtraction when computing lower bound
            it ensures that there are enough bytes on the stack to build the requested type
        "
    )]
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
}

#[expect(
    clippy::indexing_slicing,
    reason = "it is necessary to index directly in the test cases"
)]
#[cfg(test)]
mod push {
    mod byte {
        use crate::{
            constant::{Byte, Word, TEST_STACK_SIZE},
            error::ExecuteError,
            stack::Stack,
        };

        const TYPE_SIZE: usize = size_of::<Byte>();

        #[test]
        fn stack_overflow() {
            let mut s = Stack::new(TEST_STACK_SIZE);
            s.pointer = TEST_STACK_SIZE as Word;
            let expected = Err(ExecuteError::StackOverflow);

            let actual = s.push(Byte::MAX);

            assert_eq!(actual, expected);
        }

        #[test]
        fn different_byte_values() -> Result<(), ExecuteError> {
            let mut s = Stack::new(TEST_STACK_SIZE);
            let value = Byte::MAX / 2;
            let expected = value.to_le_bytes();

            s.push(value)?;
            let actual = &s.bytes[0..TYPE_SIZE];

            assert_eq!(actual, expected);
            assert_eq!(s.pointer, TYPE_SIZE as Word);

            Ok(())
        }
    }

    mod quarter {
        use crate::{
            constant::{Quarter, Word, TEST_STACK_SIZE},
            error::ExecuteError,
            stack::Stack,
        };

        const TYPE_SIZE: usize = size_of::<Quarter>();

        #[test]
        fn stack_overflow() {
            let mut s = Stack::new(TEST_STACK_SIZE);
            s.pointer = TEST_STACK_SIZE as Word;
            let expected = Err(ExecuteError::StackOverflow);

            let actual = s.push(Quarter::MAX);

            assert_eq!(actual, expected);
        }

        #[test]
        fn different_byte_values() -> Result<(), ExecuteError> {
            let mut s = Stack::new(TEST_STACK_SIZE);
            let value = Quarter::MAX / 2;
            let expected = value.to_le_bytes();

            s.push(value)?;
            let actual = &s.bytes[0..TYPE_SIZE];

            assert_eq!(actual, expected);
            assert_eq!(s.pointer, TYPE_SIZE as Word);

            Ok(())
        }
    }

    mod half {
        use crate::{
            constant::{Half, Word, TEST_STACK_SIZE},
            error::ExecuteError,
            stack::Stack,
        };

        const TYPE_SIZE: usize = size_of::<Half>();

        #[test]
        fn stack_overflow() {
            let mut s = Stack::new(TEST_STACK_SIZE);
            s.pointer = TEST_STACK_SIZE as Word;
            let expected = Err(ExecuteError::StackOverflow);

            let actual = s.push(Half::MAX);

            assert_eq!(actual, expected);
        }

        #[test]
        fn different_byte_values() -> Result<(), ExecuteError> {
            let mut s = Stack::new(TEST_STACK_SIZE);
            let value = Half::MAX / 2;
            let expected = value.to_le_bytes();

            s.push(value)?;
            let actual = &s.bytes[0..TYPE_SIZE];

            assert_eq!(actual, expected);
            assert_eq!(s.pointer, TYPE_SIZE as Word);

            Ok(())
        }
    }

    mod word {
        use crate::{
            constant::{Word, TEST_STACK_SIZE},
            error::ExecuteError,
            stack::Stack,
        };

        const TYPE_SIZE: usize = size_of::<Word>();

        #[test]
        fn stack_overflow() {
            let mut s = Stack::new(TEST_STACK_SIZE);
            s.pointer = TEST_STACK_SIZE as Word;
            let expected = Err(ExecuteError::StackOverflow);

            let actual = s.push(Word::MAX);

            assert_eq!(actual, expected);
        }

        #[test]
        fn different_byte_values() -> Result<(), ExecuteError> {
            let mut s = Stack::new(TEST_STACK_SIZE);
            let value = Word::MAX;
            let expected = value.to_le_bytes();

            s.push(value)?;
            let actual = &s.bytes[0..TYPE_SIZE];

            assert_eq!(actual, expected);
            assert_eq!(s.pointer, TYPE_SIZE as Word);

            Ok(())
        }
    }
}

#[expect(
    clippy::indexing_slicing,
    reason = "it is necessary to index directly in the test cases"
)]
#[cfg(test)]
mod pop {
    mod byte {
        use crate::{
            constant::{Byte, Word, TEST_STACK_SIZE},
            error::ExecuteError,
            stack::Stack,
        };

        const TYPE_SIZE: usize = size_of::<Byte>();

        #[test]
        fn stack_underflow() {
            let mut s = Stack::new(TEST_STACK_SIZE);
            let expected = Err(ExecuteError::StackUnderflow);

            let actual = s.pop::<Byte>();

            assert_eq!(actual, expected);
        }

        #[test]
        fn different_byte_values() -> Result<(), ExecuteError> {
            let mut s = Stack::new(TEST_STACK_SIZE);
            let value = Byte::MAX / 2;
            s.bytes[0..TYPE_SIZE].copy_from_slice(&value.to_le_bytes());
            s.pointer = TYPE_SIZE as Word;
            let expected = value;

            let actual = s.pop::<Byte>()?;

            assert_eq!(actual, expected);
            assert_eq!(s.pointer, 0);

            Ok(())
        }
    }

    mod quarter {
        use crate::{
            constant::{Quarter, Word, TEST_STACK_SIZE},
            error::ExecuteError,
            stack::Stack,
        };

        const TYPE_SIZE: usize = size_of::<Quarter>();

        #[test]
        fn stack_underflow() {
            let mut s = Stack::new(TEST_STACK_SIZE);
            let expected = Err(ExecuteError::StackUnderflow);

            let actual = s.pop::<Quarter>();

            assert_eq!(actual, expected);
        }

        #[test]
        fn different_byte_values() -> Result<(), ExecuteError> {
            let mut s = Stack::new(TEST_STACK_SIZE);
            let value = Quarter::MAX / 2;
            s.bytes[0..TYPE_SIZE].copy_from_slice(&value.to_le_bytes());
            s.pointer = TYPE_SIZE as Word;
            let expected = value;

            let actual = s.pop::<Quarter>()?;

            assert_eq!(actual, expected);
            assert_eq!(s.pointer, 0);

            Ok(())
        }
    }

    mod half {
        use crate::{
            constant::{Half, Word, TEST_STACK_SIZE},
            error::ExecuteError,
            stack::Stack,
        };

        const TYPE_SIZE: usize = size_of::<Half>();

        #[test]
        fn stack_underflow() {
            let mut s = Stack::new(TEST_STACK_SIZE);
            let expected = Err(ExecuteError::StackUnderflow);

            let actual = s.pop::<Half>();

            assert_eq!(actual, expected);
        }

        #[test]
        fn different_byte_values() -> Result<(), ExecuteError> {
            let mut s = Stack::new(TEST_STACK_SIZE);
            let value = Half::MAX / 2;
            s.bytes[0..TYPE_SIZE].copy_from_slice(&value.to_le_bytes());
            s.pointer = TYPE_SIZE as Word;
            let expected = value;

            let actual = s.pop::<Half>()?;

            assert_eq!(actual, expected);
            assert_eq!(s.pointer, 0);

            Ok(())
        }
    }

    mod word {
        use crate::{
            constant::{Word, TEST_STACK_SIZE},
            error::ExecuteError,
            stack::Stack,
        };

        const TYPE_SIZE: usize = size_of::<Word>();

        #[test]
        fn stack_underflow() {
            let mut s = Stack::new(TEST_STACK_SIZE);
            let expected = Err(ExecuteError::StackUnderflow);

            let actual = s.pop::<Word>();

            assert_eq!(actual, expected);
        }

        #[test]
        fn different_byte_values() -> Result<(), ExecuteError> {
            let mut s = Stack::new(TEST_STACK_SIZE);
            let value = Word::MAX / 2;
            s.bytes[0..TYPE_SIZE].copy_from_slice(&value.to_le_bytes());
            s.pointer = TYPE_SIZE as Word;
            let expected = value;

            let actual = s.pop::<Word>()?;

            assert_eq!(actual, expected);
            assert_eq!(s.pointer, 0);

            Ok(())
        }
    }
}

#[expect(
    clippy::indexing_slicing,
    reason = "it is necessary to index directly in the test cases"
)]
#[cfg(test)]
mod slice {
    mod byte {
        use crate::{
            constant::{Byte, Word, TEST_STACK_SIZE},
            error::ExecuteError,
            stack::Stack,
        };

        const TYPE_SIZE: usize = size_of::<Byte>();

        #[test]
        fn stack_underflow() {
            let s = Stack::new(TEST_STACK_SIZE);
            let expected = Err(ExecuteError::StackUnderflow);

            let actual = s.slice::<Byte>(1);

            assert_eq!(actual, expected);
        }

        #[test]
        fn different_byte_values() -> Result<(), ExecuteError> {
            let mut s = Stack::new(TEST_STACK_SIZE);
            let value = [(Byte::MAX / 2).to_le_bytes(), (Byte::MAX / 2).to_le_bytes()]
                .as_flattened()
                .to_vec();
            s.bytes[0..TYPE_SIZE * 2].copy_from_slice(&value);
            s.pointer = TYPE_SIZE as Word * 2;
            let expected = &[Byte::MAX / 2, Byte::MAX / 2];

            let actual = s.slice::<Byte>(2)?;

            assert_eq!(*actual, *expected);
            assert_eq!(s.pointer, TYPE_SIZE as Word * 2);

            Ok(())
        }
    }

    mod quarter {
        use crate::{
            constant::{Quarter, Word, TEST_STACK_SIZE},
            error::ExecuteError,
            stack::Stack,
        };

        const TYPE_SIZE: usize = size_of::<Quarter>();

        #[test]
        fn stack_underflow() {
            let s = Stack::new(TEST_STACK_SIZE);
            let expected = Err(ExecuteError::StackUnderflow);

            let actual = s.slice::<Quarter>(1);

            assert_eq!(actual, expected);
        }

        #[test]
        fn different_byte_values() -> Result<(), ExecuteError> {
            let mut s = Stack::new(TEST_STACK_SIZE);
            let value = [
                (Quarter::MAX / 2).to_le_bytes(),
                (Quarter::MAX / 2).to_le_bytes(),
            ]
            .as_flattened()
            .to_vec();
            s.bytes[0..TYPE_SIZE * 2].copy_from_slice(&value);
            s.pointer = TYPE_SIZE as Word * 2;
            let expected = &[Quarter::MAX / 2, Quarter::MAX / 2];

            let actual = s.slice::<Quarter>(2)?;

            assert_eq!(*actual, *expected);
            assert_eq!(s.pointer, TYPE_SIZE as Word * 2);

            Ok(())
        }
    }

    mod half {
        use crate::{
            constant::{Half, Word, TEST_STACK_SIZE},
            error::ExecuteError,
            stack::Stack,
        };

        const TYPE_SIZE: usize = size_of::<Half>();

        #[test]
        fn stack_underflow() {
            let s = Stack::new(TEST_STACK_SIZE);
            let expected = Err(ExecuteError::StackUnderflow);

            let actual = s.slice::<Half>(1);

            assert_eq!(actual, expected);
        }

        #[test]
        fn different_byte_values() -> Result<(), ExecuteError> {
            let mut s = Stack::new(TEST_STACK_SIZE);
            let value = [(Half::MAX / 2).to_le_bytes(), (Half::MAX / 2).to_le_bytes()]
                .as_flattened()
                .to_vec();
            s.bytes[0..TYPE_SIZE * 2].copy_from_slice(&value);
            s.pointer = TYPE_SIZE as Word * 2;
            let expected = &[Half::MAX / 2, Half::MAX / 2];

            let actual = s.slice::<Half>(2)?;

            assert_eq!(*actual, *expected);
            assert_eq!(s.pointer, TYPE_SIZE as Word * 2);

            Ok(())
        }
    }

    mod word {
        use crate::{
            constant::{Word, TEST_STACK_SIZE},
            error::ExecuteError,
            stack::Stack,
        };

        const TYPE_SIZE: usize = size_of::<Word>();

        #[test]
        fn stack_underflow() {
            let s = Stack::new(TEST_STACK_SIZE);
            let expected = Err(ExecuteError::StackUnderflow);

            let actual = s.slice::<Word>(1);

            assert_eq!(actual, expected);
        }

        #[test]
        fn different_byte_values() -> Result<(), ExecuteError> {
            let mut s = Stack::new(TEST_STACK_SIZE);
            let value = [(Word::MAX / 2).to_le_bytes(), (Word::MAX / 2).to_le_bytes()]
                .as_flattened()
                .to_vec();
            s.bytes[0..TYPE_SIZE * 2].copy_from_slice(&value);
            s.pointer = TYPE_SIZE as Word * 2;
            let expected = &[Word::MAX / 2, Word::MAX / 2];

            let actual = s.slice::<Word>(2)?;

            assert_eq!(*actual, *expected);
            assert_eq!(s.pointer, TYPE_SIZE as Word * 2);

            Ok(())
        }
    }
}
