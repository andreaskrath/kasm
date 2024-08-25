use crate::{constant::Word, operand::Operand};

#[derive(Debug, PartialEq)]
pub enum Jump {
    Unconditional(Operand<Word>),
    IfZero(Operand<Word>),
    IfNotZero(Operand<Word>),
    IfSign(Operand<Word>),
    IfNotSign(Operand<Word>),
    IfOverflow(Operand<Word>),
    IfNotOverflow(Operand<Word>),
    IfGreater(Operand<Word>),
    IfLesser(Operand<Word>),
    IfGreaterOrEqual(Operand<Word>),
    IfLesserOrEqual(Operand<Word>),
}
