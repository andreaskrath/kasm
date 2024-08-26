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
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn no_bits_in_common() {
        let mut p = Interpreter::new_test();
        p.registers[Register::A] = Byte::MAX as Word;
        let expected = 0;

        p.and_value::<Byte>(Register::A, Operand::Immediate(0));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(!p.flags.sign);
    }

    #[test]
    fn all_bits_in_common() {
        let mut p = Interpreter::new_test();
        p.registers[Register::A] = Byte::MAX as Word;
        let expected = Byte::MAX as Word;

        p.and_value(Register::A, Operand::Immediate(Byte::MAX));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(p.flags.sign);
    }
}

#[cfg(test)]
mod quarter {
    use crate::{
        constant::{Quarter, Word},
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn no_bits_in_common() {
        let mut p = Interpreter::new_test();
        p.registers[Register::A] = Quarter::MAX as Word;
        let expected = 0;

        p.and_value::<Quarter>(Register::A, Operand::Immediate(0));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(!p.flags.sign);
    }

    #[test]
    fn all_bits_in_common() {
        let mut p = Interpreter::new_test();
        p.registers[Register::A] = Quarter::MAX as Word;
        let expected = Quarter::MAX as Word;

        p.and_value(Register::A, Operand::Immediate(Quarter::MAX));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(p.flags.sign);
    }
}

#[cfg(test)]
mod half {
    use crate::{
        constant::{Half, Word},
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn no_bits_in_common() {
        let mut p = Interpreter::new_test();
        p.registers[Register::A] = Half::MAX as Word;
        let expected = 0;

        p.and_value::<Half>(Register::A, Operand::Immediate(0));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(!p.flags.sign);
    }

    #[test]
    fn all_bits_in_common() {
        let mut p = Interpreter::new_test();
        p.registers[Register::A] = Half::MAX as Word;
        let expected = Half::MAX as Word;

        p.and_value(Register::A, Operand::Immediate(Half::MAX));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(p.flags.sign);
    }
}

#[cfg(test)]
mod word {
    use crate::{constant::Word, operand::Operand, register::Register, Interpreter};

    #[test]
    fn no_bits_in_common() {
        let mut p = Interpreter::new_test();
        p.registers[Register::A] = Word::MAX;
        let expected = 0;

        p.and_value::<Word>(Register::A, Operand::Immediate(0));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(!p.flags.sign);
    }

    #[test]
    fn all_bits_in_common() {
        let mut p = Interpreter::new_test();
        p.registers[Register::A] = Word::MAX;
        let expected = Word::MAX;

        p.and_value(Register::A, Operand::Immediate(Word::MAX));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(p.flags.sign);
    }
}
