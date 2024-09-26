use crate::{
    constant::Word,
    error::ExecuteError,
    instruction::{Jump, Relative},
    operand::Operand,
    Interpreter,
};

impl Interpreter {
    /// Returns true if a jump was performed.
    pub(super) fn jump(
        &mut self,
        instruction: Jump,
        operand: Operand<Word>,
        relative: Option<Relative>,
    ) -> Result<bool, ExecuteError> {
        let jump_condition = match &instruction {
            Jump::Unconditional => true,
            Jump::IfZero => self.flags.zero,
            Jump::IfNotZero => !self.flags.zero,
            Jump::IfSign => self.flags.sign,
            Jump::IfNotSign => !self.flags.sign,
            Jump::IfOverflow => self.flags.overflow,
            Jump::IfNotOverflow => !self.flags.overflow,
            Jump::IfGreater => !self.flags.overflow && !self.flags.zero,
            Jump::IfLesser => self.flags.overflow && !self.flags.zero,
            Jump::IfGreaterOrEqual => !self.flags.overflow || self.flags.zero,
            Jump::IfLesserOrEqual => self.flags.overflow ^ self.flags.zero,
        };

        if jump_condition {
            let destination = self.get_operand_value(operand);

            self.program_counter = match relative {
                Some(Relative::Positive) => self
                    .program_counter
                    .checked_add(destination)
                    .ok_or(ExecuteError::ProgramCounterOverflow)?,
                Some(Relative::Negative) => self
                    .program_counter
                    .checked_sub(destination)
                    .ok_or(ExecuteError::ProgramCounterUnderflow)?,
                None => destination,
            }
        }

        Ok(jump_condition)
    }
}

// The test cases below are really only to prevent regression at a future point.

#[cfg(test)]
mod unconditional {
    use crate::{
        constant::Word,
        error::ExecuteError,
        instruction::{Instruction, Jump, Relative},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn program_counter_overflow_error() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::Unconditional,
            Operand::Immediate(1),
            Some(Relative::Positive),
        );
        i.program_counter = Word::MAX;
        let expected = Err(ExecuteError::ProgramCounterOverflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn program_counter_underflow_error() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::Unconditional,
            Operand::Immediate(2),
            Some(Relative::Negative),
        );
        let expected = Err(ExecuteError::ProgramCounterUnderflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::Unconditional, Operand::Immediate(5), None);
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Jump(Jump::Unconditional, Operand::Register(Register::A), None);
        i.registers.set(Register::A, 5);
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_positive_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::Unconditional,
            Operand::Immediate(2),
            Some(Relative::Positive),
        );
        let expected = 3;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_negative_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::Unconditional,
            Operand::Immediate(2),
            Some(Relative::Negative),
        );
        i.program_counter = 3;
        let expected = 1;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_positive_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::Unconditional,
            Operand::Register(Register::A),
            Some(Relative::Positive),
        );
        i.registers.set(Register::A, 2);
        let expected = 3;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_negative_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::Unconditional,
            Operand::Register(Register::A),
            Some(Relative::Negative),
        );
        i.program_counter = 3;
        i.registers.set(Register::A, 2);
        let expected = 1;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }
}

