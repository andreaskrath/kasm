use crate::{constant::Word, error::ExecuteError, instruction::Push, utils::ToBytes, Interpreter};

impl Interpreter {
    pub fn push(&mut self, instruction: Push) -> Result<(), ExecuteError> {
        match instruction {
            Push::Byte(operand) => {
                let value = self.get_operand_value(operand);
                self.push_value(value)?;
            }
            Push::Quarter(operand) => {
                let value = self.get_operand_value(operand);
                self.push_value(value)?;
            }
            Push::Half(operand) => {
                let value = self.get_operand_value(operand);
                self.push_value(value)?;
            }
            Push::Word(operand) => {
                let value = self.get_operand_value(operand);
                self.push_value(value)?;
            }
        }

        Ok(())
    }

    pub fn push_value<T>(&mut self, value: T) -> Result<(), ExecuteError>
    where
        T: ToBytes,
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
        instruction::{Instruction, Push},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn stack_overflow() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Push(Push::Byte(Operand::Immediate(Byte::MAX)));
        i.stack_pointer = TEST_STACK_SIZE as Word;
        let expected = Err(ExecuteError::StackOverflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn push_from_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Push(Push::Byte(Operand::Register(Register::A)));
        i.registers.set(Register::A, Byte::MAX);
        let expected = Byte::MAX;

        i.execute(instruction)?;
        let actual = i.stack[0];

        assert_eq!(actual, expected);
        assert_eq!(i.stack_pointer, size_of::<Byte>() as Word);

        Ok(())
    }

    #[test]
    fn push_from_immediate_value() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Push(Push::Byte(Operand::Immediate(Byte::MAX)));
        i.registers.set(Register::A, Byte::MAX);
        let expected = Byte::MAX;

        i.execute(instruction)?;
        let actual = i.stack[0];

        assert_eq!(actual, expected);
        assert_eq!(i.stack_pointer, size_of::<Byte>() as Word);

        Ok(())
    }
}

#[cfg(test)]
mod quarter {
    use crate::{
        constant::{Quarter, Word, TEST_STACK_SIZE},
        error::ExecuteError,
        instruction::{Instruction, Push},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn stack_overflow() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Push(Push::Quarter(Operand::Immediate(Quarter::MAX)));
        i.stack_pointer = TEST_STACK_SIZE as Word;
        let expected = Err(ExecuteError::StackOverflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn push_from_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Push(Push::Quarter(Operand::Register(Register::A)));
        i.registers.set(Register::A, Quarter::MAX);
        let expected = Quarter::MAX.to_le_bytes();

        i.execute(instruction)?;
        let actual = &i.stack[0..size_of::<Quarter>()];

        assert_eq!(actual, expected);
        assert_eq!(i.stack_pointer, size_of::<Quarter>() as Word);

        Ok(())
    }

    #[test]
    fn push_from_immediate_value() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Push(Push::Quarter(Operand::Immediate(Quarter::MAX)));
        i.registers.set(Register::A, Quarter::MAX);
        let expected = Quarter::MAX.to_le_bytes();

        i.execute(instruction)?;
        let actual = &i.stack[0..size_of::<Quarter>()];

        assert_eq!(actual, expected);
        assert_eq!(i.stack_pointer, size_of::<Quarter>() as Word);

        Ok(())
    }
}

#[cfg(test)]
mod half {
    use crate::{
        constant::{Half, Word, TEST_STACK_SIZE},
        error::ExecuteError,
        instruction::{Instruction, Push},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn stack_overflow() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Push(Push::Half(Operand::Immediate(Half::MAX)));
        i.stack_pointer = TEST_STACK_SIZE as Word;
        let expected = Err(ExecuteError::StackOverflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn push_from_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Push(Push::Half(Operand::Register(Register::A)));
        i.registers.set(Register::A, Half::MAX);
        let expected = Half::MAX.to_le_bytes();

        i.execute(instruction)?;
        let actual = &i.stack[0..size_of::<Half>()];

        assert_eq!(actual, expected);
        assert_eq!(i.stack_pointer, size_of::<Half>() as Word);

        Ok(())
    }

    #[test]
    fn push_from_immediate_value() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Push(Push::Half(Operand::Immediate(Half::MAX)));
        i.registers.set(Register::A, Half::MAX);
        let expected = Half::MAX.to_le_bytes();

        i.execute(instruction)?;
        let actual = &i.stack[0..size_of::<Half>()];

        assert_eq!(actual, expected);
        assert_eq!(i.stack_pointer, size_of::<Half>() as Word);

        Ok(())
    }
}

#[cfg(test)]
mod word {
    use crate::{
        constant::{Word, TEST_STACK_SIZE},
        error::ExecuteError,
        instruction::{Instruction, Push},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn stack_overflow() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Push(Push::Word(Operand::Immediate(Word::MAX)));
        i.stack_pointer = TEST_STACK_SIZE as Word;
        let expected = Err(ExecuteError::StackOverflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn push_from_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Push(Push::Word(Operand::Register(Register::A)));
        i.registers.set(Register::A, Word::MAX);
        let expected = Word::MAX.to_le_bytes();

        i.execute(instruction)?;
        let actual = &i.stack[0..size_of::<Word>()];

        assert_eq!(actual, expected);
        assert_eq!(i.stack_pointer, size_of::<Word>() as Word);

        Ok(())
    }

    #[test]
    fn push_from_immediate_value() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Push(Push::Word(Operand::Immediate(Word::MAX)));
        i.registers.set(Register::A, Word::MAX);
        let expected = Word::MAX.to_le_bytes();

        i.execute(instruction)?;
        let actual = &i.stack[0..size_of::<Word>()];

        assert_eq!(actual, expected);
        assert_eq!(i.stack_pointer, size_of::<Word>() as Word);

        Ok(())
    }
}
