use crate::{
    constant::{Byte, Half, Quarter, Word},
    error::ExecuteError,
    instruction::Pop,
    registers::RegisterOperations,
    utils::FromBytes,
    Interpreter,
};

impl Interpreter {
    pub fn pop(&mut self, instruction: Pop) -> Result<(), ExecuteError> {
        match instruction {
            Pop::Byte(register) => {
                let value = self.pop_value::<Byte>()?;
                self.registers.set(register, value);
            }
            Pop::Quarter(register) => {
                let value = self.pop_value::<Quarter>()?;
                self.registers.set(register, value);
            }
            Pop::Half(register) => {
                let value = self.pop_value::<Half>()?;
                self.registers.set(register, value);
            }
            Pop::Word(register) => {
                let value = self.pop_value::<Word>()?;
                self.registers.set(register, value);
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

        self.stack_pointer -= size_of::<T>() as Word;

        Ok(value)
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
        i.stack[0] = Byte::MAX;
        i.stack_pointer = 1;
        let expected = Byte::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Byte>(Register::A), expected);
        assert!(i.stack_pointer == 0);

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
        let bytes = Quarter::MAX.to_le_bytes();
        i.stack[0] = bytes[0];
        i.stack[1] = bytes[1];
        i.stack_pointer = 2;
        let expected = Quarter::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Quarter>(Register::A), expected);
        assert!(i.stack_pointer == 0);

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
        let bytes = Half::MAX.to_le_bytes();
        i.stack[0] = bytes[0];
        i.stack[1] = bytes[1];
        i.stack[2] = bytes[2];
        i.stack[3] = bytes[3];
        i.stack_pointer = 4;
        let expected = Half::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Half>(Register::A), expected);
        assert!(i.stack_pointer == 0);

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
        let bytes = Word::MAX.to_le_bytes();
        i.stack[0] = bytes[0];
        i.stack[1] = bytes[1];
        i.stack[2] = bytes[2];
        i.stack[3] = bytes[3];
        i.stack[4] = bytes[4];
        i.stack[5] = bytes[5];
        i.stack[6] = bytes[6];
        i.stack[7] = bytes[7];
        i.stack_pointer = 8;
        let expected = Word::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Word>(Register::A), expected);
        assert!(i.stack_pointer == 0);

        Ok(())
    }
}
