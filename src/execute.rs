use crate::{
    constant::{Byte, Half, Quarter, Word},
    error::ExecuteError,
    instruction::Instruction,
    operand::Operand,
    register::Register,
    registers::RegisterOperations,
    Processor,
};

impl Processor {
    fn get_byte_operand_val(&self, operand: Operand<Byte>) -> Byte {
        match operand {
            Operand::Register(register) => self.registers.get_reg_val_as_byte(register),
            Operand::Immediate(immediate) => immediate,
        }
    }

    fn get_quarter_operand_val(&self, operand: Operand<Quarter>) -> Quarter {
        match operand {
            Operand::Register(register) => self.registers.get_reg_val_as_quarter(register),
            Operand::Immediate(immediate) => immediate,
        }
    }

    fn get_half_operand_val(&self, operand: Operand<Half>) -> Half {
        match operand {
            Operand::Register(register) => self.registers.get_reg_val_as_half(register),
            Operand::Immediate(immediate) => immediate,
        }
    }

    fn get_word_operand_val(&self, operand: Operand<Word>) -> Word {
        match operand {
            Operand::Register(register) => self.registers.get_reg_val(register),
            Operand::Immediate(immediate) => immediate,
        }
    }

    pub fn execute(&mut self, instruction: Instruction) -> Result<(), ExecuteError> {
        match instruction {
            Instruction::Stop => self.execute_stop(),
            Instruction::SetByte(r, o) => self.execute_set_byte(r, o),
            Instruction::SetQuarter(r, o) => self.execute_set_quarter(r, o),
            Instruction::SetHalf(r, o) => self.execute_set_half(r, o),
            Instruction::SetWord(r, o) => self.execute_set_word(r, o),
            Instruction::AddByte(r, o) => self.execute_add_byte(r, o),
            Instruction::AddQuarter(r, o) => self.execute_add_quarter(r, o),
            Instruction::AddHalf(r, o) => self.execute_add_half(r, o),
            Instruction::AddWord(r, o) => self.execute_add_word(r, o),
            Instruction::SubByte(r, o) => self.execute_sub_byte(r, o),
            Instruction::SubQuarter(r, o) => self.execute_sub_quarter(r, o),
            Instruction::SubHalf(r, o) => self.execute_sub_half(r, o),
            Instruction::SubWord(r, o) => self.execute_sub_word(r, o),
            Instruction::MulByte(r, o) => self.execute_mul_byte(r, o),
            Instruction::MulQuarter(r, o) => self.execute_mul_quarter(r, o),
            Instruction::MulHalf(r, o) => self.execute_mul_half(r, o),
            Instruction::MulWord(r, o) => self.execute_mul_word(r, o),
            Instruction::DivByte(r, o) => self.execute_div_byte(r, o),
            Instruction::DivQuarter(r, o) => self.execute_div_quarter(r, o),
            Instruction::DivHalf(r, o) => self.execute_div_half(r, o),
        }

        Ok(())
    }

    fn execute_stop(&mut self) {
        self.running = false;
    }

    fn execute_set_byte(&mut self, register: Register, operand: Operand<Byte>) {
        let value = self.get_byte_operand_val(operand);
        self.registers[register] = value as Word;
    }

    fn execute_set_quarter(&mut self, register: Register, operand: Operand<Quarter>) {
        let value = self.get_quarter_operand_val(operand);
        self.registers[register] = value as Word;
    }

    fn execute_set_half(&mut self, register: Register, operand: Operand<Half>) {
        let value = self.get_half_operand_val(operand);
        self.registers[register] = value as Word;
    }

    fn execute_set_word(&mut self, register: Register, operand: Operand<Word>) {
        let value = self.get_word_operand_val(operand);
        self.registers[register] = value;
    }

    fn execute_add_byte(&mut self, register: Register, operand: Operand<Byte>) {
        let a = self.registers.get_reg_val_as_byte(register);
        let b = self.get_byte_operand_val(operand);

        let (result, overflow) = a.overflowing_add(b);
        self.flags.set_from_byte(result, overflow);
        self.registers[register] = result as Word;
    }

    fn execute_add_quarter(&mut self, register: Register, operand: Operand<Quarter>) {
        let a = self.registers.get_reg_val_as_quarter(register);
        let b = self.get_quarter_operand_val(operand);

        let (result, overflow) = a.overflowing_add(b);
        self.flags.set_from_quarter(result, overflow);
        self.registers[register] = result as Word;
    }

