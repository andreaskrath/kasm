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
}
