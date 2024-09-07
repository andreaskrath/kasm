use crate::{
    instruction::Addition, operand::Operand, register::Register, registers::RegisterOperations,
    utils::Arithmetic, Interpreter,
};

impl Interpreter {
    pub fn add(&mut self, instruction: Addition) {
        match instruction {
            Addition::Byte(r, o) => self.add_value(r, o),
            Addition::Quarter(r, o) => self.add_value(r, o),
            Addition::Half(r, o) => self.add_value(r, o),
            Addition::Word(r, o) => self.add_value(r, o),
        }
    }

    fn add_value<T>(&mut self, register: Register, operand: Operand<T>)
    where
        T: Arithmetic,
    {
        let a = self.registers.get::<T>(register);
        let b = self.get_operand_value(operand);

        let (result, overflow) = a.overflow_add(b);
        self.flags.set(result, overflow);
        self.registers.set(register, result)
    }
}

#[cfg(test)]
mod byte {
    use crate::{
        constant::Byte,
        error::ExecuteError,
        instruction::{Addition, Instruction},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn add_causes_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Addition(Addition::Byte(Register::A, Operand::Immediate(1)));
        i.registers.set(Register::A, Byte::MAX);
        let expected = Byte::MIN;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Byte>(Register::A), expected);
        assert!(i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn add_does_not_cause_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Addition(Addition::Byte(Register::A, Operand::Immediate(1)));
        i.registers.set(Register::A, Byte::MAX - 1);
        let expected = Byte::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Byte>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn add_two_registers_together() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Addition(Addition::Byte(Register::A, Operand::Register(Register::B)));
        i.registers.set(Register::A, 2);
        i.registers.set(Register::B, 2);
        let expected = 4;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Byte>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn add_the_register_to_itself() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Addition(Addition::Byte(Register::A, Operand::Register(Register::A)));
        i.registers.set(Register::A, 2);
        let expected = 4;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Byte>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }
}

#[cfg(test)]
mod quarter {
    use crate::{
        constant::Quarter,
        error::ExecuteError,
        instruction::{Addition, Instruction},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn add_causes_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Addition(Addition::Quarter(Register::A, Operand::Immediate(1)));
        i.registers.set(Register::A, Quarter::MAX);
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Quarter>(Register::A), expected);
        assert!(i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn add_does_not_cause_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Addition(Addition::Quarter(Register::A, Operand::Immediate(1)));
        i.registers.set(Register::A, Quarter::MAX - 1);
        let expected = Quarter::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Quarter>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn add_two_registers_together() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Addition(Addition::Quarter(
            Register::A,
            Operand::Register(Register::B),
        ));
        i.registers.set(Register::A, 2);
        i.registers.set(Register::B, 2);
        let expected = 4;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Quarter>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn add_the_register_to_itself() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Addition(Addition::Quarter(
            Register::A,
            Operand::Register(Register::A),
        ));
        i.registers.set(Register::A, 2);
        let expected = 4;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Quarter>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }
}

#[cfg(test)]
mod half {
    use crate::{
        constant::Half,
        error::ExecuteError,
        instruction::{Addition, Instruction},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn add_causes_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Addition(Addition::Half(Register::A, Operand::Immediate(1)));
        i.registers.set(Register::A, Half::MAX);
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Half>(Register::A), expected);
        assert!(i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn add_does_not_cause_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Addition(Addition::Half(Register::A, Operand::Immediate(1)));
        i.registers.set(Register::A, Half::MAX - 1);
        let expected = Half::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Half>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn add_two_registers_together() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Addition(Addition::Half(Register::A, Operand::Register(Register::B)));
        i.registers.set(Register::A, 2);
        i.registers.set(Register::B, 2);
        let expected = 4;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Half>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn add_the_register_to_itself() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Addition(Addition::Half(Register::A, Operand::Register(Register::A)));
        i.registers.set(Register::A, 2);
        let expected = 4;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Half>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }
}

#[cfg(test)]
mod word {
    use crate::{
        constant::Word,
        error::ExecuteError,
        instruction::{Addition, Instruction},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn add_causes_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Addition(Addition::Word(Register::A, Operand::Immediate(1)));
        i.registers.set(Register::A, Word::MAX);
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Word>(Register::A), expected);
        assert!(i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn add_does_not_cause_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Addition(Addition::Word(Register::A, Operand::Immediate(1)));
        i.registers.set(Register::A, Word::MAX - 1);
        let expected = Word::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Word>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn add_two_registers_together() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Addition(Addition::Word(Register::A, Operand::Register(Register::B)));
        i.registers.set(Register::A, 2);
        i.registers.set(Register::B, 2);
        let expected = 4;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Word>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn add_the_register_to_itself() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Addition(Addition::Word(Register::A, Operand::Register(Register::A)));
        i.registers.set(Register::A, 2);
        let expected = 4;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Word>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }
}
