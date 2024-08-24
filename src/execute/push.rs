use crate::{
    constant::{Word, STACK_SIZE},
    error::ExecuteError,
    instruction::Push,
    operand::Operand,
    utils::{FromBytes, ToBytes},
    Processor,
};

impl Processor {
    pub fn push(&mut self, instruction: Push) -> Result<(), ExecuteError> {
        match instruction {
            Push::Byte(operand) => self.push_value(operand)?,
            Push::Quarter(operand) => self.push_value(operand)?,
            Push::Half(operand) => self.push_value(operand)?,
            Push::Word(operand) => self.push_value(operand)?,
        }

        Ok(())
    }

    fn push_value<T>(&mut self, operand: Operand<T>) -> Result<(), ExecuteError>
    where
        T: ToBytes + FromBytes,
    {
        if self.sp() + size_of::<T>() > STACK_SIZE {
            return Err(ExecuteError::StackOverflow);
        }

        let value = self.get_operand_value(operand);
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
        constant::{Byte, Word, STACK_SIZE},
        error::ExecuteError,
        operand::Operand,
        Processor,
    };

    #[test]
    fn stack_overflow() {
        let mut p = Processor::new().unwrap();
        p.stack_pointer = STACK_SIZE as Word;
        let expected = Err(ExecuteError::StackOverflow);

        let actual = p.push_value(Operand::Immediate(Byte::MAX));

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_stack_overflow() {
        let mut p = Processor::new().unwrap();
        p.stack_pointer = (STACK_SIZE - size_of::<Byte>()) as Word;
        let expected = Byte::MAX;

        p.push_value(Operand::Immediate(Byte::MAX)).unwrap();
        let actual = p.stack[STACK_SIZE - size_of::<Byte>()];

        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod quarter {
    use crate::{
        constant::{Quarter, Word, STACK_SIZE},
        error::ExecuteError,
        operand::Operand,
        Processor,
    };

    #[test]
    fn stack_overflow() {
        let mut p = Processor::new().unwrap();
        p.stack_pointer = STACK_SIZE as Word;
        let expected = Err(ExecuteError::StackOverflow);

        let actual = p.push_value(Operand::Immediate(Quarter::MAX));

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_stack_overflow() {
        let mut p = Processor::new().unwrap();
        p.stack_pointer = (STACK_SIZE - size_of::<Quarter>()) as Word;
        let expected = Quarter::MAX.to_le_bytes();

        p.push_value(Operand::Immediate(Quarter::MAX)).unwrap();
        let actual = &p.stack[STACK_SIZE - size_of::<Quarter>()..STACK_SIZE];

        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod half {
    use crate::{
        constant::{Half, Word, STACK_SIZE},
        error::ExecuteError,
        operand::Operand,
        Processor,
    };

    #[test]
    fn stack_overflow() {
        let mut p = Processor::new().unwrap();
        p.stack_pointer = STACK_SIZE as Word;
        let expected = Err(ExecuteError::StackOverflow);

        let actual = p.push_value(Operand::Immediate(Half::MAX));

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_stack_overflow() {
        let mut p = Processor::new().unwrap();
        p.stack_pointer = (STACK_SIZE - size_of::<Half>()) as Word;
        let expected = Half::MAX.to_le_bytes();

        p.push_value(Operand::Immediate(Half::MAX)).unwrap();
        let actual = &p.stack[STACK_SIZE - size_of::<Half>()..STACK_SIZE];

        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod word {
    use crate::{
        constant::{Word, STACK_SIZE},
        error::ExecuteError,
        operand::Operand,
        Processor,
    };

    #[test]
    fn stack_overflow() {
        let mut p = Processor::new().unwrap();
        p.stack_pointer = STACK_SIZE as Word;
        let expected = Err(ExecuteError::StackOverflow);

        let actual = p.push_value(Operand::Immediate(Word::MAX));

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_stack_overflow() {
        let mut p = Processor::new().unwrap();
        p.stack_pointer = (STACK_SIZE - size_of::<Word>()) as Word;
        let expected = Word::MAX.to_le_bytes();

        p.push_value(Operand::Immediate(Word::MAX)).unwrap();
        let actual = &p.stack[STACK_SIZE - size_of::<Word>()..STACK_SIZE];

        assert_eq!(actual, expected);
    }
}
