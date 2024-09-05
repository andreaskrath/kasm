use crate::{
    error::ExecuteError, instruction::Remainder, operand::Operand, register::Register,
    registers::RegisterOperations, utils::Arithmetic, Interpreter,
};

impl Interpreter {
    pub fn rem(&mut self, instruction: Remainder) -> Result<(), ExecuteError> {
        match instruction {
            Remainder::Byte(r, o) => self.rem_value(r, o),
            Remainder::Quarter(r, o) => self.rem_value(r, o),
            Remainder::Half(r, o) => self.rem_value(r, o),
            Remainder::Word(r, o) => self.rem_value(r, o),
        }
    }

    fn rem_value<T>(&mut self, register: Register, operand: Operand<T>) -> Result<(), ExecuteError>
    where
        T: Arithmetic,
    {
        let a = self.registers.get::<T>(register);
        let b = self.get_operand_value(operand);

        if b.is_zero() {
            return Err(ExecuteError::DivideByZero);
        }

        let (result, overflow) = a.overflow_rem(b);
        self.flags.set(result, overflow);
        self.registers[register] = result.to_word();

        Ok(())
    }
}

#[cfg(test)]
mod byte {
    use crate::{
        constant::Byte,
        error::ExecuteError,
        instruction::{Instruction, Remainder},
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn divide_by_zero_error() {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Remainder(Remainder::Byte(Register::A, Operand::Immediate(0)));
        let expected = Err(ExecuteError::DivideByZero);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn dividend_is_zero() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Remainder(Remainder::Byte(Register::A, Operand::Immediate(Byte::MAX)));
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(!i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn dividing_two_registers() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Remainder(Remainder::Byte(Register::A, Operand::Register(Register::B)));
        i.registers[Register::A] = 20;
        i.registers[Register::B] = 4;
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(!i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn uneven_division() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Remainder(Remainder::Byte(Register::A, Operand::Register(Register::B)));
        i.registers[Register::A] = 20;
        i.registers[Register::B] = 6;
        let expected = 2;

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
        constant::Quarter,
        error::ExecuteError,
        instruction::{Instruction, Remainder},
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn divide_by_zero_error() {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Remainder(Remainder::Quarter(Register::A, Operand::Immediate(0)));
        let expected = Err(ExecuteError::DivideByZero);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn dividend_is_zero() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Remainder(Remainder::Quarter(
            Register::A,
            Operand::Immediate(Quarter::MAX),
        ));
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(!i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn dividing_two_registers() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Remainder(Remainder::Quarter(
            Register::A,
            Operand::Register(Register::B),
        ));
        i.registers[Register::A] = 20;
        i.registers[Register::B] = 4;
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(!i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn uneven_division() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Remainder(Remainder::Quarter(
            Register::A,
            Operand::Register(Register::B),
        ));
        i.registers[Register::A] = 20;
        i.registers[Register::B] = 6;
        let expected = 2;

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
        constant::Half,
        error::ExecuteError,
        instruction::{Instruction, Remainder},
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn divide_by_zero_error() {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Remainder(Remainder::Half(Register::A, Operand::Immediate(0)));
        let expected = Err(ExecuteError::DivideByZero);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn dividend_is_zero() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Remainder(Remainder::Half(Register::A, Operand::Immediate(Half::MAX)));
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(!i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn dividing_two_registers() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Remainder(Remainder::Half(Register::A, Operand::Register(Register::B)));
        i.registers[Register::A] = 20;
        i.registers[Register::B] = 4;
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(!i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn uneven_division() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Remainder(Remainder::Half(Register::A, Operand::Register(Register::B)));
        i.registers[Register::A] = 20;
        i.registers[Register::B] = 6;
        let expected = 2;

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
        instruction::{Instruction, Remainder},
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn divide_by_zero_error() {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Remainder(Remainder::Word(Register::A, Operand::Immediate(0)));
        let expected = Err(ExecuteError::DivideByZero);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn dividend_is_zero() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Remainder(Remainder::Word(Register::A, Operand::Immediate(Word::MAX)));
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(!i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn dividing_two_registers() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Remainder(Remainder::Word(Register::A, Operand::Register(Register::B)));
        i.registers[Register::A] = 20;
        i.registers[Register::B] = 4;
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(!i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn uneven_division() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Remainder(Remainder::Word(Register::A, Operand::Register(Register::B)));
        i.registers[Register::A] = 20;
        i.registers[Register::B] = 6;
        let expected = 2;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }
}
