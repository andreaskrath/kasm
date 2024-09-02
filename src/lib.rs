use cli::Configuration;
use constant::{Word, STACK_SIZE};
use error::InterpreterError;
use flags::Flags;
use register::Register;
use registers::Registers;

pub use cli::Arguments;

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
mod utils;

pub struct Interpreter {
    registers: Registers,
    stack_pointer: Word,
    program_counter: Word,
    flags: Flags,
    running: bool,
    stack: Box<[u8]>,
    config: Configuration,
}

impl Interpreter {
    pub fn try_new(args: Arguments) -> Result<Self, InterpreterError> {
        // Kind of a hack, but simply allocating an array inside a box causes a stack overflow.
        // https://github.com/rust-lang/rust/issues/53827
        let stack = vec![0; STACK_SIZE].into_boxed_slice();

        let p = Self {
            registers: [0; Register::VARIANT_COUNT],
            stack_pointer: 0,
            program_counter: 0,
            flags: Flags::new(),
            running: true,
            stack,
            config: Configuration::try_from(args)?,
        };
        Ok(p)
    }

    #[cfg(test)]
    pub fn new_test() -> Self {
        use constant::TEST_STACK_SIZE;

        let stack = vec![0; TEST_STACK_SIZE].into_boxed_slice();

        Self {
            registers: [0; Register::VARIANT_COUNT],
            stack_pointer: 0,
            program_counter: 0,
            flags: Flags::new(),
            running: true,
            stack,
            config: Configuration::new_test(),
        }
    }

    /// Gets the program counter as a usize.
    fn pc(&self) -> usize {
        self.program_counter as usize
    }

    /// Gets the stack pointer as a usize.
    fn sp(&self) -> usize {
        self.stack_pointer as usize
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
