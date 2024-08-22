use crate::{
    constant::{Byte, Half, Quarter, Word},
    operand::Operand,
};

#[derive(Debug, PartialEq)]
pub enum Push {
    Byte(Operand<Byte>),
    Quarter(Operand<Quarter>),
    Half(Operand<Half>),
    Word(Operand<Word>),
}
