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
        self.registers[register] = result.to_word();
    }
}

#[cfg(test)]
mod byte {
    use crate::{
        constant::{Byte, Word},
        error::ExecuteError,
        instruction::{Addition, Instruction},
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn add_causes_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Addition(Addition::Byte(Register::A, Operand::Immediate(1)));
        i.registers[Register::A] = Byte::MAX as Word;
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn add_does_not_cause_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Addition(Addition::Byte(Register::A, Operand::Immediate(1)));
        i.registers[Register::A] = Byte::MAX as Word - 1;
        let expected = Byte::MAX as Word;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
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
        i.registers[Register::A] = 2;
        i.registers[Register::B] = 2;
        let expected = 4;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
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
        i.registers[Register::A] = 2;
        let expected = 4;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }
}

#[cfg(test)]
mod quarter {
    use crate::{
        constant::{Quarter, Word},
        error::ExecuteError,
        instruction::{Addition, Instruction},
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn add_causes_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Addition(Addition::Quarter(Register::A, Operand::Immediate(1)));
        i.registers[Register::A] = Quarter::MAX as Word;
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
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
        i.registers[Register::A] = Quarter::MAX as Word - 1;
        let expected = Quarter::MAX as Word;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
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
        i.registers[Register::A] = 2;
        i.registers[Register::B] = 2;
        let expected = 4;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
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
        i.registers[Register::A] = 2;
        let expected = 4;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }
}

#[cfg(test)]
mod half {
    use crate::{
        constant::{Half, Word},
        error::ExecuteError,
        instruction::{Addition, Instruction},
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn add_causes_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Addition(Addition::Half(Register::A, Operand::Immediate(1)));
        i.registers[Register::A] = Half::MAX as Word;
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn add_does_not_cause_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Addition(Addition::Half(Register::A, Operand::Immediate(1)));
        i.registers[Register::A] = Half::MAX as Word - 1;
        let expected = Half::MAX as Word;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
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
        i.registers[Register::A] = 2;
        i.registers[Register::B] = 2;
        let expected = 4;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
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
        i.registers[Register::A] = 2;
        let expected = 4;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
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
        Interpreter,
    };

    #[test]
    fn add_causes_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Addition(Addition::Word(Register::A, Operand::Immediate(1)));
        i.registers[Register::A] = Word::MAX;
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn add_does_not_cause_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Addition(Addition::Word(Register::A, Operand::Immediate(1)));
        i.registers[Register::A] = Word::MAX - 1;
        let expected = Word::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
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
        i.registers[Register::A] = 2;
        i.registers[Register::B] = 2;
        let expected = 4;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
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
        i.registers[Register::A] = 2;
        let expected = 4;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }
}