#[cfg(test)]
mod if_zero {
    use crate::{
        constant::Word,
        error::ExecuteError,
        instruction::{Instruction, Jump, Relative},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn program_counter_overflow_error() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfZero,
            Operand::Immediate(1),
            Some(Relative::Positive),
        );
        i.flags.zero = true;
        i.program_counter = Word::MAX;
        let expected = Err(ExecuteError::ProgramCounterOverflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn program_counter_underflow_error() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfZero,
            Operand::Immediate(2),
            Some(Relative::Negative),
        );
        i.flags.zero = true;
        let expected = Err(ExecuteError::ProgramCounterUnderflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfZero, Operand::Immediate(5), None);
        i.flags.zero = true;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfZero, Operand::Register(Register::A), None);
        i.flags.zero = true;
        i.registers.set(Register::A, 5);
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_positive_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfZero,
            Operand::Immediate(2),
            Some(Relative::Positive),
        );
        i.flags.zero = true;
        let expected = 3;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_negative_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfZero,
            Operand::Immediate(2),
            Some(Relative::Negative),
        );
        i.flags.zero = true;
        i.program_counter = 3;
        let expected = 1;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_positive_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfZero,
            Operand::Register(Register::A),
            Some(Relative::Positive),
        );
        i.flags.zero = true;
        i.registers.set(Register::A, 2);
        let expected = 3;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_negative_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfZero,
            Operand::Register(Register::A),
            Some(Relative::Negative),
        );
        i.flags.zero = true;
        i.program_counter = 3;
        i.registers.set(Register::A, 2);
        let expected = 1;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }
}

#[cfg(test)]
mod if_not_zero {
    use crate::{
        constant::Word,
        error::ExecuteError,
        instruction::{Instruction, Jump, Relative},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn program_counter_overflow_error() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfNotZero,
            Operand::Immediate(1),
            Some(Relative::Positive),
        );
        i.flags.zero = false;
        i.program_counter = Word::MAX;
        let expected = Err(ExecuteError::ProgramCounterOverflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn program_counter_underflow_error() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfNotZero,
            Operand::Immediate(2),
            Some(Relative::Negative),
        );
        i.flags.zero = false;
        let expected = Err(ExecuteError::ProgramCounterUnderflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfNotZero, Operand::Immediate(5), None);
        i.flags.zero = false;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfNotZero, Operand::Register(Register::A), None);
        i.registers.set(Register::A, 5);
        i.flags.zero = false;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_positive_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfNotZero,
            Operand::Immediate(2),
            Some(Relative::Positive),
        );
        i.flags.zero = false;
        let expected = 3;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_negative_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfNotZero,
            Operand::Immediate(2),
            Some(Relative::Negative),
        );
        i.flags.zero = false;
        i.program_counter = 3;
        let expected = 1;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_positive_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfNotZero,
            Operand::Register(Register::A),
            Some(Relative::Positive),
        );
        i.flags.zero = false;
        i.registers.set(Register::A, 2);
        let expected = 3;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_negative_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfNotZero,
            Operand::Register(Register::A),
            Some(Relative::Negative),
        );
        i.flags.zero = false;
        i.program_counter = 3;
        i.registers.set(Register::A, 2);
        let expected = 1;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }
}

#[cfg(test)]
mod if_sign {
    use crate::{
        constant::Word,
        error::ExecuteError,
        instruction::{Instruction, Jump, Relative},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn program_counter_overflow_error() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfSign,
            Operand::Immediate(1),
            Some(Relative::Positive),
        );
        i.flags.sign = true;
        i.program_counter = Word::MAX;
        let expected = Err(ExecuteError::ProgramCounterOverflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn program_counter_underflow_error() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfSign,
            Operand::Immediate(2),
            Some(Relative::Negative),
        );
        i.flags.sign = true;
        let expected = Err(ExecuteError::ProgramCounterUnderflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfSign, Operand::Immediate(5), None);
        i.flags.sign = true;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfSign, Operand::Register(Register::A), None);
        i.flags.sign = true;
        i.registers.set(Register::A, 5);
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_positive_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfSign,
            Operand::Immediate(2),
            Some(Relative::Positive),
        );
        i.flags.sign = true;
        let expected = 3;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_negative_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfSign,
            Operand::Immediate(2),
            Some(Relative::Negative),
        );
        i.flags.sign = true;
        i.program_counter = 3;
        let expected = 1;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_positive_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfSign,
            Operand::Register(Register::A),
            Some(Relative::Positive),
        );
        i.flags.sign = true;
        i.registers.set(Register::A, 2);
        let expected = 3;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_negative_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfSign,
            Operand::Register(Register::A),
            Some(Relative::Negative),
        );
        i.flags.sign = true;
        i.program_counter = 3;
        i.registers.set(Register::A, 2);
        let expected = 1;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }
}

