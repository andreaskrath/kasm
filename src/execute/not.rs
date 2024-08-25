use crate::{
    constant::{Byte, Half, Quarter, Word},
    instruction::Not,
    register::Register,
    registers::RegisterOperations,
    utils::BitWise,
    Processor,
};

impl Processor {
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
        register::Register,
        Processor,
    };

    #[test]
    fn negate_all_zeros() {
        let mut p = Processor::test_instance();
        let expected = Byte::MAX as Word;

        p.not_value::<Byte>(Register::A);

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn negate_all_ones() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = Byte::MAX as Word;
        let expected = 0;

        p.not_value::<Byte>(Register::A);

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
        register::Register,
        Processor,
    };

    #[test]
    fn negate_all_zeros() {
        let mut p = Processor::test_instance();
        let expected = Quarter::MAX as Word;

        p.not_value::<Quarter>(Register::A);

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn negate_all_ones() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = Quarter::MAX as Word;
        let expected = 0;

        p.not_value::<Quarter>(Register::A);

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
        register::Register,
        Processor,
    };

    #[test]
    fn negate_all_zeros() {
        let mut p = Processor::test_instance();
        let expected = Half::MAX as Word;

        p.not_value::<Half>(Register::A);

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn negate_all_ones() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = Half::MAX as Word;
        let expected = 0;

        p.not_value::<Half>(Register::A);

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }
}

#[cfg(test)]
mod word {
    use crate::{constant::Word, register::Register, Processor};

    #[test]
    fn negate_all_zeros() {
        let mut p = Processor::test_instance();
        let expected = Word::MAX;

        p.not_value::<Word>(Register::A);

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn negate_all_ones() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = Word::MAX;
        let expected = 0;

        p.not_value::<Word>(Register::A);

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }
}
