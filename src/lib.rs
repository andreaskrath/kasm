use std::io::{stdout, Write};
use constant::{Registers, Word, REGISTER_AMOUNT, STACK_SIZE};
use decode::DECODE_TABLE;
use error::{ExecuteError, DecodeError, ProcessorError};
use execute::EXECUTE_TABLE;
use flags::Flags;
use instruction::Instruction;

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
            registers: [0; REGISTER_AMOUNT],
            stack_pointer: 0,
            program_counter: 0,
            flags: Flags::new(),
            running: true,
            output,
            stack,
        };
        Ok(p)
    }

    pub fn run(&mut self, program: &str) -> Result<(), ProcessorError> {
        let program: Vec<&str> = program.lines().collect();

        while self.running {
            let Some(code) = program.get(self.pc()) else {
                return Err(ProcessorError::InvalidProgramCounter(self.pc()));
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

    fn decode(&mut self, s: &str) -> Result<Instruction, DecodeError> {
        use DecodeError as DE;

        let mut s_iter = s.split_whitespace();
        let instruction = s_iter.next().ok_or(DE::EmptyLine)?;
        let decoder = DECODE_TABLE.get(instruction).ok_or(DE::UnknownInstruction(instruction.to_string()))?;
        let instruction = decoder(self, s_iter)?;

        Ok(instruction)
    }

    /// Gets the program counter as a usize.
    fn pc(&self) -> usize {
        self.program_counter as usize
    }

    /// Gets the stack pointer as a usize.
    fn sp(&self) -> usize {
        self.stack_pointer as usize
    }

    fn execute(&mut self, instruction: Instruction) -> Result<(), ExecuteError> {
        let executor = EXECUTE_TABLE[instruction];
        executor(self)?;
        Ok(())
    }
}
