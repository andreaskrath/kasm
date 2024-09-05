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
        self.registers[register] = result.to_word();
    }
}

#[cfg(test)]
mod byte {
    use crate::{
        constant::{Byte, Word},
        error::ExecuteError,
        instruction::{Instruction, Subtraction},
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn sub_causes_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Subtraction(Subtraction::Byte(Register::A, Operand::Immediate(1)));
        i.registers[Register::A] = Byte::MIN as Word;
        let expected = Byte::MAX as Word;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
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
        i.registers[Register::A] = Byte::MAX as Word;
        let expected = Byte::MAX as Word - 1;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
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
        i.registers[Register::A] = 2;
        i.registers[Register::B] = 2;
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
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
        i.registers[Register::A] = 2;
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(!i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }
}

#[cfg(test)]
mod quarter {
    use crate::{
        constant::{Quarter, Word},
        error::ExecuteError,
        instruction::{Instruction, Subtraction},
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn sub_causes_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Subtraction(Subtraction::Quarter(Register::A, Operand::Immediate(1)));
        i.registers[Register::A] = Quarter::MIN as Word;
        let expected = Quarter::MAX as Word;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
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
        i.registers[Register::A] = Quarter::MAX as Word;
        let expected = Quarter::MAX as Word - 1;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
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
        i.registers[Register::A] = 2;
        i.registers[Register::B] = 2;
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
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
        i.registers[Register::A] = 2;
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(!i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }
}

#[cfg(test)]
mod half {
    use crate::{
        constant::{Half, Word},
        error::ExecuteError,
        instruction::{Instruction, Subtraction},
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn sub_causes_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Subtraction(Subtraction::Half(Register::A, Operand::Immediate(1)));
        i.registers[Register::A] = Half::MIN as Word;
        let expected = Half::MAX as Word;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
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
        i.registers[Register::A] = Half::MAX as Word;
        let expected = Half::MAX as Word - 1;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
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
        i.registers[Register::A] = 2;
        i.registers[Register::B] = 2;
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
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
        i.registers[Register::A] = 2;
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
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
        Interpreter,
    };

    #[test]
    fn sub_causes_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Subtraction(Subtraction::Word(Register::A, Operand::Immediate(1)));
        i.registers[Register::A] = Word::MIN;
        let expected = Word::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
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
        i.registers[Register::A] = Word::MAX;
        let expected = Word::MAX - 1;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
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
        i.registers[Register::A] = 2;
        i.registers[Register::B] = 2;
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
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
        i.registers[Register::A] = 2;
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(!i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }
}
