use crate::{
    constant::{Byte, Half, Quarter, Word},
    error::ExecuteError,
    instruction::PrintStack,
    operand::Operand,
    utils::FromBytes,
    Interpreter,
};
use std::fmt::Debug;

impl Interpreter {
    pub fn print_stack(&mut self, instruction: PrintStack) -> Result<(), ExecuteError> {
        match instruction {
            PrintStack::Byte(o) => self.print_stack_value::<Byte>(o)?,
            PrintStack::Quarter(o) => self.print_stack_value::<Quarter>(o)?,
            PrintStack::Half(o) => self.print_stack_value::<Half>(o)?,
            PrintStack::Word(o) => self.print_stack_value::<Word>(o)?,
            PrintStack::Str(o) => self.print_stack_str(o)?,
        }

        Ok(())
    }

    fn print_stack_value<T>(&mut self, operand: Operand<Word>) -> Result<(), ExecuteError>
    where
        T: Debug + FromBytes,
    {
        let amount = self.get_operand_value(operand) as usize;
        let index_offset = amount * size_of::<T>();
        let lower_bound = self
            .sp()
            .checked_sub(index_offset)
            .ok_or(ExecuteError::StackUnderflow)?;

        let slice = &self.stack[lower_bound..self.sp()];
        let values: Vec<T> = slice
            .chunks_exact(size_of::<T>())
            .map(|chunk| T::from_bytes(chunk))
            .collect();

        writeln!(self.config.output, "{:?}", values.as_slice())
            .map_err(|err| ExecuteError::IO(err.to_string()))
    }

    fn print_stack_str(&mut self, operand: Operand<Word>) -> Result<(), ExecuteError> {
        let index_offset = self.get_operand_value(operand) as usize;
        let lower_bound = self
            .sp()
            .checked_sub(index_offset)
            .ok_or(ExecuteError::StackUnderflow)?;

        let slice = &self.stack[lower_bound..self.sp()];
        let str: String = slice.iter().map(|b| *b as char).collect();

        writeln!(self.config.output, "{str}").map_err(|err| ExecuteError::IO(err.to_string()))
    }
}

#[cfg(test)]
mod byte {
    use crate::{
        constant::Byte,
        error::ExecuteError,
        instruction::{Instruction, PrintStack},
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn stack_underflow() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::PrintStack(PrintStack::Byte(Operand::Immediate(1)));
        let expected = Err(ExecuteError::StackUnderflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn print_from_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::PrintStack(PrintStack::Byte(Operand::Immediate(2)));
        i.stack[0] = Byte::MAX;
        i.stack[1] = Byte::MAX;
        i.stack_pointer = 2;
        let expected = format!("{:?}\n", [Byte::MAX, Byte::MAX]);

        i.execute(instruction)?;
        let actual = i.config.output.get_buffer().unwrap();

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]

    fn print_from_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::PrintStack(PrintStack::Byte(Operand::Register(Register::A)));
        i.stack[0] = Byte::MAX;
        i.stack[1] = Byte::MAX;
        i.stack_pointer = 2;
        i.registers[Register::A] = 2;
        let expected = format!("{:?}\n", [Byte::MAX, Byte::MAX]);

        i.execute(instruction)?;
        let actual = i.config.output.get_buffer().unwrap();

        assert_eq!(actual, expected);

        Ok(())
    }
}

