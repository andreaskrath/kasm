use std::io::{stdout, Write};

use constant::{Word, REGISTER_AMOUNT, STACK_SIZE, WORD_BYTE_SIZE};
use error::{ExecuteError, ProcessorError};
use flags::Flags;
use instruction::{Instruction, Jump};
use operand::Operand;
use register::Register;

mod constant;
mod error;
mod flags;
mod instruction;
mod operand;
mod register;
mod decode;
mod execute;

pub struct Processor {
    registers: Registers,
    stack_pointer: Word,
    program_counter: Word,
    flags: Flags,
    output: Box<dyn Write>,
    stack: Box<[u8; STACK_SIZE]>,
}

impl Processor {
    pub fn new() -> Result<Self, ProcessorError> {
        // Kind of a hack, but simply allocating an array inside a box causes a stack overflow.
        // https://github.com/rust-lang/rust/issues/53827
        let Ok(stack) = vec![0; STACK_SIZE].into_boxed_slice().try_into() else {
            return Err(ProcessorError::FailedStackAllocation);
        };

        let output = Box::new(stdout());

        let p = Self {
            registers: [0; REGISTER_AMOUNT],
            stack_pointer: 0,
            program_counter: 0,
            flags: Flags::new(),
            output,
            stack,
        };
        Ok(p)
    }

    pub fn run(&mut self, program: &str) -> Result<(), ProcessorError> {
        let program: Vec<&str> = program.lines().collect();

        while self.running {
            let Some(code) = program.get(self.pc()) else {
                return Err(ProcessorError::OutOfBoundsProgramCounter(self.pc()));
            };

            let instruction = match self.decode(code) {
                Ok(ins) => ins,
                Err(err) => return Err(ProcessorError::Decode(self.pc(), err)),
            };

            self.program_counter += 1;

            if let Err(err) = self.execute(instruction) {
                return Err(ProcessorError::Execute(self.pc(), err));
            }
        }

        Ok(())
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

        Ok(())
    }
}
