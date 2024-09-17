use crate::{instruction::Compare, operand::Operand, utils::Arithmetic, Interpreter};

impl Interpreter {
    pub(super) fn compare(&mut self, instruction: Compare) {
        match instruction {
            Compare::Byte(o1, o2) => self.compare_value(o1, o2),
            Compare::Quarter(o1, o2) => self.compare_value(o1, o2),
            Compare::Half(o1, o2) => self.compare_value(o1, o2),
            Compare::Word(o1, o2) => self.compare_value(o1, o2),
        }
    }

    fn compare_value<T>(&mut self, operand1: Operand<T>, operand2: Operand<T>)
    where
        T: Arithmetic,
    {
        let a = self.get_operand_value(operand1);
        let b = self.get_operand_value(operand2);
        let (result, overflow) = a.overflow_sub(b);
        self.flags.set(result, overflow);
    }
}

#[cfg(test)]
mod byte {
    use crate::{
        constant::Byte,
        error::ExecuteError,
        instruction::{Compare, Instruction},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn a_and_b_equal_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Byte(
            Operand::Immediate(Byte::MAX),
            Operand::Immediate(Byte::MAX),
        ));

        i.execute(instruction)?;

        assert!(i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_greater_than_b_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Byte(
            Operand::Immediate(Byte::MAX),
            Operand::Immediate(0),
        ));

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_lesser_than_b_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Compare(Compare::Byte(Operand::Immediate(0), Operand::Immediate(1)));

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_and_b_equal_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Byte(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        ));
        i.registers.set(Register::A, Byte::MAX);
        i.registers.set(Register::B, Byte::MAX);

        i.execute(instruction)?;

        assert!(i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_greater_than_b_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Byte(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        ));
        i.registers.set(Register::A, Byte::MAX);

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_lesser_than_b_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Byte(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        ));
        i.registers.set(Register::B, 1);

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_and_b_equal_mix() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Byte(
            Operand::Register(Register::A),
            Operand::Immediate(Byte::MAX),
        ));
        i.registers.set(Register::A, Byte::MAX);

        i.execute(instruction)?;

        assert!(i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_greater_than_b_mix() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Byte(
            Operand::Register(Register::A),
            Operand::Immediate(0),
        ));
        i.registers.set(Register::A, Byte::MAX);

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_lesser_than_b_mix() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Byte(
            Operand::Register(Register::A),
            Operand::Immediate(1),
        ));

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }
}

#[cfg(test)]
mod quarter {
    use crate::{
        constant::Quarter,
        error::ExecuteError,
        instruction::{Compare, Instruction},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn a_and_b_equal_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Quarter(
            Operand::Immediate(Quarter::MAX),
            Operand::Immediate(Quarter::MAX),
        ));

        i.execute(instruction)?;

        assert!(i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_greater_than_b_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Quarter(
            Operand::Immediate(Quarter::MAX),
            Operand::Immediate(0),
        ));

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_lesser_than_b_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Quarter(
            Operand::Immediate(0),
            Operand::Immediate(1),
        ));

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_and_b_equal_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Quarter(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        ));
        i.registers.set(Register::A, Quarter::MAX);
        i.registers.set(Register::B, Quarter::MAX);

        i.execute(instruction)?;

        assert!(i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_greater_than_b_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Quarter(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        ));
        i.registers.set(Register::A, Quarter::MAX);

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_lesser_than_b_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Quarter(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        ));
        i.registers.set(Register::B, 1);

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_and_b_equal_mix() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Quarter(
            Operand::Register(Register::A),
            Operand::Immediate(Quarter::MAX),
        ));
        i.registers.set(Register::A, Quarter::MAX);

        i.execute(instruction)?;

        assert!(i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_greater_than_b_mix() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Quarter(
            Operand::Register(Register::A),
            Operand::Immediate(0),
        ));
        i.registers.set(Register::A, Quarter::MAX);

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_lesser_than_b_mix() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Quarter(
            Operand::Register(Register::A),
            Operand::Immediate(1),
        ));

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }
}

#[cfg(test)]
mod half {
    use crate::{
        constant::Half,
        error::ExecuteError,
        instruction::{Compare, Instruction},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn a_and_b_equal_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Half(
            Operand::Immediate(Half::MAX),
            Operand::Immediate(Half::MAX),
        ));

        i.execute(instruction)?;

        assert!(i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_greater_than_b_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Half(
            Operand::Immediate(Half::MAX),
            Operand::Immediate(0),
        ));

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_lesser_than_b_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Compare(Compare::Half(Operand::Immediate(0), Operand::Immediate(1)));

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_and_b_equal_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Half(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        ));
        i.registers.set(Register::A, Half::MAX);
        i.registers.set(Register::B, Half::MAX);

        i.execute(instruction)?;

        assert!(i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_greater_than_b_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Half(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        ));
        i.registers.set(Register::A, Half::MAX);

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_lesser_than_b_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Half(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        ));
        i.registers.set(Register::B, 1);

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_and_b_equal_mix() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Half(
            Operand::Register(Register::A),
            Operand::Immediate(Half::MAX),
        ));
        i.registers.set(Register::A, Half::MAX);

        i.execute(instruction)?;

        assert!(i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_greater_than_b_mix() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Half(
            Operand::Register(Register::A),
            Operand::Immediate(0),
        ));
        i.registers.set(Register::A, Half::MAX);

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_lesser_than_b_mix() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Half(
            Operand::Register(Register::A),
            Operand::Immediate(1),
        ));

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }
}

#[cfg(test)]
mod word {
    use crate::{
        constant::Word,
        error::ExecuteError,
        instruction::{Compare, Instruction},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn a_and_b_equal_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Word(
            Operand::Immediate(Word::MAX),
            Operand::Immediate(Word::MAX),
        ));

        i.execute(instruction)?;

        assert!(i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_greater_than_b_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Word(
            Operand::Immediate(Word::MAX),
            Operand::Immediate(0),
        ));

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_lesser_than_b_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Compare(Compare::Word(Operand::Immediate(0), Operand::Immediate(1)));

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_and_b_equal_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Word(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        ));
        i.registers.set(Register::A, Word::MAX);
        i.registers.set(Register::B, Word::MAX);

        i.execute(instruction)?;

        assert!(i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_greater_than_b_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Word(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        ));
        i.registers.set(Register::A, Word::MAX);

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_lesser_than_b_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Word(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        ));
        i.registers.set(Register::B, 1);

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_and_b_equal_mix() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Word(
            Operand::Register(Register::A),
            Operand::Immediate(Word::MAX),
        ));
        i.registers.set(Register::A, Word::MAX);

        i.execute(instruction)?;

        assert!(i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_greater_than_b_mix() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Word(
            Operand::Register(Register::A),
            Operand::Immediate(0),
        ));
        i.registers.set(Register::A, Word::MAX);

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn a_lesser_than_b_mix() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Compare(Compare::Word(
            Operand::Register(Register::A),
            Operand::Immediate(1),
        ));

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }
}