#[cfg(test)]
mod if_not_sign {
    use crate::{
        constant::Word,
        error::ExecuteError,
        instruction::{Instruction, Jump, Relative},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn program_counter_overflow_error() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfNotSign,
            Operand::Immediate(1),
            Some(Relative::Positive),
        );
        i.flags.sign = false;
        i.program_counter = Word::MAX;
        let expected = Err(ExecuteError::ProgramCounterOverflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn program_counter_underflow_error() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfNotSign,
            Operand::Immediate(2),
            Some(Relative::Negative),
        );
        i.flags.sign = false;
        let expected = Err(ExecuteError::ProgramCounterUnderflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfNotSign, Operand::Immediate(5), None);
        i.flags.sign = false;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfNotSign, Operand::Register(Register::A), None);
        i.flags.sign = false;
        i.registers.set(Register::A, 5);
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_positive_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfNotSign,
            Operand::Immediate(2),
            Some(Relative::Positive),
        );
        i.flags.sign = false;
        let expected = 3;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_negative_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfNotSign,
            Operand::Immediate(2),
            Some(Relative::Negative),
        );
        i.flags.sign = false;
        i.program_counter = 3;
        let expected = 1;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_positive_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfNotSign,
            Operand::Register(Register::A),
            Some(Relative::Positive),
        );
        i.flags.sign = false;
        i.registers.set(Register::A, 2);
        let expected = 3;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_negative_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfNotSign,
            Operand::Register(Register::A),
            Some(Relative::Negative),
        );
        i.flags.sign = false;
        i.program_counter = 3;
        i.registers.set(Register::A, 2);
        let expected = 1;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }
}

#[cfg(test)]
mod if_overflow {
    use crate::{
        constant::Word,
        error::ExecuteError,
        instruction::{Instruction, Jump, Relative},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn program_counter_overflow_error() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfOverflow,
            Operand::Immediate(1),
            Some(Relative::Positive),
        );
        i.flags.overflow = true;
        i.program_counter = Word::MAX;
        let expected = Err(ExecuteError::ProgramCounterOverflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn program_counter_underflow_error() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfOverflow,
            Operand::Immediate(2),
            Some(Relative::Negative),
        );
        i.flags.overflow = true;
        let expected = Err(ExecuteError::ProgramCounterUnderflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfOverflow, Operand::Immediate(5), None);
        i.flags.overflow = true;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfOverflow, Operand::Register(Register::A), None);
        i.flags.overflow = true;
        i.registers.set(Register::A, 5);
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_positive_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfOverflow,
            Operand::Immediate(2),
            Some(Relative::Positive),
        );
        i.flags.overflow = true;
        let expected = 3;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_negative_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfOverflow,
            Operand::Immediate(2),
            Some(Relative::Negative),
        );
        i.flags.overflow = true;
        i.program_counter = 3;
        let expected = 1;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_positive_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfOverflow,
            Operand::Register(Register::A),
            Some(Relative::Positive),
        );
        i.flags.overflow = true;
        i.registers.set(Register::A, 2);
        let expected = 3;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_negative_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfOverflow,
            Operand::Register(Register::A),
            Some(Relative::Negative),
        );
        i.flags.overflow = true;
        i.program_counter = 3;
        i.registers.set(Register::A, 2);
        let expected = 1;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }
}

