use crate::{
    constant::{Byte, Half, Quarter, Word},
    error::ExecuteError,
    instruction::Remainder,
    operand::Operand,
    register::Register,
    registers::RegisterOperations,
    Processor,
};

impl Processor {
    pub fn rem(&mut self, instruction: Remainder) -> Result<(), ExecuteError> {
        match instruction {
            Remainder::Byte(r, o) => self.rem_byte(r, o),
            Remainder::Quarter(r, o) => self.rem_quarter(r, o),
            Remainder::Half(r, o) => self.rem_half(r, o),
            Remainder::Word(r, o) => self.rem_word(r, o),
        }
    }

    fn rem_byte(&mut self, register: Register, operand: Operand<Byte>) -> Result<(), ExecuteError> {
        let a = self.registers.get_reg_val_as_byte(register);
        let b = self.get_byte_operand_val(operand);

        if b == 0 {
            return Err(ExecuteError::DivideByZero);
        }

        let (result, overflow) = a.overflowing_rem(b);
        self.flags.set_from_byte(result, overflow);
        self.registers[register] = result as Word;

        Ok(())
    }

    fn rem_quarter(
        &mut self,
        register: Register,
        operand: Operand<Quarter>,
    ) -> Result<(), ExecuteError> {
        let a = self.registers.get_reg_val_as_quarter(register);
        let b = self.get_quarter_operand_val(operand);

        if b == 0 {
            return Err(ExecuteError::DivideByZero);
        }

        let (result, overflow) = a.overflowing_rem(b);
        self.flags.set_from_quarter(result, overflow);
        self.registers[register] = result as Word;

        Ok(())
    }

    fn rem_half(&mut self, register: Register, operand: Operand<Half>) -> Result<(), ExecuteError> {
        let a = self.registers.get_reg_val_as_half(register);
        let b = self.get_half_operand_val(operand);

        if b == 0 {
            return Err(ExecuteError::DivideByZero);
        }

        let (result, overflow) = a.overflowing_rem(b);
        self.flags.set_from_half(result, overflow);
        self.registers[register] = result as Word;

        Ok(())
    }

    fn rem_word(&mut self, register: Register, operand: Operand<Word>) -> Result<(), ExecuteError> {
        let a = self.registers.get_reg_val(register);
        let b = self.get_word_operand_val(operand);

        if b == 0 {
            return Err(ExecuteError::DivideByZero);
        }

        let (result, overflow) = a.overflowing_rem(b);
        self.flags.set_from_word(result, overflow);
        self.registers[register] = result;

        Ok(())
    }
}
