use crate::{
    instruction::Addition, operand::Operand, register::Register, registers::RegisterOperations,
    utils::Arithmetic, Processor,
};

impl Processor {
    pub fn add(&mut self, instruction: Addition) {
        match instruction {
            Addition::Byte(r, o) => self.add_value(r, o),
            Addition::Quarter(r, o) => self.add_value(r, o),
            Addition::Half(r, o) => self.add_value(r, o),
            Addition::Word(r, o) => self.add_value(r, o),
        }
    }

    fn add_value<T>(&mut self, register: Register, operand: Operand<T>)
    where
        T: Arithmetic,
    {
        let a = self.registers.get::<T>(register);
        let b = self.get_operand_value(operand);

        let (result, overflow) = a.overflow_add(b);
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
    fn add_causes_overflow() {
        let mut p = Processor::new_test();
        p.registers[Register::A] = Byte::MAX as Word;
        let expected = 0;

        p.add_value(Register::A, Operand::Immediate(1u8));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn add_does_not_cause_overflow() {
        let mut p = Processor::new_test();
        p.registers[Register::A] = Byte::MAX as Word - 1;
        let expected = Byte::MAX as Word;

        p.add_value(Register::A, Operand::Immediate(1u8));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn add_two_registers_together() {
        let mut p = Processor::new_test();
        p.registers[Register::A] = 2;
        p.registers[Register::B] = 2;
        let expected = 4;

        p.add_value::<Byte>(Register::A, Operand::Register(Register::B));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn add_the_register_to_itself() {
        let mut p = Processor::new_test();
        p.registers[Register::A] = 2;
        let expected = 4;

        p.add_value::<Byte>(Register::A, Operand::Register(Register::A));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
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
    fn add_causes_overflow() {
        let mut p = Processor::new_test();
        p.registers[Register::A] = Quarter::MAX as Word;
        let expected = 0;

        p.add_value(Register::A, Operand::Immediate(1u16));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn add_does_not_cause_overflow() {
        let mut p = Processor::new_test();
        p.registers[Register::A] = Quarter::MAX as Word - 1;
        let expected = Quarter::MAX as Word;

        p.add_value(Register::A, Operand::Immediate(1u16));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn add_two_registers_together() {
        let mut p = Processor::new_test();
        p.registers[Register::A] = 2;
        p.registers[Register::B] = 2;
        let expected = 4;

        p.add_value::<Quarter>(Register::A, Operand::Register(Register::B));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn add_the_register_to_itself() {
        let mut p = Processor::new_test();
        p.registers[Register::A] = 2;
        let expected = 4;

        p.add_value::<Quarter>(Register::A, Operand::Register(Register::A));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
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
    fn add_causes_overflow() {
        let mut p = Processor::new_test();
        p.registers[Register::A] = Half::MAX as Word;
        let expected = 0;

        p.add_value(Register::A, Operand::Immediate(1u32));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn add_does_not_cause_overflow() {
        let mut p = Processor::new_test();
        p.registers[Register::A] = Half::MAX as Word - 1;
        let expected = Half::MAX as Word;

        p.add_value(Register::A, Operand::Immediate(1u32));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn add_two_registers_together() {
        let mut p = Processor::new_test();
        p.registers[Register::A] = 2;
        p.registers[Register::B] = 2;
        let expected = 4;

        p.add_value::<Half>(Register::A, Operand::Register(Register::B));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn add_the_register_to_itself() {
        let mut p = Processor::new_test();
        p.registers[Register::A] = 2;
        let expected = 4;

        p.add_value::<Half>(Register::A, Operand::Register(Register::A));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);
    }
}

#[cfg(test)]
mod word {
    use crate::{constant::Word, operand::Operand, register::Register, Processor};

    #[test]
    fn add_causes_overflow() {
        let mut p = Processor::new_test();
        p.registers[Register::A] = Word::MAX;
        let expected = 0;

        p.add_value(Register::A, Operand::Immediate(1u64));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn add_does_not_cause_overflow() {
        let mut p = Processor::new_test();
        p.registers[Register::A] = Word::MAX - 1;
        let expected = Word::MAX;

        p.add_value(Register::A, Operand::Immediate(1u64));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn add_two_registers_together() {
        let mut p = Processor::new_test();
        p.registers[Register::A] = 2;
        p.registers[Register::B] = 2;
        let expected = 4;

        p.add_value::<Word>(Register::A, Operand::Register(Register::B));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn add_the_register_to_itself() {
        let mut p = Processor::new_test();
        p.registers[Register::A] = 2;
        let expected = 4;

        p.add_value::<Word>(Register::A, Operand::Register(Register::A));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);
    }
}
