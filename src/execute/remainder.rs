use crate::{
    error::ExecuteError, instruction::Remainder, operand::Operand, register::Register,
    registers::RegisterOperations, utils::Arithmetic, Processor,
};

impl Processor {
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
        constant::Byte, error::ExecuteError, operand::Operand, register::Register, Processor,
    };

    #[test]
    fn divide_by_zero_error() {
        let mut p = Processor::new().unwrap();
        let expected = Err(ExecuteError::DivideByZero);

        let actual = p.rem_value(Register::A, Operand::Immediate(0u8));

        assert_eq!(actual, expected);
    }

    #[test]
    fn dividend_is_zero() -> Result<(), ExecuteError> {
        let mut p = Processor::new().unwrap();
        let expected = 0;

        p.rem_value(Register::A, Operand::Immediate(Byte::MAX))?;

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);

        Ok(())
    }

    #[test]
    fn dividing_two_registers() -> Result<(), ExecuteError> {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 20;
        p.registers[Register::B] = 4;
        let expected = 0;

        p.rem_value::<Byte>(Register::A, Operand::Register(Register::B))?;

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);

        Ok(())
    }

    #[test]
    fn uneven_division() -> Result<(), ExecuteError> {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 20;
        p.registers[Register::B] = 6;
        let expected = 2;

        p.rem_value::<Byte>(Register::A, Operand::Register(Register::B))?;

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);

        Ok(())
    }
}

#[cfg(test)]
mod quarter {
    use crate::{
        constant::Quarter, error::ExecuteError, operand::Operand, register::Register, Processor,
    };

    #[test]
    fn divide_by_zero_error() {
        let mut p = Processor::new().unwrap();
        let expected = Err(ExecuteError::DivideByZero);

        let actual = p.rem_value(Register::A, Operand::Immediate(0u16));

        assert_eq!(actual, expected);
    }

    #[test]
    fn dividend_is_zero() -> Result<(), ExecuteError> {
        let mut p = Processor::new().unwrap();
        let expected = 0;

        p.rem_value(Register::A, Operand::Immediate(Quarter::MAX))?;

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);

        Ok(())
    }

    #[test]
    fn dividing_two_registers() -> Result<(), ExecuteError> {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 20;
        p.registers[Register::B] = 4;
        let expected = 0;

        p.rem_value::<Quarter>(Register::A, Operand::Register(Register::B))?;

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);

        Ok(())
    }

    #[test]
    fn uneven_division() -> Result<(), ExecuteError> {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 20;
        p.registers[Register::B] = 6;
        let expected = 2;

        p.rem_value::<Quarter>(Register::A, Operand::Register(Register::B))?;

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);

        Ok(())
    }
}

#[cfg(test)]
mod half {
    use crate::{
        constant::Half, error::ExecuteError, operand::Operand, register::Register, Processor,
    };

    #[test]
    fn divide_by_zero_error() {
        let mut p = Processor::new().unwrap();
        let expected = Err(ExecuteError::DivideByZero);

        let actual = p.rem_value(Register::A, Operand::Immediate(0u32));

        assert_eq!(actual, expected);
    }

    #[test]
    fn dividend_is_zero() -> Result<(), ExecuteError> {
        let mut p = Processor::new().unwrap();
        let expected = 0;

        p.rem_value(Register::A, Operand::Immediate(Half::MAX))?;

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);

        Ok(())
    }

    #[test]
    fn dividing_two_registers() -> Result<(), ExecuteError> {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 20;
        p.registers[Register::B] = 4;
        let expected = 0;

        p.rem_value::<Half>(Register::A, Operand::Register(Register::B))?;

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);

        Ok(())
    }

    #[test]
    fn uneven_division() -> Result<(), ExecuteError> {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 20;
        p.registers[Register::B] = 6;
        let expected = 2;

        p.rem_value::<Half>(Register::A, Operand::Register(Register::B))?;

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);

        Ok(())
    }
}

#[cfg(test)]
mod word {
    use crate::{
        constant::Word, error::ExecuteError, operand::Operand, register::Register, Processor,
    };

    #[test]
    fn divide_by_zero_error() {
        let mut p = Processor::new().unwrap();
        let expected = Err(ExecuteError::DivideByZero);

        let actual = p.rem_value(Register::A, Operand::Immediate(0u64));

        assert_eq!(actual, expected);
    }

    #[test]
    fn dividend_is_zero() -> Result<(), ExecuteError> {
        let mut p = Processor::new().unwrap();
        let expected = 0;

        p.rem_value(Register::A, Operand::Immediate(Word::MAX))?;

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);

        Ok(())
    }

    #[test]
    fn dividing_two_registers() -> Result<(), ExecuteError> {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 20;
        p.registers[Register::B] = 4;
        let expected = 0;

        p.rem_value::<Word>(Register::A, Operand::Register(Register::B))?;

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);

        Ok(())
    }

    #[test]
    fn uneven_division() -> Result<(), ExecuteError> {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 20;
        p.registers[Register::B] = 6;
        let expected = 2;

        p.rem_value::<Word>(Register::A, Operand::Register(Register::B))?;

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);

        Ok(())
    }
}
