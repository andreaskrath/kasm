use crate::{
    instruction::Or, operand::Operand, register::Register, registers::RegisterOperations,
    utils::BitWise, Interpreter,
};

impl Interpreter {
    pub fn or(&mut self, instruction: Or) {
        match instruction {
            Or::Byte(r, o) => self.or_value(r, o),
            Or::Quarter(r, o) => self.or_value(r, o),
            Or::Half(r, o) => self.or_value(r, o),
            Or::Word(r, o) => self.or_value(r, o),
        }
    }

    fn or_value<T>(&mut self, register: Register, operand: Operand<T>)
    where
        T: BitWise,
    {
        let a = self.registers.get::<T>(register);
        let b = self.get_operand_value(operand);

        let result = a.bit_or(b);
        self.flags.set(result, false);
        self.registers.set(register, result);
    }
}

#[cfg(test)]
mod byte {
    use crate::{
        constant::Byte,
        error::ExecuteError,
        instruction::{Instruction, Or},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn no_bits_in_common() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Or(Or::Byte(Register::A, Operand::Immediate(Byte::MAX)));
        let expected = Byte::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Byte>(Register::A), expected);
        assert!(i.flags.sign);
        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);

        Ok(())
    }

    #[test]
    fn all_bits_in_common() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        i.registers.set(Register::A, Byte::MAX);
        let instruction = Instruction::Or(Or::Byte(Register::A, Operand::Immediate(Byte::MAX)));
        let expected = Byte::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Byte>(Register::A), expected);
        assert!(i.flags.sign);
        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);

        Ok(())
    }
}

#[cfg(test)]
mod quarter {
    use crate::{
        constant::Quarter,
        error::ExecuteError,
        instruction::{Instruction, Or},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn no_bits_in_common() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Or(Or::Quarter(Register::A, Operand::Immediate(Quarter::MAX)));
        let expected = Quarter::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Quarter>(Register::A), expected);
        assert!(i.flags.sign);
        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);

        Ok(())
    }

    #[test]
    fn all_bits_in_common() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        i.registers.set(Register::A, Quarter::MAX);
        let instruction =
            Instruction::Or(Or::Quarter(Register::A, Operand::Immediate(Quarter::MAX)));
        let expected = Quarter::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Quarter>(Register::A), expected);
        assert!(i.flags.sign);
        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);

        Ok(())
    }
}

#[cfg(test)]
mod half {
    use crate::{
        constant::Half,
        error::ExecuteError,
        instruction::{Instruction, Or},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn no_bits_in_common() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Or(Or::Half(Register::A, Operand::Immediate(Half::MAX)));
        let expected = Half::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Half>(Register::A), expected);
        assert!(i.flags.sign);
        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);

        Ok(())
    }

    #[test]
    fn all_bits_in_common() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        i.registers.set(Register::A, Half::MAX);
        let instruction = Instruction::Or(Or::Half(Register::A, Operand::Immediate(Half::MAX)));
        let expected = Half::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Half>(Register::A), expected);
        assert!(i.flags.sign);
        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);

        Ok(())
    }
}

#[cfg(test)]
mod word {
    use crate::{
        constant::Word,
        error::ExecuteError,
        instruction::{Instruction, Or},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn no_bits_in_common() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Or(Or::Word(Register::A, Operand::Immediate(Word::MAX)));
        let expected = Word::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Word>(Register::A), expected);
        assert!(i.flags.sign);
        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);

        Ok(())
    }

    #[test]
    fn all_bits_in_common() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        i.registers.set(Register::A, Word::MAX);
        let instruction = Instruction::Or(Or::Word(Register::A, Operand::Immediate(Word::MAX)));
        let expected = Word::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Word>(Register::A), expected);
        assert!(i.flags.sign);
        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);

        Ok(())
    }
}