#[cfg(test)]
mod quarter {
    use crate::{
        constant::{Byte, Quarter},
        error::ExecuteError,
        instruction::{Instruction, PrintStack},
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn stack_underflow() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::PrintStack(PrintStack::Quarter(Operand::Immediate(1)));
        let expected = Err(ExecuteError::StackUnderflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn print_from_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::PrintStack(PrintStack::Quarter(Operand::Immediate(2)));
        i.stack[0] = Byte::MAX;
        i.stack[1] = Byte::MAX;
        i.stack[2] = Byte::MAX;
        i.stack[3] = Byte::MAX;
        i.stack_pointer = 4;
        let expected = format!("{:?}\n", [Quarter::MAX, Quarter::MAX]);

        i.execute(instruction)?;
        let actual = i.config.output.get_buffer().unwrap();

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn print_from_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::PrintStack(PrintStack::Quarter(Operand::Register(Register::A)));
        i.stack[0] = Byte::MAX;
        i.stack[1] = Byte::MAX;
        i.stack[2] = Byte::MAX;
        i.stack[3] = Byte::MAX;
        i.stack_pointer = 4;
        i.registers[Register::A] = 2;
        let expected = format!("{:?}\n", [Quarter::MAX, Quarter::MAX]);

        i.execute(instruction)?;
        let actual = i.config.output.get_buffer().unwrap();

        assert_eq!(actual, expected);

        Ok(())
    }
}

#[cfg(test)]
mod half {
    use crate::{
        constant::{Byte, Half},
        error::ExecuteError,
        instruction::{Instruction, PrintStack},
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn stack_underflow() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::PrintStack(PrintStack::Half(Operand::Immediate(1)));
        let expected = Err(ExecuteError::StackUnderflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn print_from_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::PrintStack(PrintStack::Half(Operand::Immediate(2)));
        i.stack[0] = Byte::MAX;
        i.stack[1] = Byte::MAX;
        i.stack[2] = Byte::MAX;
        i.stack[3] = Byte::MAX;
        i.stack[4] = Byte::MAX;
        i.stack[5] = Byte::MAX;
        i.stack[6] = Byte::MAX;
        i.stack[7] = Byte::MAX;
        i.stack_pointer = 8;
        let expected = format!("{:?}\n", [Half::MAX, Half::MAX]);

        i.execute(instruction)?;
        let actual = i.config.output.get_buffer().unwrap();

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn print_from_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::PrintStack(PrintStack::Half(Operand::Register(Register::A)));
        i.stack[0] = Byte::MAX;
        i.stack[1] = Byte::MAX;
        i.stack[2] = Byte::MAX;
        i.stack[3] = Byte::MAX;
        i.stack[4] = Byte::MAX;
        i.stack[5] = Byte::MAX;
        i.stack[6] = Byte::MAX;
        i.stack[7] = Byte::MAX;
        i.stack_pointer = 8;
        i.registers[Register::A] = 2;
        let expected = format!("{:?}\n", [Half::MAX, Half::MAX]);

        i.execute(instruction)?;
        let actual = i.config.output.get_buffer().unwrap();

        assert_eq!(actual, expected);

        Ok(())
    }
}

#[cfg(test)]
mod word {
    use crate::{
        constant::{Byte, Word},
        error::ExecuteError,
        instruction::{Instruction, PrintStack},
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn stack_underflow() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::PrintStack(PrintStack::Word(Operand::Immediate(1)));
        let expected = Err(ExecuteError::StackUnderflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn print_from_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::PrintStack(PrintStack::Word(Operand::Immediate(2)));
        i.stack[0] = Byte::MAX;
        i.stack[1] = Byte::MAX;
        i.stack[2] = Byte::MAX;
        i.stack[3] = Byte::MAX;
        i.stack[4] = Byte::MAX;
        i.stack[5] = Byte::MAX;
        i.stack[6] = Byte::MAX;
        i.stack[7] = Byte::MAX;
        i.stack[8] = Byte::MAX;
        i.stack[9] = Byte::MAX;
        i.stack[10] = Byte::MAX;
        i.stack[11] = Byte::MAX;
        i.stack[12] = Byte::MAX;
        i.stack[13] = Byte::MAX;
        i.stack[14] = Byte::MAX;
        i.stack[15] = Byte::MAX;
        i.stack_pointer = 16;
        let expected = format!("{:?}\n", [Word::MAX, Word::MAX]);

        i.execute(instruction)?;
        let actual = i.config.output.get_buffer().unwrap();

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn print_from_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::PrintStack(PrintStack::Word(Operand::Register(Register::A)));
        i.stack[0] = Byte::MAX;
        i.stack[1] = Byte::MAX;
        i.stack[2] = Byte::MAX;
        i.stack[3] = Byte::MAX;
        i.stack[4] = Byte::MAX;
        i.stack[5] = Byte::MAX;
        i.stack[6] = Byte::MAX;
        i.stack[7] = Byte::MAX;
        i.stack[8] = Byte::MAX;
        i.stack[9] = Byte::MAX;
        i.stack[10] = Byte::MAX;
        i.stack[11] = Byte::MAX;
        i.stack[12] = Byte::MAX;
        i.stack[13] = Byte::MAX;
        i.stack[14] = Byte::MAX;
        i.stack[15] = Byte::MAX;
        i.stack_pointer = 16;
        i.registers[Register::A] = 2;
        let expected = format!("{:?}\n", [Word::MAX, Word::MAX]);

        i.execute(instruction)?;
        let actual = i.config.output.get_buffer().unwrap();

        assert_eq!(actual, expected);

        Ok(())
    }
}

#[cfg(test)]
mod print_stack_str {
    use crate::{
        error::ExecuteError,
        instruction::{Instruction, PrintStack},
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn stack_underflow() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::PrintStack(PrintStack::Str(Operand::Immediate(1)));
        let expected = Err(ExecuteError::StackUnderflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn print_from_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::PrintStack(PrintStack::Str(Operand::Immediate(13)));
        i.stack[0] = b'H';
        i.stack[1] = b'e';
        i.stack[2] = b'l';
        i.stack[3] = b'l';
        i.stack[4] = b'o';
        i.stack[5] = b',';
        i.stack[6] = b' ';
        i.stack[7] = b'w';
        i.stack[8] = b'o';
        i.stack[9] = b'r';
        i.stack[10] = b'l';
        i.stack[11] = b'd';
        i.stack[12] = b'!';
        i.stack_pointer = 13;
        let expected = "Hello, world!\n";

        i.execute(instruction)?;
        let actual = i.config.output.get_buffer().unwrap();

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn print_from_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::PrintStack(PrintStack::Str(Operand::Register(Register::A)));
        i.stack[0] = b'H';
        i.stack[1] = b'e';
        i.stack[2] = b'l';
        i.stack[3] = b'l';
        i.stack[4] = b'o';
        i.stack[5] = b',';
        i.stack[6] = b' ';
        i.stack[7] = b'w';
        i.stack[8] = b'o';
        i.stack[9] = b'r';
        i.stack[10] = b'l';
        i.stack[11] = b'd';
        i.stack[12] = b'!';
        i.stack_pointer = 13;
        i.registers[Register::A] = 13;
        let expected = "Hello, world!\n";

        i.execute(instruction)?;
        let actual = i.config.output.get_buffer().unwrap();

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn non_printable_ascii_character() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::PrintStack(PrintStack::Str(Operand::Immediate(1)));
        // backspace character
        i.stack[0] = 8;
        i.stack_pointer = 1;

        // expectation is that the character is escaped
        let expected = "\u{8}\n";

        i.execute(instruction)?;
        let actual = i.config.output.get_buffer().unwrap();

        assert_eq!(actual, expected);

        Ok(())
    }
}
