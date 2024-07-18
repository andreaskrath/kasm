use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ProcessorError<'a> {
    #[error("failed to allocated the stack")]
    FailedStackAllocation,
    #[error("decode error on line {0}: {1}")]
    Decode(usize, DecodeError<'a>),
    #[error("execution error on line {0}: {1}")]
    Execute(usize, ExecuteError),
    #[error("tried to execute instruction on line {0} which is out outside the defined bounds of the program")]
    OutOfBoundsProgramCounter(usize),
}

#[derive(Debug, Error, PartialEq)]
pub enum DecodeError<'a> {
    #[error("invalid register '{0}'")]
    InvalidRegister(&'a str),
    #[error("invalid operand '{0}'")]
    InvalidOperand(&'a str),
    #[error("unknown instruction '{0}'")]
    UnknownInstruction(&'a str),
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
}
