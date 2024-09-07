use crate::{
    instruction::Multiplication, operand::Operand, register::Register,
    registers::RegisterOperations, utils::Arithmetic, Interpreter,
};

impl Interpreter {
    pub fn mul(&mut self, instruction: Multiplication) {
        match instruction {
            Multiplication::Byte(r, o) => self.mul_value(r, o),
            Multiplication::Quarter(r, o) => self.mul_value(r, o),
            Multiplication::Half(r, o) => self.mul_value(r, o),
            Multiplication::Word(r, o) => self.mul_value(r, o),
        }
    }

    fn mul_value<T>(&mut self, register: Register, operand: Operand<T>)
    where
        T: Arithmetic,
    {
        let a = self.registers.get::<T>(register);
        let b = self.get_operand_value(operand);

        let (result, overflow) = a.overflow_mul(b);
        self.flags.set(result, overflow);
        self.registers.set(register, result);
    }
}

#[cfg(test)]
mod byte {
    use crate::{
        constant::Byte,
        error::ExecuteError,
        instruction::{Instruction, Multiplication},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn mul_causes_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Multiplication(Multiplication::Byte(Register::A, Operand::Immediate(2)));
        i.registers.set(Register::A, (Byte::MAX / 2) + 1);
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Byte>(Register::A), expected);
        assert!(i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn mul_does_not_cause_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Multiplication(Multiplication::Byte(Register::A, Operand::Immediate(2)));
        i.registers.set(Register::A, Byte::MAX / 2);
        let expected = Byte::MAX - 1;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Byte>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn mul_two_registers_together() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Multiplication(Multiplication::Byte(
            Register::A,
            Operand::Register(Register::B),
        ));
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
    fn mul_the_register_with_itself() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Multiplication(Multiplication::Byte(
            Register::A,
            Operand::Register(Register::A),
        ));
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
        instruction::{Instruction, Multiplication},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn mul_causes_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Multiplication(Multiplication::Quarter(
            Register::A,
            Operand::Immediate(2),
        ));
        i.registers.set(Register::A, (Quarter::MAX / 2) + 1);
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Quarter>(Register::A), expected);
        assert!(i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn mul_does_not_cause_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Multiplication(Multiplication::Quarter(
            Register::A,
            Operand::Immediate(2),
        ));
        i.registers.set(Register::A, Quarter::MAX / 2);
        let expected = Quarter::MAX - 1;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Quarter>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn mul_two_registers_together() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Multiplication(Multiplication::Quarter(
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
    fn mul_the_register_with_itself() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Multiplication(Multiplication::Quarter(
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
        instruction::{Instruction, Multiplication},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn mul_causes_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Multiplication(Multiplication::Half(Register::A, Operand::Immediate(2)));
        i.registers.set(Register::A, (Half::MAX / 2) + 1);
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Half>(Register::A), expected);
        assert!(i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn mul_does_not_cause_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Multiplication(Multiplication::Half(Register::A, Operand::Immediate(2)));
        i.registers.set(Register::A, Half::MAX / 2);
        let expected = Half::MAX - 1;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Half>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn mul_two_registers_together() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Multiplication(Multiplication::Half(
            Register::A,
            Operand::Register(Register::B),
        ));
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
    fn mul_the_register_with_itself() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Multiplication(Multiplication::Half(
            Register::A,
            Operand::Register(Register::A),
        ));
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
        instruction::{Instruction, Multiplication},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn mul_causes_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Multiplication(Multiplication::Word(Register::A, Operand::Immediate(2)));
        i.registers.set(Register::A, (Word::MAX / 2) + 1);
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Word>(Register::A), expected);
        assert!(i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn mul_does_not_cause_overflow() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Multiplication(Multiplication::Word(Register::A, Operand::Immediate(2)));
        i.registers.set(Register::A, Word::MAX / 2);
        let expected = Word::MAX - 1;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Word>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn mul_two_registers_together() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Multiplication(Multiplication::Word(
            Register::A,
            Operand::Register(Register::B),
        ));
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
    fn mul_the_register_with_itself() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Multiplication(Multiplication::Word(
            Register::A,
            Operand::Register(Register::A),
        ));
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
