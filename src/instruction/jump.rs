#[derive(Debug, PartialEq)]
pub enum Jump {
    Unconditional,
    IfZero,
    IfNotZero,
    IfSign,
    IfNotSign,
    IfOverflow,
    IfNotOverflow,
    IfGreater,
    IfLesser,
    IfGreaterOrEqual,
    IfLesserOrEqual,
}

#[derive(Debug, PartialEq)]
pub enum Relative {
    Positive,
    Negative,
}
