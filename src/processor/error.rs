use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ProcessorError<'a> {
    #[error("failed to allocated the stack for the virtual processor")]
    FailedStackAllocation,
    #[error("instruction error on line {0}: {1}")]
    Instruction(usize, InstructionError<'a>),
    #[error("execution error on line {0}: {1}")]
    Execute(usize, ExecuteError),
    #[error("tried to execute instruction on line {0} which is out of bounds")]
    OutOfBoundsProgramCounter(usize),
}

#[derive(Debug, Error, PartialEq)]
pub enum InstructionError<'a> {
    #[error("invalid register '{0}'")]
    InvalidRegister(&'a str),
    #[error("unknown instruction '{0}'")]
    UnknownInstruction(&'a str),
    #[error("incomplete instruction '{0}'")]
    IncompleteInstruction(&'a str),
    #[error("invalid value '{0}'")]
    InvalidValue(&'a str),
    #[error("empty line specified for execution")]
    EmptyLine,
}

#[derive(Debug, Error, PartialEq)]
pub enum ExecuteError {
    #[error("a stack overflow occured")]
    StackOverflow,
    #[error("a stack underflow occured")]
    StackUnderflow,
}
