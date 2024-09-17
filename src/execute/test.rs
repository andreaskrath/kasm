use crate::{instruction::Test, operand::Operand, utils::BitWise, Interpreter};

impl Interpreter {
    pub(super) fn test(&mut self, instruction: Test) {
        match instruction {
            Test::Byte(o1, o2) => self.test_value(o1, o2),
            Test::Quarter(o1, o2) => self.test_value(o1, o2),
            Test::Half(o1, o2) => self.test_value(o1, o2),
            Test::Word(o1, o2) => self.test_value(o1, o2),
        }
    }

    fn test_value<T>(&mut self, operand1: Operand<T>, operand2: Operand<T>)
    where
        T: BitWise,
    {
        let a = self.get_operand_value(operand1);
        let b = self.get_operand_value(operand2);
        let result = a.bit_and(b);
        self.flags.set(result, false);
    }
}

#[cfg(test)]
mod byte {
    use crate::{
        constant::Byte,
        error::ExecuteError,
        instruction::{Instruction, Test},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn no_bits_in_common_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Test(Test::Byte(
            Operand::Immediate(Byte::MAX),
            Operand::Immediate(0),
        ));

        i.execute(instruction)?;

        assert!(i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn all_bits_in_common_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Test(Test::Byte(
            Operand::Immediate(Byte::MAX),
            Operand::Immediate(Byte::MAX),
        ));

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn no_bits_in_common_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Test(Test::Byte(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        ));
        i.registers.set(Register::A, Byte::MAX);

        i.execute(instruction)?;

        assert!(i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn all_bits_in_common_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Test(Test::Byte(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        ));
        i.registers.set(Register::A, Byte::MAX);
        i.registers.set(Register::B, Byte::MAX);

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn no_bits_in_common_mix() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Test(Test::Byte(
            Operand::Register(Register::A),
            Operand::Immediate(Byte::MAX),
        ));

        i.execute(instruction)?;

        assert!(i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn all_bits_in_common_mix() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Test(Test::Byte(
            Operand::Register(Register::A),
            Operand::Immediate(Byte::MAX),
        ));
        i.registers.set(Register::A, Byte::MAX);

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }
}

#[cfg(test)]
mod quarter {
    use crate::{
        constant::Quarter,
        error::ExecuteError,
        instruction::{Instruction, Test},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn no_bits_in_common_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Test(Test::Quarter(
            Operand::Immediate(Quarter::MAX),
            Operand::Immediate(0),
        ));

        i.execute(instruction)?;

        assert!(i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn all_bits_in_common_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Test(Test::Quarter(
            Operand::Immediate(Quarter::MAX),
            Operand::Immediate(Quarter::MAX),
        ));

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn no_bits_in_common_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Test(Test::Quarter(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        ));
        i.registers.set(Register::A, Quarter::MAX);

        i.execute(instruction)?;

        assert!(i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn all_bits_in_common_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Test(Test::Quarter(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        ));
        i.registers.set(Register::A, Quarter::MAX);
        i.registers.set(Register::B, Quarter::MAX);

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn no_bits_in_common_mix() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Test(Test::Quarter(
            Operand::Register(Register::A),
            Operand::Immediate(Quarter::MAX),
        ));

        i.execute(instruction)?;

        assert!(i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn all_bits_in_common_mix() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Test(Test::Quarter(
            Operand::Register(Register::A),
            Operand::Immediate(Quarter::MAX),
        ));
        i.registers.set(Register::A, Quarter::MAX);

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }
}

#[cfg(test)]
mod half {
    use crate::{
        constant::Half,
        error::ExecuteError,
        instruction::{Instruction, Test},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn no_bits_in_common_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Test(Test::Half(
            Operand::Immediate(Half::MAX),
            Operand::Immediate(0),
        ));

        i.execute(instruction)?;

        assert!(i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn all_bits_in_common_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Test(Test::Half(
            Operand::Immediate(Half::MAX),
            Operand::Immediate(Half::MAX),
        ));

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn no_bits_in_common_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Test(Test::Half(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        ));
        i.registers.set(Register::A, Half::MAX);

        i.execute(instruction)?;

        assert!(i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn all_bits_in_common_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Test(Test::Half(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        ));
        i.registers.set(Register::A, Half::MAX);
        i.registers.set(Register::B, Half::MAX);

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn no_bits_in_common_mix() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Test(Test::Half(
            Operand::Register(Register::A),
            Operand::Immediate(Half::MAX),
        ));

        i.execute(instruction)?;

        assert!(i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn all_bits_in_common_mix() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Test(Test::Half(
            Operand::Register(Register::A),
            Operand::Immediate(Half::MAX),
        ));
        i.registers.set(Register::A, Half::MAX);

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }
}

#[cfg(test)]
mod word {
    use crate::{
        constant::Word,
        error::ExecuteError,
        instruction::{Instruction, Test},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn no_bits_in_common_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Test(Test::Word(
            Operand::Immediate(Word::MAX),
            Operand::Immediate(0),
        ));

        i.execute(instruction)?;

        assert!(i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn all_bits_in_common_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Test(Test::Word(
            Operand::Immediate(Word::MAX),
            Operand::Immediate(Word::MAX),
        ));

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn no_bits_in_common_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Test(Test::Word(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        ));
        i.registers.set(Register::A, Word::MAX);

        i.execute(instruction)?;

        assert!(i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn all_bits_in_common_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Test(Test::Word(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        ));
        i.registers.set(Register::A, Word::MAX);
        i.registers.set(Register::B, Word::MAX);

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn no_bits_in_common_mix() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Test(Test::Word(
            Operand::Register(Register::A),
            Operand::Immediate(Word::MAX),
        ));

        i.execute(instruction)?;

        assert!(i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn all_bits_in_common_mix() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Test(Test::Word(
            Operand::Register(Register::A),
            Operand::Immediate(Word::MAX),
        ));
        i.registers.set(Register::A, Word::MAX);

        i.execute(instruction)?;

        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }
}