    fn execute_add_half(&mut self, register: Register, operand: Operand<Half>) {
        let a = self.registers.get_reg_val_as_half(register);
        let b = self.get_half_operand_val(operand);

        let (result, overflow) = a.overflowing_add(b);
        self.flags.set_from_half(result, overflow);
        self.registers[register] = result as Word;
    }

    fn execute_add_word(&mut self, register: Register, operand: Operand<Word>) {
        let a = self.registers.get_reg_val(register);
        let b = self.get_word_operand_val(operand);

        let (result, overflow) = a.overflowing_add(b);
        self.flags.set_from_word(result, overflow);
        self.registers[register] = result;
    }

    fn execute_sub_byte(&mut self, register: Register, operand: Operand<Byte>) {
        let a = self.registers.get_reg_val_as_byte(register);
        let b = self.get_byte_operand_val(operand);

        let (result, overflow) = a.overflowing_sub(b);
        self.flags.set_from_byte(result, overflow);
        self.registers[register] = result as Word;
    }

    fn execute_sub_quarter(&mut self, register: Register, operand: Operand<Quarter>) {
        let a = self.registers.get_reg_val_as_quarter(register);
        let b = self.get_quarter_operand_val(operand);

        let (result, overflow) = a.overflowing_sub(b);
        self.flags.set_from_quarter(result, overflow);
        self.registers[register] = result as Word;
    }

    fn execute_sub_half(&mut self, register: Register, operand: Operand<Half>) {
        let a = self.registers.get_reg_val_as_half(register);
        let b = self.get_half_operand_val(operand);

        let (result, overflow) = a.overflowing_sub(b);
        self.flags.set_from_half(result, overflow);
        self.registers[register] = result as Word;
    }

    fn execute_sub_word(&mut self, register: Register, operand: Operand<Word>) {
        let a = self.registers.get_reg_val(register);
        let b = self.get_word_operand_val(operand);

        let (result, overflow) = a.overflowing_sub(b);
        self.flags.set_from_word(result, overflow);
        self.registers[register] = result;
    }

    fn execute_mul_byte(&mut self, register: Register, operand: Operand<Byte>) {
        let a = self.registers.get_reg_val_as_byte(register);
        let b = self.get_byte_operand_val(operand);

        let (result, overflow) = a.overflowing_mul(b);
        self.flags.set_from_byte(result, overflow);
        self.registers[register] = result as Word;
    }

    fn execute_mul_quarter(&mut self, register: Register, operand: Operand<Quarter>) {
        let a = self.registers.get_reg_val_as_quarter(register);
        let b = self.get_quarter_operand_val(operand);

        let (result, overflow) = a.overflowing_mul(b);
        self.flags.set_from_quarter(result, overflow);
        self.registers[register] = result as Word;
    }

    fn execute_mul_half(&mut self, register: Register, operand: Operand<Half>) {
        let a = self.registers.get_reg_val_as_half(register);
        let b = self.get_half_operand_val(operand);

        let (result, overflow) = a.overflowing_mul(b);
        self.flags.set_from_half(result, overflow);
        self.registers[register] = result as Word;
    }

    fn execute_mul_word(&mut self, register: Register, operand: Operand<Word>) {
        let a = self.registers.get_reg_val(register);
        let b = self.get_word_operand_val(operand);

        let (result, overflow) = a.overflowing_mul(b);
        self.flags.set_from_word(result, overflow);
        self.registers[register] = result;
    }

    fn execute_div_byte(&mut self, register: Register, operand: Operand<Byte>) {
        let a = self.registers.get_reg_val_as_byte(register);
        let b = self.get_byte_operand_val(operand);

        let (result, overflow) = a.overflowing_div(b);
        // I am pretty certain divsion can only cause an overflow if the integers are signed.
        self.flags.set_from_byte(result, overflow);
        self.registers[register] = result as Word;
    }

    fn execute_div_quarter(&mut self, register: Register, operand: Operand<Quarter>) {
        let a = self.registers.get_reg_val_as_quarter(register);
        let b = self.get_quarter_operand_val(operand);

        let (result, overflow) = a.overflowing_div(b);
        // I am pretty certain divsion can only cause an overflow if the integers are signed.
        self.flags.set_from_quarter(result, overflow);
        self.registers[register] = result as Word;
    }

