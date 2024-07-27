//! Defines the constants, type aliases and central types to the virtual processor.

use std::mem::size_of;

use super::register::Register;

/// The word size of the virtual processor.
///
/// This is not intended to be changed.
pub type Word = u32;

/// The amount of bytes in a [`crate::processor::constant::Word`].
pub const WORD_BYTE_SIZE: usize = size_of::<Word>();

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
