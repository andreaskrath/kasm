use crate::{
    constant::{Byte, Half, Quarter, Word},
    instruction::MulInstruction,
    operand::Operand,
    register::Register,
    registers::RegisterOperations,
    Processor,
};

impl Processor {
    pub fn mul(&mut self, instruction: MulInstruction) {
        match instruction {
            MulInstruction::Byte(r, o) => self.mul_byte(r, o),
            MulInstruction::Quarter(r, o) => self.mul_quarter(r, o),
            MulInstruction::Half(r, o) => self.mul_half(r, o),
            MulInstruction::Word(r, o) => self.mul_word(r, o),
        }
    }

    fn mul_byte(&mut self, register: Register, operand: Operand<Byte>) {
        let a = self.registers.get_reg_val_as_byte(register);
        let b = self.get_byte_operand_val(operand);

        let (result, overflow) = a.overflowing_mul(b);
        self.flags.set_from_byte(result, overflow);
        self.registers[register] = result as Word;
    }

    fn mul_quarter(&mut self, register: Register, operand: Operand<Quarter>) {
        let a = self.registers.get_reg_val_as_quarter(register);
        let b = self.get_quarter_operand_val(operand);

        let (result, overflow) = a.overflowing_mul(b);
        self.flags.set_from_quarter(result, overflow);
        self.registers[register] = result as Word;
    }

    fn mul_half(&mut self, register: Register, operand: Operand<Half>) {
        let a = self.registers.get_reg_val_as_half(register);
        let b = self.get_half_operand_val(operand);

        let (result, overflow) = a.overflowing_mul(b);
        self.flags.set_from_half(result, overflow);
        self.registers[register] = result as Word;
    }

    fn mul_word(&mut self, register: Register, operand: Operand<Word>) {
        let a = self.registers.get_reg_val(register);
        let b = self.get_word_operand_val(operand);

        let (result, overflow) = a.overflowing_mul(b);
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
    fn mul_causes_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = (Byte::MAX as Word / 2) + 1;

        p.mul_byte(Register::A, Operand::Immediate(2));

        assert!(p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn mul_does_not_cause_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Byte::MAX as Word / 2;

        p.mul_byte(Register::A, Operand::Immediate(2));

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

        p.mul_byte(Register::A, Operand::Register(Register::B));

        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);
        assert_eq!(p.registers[Register::A], expected);
    }

    #[test]
    fn mul_the_register_to_itself() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 2;
        let expected = 4;

        p.mul_byte(Register::A, Operand::Register(Register::A));

        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);
        assert_eq!(p.registers[Register::A], expected);
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

        p.mul_quarter(Register::A, Operand::Immediate(2));

        assert!(p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn mul_does_not_cause_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Quarter::MAX as Word / 2;

        p.mul_quarter(Register::A, Operand::Immediate(2));

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

        p.mul_quarter(Register::A, Operand::Register(Register::B));

        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);
        assert_eq!(p.registers[Register::A], expected);
    }

    #[test]
    fn mul_the_register_to_itself() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 2;
        let expected = 4;

        p.mul_quarter(Register::A, Operand::Register(Register::A));

        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);
        assert_eq!(p.registers[Register::A], expected);
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

        p.mul_half(Register::A, Operand::Immediate(2));

        assert!(p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn mul_does_not_cause_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Half::MAX as Word / 2;

        p.mul_half(Register::A, Operand::Immediate(2));

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

        p.mul_half(Register::A, Operand::Register(Register::B));

        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);
        assert_eq!(p.registers[Register::A], expected);
    }

    #[test]
    fn mul_the_register_to_itself() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 2;
        let expected = 4;

        p.mul_half(Register::A, Operand::Register(Register::A));

        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);
        assert_eq!(p.registers[Register::A], expected);
    }
}

#[cfg(test)]
mod word {
    use crate::{constant::Word, operand::Operand, register::Register, Processor};

    #[test]
    fn mul_causes_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = (Word::MAX / 2) + 1;

        p.mul_word(Register::A, Operand::Immediate(2));

        assert!(p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);
    }

    #[test]
    fn mul_does_not_cause_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Word::MAX / 2;

        p.mul_word(Register::A, Operand::Immediate(2));

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

        p.mul_word(Register::A, Operand::Register(Register::B));

        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);
        assert_eq!(p.registers[Register::A], expected);
    }

    #[test]
    fn mul_the_register_to_itself() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 2;
        let expected = 4;

        p.mul_word(Register::A, Operand::Register(Register::A));

        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);
        assert_eq!(p.registers[Register::A], expected);
    }
}
