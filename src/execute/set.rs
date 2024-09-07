use crate::{
    instruction::Set,
    operand::Operand,
    register::Register,
    registers::RegisterOperations,
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
        self.registers.set(register, value);
    }
}

#[cfg(test)]
mod byte {
    use crate::{
        constant::Byte,
        error::ExecuteError,
        instruction::{Instruction, Set},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn set_from_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Set(Set::Byte(Register::A, Operand::Immediate(Byte::MAX)));
        let expected = Byte::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Byte>(Register::A), expected);

        Ok(())
    }

    #[test]
    fn set_from_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Set(Set::Byte(Register::A, Operand::Register(Register::B)));
        i.registers.set(Register::B, Byte::MAX);
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
        instruction::{Instruction, Set},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn set_from_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Set(Set::Quarter(Register::A, Operand::Immediate(Quarter::MAX)));
        let expected = Quarter::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Quarter>(Register::A), expected);

        Ok(())
    }

    #[test]
    fn set_from_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Set(Set::Quarter(Register::A, Operand::Register(Register::B)));
        i.registers.set(Register::B, Quarter::MAX);
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
        instruction::{Instruction, Set},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn set_from_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Set(Set::Half(Register::A, Operand::Immediate(Half::MAX)));
        let expected = Half::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Half>(Register::A), expected);

        Ok(())
    }

    #[test]
    fn set_from_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Set(Set::Half(Register::A, Operand::Register(Register::B)));
        i.registers.set(Register::B, Half::MAX);
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
        instruction::{Instruction, Set},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn set_from_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Set(Set::Word(Register::A, Operand::Immediate(Word::MAX)));
        let expected = Word::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Word>(Register::A), expected);

        Ok(())
    }

    #[test]
    fn set_from_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Set(Set::Word(Register::A, Operand::Register(Register::B)));
        i.registers.set(Register::B, Word::MAX);
        let expected = Word::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Word>(Register::A), expected);

        Ok(())
    }
}