#[cfg(test)]
mod if_not_overflow {
    use crate::{
        constant::Word,
        error::ExecuteError,
        instruction::{Instruction, Jump, Relative},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn program_counter_overflow_error() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfNotOverflow,
            Operand::Immediate(1),
            Some(Relative::Positive),
        );
        i.flags.overflow = false;
        i.program_counter = Word::MAX;
        let expected = Err(ExecuteError::ProgramCounterOverflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn program_counter_underflow_error() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfNotOverflow,
            Operand::Immediate(2),
            Some(Relative::Negative),
        );
        i.flags.overflow = false;
        let expected = Err(ExecuteError::ProgramCounterUnderflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfNotOverflow, Operand::Immediate(5), None);
        i.flags.overflow = false;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Jump(Jump::IfNotOverflow, Operand::Register(Register::A), None);
        i.registers.set(Register::A, 5);
        i.flags.overflow = false;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_positive_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfNotOverflow,
            Operand::Immediate(2),
            Some(Relative::Positive),
        );
        i.flags.overflow = false;
        let expected = 3;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_negative_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfNotOverflow,
            Operand::Immediate(2),
            Some(Relative::Negative),
        );
        i.flags.overflow = false;
        i.program_counter = 3;
        let expected = 1;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_positive_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfNotOverflow,
            Operand::Register(Register::A),
            Some(Relative::Positive),
        );
        i.flags.overflow = false;
        i.registers.set(Register::A, 2);
        let expected = 3;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_negative_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfNotOverflow,
            Operand::Register(Register::A),
            Some(Relative::Negative),
        );
        i.flags.overflow = false;
        i.program_counter = 3;
        i.registers.set(Register::A, 2);
        let expected = 1;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }
}

#[cfg(test)]
mod if_greater {
    use crate::{
        constant::Word,
        error::ExecuteError,
        instruction::{Instruction, Jump, Relative},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn program_counter_overflow_error() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfGreater,
            Operand::Immediate(1),
            Some(Relative::Positive),
        );
        i.flags.overflow = false;
        i.flags.zero = false;
        i.program_counter = Word::MAX;
        let expected = Err(ExecuteError::ProgramCounterOverflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn program_counter_underflow_error() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfGreater,
            Operand::Immediate(2),
            Some(Relative::Negative),
        );
        i.flags.overflow = false;
        i.flags.zero = false;
        let expected = Err(ExecuteError::ProgramCounterUnderflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfGreater, Operand::Immediate(5), None);
        i.flags.overflow = false;
        i.flags.zero = false;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfGreater, Operand::Register(Register::A), None);
        i.registers.set(Register::A, 5);
        i.flags.overflow = false;
        i.flags.zero = false;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_positive_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfGreater,
            Operand::Immediate(2),
            Some(Relative::Positive),
        );
        i.flags.overflow = false;
        i.flags.zero = false;
        let expected = 3;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_negative_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfGreater,
            Operand::Immediate(2),
            Some(Relative::Negative),
        );
        i.flags.overflow = false;
        i.flags.zero = false;
        i.program_counter = 3;
        let expected = 1;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_positive_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfGreater,
            Operand::Register(Register::A),
            Some(Relative::Positive),
        );
        i.flags.overflow = false;
        i.flags.zero = false;
        i.registers.set(Register::A, 2);
        let expected = 3;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_negative_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfGreater,
            Operand::Register(Register::A),
            Some(Relative::Negative),
        );
        i.flags.overflow = false;
        i.flags.zero = false;
        i.program_counter = 3;
        i.registers.set(Register::A, 2);
        let expected = 1;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }
}

