use crate::{
    instruction::Or, operand::Operand, register::Register, registers::RegisterOperations,
    utils::BitWise, Processor,
};

impl Processor {
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
        self.registers[register] = result.to_word();
    }
}

#[cfg(test)]
mod byte {
    use crate::{
        constant::{Byte, Word},
        operand::Operand,
        register::Register,
        Processor,
    };

    #[test]
    fn no_bits_in_common() {
        let mut p = Processor::test_instance();
        let expected = Byte::MAX as Word;

        p.or_value(Register::A, Operand::Immediate(Byte::MAX));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.sign);
        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
    }

    #[test]
    fn all_bits_in_common() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = Byte::MAX as Word;
        let expected = Byte::MAX as Word;

        p.or_value(Register::A, Operand::Immediate(Byte::MAX));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.sign);
        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
    }
}

#[cfg(test)]
mod quarter {
    use crate::{
        constant::{Quarter, Word},
        operand::Operand,
        register::Register,
        Processor,
    };

    #[test]
    fn no_bits_in_common() {
        let mut p = Processor::test_instance();
        let expected = Quarter::MAX as Word;

        p.or_value(Register::A, Operand::Immediate(Quarter::MAX));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.sign);
        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
    }

    #[test]
    fn all_bits_in_common() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = Quarter::MAX as Word;
        let expected = Quarter::MAX as Word;

        p.or_value(Register::A, Operand::Immediate(Quarter::MAX));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.sign);
        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
    }
}

#[cfg(test)]
mod half {
    use crate::{
        constant::{Half, Word},
        operand::Operand,
        register::Register,
        Processor,
    };

    #[test]
    fn no_bits_in_common() {
        let mut p = Processor::test_instance();
        let expected = Half::MAX as Word;

        p.or_value(Register::A, Operand::Immediate(Half::MAX));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.sign);
        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
    }

    #[test]
    fn all_bits_in_common() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = Half::MAX as Word;
        let expected = Half::MAX as Word;

        p.or_value(Register::A, Operand::Immediate(Half::MAX));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.sign);
        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
    }
}

#[cfg(test)]
mod word {
    use crate::{constant::Word, operand::Operand, register::Register, Processor};

    #[test]
    fn no_bits_in_common() {
        let mut p = Processor::test_instance();
        let expected = Word::MAX;

        p.or_value(Register::A, Operand::Immediate(Word::MAX));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.sign);
        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
    }

    #[test]
    fn all_bits_in_common() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = Word::MAX;
        let expected = Word::MAX;

        p.or_value(Register::A, Operand::Immediate(Word::MAX));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.sign);
        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
    }
}
