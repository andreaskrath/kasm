use crate::{
    constant::{Byte, Half, Quarter, Word},
    error::ExecuteError,
    instruction::Pop,
    utils::FromBytes,
    Processor,
};

impl Processor {
    pub fn pop(&mut self, instruction: Pop) -> Result<(), ExecuteError> {
        match instruction {
            Pop::Byte(register) => {
                let value = self.pop_value::<Byte>()?;
                self.registers[register] = value as Word;
            }
            Pop::Quarter(register) => {
                let value = self.pop_value::<Quarter>()?;
                self.registers[register] = value as Word;
            }
            Pop::Half(register) => {
                let value = self.pop_value::<Half>()?;
                self.registers[register] = value as Word;
            }
            Pop::Word(register) => {
                let value = self.pop_value::<Word>()?;
                self.registers[register] = value;
            }
        }
        Ok(())
    }

    pub fn pop_value<T>(&mut self) -> Result<T, ExecuteError>
    where
        T: FromBytes,
    {
        let lower_bound = self
            .sp()
            .checked_sub(size_of::<T>())
            .ok_or(ExecuteError::StackUnderflow)?;

        let bytes = &self.stack[lower_bound..self.sp()];
        let value = T::from_bytes(bytes);

        Ok(value)
    }
}

#[cfg(test)]
mod byte {
    use crate::{constant::Byte, error::ExecuteError, Processor};

    #[test]
    fn stack_underflow() {
        let mut p = Processor::test_instance();
        let expected = Err(ExecuteError::StackUnderflow);
        let actual = p.pop_value::<Byte>();

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_stack_underflow() {
        let mut p = Processor::test_instance();
        p.stack[0] = Byte::MAX;
        p.stack_pointer += 1;
        let expected = Byte::MAX;

        let actual = p.pop_value::<Byte>().unwrap();

        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod quarter {
    use crate::{constant::Quarter, error::ExecuteError, Processor};

    #[test]
    fn stack_underflow() {
        let mut p = Processor::test_instance();
        let expected = Err(ExecuteError::StackUnderflow);
        let actual = p.pop_value::<Quarter>();

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_stack_underflow() {
        let mut p = Processor::test_instance();
        let bytes = Quarter::MAX.to_le_bytes();
        p.stack[0] = bytes[0];
        p.stack[1] = bytes[1];
        p.stack_pointer += 2;
        let expected = Quarter::MAX;

        let actual = p.pop_value::<Quarter>().unwrap();

        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod half {
    use crate::{constant::Half, error::ExecuteError, Processor};

    #[test]
    fn stack_underflow() {
        let mut p = Processor::test_instance();
        let expected = Err(ExecuteError::StackUnderflow);
        let actual = p.pop_value::<Half>();

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_stack_underflow() {
        let mut p = Processor::test_instance();
        let bytes = Half::MAX.to_le_bytes();
        p.stack[0] = bytes[0];
        p.stack[1] = bytes[1];
        p.stack[2] = bytes[2];
        p.stack[3] = bytes[3];
        p.stack_pointer += 4;
        let expected = Half::MAX;

        let actual = p.pop_value::<Half>().unwrap();

        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod word {
    use crate::{constant::Word, error::ExecuteError, Processor};

    #[test]
    fn stack_underflow() {
        let mut p = Processor::test_instance();
        let expected = Err(ExecuteError::StackUnderflow);
        let actual = p.pop_value::<Word>();

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_stack_underflow() {
        let mut p = Processor::test_instance();
        let bytes = Word::MAX.to_le_bytes();
        p.stack[0] = bytes[0];
        p.stack[1] = bytes[1];
        p.stack[2] = bytes[2];
        p.stack[3] = bytes[3];
        p.stack[4] = bytes[4];
        p.stack[5] = bytes[5];
        p.stack[6] = bytes[6];
        p.stack[7] = bytes[7];
        p.stack_pointer += 8;
        let expected = Word::MAX;

        let actual = p.pop_value::<Word>().unwrap();

        assert_eq!(actual, expected);
    }
}
