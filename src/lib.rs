use constant::{Word, STACK_SIZE};
use decode::DECODE_TABLE;
use error::{DecodeError, ProcessorError};
use flags::Flags;
use instruction::Instruction;
use register::Register;
use registers::Registers;
use std::io::{stdout, Write};

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

pub struct Processor {
    registers: Registers,
    stack_pointer: Word,
    program_counter: Word,
    flags: Flags,
    running: bool,
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
            registers: [0; Register::VARIANT_COUNT],
            stack_pointer: 0,
            program_counter: 0,
            flags: Flags::new(),
            running: true,
            output,
            stack,
        };
        Ok(p)
    }

    #[cfg(test)]
    pub fn test_instance() -> Self {
        let stack = vec![0; STACK_SIZE].into_boxed_slice().try_into().unwrap();

        Self {
            registers: [0; Register::VARIANT_COUNT],
            stack_pointer: 0,
            program_counter: 0,
            flags: Flags::new(),
            running: true,
            output: Box::new(Vec::new()),
            stack,
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

    pub fn run(&mut self, program: &str) -> Result<(), ProcessorError> {
        let program: Vec<&str> = program.lines().collect();

        while self.running {
            let code = program
                .get(self.pc())
                .ok_or(ProcessorError::InvalidProgramCounter(self.pc()))?;

            let instruction = self
                .decode(code)
                .map_err(|e| ProcessorError::Decode(self.pc(), e))?;

            self.program_counter += 1;

            self.execute(instruction)
                .map_err(|e| ProcessorError::Execute(self.pc(), e))?;
        }

        Ok(())
    }

    fn decode(&mut self, s: &str) -> Result<Instruction, DecodeError> {
        let mut s_iter = s.split_whitespace();
        let instruction = s_iter.next().ok_or(DecodeError::EmptyLine)?;
        let decoder = DECODE_TABLE
            .get(instruction)
            .ok_or(DecodeError::UnknownInstruction(instruction.to_string()))?;
        let instruction = decoder(s_iter)?;

        Ok(instruction)
    }
}
