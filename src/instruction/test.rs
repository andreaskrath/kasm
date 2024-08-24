use crate::{
    constant::{Byte, Half, Quarter, Word},
    operand::Operand,
};

#[derive(Debug, PartialEq)]
pub enum Test {
    Byte(Operand<Byte>, Operand<Byte>),
    Quarter(Operand<Quarter>, Operand<Quarter>),
    Half(Operand<Half>, Operand<Half>),
    Word(Operand<Word>, Operand<Word>),
}
