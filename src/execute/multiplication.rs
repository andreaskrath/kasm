use crate::{
    instruction::Multiplication, operand::Operand, register::Register,
    registers::RegisterOperations, utils::Arithmetic, Processor,
};

impl Processor {
    pub fn mul(&mut self, instruction: Multiplication) {
        match instruction {
            Multiplication::Byte(r, o) => self.mul_value(r, o),
            Multiplication::Quarter(r, o) => self.mul_value(r, o),
            Multiplication::Half(r, o) => self.mul_value(r, o),
            Multiplication::Word(r, o) => self.mul_value(r, o),
        }
    }

    fn mul_value<T>(&mut self, register: Register, operand: Operand<T>)
    where
        T: Arithmetic,
    {
        let a = self.registers.get::<T>(register);
        let b = self.get_operand_value(operand);

        let (result, overflow) = a.overflow_mul(b);
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
    fn mul_causes_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = (Byte::MAX as Word / 2) + 1;
        let expected = 0;

        p.mul_value(Register::A, Operand::Immediate(2u8));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn mul_does_not_cause_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Byte::MAX as Word / 2;
        let expected = Byte::MAX as Word - 1;

        p.mul_value(Register::A, Operand::Immediate(2u8));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn mul_two_registers_together() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 2;
        p.registers[Register::B] = 2;
        let expected = 4;

        p.mul_value::<Byte>(Register::A, Operand::Register(Register::B));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn mul_the_register_with_itself() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 2;
        let expected = 4;

        p.mul_value::<Byte>(Register::A, Operand::Register(Register::A));

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
    fn mul_causes_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = (Quarter::MAX as Word / 2) + 1;
        let expected = 0;

        p.mul_value(Register::A, Operand::Immediate(2u16));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn mul_does_not_cause_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Quarter::MAX as Word / 2;
        let expected = Quarter::MAX as Word - 1;

        p.mul_value(Register::A, Operand::Immediate(2u16));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn mul_two_registers_together() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 2;
        p.registers[Register::B] = 2;
        let expected = 4;

        p.mul_value::<Quarter>(Register::A, Operand::Register(Register::B));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn mul_the_register_with_itself() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 2;
        let expected = 4;

        p.mul_value::<Quarter>(Register::A, Operand::Register(Register::A));

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
    fn mul_causes_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = (Half::MAX as Word / 2) + 1;
        let expected = 0;

        p.mul_value(Register::A, Operand::Immediate(2u32));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn mul_does_not_cause_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Half::MAX as Word / 2;
        let expected = Half::MAX as Word - 1;

        p.mul_value(Register::A, Operand::Immediate(2u32));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn mul_two_registers_together() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 2;
        p.registers[Register::B] = 2;
        let expected = 4;

        p.mul_value::<Half>(Register::A, Operand::Register(Register::B));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn mul_the_register_with_itself() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 2;
        let expected = 4;

        p.mul_value::<Half>(Register::A, Operand::Register(Register::A));

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
    fn mul_causes_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = (Word::MAX / 2) + 1;
        let expected = 0;

        p.mul_value(Register::A, Operand::Immediate(2u64));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn mul_does_not_cause_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Word::MAX / 2;
        let expected = Word::MAX - 1;

        p.mul_value(Register::A, Operand::Immediate(2u64));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn mul_two_registers_together() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 2;
        p.registers[Register::B] = 2;
        let expected = 4;

        p.mul_value::<Word>(Register::A, Operand::Register(Register::B));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn mul_the_register_with_itself() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 2;
        let expected = 4;

        p.mul_value::<Word>(Register::A, Operand::Register(Register::A));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);
    }
}
