//! Defines the constants, type aliases and central types to the virtual processor.

use crate::{error::DecodeError, instruction::Instruction};
use phf::Map;
use std::str::SplitAsciiWhitespace;

pub type Byte = u8;
pub type Quarter = u16;
pub type Half = u32;
pub type Word = u64;
pub type SignedByte = i8;
pub type SignedQuarter = i16;
pub type SignedHalf = i32;
pub type SignedWord = i64;

pub const KILO_BYTE: usize = 1024;

/// The amount of bytes in a mega byte.
pub const MEGA_BYTE: usize = KILO_BYTE * KILO_BYTE;

pub const GIGA_BYTE: usize = KILO_BYTE * KILO_BYTE * KILO_BYTE;

#[cfg(test)]
pub const TEST_STACK_SIZE: usize = Quarter::MAX as usize;

type DecodeFn = fn(Parameters) -> Result<Instruction, DecodeError>;
pub type DecodeTable = Map<&'static str, DecodeFn>;

pub const COMMENT: &str = "//";

pub const DEBUG_HELP: &str = r"You can take the following actions:
  - 'next'/'n' will advance the interpretation by one instruction
  - 'stop'/'s' will stop the interpretation\n
";

pub const DEBUG_INITIAL: &str = r"You are running the program in debug mode.
Use the 'help'/'h' action for more information.";

pub type Parameters<'a> = SplitAsciiWhitespace<'a>;