    fn execute_div_half(&mut self, register: Register, operand: Operand<Half>) {
        let a = self.registers.get_reg_val_as_half(register);
        let b = self.get_half_operand_val(operand);

        let (result, overflow) = a.overflowing_div(b);
        // I am pretty certain divsion can only cause an overflow if the integers are signed.
        self.flags.set_from_half(result, overflow);
        self.registers[register] = result as Word;
    }
}

#[cfg(test)]
mod add {
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

            p.execute_add_byte(Register::A, Operand::Immediate(1));

            assert!(p.flags.overflow);
            assert!(p.flags.zero);
            assert!(!p.flags.sign);
        }

        #[test]
        fn add_does_not_cause_overflow() {
            let mut p = Processor::new().unwrap();
            p.registers[Register::A] = Byte::MAX as Word - 1;

            p.execute_add_byte(Register::A, Operand::Immediate(1));

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

            p.execute_add_byte(Register::A, Operand::Register(Register::B));

            assert!(!p.flags.overflow);
            assert!(!p.flags.zero);
            assert!(!p.flags.sign);
            assert_eq!(p.registers[Register::A], expected);
        }

        #[test]
        fn add_the_register_to_itself() {
            let mut p = Processor::new().unwrap();
            p.registers[Register::A] = 2;
            let expected = 4;

            p.execute_add_byte(Register::A, Operand::Register(Register::A));

            assert!(!p.flags.overflow);
            assert!(!p.flags.zero);
            assert!(!p.flags.sign);
            assert_eq!(p.registers[Register::A], expected);
        }
    }

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

            p.execute_add_quarter(Register::A, Operand::Immediate(1));

            assert!(p.flags.overflow);
            assert!(p.flags.zero);
            assert!(!p.flags.sign);
        }

        #[test]
        fn add_does_not_cause_overflow() {
            let mut p = Processor::new().unwrap();
            p.registers[Register::A] = Quarter::MAX as Word - 1;

            p.execute_add_quarter(Register::A, Operand::Immediate(1));

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

            p.execute_add_quarter(Register::A, Operand::Register(Register::B));

            assert!(!p.flags.overflow);
            assert!(!p.flags.zero);
            assert!(!p.flags.sign);
            assert_eq!(p.registers[Register::A], expected);
        }

        #[test]
        fn add_the_register_to_itself() {
            let mut p = Processor::new().unwrap();
            p.registers[Register::A] = 2;
            let expected = 4;

            p.execute_add_quarter(Register::A, Operand::Register(Register::A));

            assert!(!p.flags.overflow);
            assert!(!p.flags.zero);
            assert!(!p.flags.sign);
            assert_eq!(p.registers[Register::A], expected);
        }
    }

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

            p.execute_add_half(Register::A, Operand::Immediate(1));

            assert!(p.flags.overflow);
            assert!(p.flags.zero);
            assert!(!p.flags.sign);
        }

        #[test]
        fn add_does_not_cause_overflow() {
            let mut p = Processor::new().unwrap();
            p.registers[Register::A] = Half::MAX as Word - 1;

            p.execute_add_half(Register::A, Operand::Immediate(1));

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

            p.execute_add_half(Register::A, Operand::Register(Register::B));

            assert!(!p.flags.overflow);
            assert!(!p.flags.zero);
            assert!(!p.flags.sign);
            assert_eq!(p.registers[Register::A], expected);
        }

        #[test]
        fn add_the_register_to_itself() {
            let mut p = Processor::new().unwrap();
            p.registers[Register::A] = 2;
            let expected = 4;

            p.execute_add_half(Register::A, Operand::Register(Register::A));

            assert!(!p.flags.overflow);
            assert!(!p.flags.zero);
            assert!(!p.flags.sign);
            assert_eq!(p.registers[Register::A], expected);
        }
    }

    mod word {
        use crate::{constant::Word, operand::Operand, register::Register, Processor};

        #[test]
        fn add_causes_overflow() {
            let mut p = Processor::new().unwrap();
            p.registers[Register::A] = Word::MAX;

            p.execute_add_word(Register::A, Operand::Immediate(1));

            assert!(p.flags.overflow);
            assert!(p.flags.zero);
            assert!(!p.flags.sign);
        }

        #[test]
        fn add_does_not_cause_overflow() {
            let mut p = Processor::new().unwrap();
            p.registers[Register::A] = Word::MAX - 1;

            p.execute_add_word(Register::A, Operand::Immediate(1));

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

            p.execute_add_word(Register::A, Operand::Register(Register::B));

            assert!(!p.flags.overflow);
            assert!(!p.flags.zero);
            assert!(!p.flags.sign);
            assert_eq!(p.registers[Register::A], expected);
        }

        #[test]
        fn add_the_register_to_itself() {
            let mut p = Processor::new().unwrap();
            p.registers[Register::A] = 2;
            let expected = 4;

            p.execute_add_word(Register::A, Operand::Register(Register::A));

            assert!(!p.flags.overflow);
            assert!(!p.flags.zero);
            assert!(!p.flags.sign);
            assert_eq!(p.registers[Register::A], expected);
        }
    }
}

