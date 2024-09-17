use crate::{
    constant::{Byte, Half, Quarter, Word},
    error::ExecuteError,
    instruction::Pop,
    register::Register,
    registers::RegisterOperations,
    utils::{FromBytes, ToWord},
    Interpreter,
};

impl Interpreter {
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
        T: FromBytes + ToWord,
    {
        let value = self.stack.pop::<T>()?;
        self.registers.set(register, value);

        Ok(())
    }
}

#[cfg(test)]
mod byte {
    use crate::{
        constant::Byte,
        error::ExecuteError,
        instruction::{Instruction, Pop},
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn stack_underflow() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Pop(Pop::Byte(Register::A));
        let expected = Err(ExecuteError::StackUnderflow);
        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_stack_underflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Pop(Pop::Byte(Register::A));
        i.stack
            .push(Byte::MAX)
            .expect("should be able to push byte onto stack");
        let expected = Byte::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Byte>(Register::A), expected);

        Ok(())
    }
}

#[cfg(test)]
mod quarter {
    use crate::{
        constant::Quarter,
        error::ExecuteError,
        instruction::{Instruction, Pop},
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn stack_underflow() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Pop(Pop::Quarter(Register::A));
        let expected = Err(ExecuteError::StackUnderflow);
        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_stack_underflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Pop(Pop::Quarter(Register::A));
        i.stack
            .push(Quarter::MAX)
            .expect("should be able to push quarter onto stack");
        let expected = Quarter::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Quarter>(Register::A), expected);

        Ok(())
    }
}

#[cfg(test)]
mod half {
    use crate::{
        constant::Half,
        error::ExecuteError,
        instruction::{Instruction, Pop},
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn stack_underflow() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Pop(Pop::Half(Register::A));
        let expected = Err(ExecuteError::StackUnderflow);
        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_stack_underflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Pop(Pop::Half(Register::A));
        i.stack
            .push(Half::MAX)
            .expect("should be able to push half onto stack");
        let expected = Half::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Half>(Register::A), expected);

        Ok(())
    }
}

#[cfg(test)]
mod word {
    use crate::{
        constant::Word,
        error::ExecuteError,
        instruction::{Instruction, Pop},
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn stack_underflow() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Pop(Pop::Word(Register::A));
        let expected = Err(ExecuteError::StackUnderflow);
        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_stack_underflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Pop(Pop::Word(Register::A));
        i.stack
            .push(Word::MAX)
            .expect("should be able to push word onto stack");
        let expected = Word::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Word>(Register::A), expected);

        Ok(())
    }
}