#[cfg(test)]
mod if_lesser {
    use crate::{
        constant::Word,
        error::ExecuteError,
        instruction::{Instruction, Jump, Relative},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn program_counter_overflow_error() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfLesser,
            Operand::Immediate(1),
            Some(Relative::Positive),
        );
        i.flags.overflow = true;
        i.flags.zero = false;
        i.program_counter = Word::MAX;
        let expected = Err(ExecuteError::ProgramCounterOverflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn program_counter_underflow_error() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfLesser,
            Operand::Immediate(2),
            Some(Relative::Negative),
        );
        i.flags.overflow = true;
        i.flags.zero = false;
        let expected = Err(ExecuteError::ProgramCounterUnderflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfLesser, Operand::Immediate(5), None);
        i.flags.overflow = true;
        i.flags.zero = false;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfLesser, Operand::Register(Register::A), None);
        i.flags.overflow = true;
        i.flags.zero = false;
        i.registers.set(Register::A, 5);
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_positive_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfLesser,
            Operand::Immediate(2),
            Some(Relative::Positive),
        );
        i.flags.overflow = true;
        i.flags.zero = false;
        let expected = 3;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_negative_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfLesser,
            Operand::Immediate(2),
            Some(Relative::Negative),
        );
        i.flags.overflow = true;
        i.flags.zero = false;
        i.program_counter = 3;
        let expected = 1;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_positive_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfLesser,
            Operand::Register(Register::A),
            Some(Relative::Positive),
        );
        i.flags.overflow = true;
        i.flags.zero = false;
        i.registers.set(Register::A, 2);
        let expected = 3;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn relative_negative_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfLesser,
            Operand::Register(Register::A),
            Some(Relative::Negative),
        );
        i.flags.overflow = true;
        i.flags.zero = false;
        i.program_counter = 3;
        i.registers.set(Register::A, 2);
        let expected = 1;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }
}

#[cfg(test)]
mod if_greater_or_equal {
    use crate::{
        constant::Word,
        error::ExecuteError,
        instruction::{Instruction, Jump, Relative},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn program_counter_overflow_error() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfGreaterOrEqual,
            Operand::Immediate(1),
            Some(Relative::Positive),
        );
        i.flags.overflow = false;
        i.flags.zero = false;
        i.program_counter = Word::MAX;
        let expected = Err(ExecuteError::ProgramCounterOverflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn program_counter_underflow_error() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfGreaterOrEqual,
            Operand::Immediate(2),
            Some(Relative::Negative),
        );
        i.flags.overflow = false;
        i.flags.zero = false;
        let expected = Err(ExecuteError::ProgramCounterUnderflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn greater_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfGreaterOrEqual, Operand::Immediate(5), None);
        i.flags.overflow = false;
        i.flags.zero = false;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn greater_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Jump(Jump::IfGreaterOrEqual, Operand::Register(Register::A), None);
        i.registers.set(Register::A, 5);
        i.flags.overflow = false;
        i.flags.zero = false;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn equal_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfGreaterOrEqual, Operand::Immediate(5), None);
        i.flags.overflow = false;
        i.flags.zero = true;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn equal_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Jump(Jump::IfGreaterOrEqual, Operand::Register(Register::A), None);
        i.registers.set(Register::A, 5);
        i.flags.overflow = false;
        i.flags.zero = true;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn greater_relative_positive_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfGreaterOrEqual,
            Operand::Immediate(2),
            Some(Relative::Positive),
        );
        i.flags.overflow = false;
        i.flags.zero = false;
        let expected = 3;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn greater_relative_negative_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfGreaterOrEqual,
            Operand::Immediate(2),
            Some(Relative::Negative),
        );
        i.flags.overflow = false;
        i.flags.zero = false;
        i.program_counter = 3;
        let expected = 1;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn greater_relative_positive_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfGreaterOrEqual,
            Operand::Register(Register::A),
            Some(Relative::Positive),
        );
        i.flags.overflow = false;
        i.flags.zero = false;
        i.registers.set(Register::A, 2);
        let expected = 3;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn greater_relative_negative_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfGreaterOrEqual,
            Operand::Register(Register::A),
            Some(Relative::Negative),
        );
        i.flags.overflow = false;
        i.flags.zero = false;
        i.program_counter = 3;
        i.registers.set(Register::A, 2);
        let expected = 1;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn equal_relative_positive_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfGreaterOrEqual,
            Operand::Immediate(2),
            Some(Relative::Positive),
        );
        i.flags.overflow = false;
        i.flags.zero = true;
        let expected = 3;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn equal_relative_negative_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfGreaterOrEqual,
            Operand::Immediate(2),
            Some(Relative::Negative),
        );
        i.flags.overflow = false;
        i.flags.zero = true;
        i.program_counter = 3;
        let expected = 1;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn equal_relative_positive_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfGreaterOrEqual,
            Operand::Register(Register::A),
            Some(Relative::Positive),
        );
        i.flags.overflow = false;
        i.flags.zero = true;
        i.registers.set(Register::A, 2);
        let expected = 3;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn equal_relative_negative_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfGreaterOrEqual,
            Operand::Register(Register::A),
            Some(Relative::Negative),
        );
        i.flags.overflow = false;
        i.flags.zero = true;
        i.program_counter = 3;
        i.registers.set(Register::A, 2);
        let expected = 1;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }
}

