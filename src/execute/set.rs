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
