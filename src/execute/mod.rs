use crate::{
    error::ExecuteError, instruction::Instruction, operand::Operand, registers::RegisterOperations,
    utils::FromBytes, Processor,
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
    fn get_operand_value<T>(&self, operand: Operand<T>) -> T
    where
        T: FromBytes,
    {
        match operand {
            Operand::Register(register) => self.registers.get::<T>(register),
            Operand::Immediate(value) => value,
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
