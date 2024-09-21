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
