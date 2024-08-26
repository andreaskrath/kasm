use crate::{constant::Word, operand::Operand};

#[derive(Debug, PartialEq)]
pub enum PrintStack {
    Byte(Operand<Word>),
    Quarter(Operand<Word>),
    Half(Operand<Word>),
    Word(Operand<Word>),
    Str(Operand<Word>),
}