#[cfg(test)]
mod sub {
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

            p.execute_sub_byte(Register::A, Operand::Immediate(1));

            assert!(p.flags.overflow);
            assert!(!p.flags.zero);
            assert!(p.flags.sign);
        }

        #[test]
        fn sub_does_not_cause_overflow() {
            let mut p = Processor::new().unwrap();
            p.registers[Register::A] = Byte::MAX as Word;

            p.execute_sub_byte(Register::A, Operand::Immediate(1));

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

            p.execute_sub_byte(Register::A, Operand::Register(Register::B));

            assert!(!p.flags.overflow);
            assert!(p.flags.zero);
            assert!(!p.flags.sign);
            assert_eq!(p.registers[Register::A], expected);
        }

        #[test]
        fn sub_the_register_to_itself() {
            let mut p = Processor::new().unwrap();
            p.registers[Register::A] = 2;
            let expected = 0;

            p.execute_sub_byte(Register::A, Operand::Register(Register::A));

            assert!(!p.flags.overflow);
            assert!(p.flags.zero);
            assert!(!p.flags.sign);
            assert_eq!(p.registers[Register::A], expected);
        }
    }

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

            p.execute_sub_quarter(Register::A, Operand::Immediate(1));

            assert!(p.flags.overflow);
            assert!(!p.flags.zero);
            assert!(p.flags.sign);
        }

        #[test]
        fn sub_does_not_cause_overflow() {
            let mut p = Processor::new().unwrap();
            p.registers[Register::A] = Quarter::MAX as Word;

            p.execute_sub_quarter(Register::A, Operand::Immediate(1));

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

            p.execute_sub_quarter(Register::A, Operand::Register(Register::B));

            assert!(!p.flags.overflow);
            assert!(p.flags.zero);
            assert!(!p.flags.sign);
            assert_eq!(p.registers[Register::A], expected);
        }

        #[test]
        fn sub_the_register_to_itself() {
            let mut p = Processor::new().unwrap();
            p.registers[Register::A] = 2;
            let expected = 0;

            p.execute_sub_quarter(Register::A, Operand::Register(Register::A));

            assert!(!p.flags.overflow);
            assert!(p.flags.zero);
            assert!(!p.flags.sign);
            assert_eq!(p.registers[Register::A], expected);
        }
    }

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

            p.execute_sub_half(Register::A, Operand::Immediate(1));

            assert!(p.flags.overflow);
            assert!(!p.flags.zero);
            assert!(p.flags.sign);
        }

        #[test]
        fn sub_does_not_cause_overflow() {
            let mut p = Processor::new().unwrap();
            p.registers[Register::A] = Half::MAX as Word;

            p.execute_sub_half(Register::A, Operand::Immediate(1));

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

            p.execute_sub_half(Register::A, Operand::Register(Register::B));

            assert!(!p.flags.overflow);
            assert!(p.flags.zero);
            assert!(!p.flags.sign);
            assert_eq!(p.registers[Register::A], expected);
        }

        #[test]
        fn sub_the_register_to_itself() {
            let mut p = Processor::new().unwrap();
            p.registers[Register::A] = 2;
            let expected = 0;

            p.execute_sub_half(Register::A, Operand::Register(Register::A));

            assert!(!p.flags.overflow);
            assert!(p.flags.zero);
            assert!(!p.flags.sign);
            assert_eq!(p.registers[Register::A], expected);
        }
    }

    mod word {
        use crate::{constant::Word, operand::Operand, register::Register, Processor};

        #[test]
        fn sub_causes_overflow() {
            let mut p = Processor::new().unwrap();
            p.registers[Register::A] = Word::MIN;

            p.execute_sub_word(Register::A, Operand::Immediate(1));

            assert!(p.flags.overflow);
            assert!(!p.flags.zero);
            assert!(p.flags.sign);
        }

        #[test]
        fn sub_does_not_cause_overflow() {
            let mut p = Processor::new().unwrap();
            p.registers[Register::A] = Word::MAX;

            p.execute_sub_word(Register::A, Operand::Immediate(1));

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

            p.execute_sub_word(Register::A, Operand::Register(Register::B));

            assert!(!p.flags.overflow);
            assert!(p.flags.zero);
            assert!(!p.flags.sign);
            assert_eq!(p.registers[Register::A], expected);
        }

        #[test]
        fn sub_the_register_to_itself() {
            let mut p = Processor::new().unwrap();
            p.registers[Register::A] = 2;
            let expected = 0;

            p.execute_sub_word(Register::A, Operand::Register(Register::A));

            assert!(!p.flags.overflow);
            assert!(p.flags.zero);
            assert!(!p.flags.sign);
            assert_eq!(p.registers[Register::A], expected);
        }
    }
}

