use crate::{constant::Word, instruction::Jump, operand::Operand, Processor};

impl Processor {
    pub fn jump(&mut self, instruction: Jump) {
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
    use crate::{operand::Operand, register::Register, Processor};

    #[test]
    fn jump_immediate() {
        let mut p = Processor::test_instance();
        let expected = 5;

        p.jump_unconditional(Operand::Immediate(5));

        assert_eq!(p.program_counter, expected);
    }

    #[test]
    fn jump_register() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = 5;
        let expected = 5;

        p.jump_unconditional(Operand::Register(Register::A));

        assert_eq!(p.program_counter, expected);
    }
}

#[cfg(test)]
mod if_zero {
    use crate::{operand::Operand, register::Register, Processor};

    #[test]
    fn jump_immediate() {
        let mut p = Processor::test_instance();
        p.flags.zero = true;
        let expected = 5;

        p.jump_if_zero(Operand::Immediate(5));

        assert_eq!(p.program_counter, expected);
    }

    #[test]
    fn jump_register() {
        let mut p = Processor::test_instance();
        p.flags.zero = true;
        p.registers[Register::A] = 5;
        let expected = 5;

        p.jump_if_zero(Operand::Register(Register::A));

        assert_eq!(p.program_counter, expected);
    }
}

#[cfg(test)]
mod if_not_zero {
    use crate::{operand::Operand, register::Register, Processor};

    #[test]
    fn jump_immediate() {
        let mut p = Processor::test_instance();
        let expected = 5;

        p.jump_if_not_zero(Operand::Immediate(5));

        assert_eq!(p.program_counter, expected);
    }

    #[test]
    fn jump_register() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = 5;
        let expected = 5;

        p.jump_if_not_zero(Operand::Register(Register::A));

        assert_eq!(p.program_counter, expected);
    }
}

#[cfg(test)]
mod if_sign {
    use crate::{operand::Operand, register::Register, Processor};

    #[test]
    fn jump_immediate() {
        let mut p = Processor::test_instance();
        p.flags.sign = true;
        let expected = 5;

        p.jump_if_sign(Operand::Immediate(5));

        assert_eq!(p.program_counter, expected);
    }

    #[test]
    fn jump_register() {
        let mut p = Processor::test_instance();
        p.flags.sign = true;
        p.registers[Register::A] = 5;
        let expected = 5;

        p.jump_if_sign(Operand::Register(Register::A));

        assert_eq!(p.program_counter, expected);
    }
}

#[cfg(test)]
mod if_not_sign {
    use crate::{operand::Operand, register::Register, Processor};

    #[test]
    fn jump_immediate() {
        let mut p = Processor::test_instance();
        let expected = 5;

        p.jump_if_not_sign(Operand::Immediate(5));

        assert_eq!(p.program_counter, expected);
    }

    #[test]
    fn jump_register() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = 5;
        let expected = 5;

        p.jump_if_not_sign(Operand::Register(Register::A));

        assert_eq!(p.program_counter, expected);
    }
}

#[cfg(test)]
mod if_overflow {
    use crate::{operand::Operand, register::Register, Processor};

    #[test]
    fn jump_immediate() {
        let mut p = Processor::test_instance();
        p.flags.overflow = true;
        let expected = 5;

        p.jump_if_overflow(Operand::Immediate(5));

        assert_eq!(p.program_counter, expected);
    }

    #[test]
    fn jump_register() {
        let mut p = Processor::test_instance();
        p.flags.overflow = true;
        p.registers[Register::A] = 5;
        let expected = 5;

        p.jump_if_overflow(Operand::Register(Register::A));

        assert_eq!(p.program_counter, expected);
    }
}

#[cfg(test)]
mod if_not_overflow {
    use crate::{operand::Operand, register::Register, Processor};

    #[test]
    fn jump_immediate() {
        let mut p = Processor::test_instance();
        let expected = 5;

        p.jump_if_not_overflow(Operand::Immediate(5));

        assert_eq!(p.program_counter, expected);
    }

