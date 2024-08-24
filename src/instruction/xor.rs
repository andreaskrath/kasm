use crate::{
    constant::{Byte, Half, Quarter, Word},
    operand::Operand,
    register::Register,
};

#[derive(Debug, PartialEq)]
pub enum Xor {
    Byte(Register, Operand<Byte>),
    Quarter(Register, Operand<Quarter>),
    Half(Register, Operand<Half>),
    Word(Register, Operand<Word>),
}
