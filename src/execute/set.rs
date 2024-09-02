use crate::{
    instruction::Set,
    operand::Operand,
    register::Register,
    utils::{FromBytes, ToWord},
    Interpreter,
};

impl Interpreter {
    pub fn set(&mut self, instruction: Set) {
        match instruction {
            Set::Byte(r, o) => self.set_value(r, o),
            Set::Quarter(r, o) => self.set_value(r, o),
            Set::Half(r, o) => self.set_value(r, o),
            Set::Word(r, o) => self.set_value(r, o),
        }
    }

    fn set_value<T>(&mut self, register: Register, operand: Operand<T>)
    where
        T: FromBytes + ToWord,
    {
        let value = self.get_operand_value(operand);
        self.registers[register] = value.to_word();
    }
}

#[cfg(test)]
mod byte {
    use crate::{
        constant::{Byte, Word},
        error::ExecuteError,
        instruction::{Instruction, Set},
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn set_from_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Set(Set::Byte(Register::A, Operand::Immediate(Byte::MAX)));
        let expected = Byte::MAX as Word;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);

        Ok(())
    }

    #[test]
    fn set_from_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Set(Set::Byte(Register::A, Operand::Register(Register::B)));
        i.registers[Register::B] = Byte::MAX as Word;
        let expected = Byte::MAX as Word;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);

        Ok(())
    }
}

#[cfg(test)]
mod quarter {
    use crate::{
        constant::{Quarter, Word},
        error::ExecuteError,
        instruction::{Instruction, Set},
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn set_from_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Set(Set::Quarter(Register::A, Operand::Immediate(Quarter::MAX)));
        let expected = Quarter::MAX as Word;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);

        Ok(())
    }

    #[test]
    fn set_from_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Set(Set::Quarter(Register::A, Operand::Register(Register::B)));
        i.registers[Register::B] = Quarter::MAX as Word;
        let expected = Quarter::MAX as Word;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);

        Ok(())
    }
}

#[cfg(test)]
mod half {
    use crate::{
        constant::{Half, Word},
        error::ExecuteError,
        instruction::{Instruction, Set},
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn set_from_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Set(Set::Half(Register::A, Operand::Immediate(Half::MAX)));
        let expected = Half::MAX as Word;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);

        Ok(())
    }

    #[test]
    fn set_from_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Set(Set::Half(Register::A, Operand::Register(Register::B)));
        i.registers[Register::B] = Half::MAX as Word;
        let expected = Half::MAX as Word;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);

        Ok(())
    }
}

#[cfg(test)]
mod word {
    use crate::{
        constant::Word,
        error::ExecuteError,
        instruction::{Instruction, Set},
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn set_from_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Set(Set::Word(Register::A, Operand::Immediate(Word::MAX)));
        let expected = Word::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);

        Ok(())
    }

    #[test]
    fn set_from_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Set(Set::Word(Register::A, Operand::Register(Register::B)));
        i.registers[Register::B] = Word::MAX;
        let expected = Word::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);

        Ok(())
    }
}
