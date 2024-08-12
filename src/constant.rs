//! Defines the constants, type aliases and central types to the virtual processor.

use std::str::SplitWhitespace;

use phf::Map;

use crate::{error::{DecodeError, ExecuteError}, instruction::Instruction, Processor};
use super::register::Register;

pub type Byte = u8;
pub type Quarter = u16;
pub type Half = u32;
pub type Word = u64;

pub type Registers = [Word; REGISTER_AMOUNT];

/// The amount of registers in the virtual processor.
///
/// The value of this constant is directly tied to the variant count of
/// [`crate::processor::register::Register`].
pub const REGISTER_AMOUNT: usize = Register::VARIANT_COUNT;

/// The amount of bytes in a mega byte.
const MEGA_BYTE: usize = 1_048_576;

/// The size of the stack in the virtual processor.
///
/// The value is in bytes.
pub const STACK_SIZE: usize = MEGA_BYTE * 4;

pub type ExecuteFn = fn(&mut Processor) -> Result<(), ExecuteError>;
pub type ExecuteTable = [ExecuteFn; Instruction::VARIANT_COUNT];