#[cfg(test)]
mod mul {
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

            p.execute_mul_byte(Register::A, Operand::Immediate(2));

            assert!(p.flags.overflow);
            assert!(p.flags.zero);
            assert!(!p.flags.sign);
        }

        #[test]
        fn mul_does_not_cause_overflow() {
            let mut p = Processor::new().unwrap();
            p.registers[Register::A] = Byte::MAX as Word / 2;

            p.execute_mul_byte(Register::A, Operand::Immediate(2));

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

            p.execute_mul_byte(Register::A, Operand::Register(Register::B));

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

            p.execute_mul_byte(Register::A, Operand::Register(Register::A));

            assert!(!p.flags.overflow);
            assert!(!p.flags.zero);
            assert!(!p.flags.sign);
            assert_eq!(p.registers[Register::A], expected);
        }
    }

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

            p.execute_mul_quarter(Register::A, Operand::Immediate(2));

            assert!(p.flags.overflow);
            assert!(p.flags.zero);
            assert!(!p.flags.sign);
        }

        #[test]
        fn mul_does_not_cause_overflow() {
            let mut p = Processor::new().unwrap();
            p.registers[Register::A] = Quarter::MAX as Word / 2;

            p.execute_mul_quarter(Register::A, Operand::Immediate(2));

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

            p.execute_mul_quarter(Register::A, Operand::Register(Register::B));

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

            p.execute_mul_quarter(Register::A, Operand::Register(Register::A));

            assert!(!p.flags.overflow);
            assert!(!p.flags.zero);
            assert!(!p.flags.sign);
            assert_eq!(p.registers[Register::A], expected);
        }
    }

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

            p.execute_mul_half(Register::A, Operand::Immediate(2));

            assert!(p.flags.overflow);
            assert!(p.flags.zero);
            assert!(!p.flags.sign);
        }

        #[test]
        fn mul_does_not_cause_overflow() {
            let mut p = Processor::new().unwrap();
            p.registers[Register::A] = Half::MAX as Word / 2;

            p.execute_mul_half(Register::A, Operand::Immediate(2));

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

            p.execute_mul_half(Register::A, Operand::Register(Register::B));

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

            p.execute_mul_half(Register::A, Operand::Register(Register::A));

            assert!(!p.flags.overflow);
            assert!(!p.flags.zero);
            assert!(!p.flags.sign);
            assert_eq!(p.registers[Register::A], expected);
        }
    }

    mod word {
        use crate::{constant::Word, operand::Operand, register::Register, Processor};

        #[test]
        fn mul_causes_overflow() {
            let mut p = Processor::new().unwrap();
            p.registers[Register::A] = (Word::MAX / 2) + 1;

            p.execute_mul_word(Register::A, Operand::Immediate(2));

            assert!(p.flags.overflow);
            assert!(p.flags.zero);
            assert!(!p.flags.sign);
        }

        #[test]
        fn mul_does_not_cause_overflow() {
            let mut p = Processor::new().unwrap();
            p.registers[Register::A] = Word::MAX / 2;

            p.execute_mul_word(Register::A, Operand::Immediate(2));

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

            p.execute_mul_word(Register::A, Operand::Register(Register::B));

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

            p.execute_mul_word(Register::A, Operand::Register(Register::A));

            assert!(!p.flags.overflow);
            assert!(!p.flags.zero);
            assert!(!p.flags.sign);
            assert_eq!(p.registers[Register::A], expected);
        }
    }
}
