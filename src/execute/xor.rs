use crate::{
    instruction::Xor, operand::Operand, register::Register, registers::RegisterOperations,
    utils::BitWise, Processor,
};

impl Processor {
    pub fn xor(&mut self, instruction: Xor) {
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
        let mut p = Processor::new().unwrap();
        let expected = Byte::MAX as Word;

        p.xor_value(Register::A, Operand::Immediate(Byte::MAX));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn all_bits_in_common() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Byte::MAX as Word;
        let expected = 0;

        p.xor_value(Register::A, Operand::Immediate(Byte::MAX));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
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
        let expected = Quarter::MAX as Word;

        p.xor_value(Register::A, Operand::Immediate(Quarter::MAX));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn all_bits_in_common() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Quarter::MAX as Word;
        let expected = 0;

        p.xor_value(Register::A, Operand::Immediate(Quarter::MAX));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
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
        let expected = Half::MAX as Word;

        p.xor_value(Register::A, Operand::Immediate(Half::MAX));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn all_bits_in_common() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Half::MAX as Word;
        let expected = 0;

        p.xor_value(Register::A, Operand::Immediate(Half::MAX));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }
}

#[cfg(test)]
mod word {
    use crate::{constant::Word, operand::Operand, register::Register, Processor};

    #[test]
    fn no_bits_in_common() {
        let mut p = Processor::new().unwrap();
        let expected = Word::MAX as Word;

        p.xor_value(Register::A, Operand::Immediate(Word::MAX));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn all_bits_in_common() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Word::MAX as Word;
        let expected = 0;

        p.xor_value(Register::A, Operand::Immediate(Word::MAX));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }
}
