use crate::{
    constant::{Byte, Half, Quarter, Word},
    instruction::Addition,
    operand::Operand,
    register::Register,
    registers::RegisterOperations,
    Processor,
};

impl Processor {
    pub fn add(&mut self, instruction: Addition) {
        match instruction {
            Addition::Byte(r, o) => self.add_byte(r, o),
            Addition::Quarter(r, o) => self.add_quarter(r, o),
            Addition::Half(r, o) => self.add_half(r, o),
            Addition::Word(r, o) => self.add_word(r, o),
        }
    }

    fn add_byte(&mut self, register: Register, operand: Operand<Byte>) {
        let a = self.registers.get_reg_val_as_byte(register);
        let b = self.get_byte_operand_val(operand);

        let (result, overflow) = a.overflowing_add(b);
        self.flags.set_from_byte(result, overflow);
        self.registers[register] = result as Word;
    }

    fn add_quarter(&mut self, register: Register, operand: Operand<Quarter>) {
        let a = self.registers.get_reg_val_as_quarter(register);
        let b = self.get_quarter_operand_val(operand);

        let (result, overflow) = a.overflowing_add(b);
        self.flags.set_from_quarter(result, overflow);
        self.registers[register] = result as Word;
    }

    fn add_half(&mut self, register: Register, operand: Operand<Half>) {
        let a = self.registers.get_reg_val_as_half(register);
        let b = self.get_half_operand_val(operand);

        let (result, overflow) = a.overflowing_add(b);
        self.flags.set_from_half(result, overflow);
        self.registers[register] = result as Word;
    }

    fn add_word(&mut self, register: Register, operand: Operand<Word>) {
        let a = self.registers.get_reg_val(register);
        let b = self.get_word_operand_val(operand);

        let (result, overflow) = a.overflowing_add(b);
        self.flags.set_from_word(result, overflow);
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
    fn add_causes_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Byte::MAX as Word;
        let expected = 0;

        p.add_byte(Register::A, Operand::Immediate(1));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn add_does_not_cause_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Byte::MAX as Word - 1;
        let expected = Byte::MAX as Word;

        p.add_byte(Register::A, Operand::Immediate(1));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn add_two_registers_together() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 2;
        p.registers[Register::B] = 2;
        let expected = 4;

        p.add_byte(Register::A, Operand::Register(Register::B));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn add_the_register_to_itself() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 2;
        let expected = 4;

        p.add_byte(Register::A, Operand::Register(Register::A));

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
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Quarter::MAX as Word;
        let expected = 0;

        p.add_quarter(Register::A, Operand::Immediate(1));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn add_does_not_cause_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Quarter::MAX as Word - 1;
        let expected = Quarter::MAX as Word;

        p.add_quarter(Register::A, Operand::Immediate(1));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn add_two_registers_together() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 2;
        p.registers[Register::B] = 2;
        let expected = 4;

        p.add_quarter(Register::A, Operand::Register(Register::B));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn add_the_register_to_itself() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 2;
        let expected = 4;

        p.add_quarter(Register::A, Operand::Register(Register::A));

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
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Half::MAX as Word;
        let expected = 0;

        p.add_half(Register::A, Operand::Immediate(1));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn add_does_not_cause_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Half::MAX as Word - 1;
        let expected = Half::MAX as Word;

        p.add_half(Register::A, Operand::Immediate(1));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn add_two_registers_together() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 2;
        p.registers[Register::B] = 2;
        let expected = 4;

        p.add_half(Register::A, Operand::Register(Register::B));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn add_the_register_to_itself() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 2;
        let expected = 4;

        p.add_half(Register::A, Operand::Register(Register::A));

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
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Word::MAX;
        let expected = 0;

        p.add_word(Register::A, Operand::Immediate(1));

        assert_eq!(p.registers[Register::A], expected);
        assert!(p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn add_does_not_cause_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Word::MAX - 1;
        let expected = Word::MAX;

        p.add_word(Register::A, Operand::Immediate(1));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(p.flags.sign);
    }

    #[test]
    fn add_two_registers_together() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 2;
        p.registers[Register::B] = 2;
        let expected = 4;

        p.add_word(Register::A, Operand::Register(Register::B));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn add_the_register_to_itself() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 2;
        let expected = 4;

        p.add_word(Register::A, Operand::Register(Register::A));

        assert_eq!(p.registers[Register::A], expected);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);
    }
}
