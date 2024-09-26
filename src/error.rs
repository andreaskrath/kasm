use std::path::PathBuf;
use thiserror::Error;

/// The application level error.
///
/// It is the only error directly presented to the user.
#[derive(Debug, Error, PartialEq)]
pub enum InterpreterError {
    /// A wrapper for a decode error.
    #[error("failed to decode line {0}: {1}")]
    Decode(usize, DecodeError),

    /// A wrapper for an execute error.
    #[error("failed to execute line {0}: {1}")]
    Execute(usize, ExecuteError),

    /// A wrapper for a preprocess error.
    #[error("failed during preprocessing: {0}")]
    PreProcess(PreProcessError),

    /// A wrapper for an argument error.
    #[error("faled to process command line arguments: {0}")]
    Argument(ArgumentError),

    /// Used to indicate the program counter being outside bounds of the program.
    #[error("line '{0}' is not part of the specified program")]
    InvalidProgramCounter(usize),

    /// Used to indicate an error during the creation of an output file.
    ///
    /// This error can only occur if the output flag is used to specify
    /// another output than stdout.
    ///
    /// The underlying error is from the standard library and is only provided to the user
    /// in case it may be helpful to determine the underlying cause of the error.
    #[error("failed to create or open output file, underlying cause is: {0}")]
    FailedOutputFileCreation(String),
}

/// Represents an error during the decoding of an instruction.
#[derive(Debug, Error, PartialEq)]
pub enum DecodeError {
    /// Indicates a parameter could not be parsed to a register.
    #[error("invalid register '{0}'")]
    InvalidRegister(String),

    /// Indicates a parameter could not be parsed to an immediate value.
    ///
    /// This includes when the immediate value does not fit within the specified size bounds,
    /// such as trying to parse 300 as a [`crate::constant::Byte`].
    #[error("invalid immediate value '{0}'")]
    InvalidImmediateValue(String),

    /// Indicates an operand parameter could not be parsed to an operand variant.
    #[error("invalid operand '{0}'")]
    InvalidOperand(String),

    /// Indicates an instruction is not part of the instruction set.
    #[error("unknown instruction '{0}'")]
    UnknownInstruction(String),

    /// Indicates missing parameters for an instruction.
    #[error("incomplete instruction")]
    IncompleteInstruction,

    /// Indicates an empty line was specified for decoding.
    #[error("could not decode empty line")]
    EmptyLine,
}

/// Represents an error during the execution of an instruction.
#[derive(Debug, Error, PartialEq)]
pub enum ExecuteError {
    /// Indicates a stack overflow occured during execution of an instruction.
    #[error("a stack overflow occured")]
    StackOverflow,

    /// Indicates a stack underflow occured during execution of an instruction.
    #[error("a stack underflow occured")]
    StackUnderflow,

    /// Indicates an IO error occured during execution of an instruction.
    ///
    /// The underlying IO error is from the standard library and is only provided to the user
    /// in case it may be helpful to determine the underlying cause of the error.
    #[error("an IO error occured, underlying cause is: {0}")]
    IO(String),

    /// Indicates a division operation tried to divide by zero.
    #[error("attempted to divide by zero")]
    DivideByZero,

    /// Indicates a program counter overflow during increment or relative jump.
    #[error("the program counter overflowed")]
    ProgramCounterOverflow,

    /// Indicates a program counter underflow during relative jump.
    #[error("the program counter underflowed")]
    ProgramCounterUnderflow,
}

/// Represents an error during the parsing and substitution during preprocessing.
#[derive(Debug, Error, PartialEq)]
pub enum PreProcessError {
    /// Indicates a key in the data section is missing a corresponding value.
    #[error("there is no value defined for the key '{0}'")]
    MissingValue(String),

    /// Indicates a key in the data section not conforming to the screaming snake case format.
    #[error("the format for the key '{0}' is invalid")]
    InvalidKeyFormat(String),

    /// Indicates a missing function name after the 'fn' keyword.
    #[error("no function name is specified")]
    MissingFunctionName,

    /// Indicates a missing colon suffix on function name.
    #[error("missing colon suffix on function name")]
    MissingColonSuffix,

    /// Indicates the same function name defined multiple times.
    #[error("function name '{0}' is defined multiple times")]
    DuplicateFunctionName(String),

    /// Indicates a call was made to a function that is not defined in the program.
    #[error("called undefined function named '{0}'")]
    UndefinedFunctionCalled(String),

    #[error("function name '{0}' is not snake case")]
    InvalidFunctionNameFormat(String),

    #[error("a function was named after an instruction")]
    FunctionNamedAfterInstruction,
}

#[derive(Debug, Error, PartialEq)]
pub enum ArgumentError {
    #[error(
        "could not retrieve the size suffix of the stack flag, this may be due to encoding issues, only use ascii"
    )]
    CouldNotSplitSuffix,

    /// Indicates the suffix of the stack flag is not a known variant.
    #[error("invalid stack size suffix'{0}'")]
    InvalidStackSizeSuffix(String),

    /// Indicates the initial numerical value supplied for the stack size is not valid.
    #[error(
        "the number '{0}' could not be parsed to a {}-bit unsigned integer",
        usize::BITS
    )]
    InvalidInitialStackSize(String),

    /// Indicates the computed numerical value for the stack size is not valid.
    #[error(
        "the computed stack size in bytes cannot be represented by a {}-bit unsigned integer",
        usize::BITS
    )]
    InvalidComputedStackSize,

    /// Indicates the specified program file could not be found.
    #[error("could not find a file named '{0}'")]
    FileNotFound(PathBuf),

    /// Indicates an issue with permissions, the specific issue is specified by the String parameter.
    #[error("could not {0} due to lacking permissions")]
    LackingPermissions(String),

    /// Indicates an unknown error occured while trying to open the specified input file.
    #[error("an unknown error occured while trying to open specified program file, underlying cause is: {0}")]
    UnknownProgramFileIssue(String),

    /// Indicates an issue with encoding in the program file making reading the file impossible.
    #[error("the program file contains invalid encoding, it must be UTF-8 compatible")]
    ProgramFileInvalidEncoding,

    /// Indicates the specified program file is not in fact a file.
    #[error("'{0}' is not a file")]
    NotAFile(PathBuf),
}
