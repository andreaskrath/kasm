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
