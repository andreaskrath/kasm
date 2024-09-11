use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum InterpreterError {
    #[error("failed to create or open output file, underlying cause is: {0}")]
    FailedOutputFileCreation(String),
    #[error("failed to decode line {0}: {1}")]
    Decode(usize, DecodeError),
    #[error("failed to execute line {0}: {1}")]
    Execute(usize, ExecuteError),
    #[error("failed to preprocess data section: {0}")]
    Data(DataError),
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

#[derive(Debug, Error, PartialEq)]
pub enum DataError {
    #[error("there is no value defined for the key '{0}'")]
    MissingValue(String),
    #[error("the format for the key '{0}' is invalid, only use upper case ascii")]
    InvalidKeyFormat(String),

    /// Not sure when this error occurs, or if it even can occur at all given the string that is split on.
    ///
    /// However, for good measuer this provides a better error message than unwrapping.
    #[error("there is an issue with the encoding, only use ascii")]
    Encoding,
}
