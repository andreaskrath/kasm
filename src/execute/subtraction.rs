use crate::{
    instruction::Subtraction, operand::Operand, register::Register, registers::RegisterOperations,
    utils::Arithmetic, Processor,
};

impl Processor {
    pub fn sub(&mut self, instruction: Subtraction) {
        match instruction {
            Subtraction::Byte(r, o) => self.sub_value(r, o),
            Subtraction::Quarter(r, o) => self.sub_value(r, o),
            Subtraction::Half(r, o) => self.sub_value(r, o),
            Subtraction::Word(r, o) => self.sub_value(r, o),
        }
    }

    fn sub_value<T>(&mut self, register: Register, operand: Operand<T>)
    where
        T: Arithmetic,
    {
        let a = self.registers.get::<T>(register);
        let b = self.get_operand_value(operand);

        let (result, overflow) = a.overflow_sub(b);
        self.flags.set(result, overflow);
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
    fn sub_causes_overflow() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = Byte::MIN as Word;
        let expected = Byte::MAX as Word;

        p.sub_value(Register::A, Operand::Immediate(1u8));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn sub_does_not_cause_overflow() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = Byte::MAX as Word;
        let expected = Byte::MAX as Word - 1;

        p.sub_value(Register::A, Operand::Immediate(1u8));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn sub_two_registers_together() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = 2;
        p.registers[Register::B] = 2;
        let expected = 0;

        p.sub_value::<Byte>(Register::A, Operand::Register(Register::B));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn sub_the_register_to_itself() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = 2;
        let expected = 0;

        p.sub_value::<Byte>(Register::A, Operand::Register(Register::A));

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
    fn sub_causes_overflow() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = Quarter::MIN as Word;
        let expected = Quarter::MAX as Word;

        p.sub_value(Register::A, Operand::Immediate(1u16));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn sub_does_not_cause_overflow() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = Quarter::MAX as Word;
        let expected = Quarter::MAX as Word - 1;

        p.sub_value(Register::A, Operand::Immediate(1u16));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn sub_two_registers_together() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = 2;
        p.registers[Register::B] = 2;
        let expected = 0;

        p.sub_value::<Quarter>(Register::A, Operand::Register(Register::B));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn sub_the_register_to_itself() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = 2;
        let expected = 0;

        p.sub_value::<Quarter>(Register::A, Operand::Register(Register::A));

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
    fn sub_causes_overflow() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = Half::MIN as Word;
        let expected = Half::MAX as Word;

        p.sub_value(Register::A, Operand::Immediate(1u32));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn sub_does_not_cause_overflow() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = Half::MAX as Word;
        let expected = Half::MAX as Word - 1;

        p.sub_value(Register::A, Operand::Immediate(1u32));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn sub_two_registers_together() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = 2;
        p.registers[Register::B] = 2;
        let expected = 0;

        p.sub_value::<Half>(Register::A, Operand::Register(Register::B));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn sub_the_register_to_itself() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = 2;
        let expected = 0;

        p.sub_value::<Half>(Register::A, Operand::Register(Register::A));

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
    fn sub_causes_overflow() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = Word::MIN;
        let expected = Word::MAX;

        p.sub_value(Register::A, Operand::Immediate(1u64));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn sub_does_not_cause_overflow() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = Word::MAX;
        let expected = Word::MAX - 1;

        p.sub_value(Register::A, Operand::Immediate(1u64));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn sub_two_registers_together() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = 2;
        p.registers[Register::B] = 2;
        let expected = 0;

        p.sub_value::<Word>(Register::A, Operand::Register(Register::B));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn sub_the_register_to_itself() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = 2;
        let expected = 0;

        p.sub_value::<Word>(Register::A, Operand::Register(Register::A));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }
}
