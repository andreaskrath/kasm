use crate::error::ParseError;

#[derive(Debug, PartialEq)]
pub enum Jump {
    Unconditional,
    IfZero,
    NotZero,
    IfSign,
    NotSign,
    IfOverflow,
    NotOverflow,
    LesserThan,
    LesserThanOrEqual,
    EqualTo,
    GreaterThan,
    GreaterThanOrEqual,
}

impl Jump {
    const UNCONDITIONAL: &'static str = "jmp";
    const IF_ZERO: &'static str = "jiz";
    const NOT_ZERO: &'static str = "jnz";
    const IF_SIGN: &'static str = "jis";
    const NOT_SIGN: &'static str = "jns";
    const IF_OVERFLOW: &'static str = "jio";
    const NOT_OVERFLOW: &'static str = "jno";
    const LESSER_THAN: &'static str = "jlt";
    const LESSER_THAN_OR_EQUAL: &'static str = "jle";
    const EQUAL_TO: &'static str = "jet";
    const GREATER_THAN: &'static str = "jgt";
    const GREATER_THAN_OR_EQUAL: &'static str = "jge";

    pub fn parse(s_jump: &str) -> Result<Jump, ParseError> {
        let jump = match s_jump {
            Jump::UNCONDITIONAL => Jump::Unconditional,
            Jump::IF_ZERO => Jump::IfZero,
            Jump::NOT_ZERO => Jump::NotZero,
            Jump::IF_SIGN => Jump::IfSign,
            Jump::NOT_SIGN => Jump::NotSign,
            Jump::IF_OVERFLOW => Jump::IfOverflow,
            Jump::NOT_OVERFLOW => Jump::NotOverflow,
            Jump::LESSER_THAN => Jump::LesserThan,
            Jump::LESSER_THAN_OR_EQUAL => Jump::LesserThanOrEqual,
            Jump::EQUAL_TO => Jump::EqualTo,
            Jump::GREATER_THAN => Jump::GreaterThan,
            Jump::GREATER_THAN_OR_EQUAL => Jump::GreaterThanOrEqual,
            unknown => return Err(ParseError::UnknownInstruction(unknown)),
        };

        Ok(jump)
    }
}
