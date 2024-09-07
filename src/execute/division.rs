use crate::{
    error::ExecuteError, instruction::Division, operand::Operand, register::Register,
    registers::RegisterOperations, utils::Arithmetic, Interpreter,
};

impl Interpreter {
    pub fn div(&mut self, instruction: Division) -> Result<(), ExecuteError> {
        match instruction {
            Division::Byte(r, o) => self.div_value(r, o),
            Division::Quarter(r, o) => self.div_value(r, o),
            Division::Half(r, o) => self.div_value(r, o),
            Division::Word(r, o) => self.div_value(r, o),
        }
    }

    fn div_value<T>(&mut self, register: Register, operand: Operand<T>) -> Result<(), ExecuteError>
    where
        T: Arithmetic,
    {
        let a = self.registers.get::<T>(register);
        let b = self.get_operand_value(operand);

        if b.is_zero() {
            return Err(ExecuteError::DivideByZero);
        }

        let (result, overflow) = a.overflow_div(b);
        self.flags.set(result, overflow);
        self.registers.set(register, result);

        Ok(())
    }
}

#[cfg(test)]
mod byte {
    use crate::{
        constant::Byte,
        error::ExecuteError,
        instruction::{Division, Instruction},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn divide_by_zero_error() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Division(Division::Byte(Register::A, Operand::Immediate(0)));
        let expected = Err(ExecuteError::DivideByZero);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn dividend_is_zero() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Division(Division::Byte(Register::A, Operand::Immediate(Byte::MAX)));
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Byte>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn dividing_two_registers() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Division(Division::Byte(Register::A, Operand::Register(Register::B)));
        i.registers.set(Register::A, 20);
        i.registers.set(Register::B, 4);
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Byte>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn uneven_division() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Division(Division::Byte(Register::A, Operand::Register(Register::B)));
        i.registers.set(Register::A, 20);
        i.registers.set(Register::B, 6);
        let expected = 3;

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
        instruction::{Division, Instruction},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn divide_by_zero_error() {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Division(Division::Quarter(Register::A, Operand::Immediate(0)));
        let expected = Err(ExecuteError::DivideByZero);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn dividend_is_zero() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Division(Division::Quarter(
            Register::A,
            Operand::Immediate(Quarter::MAX),
        ));
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Quarter>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn dividing_two_registers() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Division(Division::Quarter(
            Register::A,
            Operand::Register(Register::B),
        ));
        i.registers.set(Register::A, 20);
        i.registers.set(Register::B, 4);
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Quarter>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn uneven_division() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Division(Division::Quarter(
            Register::A,
            Operand::Register(Register::B),
        ));
        i.registers.set(Register::A, 20);
        i.registers.set(Register::B, 6);
        let expected = 3;

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
        instruction::{Division, Instruction},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn divide_by_zero_error() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Division(Division::Half(Register::A, Operand::Immediate(0)));
        let expected = Err(ExecuteError::DivideByZero);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn dividend_is_zero() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Division(Division::Half(Register::A, Operand::Immediate(Half::MAX)));
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Half>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn dividing_two_registers() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Division(Division::Half(Register::A, Operand::Register(Register::B)));
        i.registers.set(Register::A, 20);
        i.registers.set(Register::B, 4);
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Half>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn uneven_division() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Division(Division::Half(Register::A, Operand::Register(Register::B)));
        i.registers.set(Register::A, 20);
        i.registers.set(Register::B, 6);
        let expected = 3;

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
        instruction::{Division, Instruction},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn divide_by_zero_error() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Division(Division::Word(Register::A, Operand::Immediate(0)));
        let expected = Err(ExecuteError::DivideByZero);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn dividend_is_zero() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Division(Division::Word(Register::A, Operand::Immediate(Word::MAX)));
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Word>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn dividing_two_registers() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Division(Division::Word(Register::A, Operand::Register(Register::B)));
        i.registers.set(Register::A, 20);
        i.registers.set(Register::B, 4);
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Word>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn uneven_division() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Division(Division::Word(Register::A, Operand::Register(Register::B)));
        i.registers.set(Register::A, 20);
        i.registers.set(Register::B, 6);
        let expected = 3;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Word>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }
}
