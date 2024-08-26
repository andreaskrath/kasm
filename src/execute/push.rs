use crate::{
    constant::Word,
    error::ExecuteError,
    instruction::Push,
    utils::{FromBytes, ToBytes},
    Processor,
};

impl Processor {
    pub fn push(&mut self, instruction: Push) -> Result<(), ExecuteError> {
        match instruction {
            Push::Byte(operand) => {
                let value = self.get_operand_value(operand);
                self.push_value(value)?
            }
            Push::Quarter(operand) => {
                let value = self.get_operand_value(operand);
                self.push_value(value)?
            }
            Push::Half(operand) => {
                let value = self.get_operand_value(operand);
                self.push_value(value)?
            }
            Push::Word(operand) => {
                let value = self.get_operand_value(operand);
                self.push_value(value)?
            }
        }

        Ok(())
    }

    pub fn push_value<T>(&mut self, value: T) -> Result<(), ExecuteError>
    where
        T: ToBytes + FromBytes,
    {
        if self.sp() + size_of::<T>() > self.stack.len() {
            return Err(ExecuteError::StackOverflow);
        }

        for (index, byte) in value.to_bytes().iter().enumerate() {
            self.stack[self.sp() + index] = *byte;
        }

        self.stack_pointer += size_of::<T>() as Word;

        Ok(())
    }
}

#[cfg(test)]
mod byte {
    use crate::{
        constant::{Byte, Word, TEST_STACK_SIZE},
        error::ExecuteError,
        Processor,
    };

    #[test]
    fn stack_overflow() {
        let mut p = Processor::new_test();
        p.stack_pointer = TEST_STACK_SIZE as Word;
        let expected = Err(ExecuteError::StackOverflow);

        let actual = p.push_value(Byte::MAX);

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_stack_overflow() {
        let mut p = Processor::new_test();
        p.stack_pointer = (TEST_STACK_SIZE - size_of::<Byte>()) as Word;
        let expected = Byte::MAX;

        p.push_value(Byte::MAX).unwrap();
        let actual = p.stack[TEST_STACK_SIZE - size_of::<Byte>()];

        assert_eq!(actual, expected);
        assert!(p.stack_pointer == TEST_STACK_SIZE as u64);
    }
}

#[cfg(test)]
mod quarter {
    use crate::{
        constant::{Quarter, Word, TEST_STACK_SIZE},
        error::ExecuteError,
        Processor,
    };

    #[test]
    fn stack_overflow() {
        let mut p = Processor::new_test();
        p.stack_pointer = TEST_STACK_SIZE as Word;
        let expected = Err(ExecuteError::StackOverflow);

        let actual = p.push_value(Quarter::MAX);

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_stack_overflow() {
        let mut p = Processor::new_test();
        p.stack_pointer = (TEST_STACK_SIZE - size_of::<Quarter>()) as Word;
        let expected = Quarter::MAX.to_le_bytes();

        p.push_value(Quarter::MAX).unwrap();
        let actual = &p.stack[TEST_STACK_SIZE - size_of::<Quarter>()..TEST_STACK_SIZE];

        assert_eq!(actual, expected);
        assert!(p.stack_pointer == TEST_STACK_SIZE as u64);
    }
}

#[cfg(test)]
mod half {
    use crate::{
        constant::{Half, Word, TEST_STACK_SIZE},
        error::ExecuteError,
        Processor,
    };

    #[test]
    fn stack_overflow() {
        let mut p = Processor::new_test();
        p.stack_pointer = TEST_STACK_SIZE as Word;
        let expected = Err(ExecuteError::StackOverflow);

        let actual = p.push_value(Half::MAX);

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_stack_overflow() {
        let mut p = Processor::new_test();
        p.stack_pointer = (TEST_STACK_SIZE - size_of::<Half>()) as Word;
        let expected = Half::MAX.to_le_bytes();

        p.push_value(Half::MAX).unwrap();
        let actual = &p.stack[TEST_STACK_SIZE - size_of::<Half>()..TEST_STACK_SIZE];

        assert_eq!(actual, expected);
        assert!(p.stack_pointer == TEST_STACK_SIZE as u64);
    }
}

#[cfg(test)]
mod word {
    use crate::{
        constant::{Word, TEST_STACK_SIZE},
        error::ExecuteError,
        Processor,
    };

    #[test]
    fn stack_overflow() {
        let mut p = Processor::new_test();
        p.stack_pointer = TEST_STACK_SIZE as Word;
        let expected = Err(ExecuteError::StackOverflow);

        let actual = p.push_value(Word::MAX);

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_stack_overflow() {
        let mut p = Processor::new_test();
        p.stack_pointer = (TEST_STACK_SIZE - size_of::<Word>()) as Word;
        let expected = Word::MAX.to_le_bytes();

        p.push_value(Word::MAX).unwrap();
        let actual = &p.stack[TEST_STACK_SIZE - size_of::<Word>()..TEST_STACK_SIZE];

        assert_eq!(actual, expected);
        assert!(p.stack_pointer == TEST_STACK_SIZE as u64);
    }
}
