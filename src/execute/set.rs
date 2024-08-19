use crate::{
    constant::{Byte, Half, Quarter, Word},
    instruction::Set,
    operand::Operand,
    register::Register,
    Processor,
};

impl Processor {
    pub fn set(&mut self, instruction: Set) {
        match instruction {
            Set::Byte(r, o) => self.set_byte(r, o),
            Set::Quarter(r, o) => self.set_quarter(r, o),
            Set::Half(r, o) => self.set_half(r, o),
            Set::Word(r, o) => self.set_word(r, o),
        }
    }

    fn set_byte(&mut self, register: Register, operand: Operand<Byte>) {
        let value = self.get_byte_operand_val(operand);
        self.registers[register] = value as Word;
    }

    fn set_quarter(&mut self, register: Register, operand: Operand<Quarter>) {
        let value = self.get_quarter_operand_val(operand);
        self.registers[register] = value as Word;
    }

    fn set_half(&mut self, register: Register, operand: Operand<Half>) {
        let value = self.get_half_operand_val(operand);
        self.registers[register] = value as Word;
    }

    fn set_word(&mut self, register: Register, operand: Operand<Word>) {
        let value = self.get_word_operand_val(operand);
        self.registers[register] = value;
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
    fn set_byte() {
        let mut p = Processor::new().unwrap();
        let expected = Byte::MAX;

        p.set_byte(Register::A, Operand::Immediate(Byte::MAX));

        assert_eq!(p.registers[Register::A], expected as Word);
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
    fn set_quarter() {
        let mut p = Processor::new().unwrap();
        let expected = Quarter::MAX;

        p.set_quarter(Register::A, Operand::Immediate(Quarter::MAX));

        assert_eq!(p.registers[Register::A], expected as Word);
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
    fn set_half() {
        let mut p = Processor::new().unwrap();
        let expected = Half::MAX;

        p.set_half(Register::A, Operand::Immediate(Half::MAX));

        assert_eq!(p.registers[Register::A], expected as Word);
    }
}

#[cfg(test)]
mod word {
    use crate::{constant::Word, operand::Operand, register::Register, Processor};

    #[test]
    fn set_word() {
        let mut p = Processor::new().unwrap();
        let expected = Word::MAX;

        p.set_word(Register::A, Operand::Immediate(Word::MAX));

        assert_eq!(p.registers[Register::A], expected);
    }
}
