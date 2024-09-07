use crate::{
    instruction::Subtraction, operand::Operand, register::Register, registers::RegisterOperations,
    utils::Arithmetic, Interpreter,
};

impl Interpreter {
    pub fn sub(&mut self, instruction: Subtraction) {
        match instruction {
            Subtraction::Byte(r, o) => self.sub_value(r, o),
            Subtraction::Quarter(r, o) => self.sub_value(r, o),
            Subtraction::Half(r, o) => self.sub_value(r, o),
            Subtraction::Word(r, o) => self.sub_value(r, o),
        }
    }

    fn sub_value<T>(&mut self, register: Register, operand: Operand<T>)
    where
        T: Arithmetic,
    {
        let a = self.registers.get::<T>(register);
        let b = self.get_operand_value(operand);

        let (result, overflow) = a.overflow_sub(b);
        self.flags.set(result, overflow);
        self.registers.set(register, result);
    }
}

#[cfg(test)]
mod byte {
    use crate::{
        constant::Byte,
        error::ExecuteError,
        instruction::{Instruction, Subtraction},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn sub_causes_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Subtraction(Subtraction::Byte(Register::A, Operand::Immediate(1)));
        i.registers.set(Register::A, Byte::MIN);
        let expected = Byte::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Byte>(Register::A), expected);
        assert!(i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn sub_does_not_cause_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Subtraction(Subtraction::Byte(Register::A, Operand::Immediate(1)));
        i.registers.set(Register::A, Byte::MAX);
        let expected = Byte::MAX - 1;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Byte>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn sub_two_registers_together() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Subtraction(Subtraction::Byte(
            Register::A,
            Operand::Register(Register::B),
        ));
        i.registers.set(Register::A, 2);
        i.registers.set(Register::B, 2);
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Byte>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn sub_the_register_to_itself() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Subtraction(Subtraction::Byte(
            Register::A,
            Operand::Register(Register::A),
        ));
        i.registers.set(Register::A, 2);
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Byte>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }
}

#[cfg(test)]
mod quarter {
    use crate::{
        constant::Quarter,
        error::ExecuteError,
        instruction::{Instruction, Subtraction},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn sub_causes_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Subtraction(Subtraction::Quarter(Register::A, Operand::Immediate(1)));
        i.registers.set(Register::A, Quarter::MIN);
        let expected = Quarter::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Quarter>(Register::A), expected);
        assert!(i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn sub_does_not_cause_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Subtraction(Subtraction::Quarter(Register::A, Operand::Immediate(1)));
        i.registers.set(Register::A, Quarter::MAX);
        let expected = Quarter::MAX - 1;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Quarter>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn sub_two_registers_together() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Subtraction(Subtraction::Quarter(
            Register::A,
            Operand::Register(Register::B),
        ));
        i.registers.set(Register::A, 2);
        i.registers.set(Register::B, 2);
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Quarter>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn sub_the_register_to_itself() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Subtraction(Subtraction::Quarter(
            Register::A,
            Operand::Register(Register::A),
        ));
        i.registers.set(Register::A, 2);
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Quarter>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }
}

#[cfg(test)]
mod half {
    use crate::{
        constant::Half,
        error::ExecuteError,
        instruction::{Instruction, Subtraction},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn sub_causes_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Subtraction(Subtraction::Half(Register::A, Operand::Immediate(1)));
        i.registers.set(Register::A, Half::MIN);
        let expected = Half::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Half>(Register::A), expected);
        assert!(i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn sub_does_not_cause_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Subtraction(Subtraction::Half(Register::A, Operand::Immediate(1)));
        i.registers.set(Register::A, Half::MAX);
        let expected = Half::MAX - 1;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Half>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn sub_two_registers_together() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Subtraction(Subtraction::Half(
            Register::A,
            Operand::Register(Register::B),
        ));
        i.registers.set(Register::A, 2);
        i.registers.set(Register::B, 2);
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Half>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn sub_the_register_to_itself() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Subtraction(Subtraction::Half(
            Register::A,
            Operand::Register(Register::A),
        ));
        i.registers.set(Register::A, 2);
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Half>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }
}

#[cfg(test)]
mod word {
    use crate::{
        constant::Word,
        error::ExecuteError,
        instruction::{Instruction, Subtraction},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn sub_causes_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Subtraction(Subtraction::Word(Register::A, Operand::Immediate(1)));
        i.registers.set(Register::A, Word::MIN);
        let expected = Word::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Word>(Register::A), expected);
        assert!(i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn sub_does_not_cause_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Subtraction(Subtraction::Word(Register::A, Operand::Immediate(1)));
        i.registers.set(Register::A, Word::MAX);
        let expected = Word::MAX - 1;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Word>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn sub_two_registers_together() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Subtraction(Subtraction::Word(
            Register::A,
            Operand::Register(Register::B),
        ));
        i.registers.set(Register::A, 2);
        i.registers.set(Register::B, 2);
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Word>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn sub_the_register_to_itself() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Subtraction(Subtraction::Word(
            Register::A,
            Operand::Register(Register::A),
        ));
        i.registers.set(Register::A, 2);
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Word>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }
}
