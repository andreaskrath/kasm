use cli::Configuration;
use constant::{Word, STACK_SIZE};
use error::InterpreterError;
use flags::Flags;
use register::Register;
use registers::Registers;

pub use cli::Arguments;
use stack::Stack;

mod cli;
mod constant;
mod decode;
mod error;
mod execute;
mod flags;
mod instruction;
mod operand;
mod register;
mod registers;
mod stack;
mod utils;

pub struct Interpreter {
    registers: Registers,
    program_counter: Word,
    flags: Flags,
    running: bool,
    stack: Stack,
    config: Configuration,
}

impl Interpreter {
    pub fn try_new(args: Arguments) -> Result<Self, InterpreterError> {
        let p = Self {
            registers: [0; Register::VARIANT_COUNT],
            program_counter: 0,
            flags: Flags::new(),
            running: true,
            stack: Stack::new(STACK_SIZE),
            config: Configuration::try_from(args)?,
        };
        Ok(p)
    }

    #[cfg(test)]
    pub fn new_test() -> Self {
        use constant::TEST_STACK_SIZE;

        Self {
            registers: [0; Register::VARIANT_COUNT],
            program_counter: 0,
            flags: Flags::new(),
            running: true,
            stack: Stack::new(TEST_STACK_SIZE),
            config: Configuration::new_test(),
        }
    }

    /// Gets the program counter as a usize.
    fn pc(&self) -> usize {
        self.program_counter as usize
    }

    pub fn run(&mut self, program: &str) -> Result<(), InterpreterError> {
        let program: Vec<&str> = program.lines().collect();

        while self.running {
            let code = program
                .get(self.pc())
                .ok_or(InterpreterError::InvalidProgramCounter(self.pc()))?;

            let instruction = self
                .decode(code)
                .map_err(|e| InterpreterError::Decode(self.pc(), e))?;

            self.program_counter += 1;

            self.execute(instruction)
                .map_err(|e| InterpreterError::Execute(self.pc(), e))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod integration {
    use crate::{
        constant::{Byte, Word},
        error::InterpreterError,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn min_sub_max_compare_edge_case_jump_if_lesser() -> Result<(), InterpreterError> {
        let mut i = Interpreter::new_test();
        let program = [
            format!("cmpb 0 {}", Byte::MAX).as_ref(), // this is 1 after wrapping around
            "jil 3",
            "setb ra 10",
            "stop",
        ]
        .join("\n");

        i.run(&program)?;

        assert_eq!(i.registers.get::<Byte>(Register::A), 0);
        assert!(i.flags.overflow);
        assert!(!i.flags.zero);

        Ok(())
    }
}
