use crate::{
    constant::{Byte, Half, Quarter, Word},
    error::ExecuteError,
    instruction::Instruction,
    operand::Operand,
    registers::RegisterOperations,
    Processor,
};

mod addition;
mod division;
mod multiplication;
mod pop;
mod push;
mod remainder;
mod set;
mod subtraction;

impl Processor {
    fn get_byte_operand_val(&self, operand: Operand<Byte>) -> Byte {
        match operand {
            Operand::Register(register) => self.registers.get::<Byte>(register),
            Operand::Immediate(immediate) => immediate,
        }
    }

    fn get_quarter_operand_val(&self, operand: Operand<Quarter>) -> Quarter {
        match operand {
            Operand::Register(register) => self.registers.get::<Quarter>(register),
            Operand::Immediate(immediate) => immediate,
        }
    }

    fn get_half_operand_val(&self, operand: Operand<Half>) -> Half {
        match operand {
            Operand::Register(register) => self.registers.get::<Half>(register),
            Operand::Immediate(immediate) => immediate,
        }
    }

    fn get_word_operand_val(&self, operand: Operand<Word>) -> Word {
        match operand {
            Operand::Register(register) => self.registers.get::<Word>(register),
            Operand::Immediate(immediate) => immediate,
        }
    }

    pub fn execute(&mut self, instruction: Instruction) -> Result<(), ExecuteError> {
        use Instruction::*;

        match instruction {
            Stop => self.stop(),
            Set(instruction) => self.set(instruction),
            Addition(instruction) => self.add(instruction),
            Subtraction(instruction) => self.sub(instruction),
            Multiplication(instruction) => self.mul(instruction),
            Division(instruction) => self.div(instruction)?,
            Remainder(instruction) => self.rem(instruction)?,
            Push(instruction) => self.push(instruction)?,
            Pop(instruction) => self.pop(instruction)?,
        }

        Ok(())
    }

    fn stop(&mut self) {
        self.running = false;
    }
}
