use crate::error::DecodeError;

#[derive(Debug, PartialEq)]
pub enum Jump {
    Unconditional,
    IfZero,
    NotZero,
    IfSign,
    NotSign,
    IfOverflow,
    NotOverflow,
}

impl Jump {
    const UNCONDITIONAL: &'static str = "jmp";
    const IF_ZERO: &'static str = "jiz";
    const NOT_ZERO: &'static str = "jnz";
    const IF_SIGN: &'static str = "jis";
    const NOT_SIGN: &'static str = "jns";
    const IF_OVERFLOW: &'static str = "jio";
    const NOT_OVERFLOW: &'static str = "jno";

    pub fn decode(s_jump: &str) -> Result<Jump, DecodeError> {
        let jump = match s_jump {
            Jump::UNCONDITIONAL => Jump::Unconditional,
            Jump::IF_ZERO => Jump::IfZero,
            Jump::NOT_ZERO => Jump::NotZero,
            Jump::IF_SIGN => Jump::IfSign,
            Jump::NOT_SIGN => Jump::NotSign,
            Jump::IF_OVERFLOW => Jump::IfOverflow,
            Jump::NOT_OVERFLOW => Jump::NotOverflow,
            unknown => return Err(DecodeError::UnknownInstruction(unknown)),
        };

        Ok(jump)
    }
}
