use crate::InterpreterError;

pub struct Program(Box<[String]>);

impl Program {
    pub fn new(program: Box<[String]>) -> Self {
        Self(program)
    }

    pub fn get(&self, index: usize) -> Result<&str, InterpreterError> {
        if index == 0 {
            return Err(InterpreterError::InvalidProgramCounter(index));
        }

        let line = self
            .0
            .get(index - 1)
            .ok_or(InterpreterError::InvalidProgramCounter(index))?;

        Ok(line)
    }
}
