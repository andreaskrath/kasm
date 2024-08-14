use crate::{constant::ExecuteTable, error::ExecuteError, register::Register, Processor};

pub const EXECUTE_TABLE: ExecuteTable = [
    Processor::execute_stop,
    Processor::execute_set_byte,
    Processor::execute_set_quarter,
    Processor::execute_set_half,
    Processor::execute_set_word,
];

impl Processor {
    fn execute_stop(&mut self) -> Result<(), ExecuteError> {
        self.running = false;

        Ok(())
    }

    /// The underlying value to be set is confirmed applicable to the given size operation when decoded.
    ///
    /// Therefore the execute phase of setting a register is identical for each size variant.
    fn execute_set(&mut self) -> Result<(), ExecuteError> {
        let register = Register::try_from(self.registers[Register::P1])?;
        let value = self.registers[Register::P2];
        self.registers[register] = value;

        Ok(())
    }

    fn execute_set_byte(&mut self) -> Result<(), ExecuteError> {
        self.execute_set()
    }

    fn execute_set_quarter(&mut self) -> Result<(), ExecuteError> {
        self.execute_set()
    }

    fn execute_set_half(&mut self) -> Result<(), ExecuteError> {
        self.execute_set()
    }

    fn execute_set_word(&mut self) -> Result<(), ExecuteError> {
        self.execute_set()
    }
}

#[cfg(test)]
mod execute_set {
    use crate::{error::ExecuteError, register::Register, Processor};

    #[test]
    fn destination_is_valid_register() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::P1] = Register::A.as_word();
        p.registers[Register::P2] = 100;
        p.execute_set().unwrap();
        let expected = 100;
        let actual = p.registers[Register::A];
        assert_eq!(actual, expected);
    }

    #[test]
    fn destination_is_invalid_register() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::P1] = Register::P1.as_word();
        p.registers[Register::P2] = 100;
        let expected = Err(ExecuteError::InvalidRegisterCast(Register::P1.as_word()));
        let actual = p.execute_set();
        assert_eq!(actual, expected);
    }
}
