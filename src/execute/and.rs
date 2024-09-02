use crate::{
    instruction::And, operand::Operand, register::Register, registers::RegisterOperations,
    utils::BitWise, Interpreter,
};

impl Interpreter {
    pub fn and(&mut self, instruction: And) {
        match instruction {
            And::Byte(r, o) => self.and_value(r, o),
            And::Quarter(r, o) => self.and_value(r, o),
            And::Half(r, o) => self.and_value(r, o),
            And::Word(r, o) => self.and_value(r, o),
        }
    }

    fn and_value<T>(&mut self, register: Register, operand: Operand<T>)
    where
        T: BitWise,
    {
        let a = self.registers.get::<T>(register);
        let b = self.get_operand_value(operand);

        let result = a.bit_and(b);
        self.flags.set(result, false);
        self.registers[register] = result.to_word();
    }
}

#[cfg(test)]
mod byte {
    use crate::{
        constant::{Byte, Word},
        error::ExecuteError,
        instruction::{And, Instruction},
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn no_bits_in_common() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::And(And::Byte(Register::A, Operand::Immediate(0)));
        i.registers[Register::A] = Byte::MAX as Word;
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn all_bits_in_common() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::And(And::Byte(Register::A, Operand::Immediate(Byte::MAX)));
        i.registers[Register::A] = Byte::MAX as Word;
        let expected = Byte::MAX as Word;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }
}

#[cfg(test)]
mod quarter {
    use crate::{
        constant::{Quarter, Word},
        error::ExecuteError,
        instruction::{And, Instruction},
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn no_bits_in_common() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::And(And::Quarter(Register::A, Operand::Immediate(0)));
        i.registers[Register::A] = Quarter::MAX as Word;
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn all_bits_in_common() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::And(And::Quarter(Register::A, Operand::Immediate(Quarter::MAX)));
        i.registers[Register::A] = Quarter::MAX as Word;
        let expected = Quarter::MAX as Word;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }
}

#[cfg(test)]
mod half {
    use crate::{
        constant::{Half, Word},
        error::ExecuteError,
        instruction::{And, Instruction},
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn no_bits_in_common() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::And(And::Half(Register::A, Operand::Immediate(0)));
        i.registers[Register::A] = Half::MAX as Word;
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn all_bits_in_common() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::And(And::Half(Register::A, Operand::Immediate(Half::MAX)));
        i.registers[Register::A] = Half::MAX as Word;
        let expected = Half::MAX as Word;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
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
        instruction::{And, Instruction},
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn no_bits_in_common() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::And(And::Word(Register::A, Operand::Immediate(0)));
        i.registers[Register::A] = Word::MAX;
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(!i.flags.sign);

        Ok(())
    }

    #[test]
    fn all_bits_in_common() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::And(And::Word(Register::A, Operand::Immediate(Word::MAX)));
        i.registers[Register::A] = Word::MAX;
        let expected = Word::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers[Register::A], expected);
        assert!(!i.flags.zero);
        assert!(!i.flags.overflow);
        assert!(i.flags.sign);

        Ok(())
    }
}
