use crate::{
    constant::{Byte, Half, Quarter, Word},
    error::ExecuteError,
    instruction::PrintRegister,
    register::Register,
    registers::RegisterOperations,
    utils::FromBytes,
    Processor,
};
use std::fmt::Display;

impl Processor {
    pub fn print_register(&mut self, instruction: PrintRegister) -> Result<(), ExecuteError> {
        match instruction {
            PrintRegister::Byte(r) => self.print_register_value::<Byte>(r)?,
            PrintRegister::Quarter(r) => self.print_register_value::<Quarter>(r)?,
            PrintRegister::Half(r) => self.print_register_value::<Half>(r)?,
            PrintRegister::Word(r) => self.print_register_value::<Word>(r)?,
        }

        Ok(())
    }

    fn print_register_value<T>(&mut self, register: Register) -> Result<(), ExecuteError>
    where
        T: Display + FromBytes,
    {
        let value = self.registers.get::<T>(register);
        writeln!(self.output, "{}: {}", register, value)
            .map_err(|err| ExecuteError::IO(err.to_string()))
    }
}
