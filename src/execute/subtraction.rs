use crate::{
    constant::{Byte, Half, Quarter, Word},
    instruction::Subtraction,
    operand::Operand,
    register::Register,
    registers::RegisterOperations,
    Processor,
};

impl Processor {
    pub fn sub(&mut self, instruction: Subtraction) {
        match instruction {
            Subtraction::Byte(r, o) => self.sub_byte(r, o),
            Subtraction::Quarter(r, o) => self.sub_quarter(r, o),
            Subtraction::Half(r, o) => self.sub_half(r, o),
            Subtraction::Word(r, o) => self.sub_word(r, o),
        }
    }

    fn sub_byte(&mut self, register: Register, operand: Operand<Byte>) {
        let a = self.registers.get_reg_val_as_byte(register);
        let b = self.get_byte_operand_val(operand);

        let (result, overflow) = a.overflowing_sub(b);
        self.flags.set(result, overflow);
        self.registers[register] = result as Word;
    }

    fn sub_quarter(&mut self, register: Register, operand: Operand<Quarter>) {
        let a = self.registers.get_reg_val_as_quarter(register);
        let b = self.get_quarter_operand_val(operand);

        let (result, overflow) = a.overflowing_sub(b);
        self.flags.set(result, overflow);
        self.registers[register] = result as Word;
    }

    fn sub_half(&mut self, register: Register, operand: Operand<Half>) {
        let a = self.registers.get_reg_val_as_half(register);
        let b = self.get_half_operand_val(operand);

        let (result, overflow) = a.overflowing_sub(b);
        self.flags.set(result, overflow);
        self.registers[register] = result as Word;
    }

    fn sub_word(&mut self, register: Register, operand: Operand<Word>) {
        let a = self.registers.get_reg_val(register);
        let b = self.get_word_operand_val(operand);

        let (result, overflow) = a.overflowing_sub(b);
        self.flags.set(result, overflow);
        self.registers[register] = result;
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
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Byte::MIN as Word;
        let expected = Byte::MAX as Word;

        p.sub_byte(Register::A, Operand::Immediate(1));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn sub_does_not_cause_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Byte::MAX as Word;
        let expected = Byte::MAX as Word - 1;

        p.sub_byte(Register::A, Operand::Immediate(1));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn sub_two_registers_together() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 2;
        p.registers[Register::B] = 2;
        let expected = 0;

        p.sub_byte(Register::A, Operand::Register(Register::B));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn sub_the_register_to_itself() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 2;
        let expected = 0;

        p.sub_byte(Register::A, Operand::Register(Register::A));

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
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Quarter::MIN as Word;
        let expected = Quarter::MAX as Word;

        p.sub_quarter(Register::A, Operand::Immediate(1));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn sub_does_not_cause_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Quarter::MAX as Word;
        let expected = Quarter::MAX as Word - 1;

        p.sub_quarter(Register::A, Operand::Immediate(1));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn sub_two_registers_together() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 2;
        p.registers[Register::B] = 2;
        let expected = 0;

        p.sub_quarter(Register::A, Operand::Register(Register::B));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn sub_the_register_to_itself() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 2;
        let expected = 0;

        p.sub_quarter(Register::A, Operand::Register(Register::A));

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
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Half::MIN as Word;
        let expected = Half::MAX as Word;

        p.sub_half(Register::A, Operand::Immediate(1));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn sub_does_not_cause_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Half::MAX as Word;
        let expected = Half::MAX as Word - 1;

        p.sub_half(Register::A, Operand::Immediate(1));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn sub_two_registers_together() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 2;
        p.registers[Register::B] = 2;
        let expected = 0;

        p.sub_half(Register::A, Operand::Register(Register::B));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn sub_the_register_to_itself() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 2;
        let expected = 0;

        p.sub_half(Register::A, Operand::Register(Register::A));

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
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Word::MIN;
        let expected = Word::MAX;

        p.sub_word(Register::A, Operand::Immediate(1));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn sub_does_not_cause_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Word::MAX;
        let expected = Word::MAX - 1;

        p.sub_word(Register::A, Operand::Immediate(1));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn sub_two_registers_together() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 2;
        p.registers[Register::B] = 2;
        let expected = 0;

        p.sub_word(Register::A, Operand::Register(Register::B));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn sub_the_register_to_itself() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 2;
        let expected = 0;

        p.sub_word(Register::A, Operand::Register(Register::A));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }
}
