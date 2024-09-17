use crate::{
    instruction::Xor, operand::Operand, register::Register, registers::RegisterOperations,
    utils::BitWise, Interpreter,
};

impl Interpreter {
    pub(super) fn xor(&mut self, instruction: Xor) {
        match instruction {
            Xor::Byte(r, o) => self.xor_value(r, o),
            Xor::Quarter(r, o) => self.xor_value(r, o),
            Xor::Half(r, o) => self.xor_value(r, o),
            Xor::Word(r, o) => self.xor_value(r, o),
        }
    }

    fn xor_value<T>(&mut self, register: Register, operand: Operand<T>)
    where
        T: BitWise,
    {
        let a = self.registers.get::<T>(register);
        let b = self.get_operand_value(operand);

        let result = a.bit_xor(b);
        self.flags.set(result, false);
        self.registers.set(register, result);
    }
}

#[cfg(test)]
mod byte {
    use crate::{
        constant::Byte,
        error::ExecuteError,
        instruction::{Instruction, Xor},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn no_bits_in_common() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Xor(Xor::Byte(Register::A, Operand::Immediate(Byte::MAX)));
        let expected = Byte::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Byte>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn all_bits_in_common() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Xor(Xor::Byte(Register::A, Operand::Immediate(Byte::MAX)));
        i.registers.set(Register::A, Byte::MAX);
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Byte>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }
}

#[cfg(test)]
mod quarter {
    use crate::{
        constant::Quarter,
        error::ExecuteError,
        instruction::{Instruction, Xor},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn no_bits_in_common() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Xor(Xor::Quarter(Register::A, Operand::Immediate(Quarter::MAX)));
        let expected = Quarter::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Quarter>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn all_bits_in_common() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Xor(Xor::Quarter(Register::A, Operand::Immediate(Quarter::MAX)));
        i.registers.set(Register::A, Quarter::MAX);
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Quarter>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }
}

#[cfg(test)]
mod half {
    use crate::{
        constant::Half,
        error::ExecuteError,
        instruction::{Instruction, Xor},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn no_bits_in_common() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Xor(Xor::Half(Register::A, Operand::Immediate(Half::MAX)));
        let expected = Half::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Half>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn all_bits_in_common() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Xor(Xor::Half(Register::A, Operand::Immediate(Half::MAX)));
        i.registers.set(Register::A, Half::MAX);
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Half>(Register::A), expected);
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
        instruction::{Instruction, Xor},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn no_bits_in_common() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Xor(Xor::Word(Register::A, Operand::Immediate(Word::MAX)));
        let expected = Word::MAX;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Word>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(!i.flags.zero);
        assert!(i.flags.sign);

        Ok(())
    }

    #[test]
    fn all_bits_in_common() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Xor(Xor::Word(Register::A, Operand::Immediate(Word::MAX)));
        i.registers.set(Register::A, Word::MAX);
        let expected = 0;

        i.execute(instruction)?;

        assert_eq!(i.registers.get::<Word>(Register::A), expected);
        assert!(!i.flags.overflow);
        assert!(i.flags.zero);
        assert!(!i.flags.sign);

        Ok(())
    }
}
