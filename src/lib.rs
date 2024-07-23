use constant::{Word, REGISTER_AMOUNT, STACK_SIZE, WORD_BYTE_SIZE};
use error::{ExecuteError, ProcessorError};
use flags::Flags;
use instruction::{Instruction, Jump};
use operand::Operand;
use register::Register;

mod error;
mod flags;
mod instruction;
mod register;
mod constant;
mod operand;

pub struct Processor {
    registers: [Word; REGISTER_AMOUNT],
    stack_pointer: Word,
    program_counter: Word,
    flags: Flags,
    stack: Box<[u8; STACK_SIZE]>, 

}

impl<'a> Processor {
    pub fn new() -> Result<Self, ProcessorError<'a>>{
        let Ok(stack) = vec![0; STACK_SIZE].into_boxed_slice().try_into() else {
            return Err(ProcessorError::FailedStackAllocation);
        };

        let p = Self { 
                    registers: [0; REGISTER_AMOUNT],
                    stack_pointer: 0, 
                    program_counter: 0,
                    flags: Flags::new(),
                    // Kind of a hack, but simply allocating an array inside a box causes a stack overflow.
                    // https://github.com/rust-lang/rust/issues/53827
                    stack,
                };
        Ok(p)
    }

    pub fn start(&'a mut self, program: &'a str) -> Result<(), ProcessorError> {
        let program: Vec<&str> = program.lines().collect();

        loop {
            let Some(code) = program.get(self.pc()) else {
                return Err(ProcessorError::OutOfBoundsProgramCounter(self.pc()));
            };

            let instruction = match Instruction::parse(code) {
                Ok(ins) => ins,
                Err(err) => return Err(ProcessorError::Parse(self.pc(), err)), 
            };

            self.program_counter += 1;

            if instruction == Instruction::Stop {
                return Ok(());
            } else if let Err(err) = self.execute_instruction(instruction) {
                return Err(ProcessorError::Execute(self.pc(), err));
            }
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

    /// The value associated with an operand.
    fn get_value(&self, operand: Operand) -> Word {
        match operand {
            Operand::Register(reg) => self.registers[reg],
            Operand::Constant(val) => val,
        }
    }

    fn set_register(&mut self, reg: Register, operand: Operand) {
        let value = self.get_value(operand);

        self.registers[reg] = value;
    }

    fn push(&mut self, operand: Operand) -> Result<(), ExecuteError> {
        let value = self.get_value(operand);
        self.push_underlying(value)?;

        Ok(())
    }
    
    fn push_underlying(&mut self, value: Word) -> Result<(), ExecuteError> {
        if self.sp() + WORD_BYTE_SIZE > STACK_SIZE {
            return Err(ExecuteError::StackOverflow);
        }

        let bytes = value.to_le_bytes();
        for (index, byte) in bytes.into_iter().enumerate() {
            self.stack[self.sp() + index] = byte;
        }
        self.stack_pointer += WORD_BYTE_SIZE as u32;

        Ok(())
    }

    fn pop(&mut self, reg: Register) -> Result<(), ExecuteError> {
        self.registers[reg] = self.pop_underlying()?;

        Ok(())
    }

    fn pop_underlying(&mut self) -> Result<Word, ExecuteError> {
        if self.sp().checked_sub(WORD_BYTE_SIZE).is_none() {
            return Err(ExecuteError::StackUnderflow);
        }

        let mut bytes: [u8; WORD_BYTE_SIZE] = [0; WORD_BYTE_SIZE];
        bytes.copy_from_slice(&self.stack[self.sp() - WORD_BYTE_SIZE..self.sp()]);
        self.stack_pointer -= WORD_BYTE_SIZE as u32;

        Ok(Word::from_le_bytes(bytes))
    }

    fn add(&mut self, reg: Register, operand: Operand) {
        let a = self.registers[reg];
        let b = self.get_value(operand);

        let (result, overflow) = a.overflowing_add(b);
        self.flags.set(result, overflow);
        self.registers[reg] = result;
    }

    fn sub(&mut self, reg: Register, operand: Operand) {
        let a = self.registers[reg];
        let b = self.get_value(operand);

        let (result, overflow) = a.overflowing_sub(b);
        self.flags.set(result, overflow);
        self.registers[reg] = result;
    }

    fn mul(&mut self, reg: Register, operand: Operand) {
        let a = self.registers[reg];
        let b = self.get_value(operand);

        let (result, overflow) = a.overflowing_mul(b);
        self.flags.set(result, overflow);
        self.registers[reg] = result;
    }

    fn div(&mut self, reg: Register, operand: Operand) {
        let a = self.registers[reg];
        let b = self.get_value(operand);

        let (result, overflow) = a.overflowing_div(b);
        self.flags.set(result, overflow);
        self.registers[reg] = result;
    }

    fn jump(&mut self, jump: Jump, operand: Operand) {
        let destination = self.get_value(operand);

        match jump {
            Jump::Unconditional => self.program_counter = destination,
            Jump::IfZero => if self.flags.zero { self.program_counter = destination },
            Jump::NotZero => if !self.flags.zero { self.program_counter = destination },
            Jump::IfSign => if self.flags.sign { self.program_counter = destination },
            Jump::NotSign => if !self.flags.sign { self.program_counter = destination },
            Jump::IfOverflow => if self.flags.overflow { self.program_counter = destination },
            Jump::NotOverflow => if !self.flags.overflow { self.program_counter = destination },
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) -> Result<(), ExecuteError> {
        use Instruction::*;

        match instruction {
            Stop => unreachable!("guarded in 'start' method of processor"),
            Set(reg, operand) => self.set_register(reg, operand),
            Push(operand) => self.push(operand)?,
            Pop(reg) => self.pop(reg)?,
            Add(reg, operand) => self.add(reg, operand),
            Sub(reg, operand) => self.sub(reg, operand),
            Mul(reg, operand) => self.mul(reg, operand),
            Div(reg, operand) => self.div(reg, operand),
            Jump(jump, operand) => self.jump(jump, operand),
        }        

        Ok(())
    }
}

