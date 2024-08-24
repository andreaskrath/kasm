use crate::{instruction::Test, operand::Operand, utils::BitWise, Processor};

impl Processor {
    pub fn test(&mut self, instruction: Test) {
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
        constant::{Byte, Word},
        operand::Operand,
        register::Register,
        Processor,
    };

    #[test]
    fn no_bits_in_common() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Byte::MAX as Word;

        p.test_value::<Byte>(Operand::Register(Register::A), Operand::Immediate(0));

        assert!(p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(!p.flags.sign);
    }

    #[test]
    fn all_bits_in_common() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Byte::MAX as Word;

        p.test_value(
            Operand::Register(Register::A),
            Operand::Immediate(Byte::MAX),
        );

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
        Processor,
    };

    #[test]
    fn no_bits_in_common() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Quarter::MAX as Word;

        p.test_value::<Quarter>(Operand::Register(Register::A), Operand::Immediate(0));

        assert!(p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(!p.flags.sign);
    }

    #[test]
    fn all_bits_in_common() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Quarter::MAX as Word;

        p.test_value(
            Operand::Register(Register::A),
            Operand::Immediate(Quarter::MAX),
        );

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
        Processor,
    };

    #[test]
    fn no_bits_in_common() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Half::MAX as Word;

        p.test_value::<Half>(Operand::Register(Register::A), Operand::Immediate(0));

        assert!(p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(!p.flags.sign);
    }

    #[test]
    fn all_bits_in_common() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Half::MAX as Word;

        p.test_value(
            Operand::Register(Register::A),
            Operand::Immediate(Half::MAX),
        );

        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(p.flags.sign);
    }
}

#[cfg(test)]
mod word {
    use crate::{constant::Word, operand::Operand, register::Register, Processor};

    #[test]
    fn no_bits_in_common() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Word::MAX;

        p.test_value::<Word>(Operand::Register(Register::A), Operand::Immediate(0));

        assert!(p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(!p.flags.sign);
    }

    #[test]
    fn all_bits_in_common() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Word::MAX;

        p.test_value(
            Operand::Register(Register::A),
            Operand::Immediate(Word::MAX),
        );

        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(p.flags.sign);
    }
}
