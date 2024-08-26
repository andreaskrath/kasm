use crate::constant::Word;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ProcessorError {
    #[error("failed to allocated the stack")]
    FailedStackAllocation,
    #[error("failed to create or open output file, underlying cause is: {0}")]
    FailedOutputFileCreation(String),
    #[error("failed to decode line {0}: {1}")]
    Decode(usize, DecodeError),
    #[error("failed to execute line {0}: {1}")]
    Execute(usize, ExecuteError),
    #[error("program counter out of bounds on line '{0}'")]
    InvalidProgramCounter(usize),
}

#[derive(Debug, Error, PartialEq)]
pub enum DecodeError {
    #[error("invalid register '{0}'")]
    InvalidRegister(String),
    #[error("invalid immediate value '{0}'")]
    InvalidImmediateValue(String),
    #[error("invalid operand '{0}'")]
    InvalidOperand(String),
    #[error("unknown instruction '{0}'")]
    UnknownInstruction(String),
    #[error("incomplete instruction")]
    IncompleteInstruction,
    #[error("could not decode empty line")]
    EmptyLine,
}

#[derive(Debug, Error, PartialEq)]
pub enum ExecuteError {
    #[error("a stack overflow occured")]
    StackOverflow,
    #[error("a stack underflow occured")]
    StackUnderflow,
    #[error("an IO error occured, underlying cause is: {0}")]
    IO(String),
    #[error("attempted to divide by zero")]
    DivideByZero,
}
