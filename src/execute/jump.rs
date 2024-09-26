use crate::{
    constant::Word,
    instruction::{Jump, Relative},
    operand::Operand,
    Interpreter,
};

impl Interpreter {
    pub(super) fn jump(
        &mut self,
        instruction: Jump,
        operand: Operand<Word>,
        relative: Option<Relative>,
    ) {
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

            match relative {
                Some(Relative::Positive) => self.program_counter += destination,
                Some(Relative::Negative) => self.program_counter -= destination,
                None => self.program_counter = destination,
            }
        }
    }
}

// The test cases below are really only to prevent regression at a future point.

#[cfg(test)]
mod unconditional {
    use crate::{
        error::ExecuteError,
        instruction::{Instruction, Jump},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

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
}

#[cfg(test)]
mod if_zero {
    use crate::{
        error::ExecuteError,
        instruction::{Instruction, Jump},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

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
}

#[cfg(test)]
mod if_not_zero {
    use crate::{
        error::ExecuteError,
        instruction::{Instruction, Jump},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfNotZero, Operand::Immediate(5), None);
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
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }
}

#[cfg(test)]
mod if_sign {
    use crate::{
        error::ExecuteError,
        instruction::{Instruction, Jump},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

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
}

#[cfg(test)]
mod if_not_sign {
    use crate::{
        error::ExecuteError,
        instruction::{Instruction, Jump},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfNotSign, Operand::Immediate(5), None);
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfNotSign, Operand::Register(Register::A), None);
        i.registers.set(Register::A, 5);
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }
}

#[cfg(test)]
mod if_overflow {
    use crate::{
        error::ExecuteError,
        instruction::{Instruction, Jump},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

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
}

#[cfg(test)]
mod if_not_overflow {
    use crate::{
        error::ExecuteError,
        instruction::{Instruction, Jump},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfNotOverflow, Operand::Immediate(5), None);
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
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }
}

#[cfg(test)]
mod if_greater {
    use crate::{
        error::ExecuteError,
        instruction::{Instruction, Jump},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfGreater, Operand::Immediate(5), None);
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
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }
}

#[cfg(test)]
mod if_lesser {
    use crate::{
        error::ExecuteError,
        instruction::{Instruction, Jump},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfLesser, Operand::Immediate(5), None);
        i.flags.overflow = true;
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
        i.registers.set(Register::A, 5);
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }
}

#[cfg(test)]
mod if_greater_or_equal {
    use crate::{
        error::ExecuteError,
        instruction::{Instruction, Jump},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn greater_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfGreaterOrEqual, Operand::Immediate(5), None);
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
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn equal_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfGreaterOrEqual, Operand::Immediate(5), None);
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
        i.flags.zero = true;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }
}

#[cfg(test)]
mod if_lesser_or_equal {
    use crate::{
        error::ExecuteError,
        instruction::{Instruction, Jump},
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn greater_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfLesserOrEqual, Operand::Immediate(5), None);
        i.flags.overflow = true;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn greater_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction =
            Instruction::Jump(Jump::IfLesserOrEqual, Operand::Register(Register::A), None);
        i.registers.set(Register::A, 5);
        i.flags.overflow = true;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn equal_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfLesserOrEqual, Operand::Immediate(5), None);
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
        i.flags.zero = true;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }
}
