use crate::{
    constant::{Word, STACK_SIZE},
    error::ExecuteError,
    instruction::Push,
    utils::ToLeBytes,
    Processor,
};

impl Processor {
    pub fn push(&mut self, instruction: Push) -> Result<(), ExecuteError> {
        match instruction {
            Push::Byte(operand) => {
                let value = self.get_byte_operand_val(operand);
                self.push_value(value)?
            }
            Push::Quarter(operand) => {
                let value = self.get_quarter_operand_val(operand);
                self.push_value(value)?
            }
            Push::Half(operand) => {
                let value = self.get_half_operand_val(operand);
                self.push_value(value)?
            }
            Push::Word(operand) => {
                let value = self.get_word_operand_val(operand);
                self.push_value(value)?
            }
        }

        Ok(())
    }

    fn push_value<T: ToLeBytes>(&mut self, value: T) -> Result<(), ExecuteError> {
        if self.sp() + size_of::<T>() > STACK_SIZE {
            return Err(ExecuteError::StackOverflow);
        }

        for (index, byte) in value.to_le_bytes().iter().enumerate() {
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
        instruction::Push,
        operand::Operand,
        Processor,
    };

    #[test]
    fn stack_overflow() {
        let mut p = Processor::new().unwrap();
        p.stack_pointer = STACK_SIZE as Word;
        let instruction = Push::Byte(Operand::Immediate(10));
        let expected = Err(ExecuteError::StackOverflow);

        let actual = p.push(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_stack_overflow() {
        let mut p = Processor::new().unwrap();
        p.stack_pointer = (STACK_SIZE - size_of::<Byte>()) as Word;
        let instruction = Push::Byte(Operand::Immediate(10));
        let expected = 10;

        p.push(instruction).unwrap();
        let actual = p.stack[STACK_SIZE - size_of::<Byte>()];

        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod quarter {
    use crate::{
        constant::{Quarter, Word, STACK_SIZE},
        error::ExecuteError,
        instruction::Push,
        operand::Operand,
        Processor,
    };

    #[test]
    fn stack_overflow() {
        let mut p = Processor::new().unwrap();
        p.stack_pointer = STACK_SIZE as Word;
        let instruction = Push::Quarter(Operand::Immediate(10));
        let expected = Err(ExecuteError::StackOverflow);

        let actual = p.push(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_stack_overflow() {
        let mut p = Processor::new().unwrap();
        p.stack_pointer = (STACK_SIZE - size_of::<Quarter>()) as Word;
        let instruction = Push::Byte(Operand::Immediate(10));
        let expected = 10;

        p.push(instruction).unwrap();
        let actual = p.stack[STACK_SIZE - size_of::<Quarter>()];

        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod half {
    use crate::{
        constant::{Half, Word, STACK_SIZE},
        error::ExecuteError,
        instruction::Push,
        operand::Operand,
        Processor,
    };

    #[test]
    fn stack_overflow() {
        let mut p = Processor::new().unwrap();
        p.stack_pointer = STACK_SIZE as Word;
        let instruction = Push::Half(Operand::Immediate(10));
        let expected = Err(ExecuteError::StackOverflow);

        let actual = p.push(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_stack_overflow() {
        let mut p = Processor::new().unwrap();
        p.stack_pointer = (STACK_SIZE - size_of::<Half>()) as Word;
        let instruction = Push::Byte(Operand::Immediate(10));
        let expected = 10;

        p.push(instruction).unwrap();
        let actual = p.stack[STACK_SIZE - size_of::<Half>()];

        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod word {
    use crate::{
        constant::{Word, STACK_SIZE},
        error::ExecuteError,
        instruction::Push,
        operand::Operand,
        Processor,
    };

    #[test]
    fn stack_overflow() {
        let mut p = Processor::new().unwrap();
        p.stack_pointer = STACK_SIZE as Word;
        let instruction = Push::Quarter(Operand::Immediate(10));
        let expected = Err(ExecuteError::StackOverflow);

        let actual = p.push(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_stack_overflow() {
        let mut p = Processor::new().unwrap();
        p.stack_pointer = (STACK_SIZE - size_of::<Word>()) as Word;
        let instruction = Push::Byte(Operand::Immediate(10));
        let expected = 10;

        p.push(instruction).unwrap();
        let actual = p.stack[STACK_SIZE - size_of::<Word>()];

        assert_eq!(actual, expected);
    }
}
