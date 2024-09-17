use crate::{
    constant::{Byte, Half, Quarter, Word},
    error::ExecuteError,
    instruction::PrintRegister,
    register::Register,
    registers::RegisterOperations,
    utils::FromBytes,
    Interpreter,
};
use std::fmt::Display;

impl Interpreter {
    pub(super) fn print_register(
        &mut self,
        instruction: PrintRegister,
    ) -> Result<(), ExecuteError> {
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
        writeln!(self.config.output, "{register}: {value}")
            .map_err(|err| ExecuteError::IO(err.to_string()))
    }
}

#[cfg(test)]
mod byte {
    use crate::{
        constant::Byte,
        error::ExecuteError,
        instruction::{Instruction, PrintRegister},
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn print() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::PrintRegister(PrintRegister::Byte(Register::A));
        i.registers.set(Register::A, Byte::MAX);
        let expected = format!("{}: {}\n", Register::A, Byte::MAX);
        i.execute(instruction)?;

        let actual = i.config.output.get_buffer().unwrap();
        assert_eq!(actual, expected);

        Ok(())
    }
}

#[cfg(test)]
mod quarter {
    use crate::{
        constant::Quarter,
        error::ExecuteError,
        instruction::{Instruction, PrintRegister},
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn print() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::PrintRegister(PrintRegister::Quarter(Register::A));
        i.registers.set(Register::A, Quarter::MAX);
        let expected = format!("{}: {}\n", Register::A, Quarter::MAX);
        i.execute(instruction)?;

        let actual = i.config.output.get_buffer().unwrap();
        assert_eq!(actual, expected);

        Ok(())
    }
}

#[cfg(test)]
mod half {
    use crate::{
        constant::Half,
        error::ExecuteError,
        instruction::{Instruction, PrintRegister},
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn print() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::PrintRegister(PrintRegister::Half(Register::A));
        i.registers.set(Register::A, Half::MAX);
        let expected = format!("{}: {}\n", Register::A, Half::MAX);
        i.execute(instruction)?;

        let actual = i.config.output.get_buffer().unwrap();
        assert_eq!(actual, expected);

        Ok(())
    }
}

#[cfg(test)]
mod word {
    use crate::{
        constant::Word,
        error::ExecuteError,
        instruction::{Instruction, PrintRegister},
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn print() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::PrintRegister(PrintRegister::Word(Register::A));
        i.registers.set(Register::A, Word::MAX);
        let expected = format!("{}: {}\n", Register::A, Word::MAX);
        i.execute(instruction)?;

        let actual = i.config.output.get_buffer().unwrap();
        assert_eq!(actual, expected);

        Ok(())
    }
}
