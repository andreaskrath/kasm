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
    LesserThanOrEqual,
    GreaterThanOrEqual,
}

impl Jump {
    // Below are actual jump instructions.

    const UNCONDITIONAL: &'static str = "jmp";
    const IF_ZERO: &'static str = "jiz";
    const NOT_ZERO: &'static str = "jnz";
    const IF_SIGN: &'static str = "jis";
    const NOT_SIGN: &'static str = "jns";
    const IF_OVERFLOW: &'static str = "jio";
    const NOT_OVERFLOW: &'static str = "jno";
    const LESSER_THAN_OR_EQUAL: &'static str = "jle";
    const GREATER_THAN_OR_EQUAL: &'static str = "jge";

    // Below are aliases for ease of use.

    const LESSER_THAN: &'static str = "jlt";
    const IF_EQUAL: &'static str = "jie";
    const NOT_EQUAL: &'static str = "jne";
    const GREATER_THAN: &'static str = "jgt";

    pub fn parse(s_jump: &str) -> Result<Jump, ParseError> {
        let jump = match s_jump {
            Jump::UNCONDITIONAL => Jump::Unconditional,
            Jump::IF_ZERO | Jump::IF_EQUAL => Jump::IfZero,
            Jump::NOT_ZERO | Jump::NOT_EQUAL => Jump::NotZero,
            Jump::IF_SIGN => Jump::IfSign,
            Jump::NOT_SIGN => Jump::NotSign,
            Jump::IF_OVERFLOW | Jump::LESSER_THAN => Jump::IfOverflow,
            Jump::NOT_OVERFLOW | Jump::GREATER_THAN => Jump::NotOverflow,
            Jump::LESSER_THAN_OR_EQUAL => Jump::LesserThanOrEqual,
            Jump::GREATER_THAN_OR_EQUAL => Jump::GreaterThanOrEqual,
            unknown => return Err(ParseError::UnknownInstruction(unknown)),
        };

        Ok(jump)
    }
}
