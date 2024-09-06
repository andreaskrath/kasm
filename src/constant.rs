//! Defines the constants, type aliases and central types to the virtual processor.

use crate::{error::DecodeError, instruction::Instruction};
use phf::Map;
use std::str::SplitWhitespace;

pub type Byte = u8;
pub type Quarter = u16;
pub type Half = u32;
pub type Word = u64;
pub type SignedByte = i8;
pub type SignedQuarter = i16;
pub type SignedHalf = i32;
pub type SignedWord = i64;

/// The amount of bytes in a mega byte.
const MEGA_BYTE: usize = 1_048_576;

/// The size of the stack in the virtual processor.
///
/// The value is in bytes.
pub const STACK_SIZE: usize = MEGA_BYTE * 4;

#[cfg(test)]
pub const TEST_STACK_SIZE: usize = Quarter::MAX as usize;

type DecodeFn = fn(SplitWhitespace) -> Result<Instruction, DecodeError>;
pub type DecodeTable = Map<&'static str, DecodeFn>;
