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

        writeln!(self.output, "stack: {:?}", values.as_slice())
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

        writeln!(self.output, "{str}").map_err(|err| ExecuteError::IO(err.to_string()))
    }
}

#[cfg(test)]
mod byte {
    use crate::{constant::Byte, error::ExecuteError, operand::Operand, Interpreter};

    #[test]
    fn stack_underflow() {
        let mut p = Interpreter::new_test();
        let expected = Err(ExecuteError::StackUnderflow);

        let actual = p.print_stack_value::<Byte>(Operand::Immediate(1));

        assert_eq!(actual, expected);
    }

    #[test]
    fn print() {
        let mut p = Interpreter::new_test();
        p.stack[0] = Byte::MAX;
        p.stack[1] = Byte::MAX;
        p.stack_pointer = 2;
        let expected = format!("stack: {:?}\n", [Byte::MAX, Byte::MAX]);

        p.print_stack_value::<Byte>(Operand::Immediate(2)).unwrap();
        let actual = p.output.get_buffer().unwrap();

        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod quarter {
    use crate::{
        constant::{Byte, Quarter},
        error::ExecuteError,
        operand::Operand,
        Interpreter,
    };

    #[test]
    fn stack_underflow() {
        let mut p = Interpreter::new_test();
        let expected = Err(ExecuteError::StackUnderflow);

        let actual = p.print_stack_value::<Quarter>(Operand::Immediate(1));

        assert_eq!(actual, expected);
    }

    #[test]
    fn print() {
        let mut p = Interpreter::new_test();
        p.stack[0] = Byte::MAX;
        p.stack[1] = Byte::MAX;
        p.stack[2] = Byte::MAX;
        p.stack[3] = Byte::MAX;
        p.stack_pointer = 4;
        let expected = format!("stack: {:?}\n", [Quarter::MAX, Quarter::MAX]);

        p.print_stack_value::<Quarter>(Operand::Immediate(2))
            .unwrap();
        let actual = p.output.get_buffer().unwrap();

        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod half {
    use crate::{
        constant::{Byte, Half},
        error::ExecuteError,
        operand::Operand,
        Interpreter,
    };

    #[test]
    fn stack_underflow() {
        let mut p = Interpreter::new_test();
        let expected = Err(ExecuteError::StackUnderflow);

        let actual = p.print_stack_value::<Half>(Operand::Immediate(1));

        assert_eq!(actual, expected);
    }

    #[test]
    fn print() {
        let mut p = Interpreter::new_test();
        p.stack[0] = Byte::MAX;
        p.stack[1] = Byte::MAX;
        p.stack[2] = Byte::MAX;
        p.stack[3] = Byte::MAX;
        p.stack[4] = Byte::MAX;
        p.stack[5] = Byte::MAX;
        p.stack[6] = Byte::MAX;
        p.stack[7] = Byte::MAX;
        p.stack_pointer = 8;
        let expected = format!("stack: {:?}\n", [Half::MAX, Half::MAX]);

        p.print_stack_value::<Half>(Operand::Immediate(2)).unwrap();
        let actual = p.output.get_buffer().unwrap();

        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod word {
    use crate::{
        constant::{Byte, Word},
        error::ExecuteError,
        operand::Operand,
        Interpreter,
    };

    #[test]
    fn stack_underflow() {
        let mut p = Interpreter::new_test();
        let expected = Err(ExecuteError::StackUnderflow);

        let actual = p.print_stack_value::<Word>(Operand::Immediate(1));

        assert_eq!(actual, expected);
    }

    #[test]
    fn print() {
        let mut p = Interpreter::new_test();
        p.stack[0] = Byte::MAX;
        p.stack[1] = Byte::MAX;
        p.stack[2] = Byte::MAX;
        p.stack[3] = Byte::MAX;
        p.stack[4] = Byte::MAX;
        p.stack[5] = Byte::MAX;
        p.stack[6] = Byte::MAX;
        p.stack[7] = Byte::MAX;
        p.stack[8] = Byte::MAX;
        p.stack[9] = Byte::MAX;
        p.stack[10] = Byte::MAX;
        p.stack[11] = Byte::MAX;
        p.stack[12] = Byte::MAX;
        p.stack[13] = Byte::MAX;
        p.stack[14] = Byte::MAX;
        p.stack[15] = Byte::MAX;
        p.stack_pointer = 16;
        let expected = format!("stack: {:?}\n", [Word::MAX, Word::MAX]);

        p.print_stack_value::<Word>(Operand::Immediate(2)).unwrap();
        let actual = p.output.get_buffer().unwrap();

        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod print_stack_str {
    use crate::{error::ExecuteError, operand::Operand, Interpreter};

    #[test]
    fn stack_underflow() {
        let mut p = Interpreter::new_test();
        let expected = Err(ExecuteError::StackUnderflow);

        let actual = p.print_stack_str(Operand::Immediate(1));

        assert_eq!(actual, expected);
    }

    #[test]
    fn hello_world() {
        let mut p = Interpreter::new_test();
        p.stack[0] = b'H';
        p.stack[1] = b'e';
        p.stack[2] = b'l';
        p.stack[3] = b'l';
        p.stack[4] = b'o';
        p.stack[5] = b',';
        p.stack[6] = b' ';
        p.stack[7] = b'w';
        p.stack[8] = b'o';
        p.stack[9] = b'r';
        p.stack[10] = b'l';
        p.stack[11] = b'd';
        p.stack[12] = b'!';
        p.stack_pointer = 13;
        let expected = "Hello, world!\n";

        p.print_stack_str(Operand::Immediate(13)).unwrap();
        let actual = p.output.get_buffer().unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn non_printable_ascii_character() {
        let mut p = Interpreter::new_test();
        // backspace character
        p.stack[0] = 8;
        p.stack_pointer = 1;

        // expectation is that the character is escaped
        let expected = "\u{8}\n";

        p.print_stack_str(Operand::Immediate(1)).unwrap();
        let actual = p.output.get_buffer().unwrap();

        assert_eq!(actual, expected);
    }
}
