use crate::{
    constant::{Byte, Half, Quarter, Word},
    instruction::Not,
    register::Register,
    registers::RegisterOperations,
    utils::BitWise,
    Interpreter,
};

impl Interpreter {
    pub fn not(&mut self, instruction: Not) {
        match instruction {
            Not::Byte(r) => self.not_value::<Byte>(r),
            Not::Quarter(r) => self.not_value::<Quarter>(r),
            Not::Half(r) => self.not_value::<Half>(r),
            Not::Word(r) => self.not_value::<Word>(r),
        }
    }

    fn not_value<T>(&mut self, register: Register)
    where
        T: BitWise,
    {
        let value = self.registers.get::<T>(register);
        let result = value.bit_not();
        self.flags.set(result, false);
        self.registers[register] = result.to_word();
    }
}

#[cfg(test)]
mod byte {
    use crate::{
        constant::{Byte, Word},
        error::ExecuteError,
        instruction::{Instruction, Not},
        register::Register,
        Interpreter,
    };

    #[test]
    fn negate_all_zeros() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Not(Not::Byte(Register::A));
        let expected = Byte::MAX as Word;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn negate_all_ones() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Not(Not::Byte(Register::A));
        i.registers[Register::A] = Byte::MAX as Word;
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
        instruction::{Instruction, Not},
        register::Register,
        Interpreter,
    };

    #[test]
    fn negate_all_zeros() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Not(Not::Quarter(Register::A));
        let expected = Quarter::MAX as Word;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn negate_all_ones() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Not(Not::Quarter(Register::A));
        i.registers[Register::A] = Quarter::MAX as Word;
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
        instruction::{Instruction, Not},
        register::Register,
        Interpreter,
    };

    #[test]
    fn negate_all_zeros() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Not(Not::Half(Register::A));
        let expected = Half::MAX as Word;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn negate_all_ones() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Not(Not::Half(Register::A));
        i.registers[Register::A] = Half::MAX as Word;
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
        instruction::{Instruction, Not},
        register::Register,
        Interpreter,
    };

    #[test]
    fn negate_all_zeros() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Not(Not::Word(Register::A));
        let expected = Word::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn negate_all_ones() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Not(Not::Word(Register::A));
        i.registers[Register::A] = Word::MAX;
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(!i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }
}
