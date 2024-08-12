use thiserror::Error;
use crate::constant::Word;

#[derive(Debug, Error, PartialEq)]
pub enum ProcessorError {
    #[error("failed to allocated the stack")]
    FailedStackAllocation,
    #[error("failed to decode line {0}: {1}")]
    Decode(usize, DecodeError),
    #[error("execution error on line {0}: {1}")]
    Execute(usize, ExecuteError),
    #[error("tried to execute instruction on line {0} which is outside the defined bounds of the program")]
    OutOfBoundsProgramCounter(usize),
}

#[derive(Debug, Error, PartialEq)]
pub enum DecodeError {
    #[error("invalid register '{0}'")]
    InvalidRegister(String),
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
}
