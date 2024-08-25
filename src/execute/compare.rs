use crate::{instruction::Compare, operand::Operand, utils::Arithmetic, Processor};

impl Processor {
    pub fn compare(&mut self, instruction: Compare) {
        match instruction {
            Compare::Byte(o1, o2) => self.compare_value(o1, o2),
            Compare::Quarter(o1, o2) => self.compare_value(o1, o2),
            Compare::Half(o1, o2) => self.compare_value(o1, o2),
            Compare::Word(o1, o2) => self.compare_value(o1, o2),
        }
    }

    fn compare_value<T>(&mut self, operand1: Operand<T>, operand2: Operand<T>)
    where
        T: Arithmetic,
    {
        let a = self.get_operand_value(operand1);
        let b = self.get_operand_value(operand2);
        let (result, overflow) = a.overflow_sub(b);
        self.flags.set(result, overflow);
    }
}