#[cfg(test)]
mod if_lesser_or_equal {
    use crate::{
        constant::Word,
        error::ExecuteError,
        instruction::{Instruction, Jump, Relative},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn program_counter_overflow_error() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfLesserOrEqual,
            Operand::Immediate(1),
            Some(Relative::Positive),
        );
        i.flags.overflow = true;
        i.flags.zero = false;
        i.program_counter = Word::MAX;
        let expected = Err(ExecuteError::ProgramCounterOverflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn program_counter_underflow_error() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfLesserOrEqual,
            Operand::Immediate(2),
            Some(Relative::Negative),
        );
        i.flags.overflow = true;
        i.flags.zero = false;
        let expected = Err(ExecuteError::ProgramCounterUnderflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn lesser_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfLesserOrEqual, Operand::Immediate(5), None);
        i.flags.overflow = true;
        i.flags.zero = false;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn lesser_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Jump(Jump::IfLesserOrEqual, Operand::Register(Register::A), None);
        i.registers.set(Register::A, 5);
        i.flags.overflow = true;
        i.flags.zero = false;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn equal_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfLesserOrEqual, Operand::Immediate(5), None);
        i.flags.overflow = false;
        i.flags.zero = true;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn equal_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Jump(Jump::IfLesserOrEqual, Operand::Register(Register::A), None);
        i.registers.set(Register::A, 5);
        i.flags.overflow = false;
        i.flags.zero = true;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn lesser_relative_positive_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfLesserOrEqual,
            Operand::Immediate(2),
            Some(Relative::Positive),
        );
        i.flags.overflow = true;
        i.flags.zero = false;
        let expected = 3;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn lesser_relative_negative_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfLesserOrEqual,
            Operand::Immediate(2),
            Some(Relative::Negative),
        );
        i.flags.overflow = true;
        i.flags.zero = false;
        i.program_counter = 3;
        let expected = 1;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn lesser_relative_positive_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfLesserOrEqual,
            Operand::Register(Register::A),
            Some(Relative::Positive),
        );
        i.flags.overflow = true;
        i.flags.zero = false;
        i.registers.set(Register::A, 2);
        let expected = 3;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn lesser_relative_negative_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfLesserOrEqual,
            Operand::Register(Register::A),
            Some(Relative::Negative),
        );
        i.flags.overflow = true;
        i.flags.zero = false;
        i.program_counter = 3;
        i.registers.set(Register::A, 2);
        let expected = 1;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn equal_relative_positive_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfLesserOrEqual,
            Operand::Immediate(2),
            Some(Relative::Positive),
        );
        i.flags.overflow = false;
        i.flags.zero = true;
        let expected = 3;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn equal_relative_negative_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfLesserOrEqual,
            Operand::Immediate(2),
            Some(Relative::Negative),
        );
        i.flags.overflow = false;
        i.flags.zero = true;
        i.program_counter = 3;
        let expected = 1;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn equal_relative_positive_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfLesserOrEqual,
            Operand::Register(Register::A),
            Some(Relative::Positive),
        );
        i.flags.overflow = false;
        i.flags.zero = true;
        i.registers.set(Register::A, 2);
        let expected = 3;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn equal_relative_negative_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(
            Jump::IfLesserOrEqual,
            Operand::Register(Register::A),
            Some(Relative::Negative),
        );
        i.flags.overflow = false;
        i.flags.zero = true;
        i.program_counter = 3;
        i.registers.set(Register::A, 2);
        let expected = 1;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }
}
