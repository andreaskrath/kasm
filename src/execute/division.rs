use crate::{
    constant::{Byte, Half, Quarter, Word},
    error::ExecuteError,
    instruction::Division,
    operand::Operand,
    register::Register,
    registers::RegisterOperations,
    Processor,
};

impl Processor {
    pub fn div(&mut self, instruction: Division) -> Result<(), ExecuteError> {
        match instruction {
            Division::Byte(r, o) => self.div_byte(r, o),
            Division::Quarter(r, o) => self.div_quarter(r, o),
            Division::Half(r, o) => self.div_half(r, o),
            Division::Word(r, o) => self.div_word(r, o),
        }
    }

    fn div_byte(&mut self, register: Register, operand: Operand<Byte>) -> Result<(), ExecuteError> {
        let a = self.registers.get_reg_val_as_byte(register);
        let b = self.get_byte_operand_val(operand);

        if b == 0 {
            return Err(ExecuteError::DivideByZero);
        }

        let (result, overflow) = a.overflowing_div(b);
        self.flags.set_from_byte(result, overflow);
        self.registers[register] = result as Word;

        Ok(())
    }

    fn div_quarter(
        &mut self,
        register: Register,
        operand: Operand<Quarter>,
    ) -> Result<(), ExecuteError> {
        let a = self.registers.get_reg_val_as_quarter(register);
        let b = self.get_quarter_operand_val(operand);

        if b == 0 {
            return Err(ExecuteError::DivideByZero);
        }

        let (result, overflow) = a.overflowing_div(b);
        self.flags.set_from_quarter(result, overflow);
        self.registers[register] = result as Word;

        Ok(())
    }

    fn div_half(&mut self, register: Register, operand: Operand<Half>) -> Result<(), ExecuteError> {
        let a = self.registers.get_reg_val_as_half(register);
        let b = self.get_half_operand_val(operand);

        if b == 0 {
            return Err(ExecuteError::DivideByZero);
        }

        let (result, overflow) = a.overflowing_div(b);
        self.flags.set_from_half(result, overflow);
        self.registers[register] = result as Word;

        Ok(())
    }

    fn div_word(&mut self, register: Register, operand: Operand<Word>) -> Result<(), ExecuteError> {
        let a = self.registers.get_reg_val(register);
        let b = self.get_word_operand_val(operand);

        if b == 0 {
            return Err(ExecuteError::DivideByZero);
        }

        let (result, overflow) = a.overflowing_div(b);
        self.flags.set_from_word(result, overflow);
        self.registers[register] = result;

        Ok(())
    }
}
