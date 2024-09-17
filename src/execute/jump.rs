use crate::{constant::Word, instruction::Jump, operand::Operand, Interpreter};

impl Interpreter {
    pub(super) fn jump(&mut self, instruction: Jump) {
        match instruction {
            Jump::Unconditional(o) => self.jump_unconditional(o),
            Jump::IfZero(o) => self.jump_if_zero(o),
            Jump::IfNotZero(o) => self.jump_if_not_zero(o),
            Jump::IfSign(o) => self.jump_if_sign(o),
            Jump::IfNotSign(o) => self.jump_if_not_sign(o),
            Jump::IfOverflow(o) => self.jump_if_overflow(o),
            Jump::IfNotOverflow(o) => self.jump_if_not_overflow(o),
            Jump::IfGreater(o) => self.jump_if_greater(o),
            Jump::IfLesser(o) => self.jump_if_lesser(o),
            Jump::IfGreaterOrEqual(o) => self.jump_if_greater_or_equal(o),
            Jump::IfLesserOrEqual(o) => self.jump_if_lesser_or_equal(o),
        }
    }

    fn jump_unconditional(&mut self, operand: Operand<Word>) {
        self.program_counter = self.get_operand_value(operand);
    }

    fn jump_if_zero(&mut self, operand: Operand<Word>) {
        if self.flags.zero {
            self.program_counter = self.get_operand_value(operand);
        }
    }

    fn jump_if_not_zero(&mut self, operand: Operand<Word>) {
        if !self.flags.zero {
            self.program_counter = self.get_operand_value(operand);
        }
    }

    fn jump_if_sign(&mut self, operand: Operand<Word>) {
        if self.flags.sign {
            self.program_counter = self.get_operand_value(operand);
        }
    }

    fn jump_if_not_sign(&mut self, operand: Operand<Word>) {
        if !self.flags.sign {
            self.program_counter = self.get_operand_value(operand);
        }
    }

    fn jump_if_overflow(&mut self, operand: Operand<Word>) {
        if self.flags.overflow {
            self.program_counter = self.get_operand_value(operand);
        }
    }

    fn jump_if_not_overflow(&mut self, operand: Operand<Word>) {
        if !self.flags.overflow {
            self.program_counter = self.get_operand_value(operand);
        }
    }

    fn jump_if_greater(&mut self, operand: Operand<Word>) {
        if !self.flags.overflow && !self.flags.zero {
            self.program_counter = self.get_operand_value(operand);
        }
    }

    fn jump_if_lesser(&mut self, operand: Operand<Word>) {
        if self.flags.overflow && !self.flags.zero {
            self.program_counter = self.get_operand_value(operand);
        }
    }

    fn jump_if_greater_or_equal(&mut self, operand: Operand<Word>) {
        if !self.flags.overflow || self.flags.zero {
            self.program_counter = self.get_operand_value(operand);
        }
    }

    fn jump_if_lesser_or_equal(&mut self, operand: Operand<Word>) {
        if self.flags.overflow || self.flags.zero {
            self.program_counter = self.get_operand_value(operand);
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
    fn jump_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::Unconditional(Operand::Immediate(5)));
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn jump_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::Unconditional(Operand::Register(Register::A)));
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
    fn jump_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfZero(Operand::Immediate(5)));
        i.flags.zero = true;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn jump_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfZero(Operand::Register(Register::A)));
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
    fn jump_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfNotZero(Operand::Immediate(5)));
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn jump_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfNotZero(Operand::Register(Register::A)));
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
    fn jump_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfSign(Operand::Immediate(5)));
        i.flags.sign = true;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn jump_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfSign(Operand::Register(Register::A)));
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
    fn jump_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfNotSign(Operand::Immediate(5)));
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn jump_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfNotSign(Operand::Register(Register::A)));
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
    fn jump_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfOverflow(Operand::Immediate(5)));
        i.flags.overflow = true;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn jump_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfOverflow(Operand::Register(Register::A)));
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
    fn jump_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfNotOverflow(Operand::Immediate(5)));
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn jump_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfNotOverflow(Operand::Register(Register::A)));
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
    fn jump_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfGreater(Operand::Immediate(5)));
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn jump_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfGreater(Operand::Register(Register::A)));
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
    fn jump_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfLesser(Operand::Immediate(5)));
        i.flags.overflow = true;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn jump_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfLesser(Operand::Register(Register::A)));
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
    fn jump_greater_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfGreaterOrEqual(Operand::Immediate(5)));
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn jump_greater_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfGreaterOrEqual(Operand::Register(Register::A)));
        i.registers.set(Register::A, 5);
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn jump_equal_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfGreaterOrEqual(Operand::Immediate(5)));
        i.flags.zero = true;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn jump_equal_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfGreaterOrEqual(Operand::Register(Register::A)));
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
    fn jump_greater_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfLesserOrEqual(Operand::Immediate(5)));
        i.flags.overflow = true;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn jump_greater_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfLesserOrEqual(Operand::Register(Register::A)));
        i.registers.set(Register::A, 5);
        i.flags.overflow = true;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn jump_equal_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfLesserOrEqual(Operand::Immediate(5)));
        i.flags.zero = true;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }

    #[test]
    fn jump_equal_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Jump(Jump::IfLesserOrEqual(Operand::Register(Register::A)));
        i.registers.set(Register::A, 5);
        i.flags.zero = true;
        let expected = 5;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }
}