    #[test]
    fn jump_register() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = 5;
        let expected = 5;

        p.jump_if_not_overflow(Operand::Register(Register::A));

        assert_eq!(p.program_counter, expected);
    }
}

#[cfg(test)]
mod if_greater {
    use crate::{operand::Operand, register::Register, Processor};

    #[test]
    fn jump_immediate() {
        let mut p = Processor::test_instance();
        let expected = 5;

        p.jump_if_greater(Operand::Immediate(5));

        assert_eq!(p.program_counter, expected);
    }

    #[test]
    fn jump_register() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = 5;
        let expected = 5;

        p.jump_if_greater(Operand::Register(Register::A));

        assert_eq!(p.program_counter, expected);
    }
}

#[cfg(test)]
mod if_lesser {
    use crate::{operand::Operand, register::Register, Processor};

    #[test]
    fn jump_immediate() {
        let mut p = Processor::test_instance();
        p.flags.overflow = true;
        let expected = 5;

        p.jump_if_lesser(Operand::Immediate(5));

        assert_eq!(p.program_counter, expected);
    }

    #[test]
    fn jump_register() {
        let mut p = Processor::test_instance();
        p.flags.overflow = true;
        p.registers[Register::A] = 5;
        let expected = 5;

        p.jump_if_lesser(Operand::Register(Register::A));

        assert_eq!(p.program_counter, expected);
    }
}

#[cfg(test)]
mod if_greater_or_equal {
    use crate::{operand::Operand, register::Register, Processor};

    #[test]
    fn jump_greater_immediate() {
        let mut p = Processor::test_instance();
        let expected = 5;

        p.jump_if_greater_or_equal(Operand::Immediate(5));

        assert_eq!(p.program_counter, expected);
    }

    #[test]
    fn jump_greater_register() {
        let mut p = Processor::test_instance();
        p.registers[Register::A] = 5;
        let expected = 5;

        p.jump_if_greater_or_equal(Operand::Register(Register::A));

        assert_eq!(p.program_counter, expected);
    }

    #[test]
    fn jump_equal_immediate() {
        let mut p = Processor::test_instance();
        p.flags.zero = true;
        let expected = 5;

        p.jump_if_greater_or_equal(Operand::Immediate(5));

        assert_eq!(p.program_counter, expected);
    }

    #[test]
    fn jump_equal_register() {
        let mut p = Processor::test_instance();
        p.flags.zero = true;
        p.registers[Register::A] = 5;
        let expected = 5;

        p.jump_if_greater_or_equal(Operand::Register(Register::A));

        assert_eq!(p.program_counter, expected);
    }
}

#[cfg(test)]
mod if_lesser_or_equal {
    use crate::{operand::Operand, register::Register, Processor};

    #[test]
    fn jump_lesser_immediate() {
        let mut p = Processor::test_instance();
        p.flags.overflow = true;
        let expected = 5;

        p.jump_if_lesser_or_equal(Operand::Immediate(5));

        assert_eq!(p.program_counter, expected);
    }

    #[test]
    fn jump_lesser_register() {
        let mut p = Processor::test_instance();
        p.flags.overflow = true;
        p.registers[Register::A] = 5;
        let expected = 5;

        p.jump_if_lesser_or_equal(Operand::Register(Register::A));

        assert_eq!(p.program_counter, expected);
    }

    #[test]
    fn jump_equal_immediate() {
        let mut p = Processor::test_instance();
        p.flags.zero = true;
        let expected = 5;

        p.jump_if_lesser_or_equal(Operand::Immediate(5));

        assert_eq!(p.program_counter, expected);
    }

    #[test]
    fn jump_equal_register() {
        let mut p = Processor::test_instance();
        p.flags.zero = true;
        p.registers[Register::A] = 5;
        let expected = 5;

        p.jump_if_lesser_or_equal(Operand::Register(Register::A));

        assert_eq!(p.program_counter, expected);
    }
}
