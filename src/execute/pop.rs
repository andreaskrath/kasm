use crate::{
    constant::{Byte, Half, Quarter, Word},
    error::ExecuteError,
    instruction::Pop,
    register::Register,
    utils::FromBytes,
    Processor,
};

impl Processor {
    pub fn pop(&mut self, instruction: Pop) -> Result<(), ExecuteError> {
        match instruction {
            Pop::Byte(register) => self.pop_value::<Byte>(register)?,
            Pop::Quarter(register) => self.pop_value::<Quarter>(register)?,
            Pop::Half(register) => self.pop_value::<Half>(register)?,
            Pop::Word(register) => self.pop_value::<Word>(register)?,
        }
        Ok(())
    }

    fn pop_value<T>(&mut self, register: Register) -> Result<(), ExecuteError>
    where
        T: FromBytes,
    {
        if self.sp().checked_sub(size_of::<T>()).is_none() {
            return Err(ExecuteError::StackUnderflow);
        }

        let bytes = &self.stack[self.sp() - size_of::<T>()..self.sp()];
        self.registers[register] = T::from_bytes(bytes).to_word();

        Ok(())
    }
}

#[cfg(test)]
mod byte {
    use crate::{
        constant::{Byte, Word},
        error::ExecuteError,
        register::Register,
        Processor,
    };

    #[test]
    fn stack_underflow() {
        let mut p = Processor::new().unwrap();
        let expected = Err(ExecuteError::StackUnderflow);
        let actual = p.pop_value::<Byte>(Register::A);

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_stack_underflow() {
        let mut p = Processor::new().unwrap();
        p.stack[0] = Byte::MAX;
        p.stack_pointer += 1;
        let expected = Byte::MAX as Word;

        p.pop_value::<Byte>(Register::A).unwrap();

        assert_eq!(p.registers[Register::A], expected);
    }
}

#[cfg(test)]
mod quarter {
    use crate::{
        constant::{Quarter, Word},
        error::ExecuteError,
        register::Register,
        Processor,
    };

    #[test]
    fn stack_underflow() {
        let mut p = Processor::new().unwrap();
        let expected = Err(ExecuteError::StackUnderflow);
        let actual = p.pop_value::<Quarter>(Register::A);

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_stack_underflow() {
        let mut p = Processor::new().unwrap();
        let bytes = Quarter::MAX.to_le_bytes();
        p.stack[0] = bytes[0];
        p.stack[1] = bytes[1];
        p.stack_pointer += 2;
        let expected = Quarter::MAX as Word;

        p.pop_value::<Quarter>(Register::A).unwrap();

        assert_eq!(p.registers[Register::A], expected);
    }
}

#[cfg(test)]
mod half {
    use crate::{
        constant::{Half, Word},
        error::ExecuteError,
        register::Register,
        Processor,
    };

    #[test]
    fn stack_underflow() {
        let mut p = Processor::new().unwrap();
        let expected = Err(ExecuteError::StackUnderflow);
        let actual = p.pop_value::<Half>(Register::A);

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_stack_underflow() {
        let mut p = Processor::new().unwrap();
        let bytes = Half::MAX.to_le_bytes();
        p.stack[0] = bytes[0];
        p.stack[1] = bytes[1];
        p.stack[2] = bytes[2];
        p.stack[3] = bytes[3];
        p.stack_pointer += 4;
        let expected = Half::MAX as Word;

        p.pop_value::<Half>(Register::A).unwrap();

        assert_eq!(p.registers[Register::A], expected);
    }
}

#[cfg(test)]
mod word {
    use crate::{constant::Word, error::ExecuteError, register::Register, Processor};

    #[test]
    fn stack_underflow() {
        let mut p = Processor::new().unwrap();
        let expected = Err(ExecuteError::StackUnderflow);
        let actual = p.pop_value::<Word>(Register::A);

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_stack_underflow() {
        let mut p = Processor::new().unwrap();
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

        p.pop_value::<Word>(Register::A).unwrap();

        assert_eq!(p.registers[Register::A], expected);
    }
}
