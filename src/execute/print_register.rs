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

#[cfg(test)]
mod byte {
    use crate::{
        constant::{Byte, Word},
        error::ExecuteError,
        register::Register,
        Processor,
    };

    #[test]
    fn print() -> Result<(), ExecuteError> {
        let mut p = Processor::new_test();
        p.registers[Register::A] = Byte::MAX as Word;
        let expected = format!("{}: {}\n", Register::A, Byte::MAX);
        p.print_register_value::<Byte>(Register::A)?;

        let actual = p.output.get_buffer().unwrap();
        assert_eq!(actual, expected);

        Ok(())
    }
}

#[cfg(test)]
mod quarter {
    use crate::{
        constant::{Quarter, Word},
        error::ExecuteError,
        register::Register,
        Processor,
    };

    #[test]
    fn print() -> Result<(), ExecuteError> {
        let mut p = Processor::new_test();
        p.registers[Register::A] = Quarter::MAX as Word;
        let expected = format!("{}: {}\n", Register::A, Quarter::MAX);

        p.print_register_value::<Quarter>(Register::A)?;

        let actual = p.output.get_buffer().unwrap();
        assert_eq!(actual, expected);

        Ok(())
    }
}

#[cfg(test)]
mod half {
    use crate::{
        constant::{Half, Word},
        error::ExecuteError,
        register::Register,
        Processor,
    };

    #[test]
    fn print() -> Result<(), ExecuteError> {
        let mut p = Processor::new_test();
        p.registers[Register::A] = Half::MAX as Word;
        let expected = format!("{}: {}\n", Register::A, Half::MAX);

        p.print_register_value::<Half>(Register::A)?;

        let actual = p.output.get_buffer().unwrap();
        assert_eq!(actual, expected);

        Ok(())
    }
}

#[cfg(test)]
mod word {
    use crate::{constant::Word, error::ExecuteError, register::Register, Processor};

    #[test]
    fn print() -> Result<(), ExecuteError> {
        let mut p = Processor::new_test();
        p.registers[Register::A] = Word::MAX;
        let expected = format!("{}: {}\n", Register::A, Word::MAX);

        p.print_register_value::<Word>(Register::A)?;

        let actual = p.output.get_buffer().unwrap();
        assert_eq!(actual, expected);

        Ok(())
    }
}
