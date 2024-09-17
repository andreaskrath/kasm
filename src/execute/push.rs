use crate::{
    error::ExecuteError,
    instruction::Push,
    operand::Operand,
    utils::{FromBytes, ToBytes},
    Interpreter,
};

impl Interpreter {
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
        let value = self.get_operand_value(operand);
        self.stack.push(value)?;

        Ok(())
    }
}

#[cfg(test)]
mod byte {
    use crate::{
        constant::{Byte, TEST_STACK_SIZE},
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
        // filling the stack with values
        for _ in 0..TEST_STACK_SIZE {
            i.stack
                .push::<Byte>(0)
                .expect("should be able to fill stack");
        }
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
        let actual = i.stack.pop::<Byte>()?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn push_from_immediate_value() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Push(Push::Byte(Operand::Immediate(Byte::MAX)));
        i.registers.set(Register::A, Byte::MAX);
        let expected = Byte::MAX;

        i.execute(instruction)?;
        let actual = i.stack.pop::<Byte>()?;

        assert_eq!(actual, expected);

        Ok(())
    }
}

#[cfg(test)]
mod quarter {
    use crate::{
        constant::{Byte, Quarter, TEST_STACK_SIZE},
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
        // filling the stack with values
        for _ in 0..TEST_STACK_SIZE {
            i.stack
                .push::<Byte>(0)
                .expect("should be able to fill stack");
        }
        let expected = Err(ExecuteError::StackOverflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn push_from_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Push(Push::Quarter(Operand::Register(Register::A)));
        i.registers.set(Register::A, Quarter::MAX);
        let expected = Quarter::MAX;

        i.execute(instruction)?;
        let actual = i.stack.pop::<Quarter>()?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn push_from_immediate_value() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Push(Push::Quarter(Operand::Immediate(Quarter::MAX)));
        i.registers.set(Register::A, Quarter::MAX);
        let expected = Quarter::MAX;

        i.execute(instruction)?;
        let actual = i.stack.pop::<Quarter>()?;

        assert_eq!(actual, expected);

        Ok(())
    }
}

#[cfg(test)]
mod half {
    use crate::{
        constant::{Byte, Half, TEST_STACK_SIZE},
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
        // filling the stack with values
        for _ in 0..TEST_STACK_SIZE {
            i.stack
                .push::<Byte>(0)
                .expect("should be able to fill stack");
        }
        let expected = Err(ExecuteError::StackOverflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn push_from_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Push(Push::Half(Operand::Register(Register::A)));
        i.registers.set(Register::A, Half::MAX);
        let expected = Half::MAX;

        i.execute(instruction)?;
        let actual = i.stack.pop::<Half>()?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn push_from_immediate_value() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Push(Push::Half(Operand::Immediate(Half::MAX)));
        i.registers.set(Register::A, Half::MAX);
        let expected = Half::MAX;

        i.execute(instruction)?;
        let actual = i.stack.pop::<Half>()?;

        assert_eq!(actual, expected);

        Ok(())
    }
}

#[cfg(test)]
mod word {
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
        let instruction = Instruction::Push(Push::Word(Operand::Immediate(Word::MAX)));
        // filling the stack with values
        for _ in 0..TEST_STACK_SIZE {
            i.stack
                .push::<Byte>(0)
                .expect("should be able to fill stack");
        }
        let expected = Err(ExecuteError::StackOverflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn push_from_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Push(Push::Word(Operand::Register(Register::A)));
        i.registers.set(Register::A, Word::MAX);
        let expected = Word::MAX;

        i.execute(instruction)?;
        let actual = i.stack.pop::<Word>()?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn push_from_immediate_value() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Push(Push::Word(Operand::Immediate(Word::MAX)));
        i.registers.set(Register::A, Word::MAX);
        let expected = Word::MAX;

        i.execute(instruction)?;
        let actual = i.stack.pop::<Word>()?;

        assert_eq!(actual, expected);

        Ok(())
    }
}
