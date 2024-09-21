use crate::{
    constant::{Byte, Half, Quarter, Word},
    error::ExecuteError,
    instruction::PrintStack,
    operand::Operand,
    utils::FromBytes,
    Interpreter,
};
use std::{char, fmt::Debug};

impl Interpreter {
    pub(super) fn print_stack(&mut self, instruction: PrintStack) -> Result<(), ExecuteError> {
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
        let amount = self.get_operand_value(operand);
        let values = self.stack.slice::<T>(amount)?;

        writeln!(self.config.output, "{values:?}").map_err(|err| ExecuteError::IO(err.to_string()))
    }

    fn print_stack_str(&mut self, operand: Operand<Word>) -> Result<(), ExecuteError> {
        let amount = self.get_operand_value(operand);
        let chars = self.stack.slice::<Byte>(amount)?;
        let str: String = chars.iter().map(|b| char::from(*b)).collect();

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
        registers::RegisterOperations,
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
        i.stack.push(Byte::MAX)?;
        i.stack.push(Byte::MAX)?;
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
        i.stack.push(Byte::MAX)?;
        i.stack.push(Byte::MAX)?;
        i.registers.set(Register::A, 2);
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
        constant::Quarter,
        error::ExecuteError,
        instruction::{Instruction, PrintStack},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
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
        i.stack.push(Quarter::MAX)?;
        i.stack.push(Quarter::MAX)?;
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
        i.stack.push(Quarter::MAX)?;
        i.stack.push(Quarter::MAX)?;
        i.registers.set(Register::A, 2);
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
        constant::Half,
        error::ExecuteError,
        instruction::{Instruction, PrintStack},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
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
        i.stack.push(Half::MAX)?;
        i.stack.push(Half::MAX)?;
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
        i.stack.push(Half::MAX)?;
        i.stack.push(Half::MAX)?;
        i.registers.set(Register::A, 2);
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
        constant::Word,
        error::ExecuteError,
        instruction::{Instruction, PrintStack},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
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
        i.stack.push(Word::MAX)?;
        i.stack.push(Word::MAX)?;
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
        i.stack.push(Word::MAX)?;
        i.stack.push(Word::MAX)?;
        i.registers.set(Register::A, 2);
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
        constant::Byte,
        error::ExecuteError,
        instruction::{Instruction, PrintStack},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
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
        i.stack.push::<Byte>(b'H')?;
        i.stack.push::<Byte>(b'e')?;
        i.stack.push::<Byte>(b'l')?;
        i.stack.push::<Byte>(b'l')?;
        i.stack.push::<Byte>(b'o')?;
        i.stack.push::<Byte>(b',')?;
        i.stack.push::<Byte>(b' ')?;
        i.stack.push::<Byte>(b'w')?;
        i.stack.push::<Byte>(b'o')?;
        i.stack.push::<Byte>(b'r')?;
        i.stack.push::<Byte>(b'l')?;
        i.stack.push::<Byte>(b'd')?;
        i.stack.push::<Byte>(b'!')?;
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
        i.stack.push::<Byte>(b'H')?;
        i.stack.push::<Byte>(b'e')?;
        i.stack.push::<Byte>(b'l')?;
        i.stack.push::<Byte>(b'l')?;
        i.stack.push::<Byte>(b'o')?;
        i.stack.push::<Byte>(b',')?;
        i.stack.push::<Byte>(b' ')?;
        i.stack.push::<Byte>(b'w')?;
        i.stack.push::<Byte>(b'o')?;
        i.stack.push::<Byte>(b'r')?;
        i.stack.push::<Byte>(b'l')?;
        i.stack.push::<Byte>(b'd')?;
        i.stack.push::<Byte>(b'!')?;
        i.registers.set(Register::A, 13);
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
        i.stack.push::<Byte>(8)?;

        // expectation is that the character is escaped
        let expected = "\u{8}\n";

        i.execute(instruction)?;
        let actual = i.config.output.get_buffer().unwrap();

        assert_eq!(actual, expected);

        Ok(())
    }
}
