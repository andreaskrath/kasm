use crate::{
    constant::{
        Byte, Half, Quarter, Word, BYTE, BYTES_IN_HALF, BYTES_IN_QUARTER, BYTES_IN_WORD, STACK_SIZE,
    },
    error::ExecuteError,
    instruction::Push,
    operand::Operand,
    Processor,
};

impl Processor {
    pub fn push(&mut self, instruction: Push) -> Result<(), ExecuteError> {
        match instruction {
            Push::Byte(operand) => self.push_byte(operand)?,
            Push::Quarter(operand) => self.push_quarter(operand)?,
            Push::Half(operand) => self.push_half(operand)?,
            Push::Word(operand) => self.push_word(operand)?,
        }

        Ok(())
    }

    fn push_byte(&mut self, operand: Operand<Byte>) -> Result<(), ExecuteError> {
        if self.sp() + BYTE > STACK_SIZE {
            return Err(ExecuteError::StackOverflow);
        }

        let value = self.get_byte_operand_val(operand);
        self.stack[self.sp()] = value;
        self.stack_pointer += BYTE as Word;

        Ok(())
    }

    fn push_quarter(&mut self, operand: Operand<Quarter>) -> Result<(), ExecuteError> {
        if self.sp() + BYTES_IN_QUARTER > STACK_SIZE {
            return Err(ExecuteError::StackOverflow);
        }

        let value = self.get_quarter_operand_val(operand);
        for (index, byte) in value.to_be_bytes().into_iter().enumerate() {
            self.stack[self.sp() + index] = byte;
        }
        self.stack_pointer += BYTES_IN_QUARTER as Word;

        Ok(())
    }

    fn push_half(&mut self, operand: Operand<Half>) -> Result<(), ExecuteError> {
        if self.sp() + BYTES_IN_HALF > STACK_SIZE {
            return Err(ExecuteError::StackOverflow);
        }

        let value = self.get_half_operand_val(operand);
        for (index, byte) in value.to_be_bytes().into_iter().enumerate() {
            self.stack[self.sp() + index] = byte;
        }
        self.stack_pointer += BYTES_IN_HALF as Word;

        Ok(())
    }

    fn push_word(&mut self, operand: Operand<Word>) -> Result<(), ExecuteError> {
        if self.sp() + BYTES_IN_WORD > STACK_SIZE {
            return Err(ExecuteError::StackOverflow);
        }

        let value = self.get_word_operand_val(operand);
        for (index, byte) in value.to_be_bytes().into_iter().enumerate() {
            self.stack[self.sp() + index] = byte;
        }
        self.stack_pointer += BYTES_IN_WORD as Word;

        Ok(())
    }
}
