use crate::{instruction::Compare, operand::Operand, utils::Arithmetic, Interpreter};

impl Interpreter {
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

#[cfg(test)]
mod byte {
    use crate::{
        constant::{Byte, Word},
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn a_and_b_equal_immediate() {
        let mut p = Interpreter::new_test();

        p.compare_value(Operand::Immediate(Byte::MAX), Operand::Immediate(Byte::MAX));

        assert!(p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(!p.flags.sign);
    }

    #[test]
    fn a_greater_than_b_immediate() {
        let mut p = Interpreter::new_test();

        p.compare_value(Operand::Immediate(Byte::MAX), Operand::Immediate(0));

        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(p.flags.sign);
    }

    #[test]
    fn a_lesser_than_b_immediate() {
        let mut p = Interpreter::new_test();

        p.compare_value::<Byte>(Operand::Immediate(0), Operand::Immediate(1));

        assert!(!p.flags.zero);
        assert!(p.flags.overflow);
        assert!(p.flags.sign);
    }

    #[test]
    fn a_and_b_equal_register() {
        let mut p = Interpreter::new_test();
        p.registers[Register::A] = Byte::MAX as Word;
        p.registers[Register::B] = Byte::MAX as Word;

        p.compare_value::<Byte>(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        );

        assert!(p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(!p.flags.sign);
    }

    #[test]
    fn a_greater_than_b_register() {
        let mut p = Interpreter::new_test();
        p.registers[Register::A] = Byte::MAX as Word;

        p.compare_value::<Byte>(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        );

        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(p.flags.sign);
    }

    #[test]
    fn a_lesser_than_b_register() {
        let mut p = Interpreter::new_test();
        p.registers[Register::B] = 1;

        p.compare_value::<Byte>(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        );

        assert!(!p.flags.zero);
        assert!(p.flags.overflow);
        assert!(p.flags.sign);
    }

    #[test]
    fn a_and_b_equal_mix() {
        let mut p = Interpreter::new_test();
        p.registers[Register::A] = Byte::MAX as Word;

        p.compare_value(
            Operand::Register(Register::A),
            Operand::Immediate(Byte::MAX),
        );

        assert!(p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(!p.flags.sign);
    }

    #[test]
    fn a_greater_than_b_mix() {
        let mut p = Interpreter::new_test();
        p.registers[Register::A] = Byte::MAX as Word;

        p.compare_value::<Byte>(Operand::Register(Register::A), Operand::Immediate(0));

        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(p.flags.sign);
    }

    #[test]
    fn a_lesser_than_b_mix() {
        let mut p = Interpreter::new_test();

        p.compare_value::<Byte>(Operand::Register(Register::A), Operand::Immediate(1));

        assert!(!p.flags.zero);
        assert!(p.flags.overflow);
        assert!(p.flags.sign);
    }
}

#[cfg(test)]
mod quarter {
    use crate::{
        constant::{Quarter, Word},
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn a_and_b_equal_immediate() {
        let mut p = Interpreter::new_test();

        p.compare_value(
            Operand::Immediate(Quarter::MAX),
            Operand::Immediate(Quarter::MAX),
        );

        assert!(p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(!p.flags.sign);
    }

    #[test]
    fn a_greater_than_b_immediate() {
        let mut p = Interpreter::new_test();

        p.compare_value(Operand::Immediate(Quarter::MAX), Operand::Immediate(0));

        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(p.flags.sign);
    }

    #[test]
    fn a_lesser_than_b_immediate() {
        let mut p = Interpreter::new_test();

        p.compare_value::<Quarter>(Operand::Immediate(0), Operand::Immediate(1));

        assert!(!p.flags.zero);
        assert!(p.flags.overflow);
        assert!(p.flags.sign);
    }

    #[test]
    fn a_and_b_equal_register() {
        let mut p = Interpreter::new_test();
        p.registers[Register::A] = Quarter::MAX as Word;
        p.registers[Register::B] = Quarter::MAX as Word;

        p.compare_value::<Quarter>(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        );

        assert!(p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(!p.flags.sign);
    }

    #[test]
    fn a_greater_than_b_register() {
        let mut p = Interpreter::new_test();
        p.registers[Register::A] = Quarter::MAX as Word;

        p.compare_value::<Quarter>(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        );

        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(p.flags.sign);
    }

    #[test]
    fn a_lesser_than_b_register() {
        let mut p = Interpreter::new_test();
        p.registers[Register::B] = 1;

        p.compare_value::<Quarter>(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        );

        assert!(!p.flags.zero);
        assert!(p.flags.overflow);
        assert!(p.flags.sign);
    }

    #[test]
    fn a_and_b_equal_mix() {
        let mut p = Interpreter::new_test();
        p.registers[Register::A] = Quarter::MAX as Word;

        p.compare_value(
            Operand::Register(Register::A),
            Operand::Immediate(Quarter::MAX),
        );

        assert!(p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(!p.flags.sign);
    }

    #[test]
    fn a_greater_than_b_mix() {
        let mut p = Interpreter::new_test();
        p.registers[Register::A] = Quarter::MAX as Word;

        p.compare_value::<Quarter>(Operand::Register(Register::A), Operand::Immediate(0));

        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(p.flags.sign);
    }

    #[test]
    fn a_lesser_than_b_mix() {
        let mut p = Interpreter::new_test();

        p.compare_value::<Quarter>(Operand::Register(Register::A), Operand::Immediate(1));

        assert!(!p.flags.zero);
        assert!(p.flags.overflow);
        assert!(p.flags.sign);
    }
}

#[cfg(test)]
mod half {
    use crate::{
        constant::{Half, Word},
        operand::Operand,
        register::Register,
        Interpreter,
    };

    #[test]
    fn a_and_b_equal_immediate() {
        let mut p = Interpreter::new_test();

        p.compare_value(Operand::Immediate(Half::MAX), Operand::Immediate(Half::MAX));

        assert!(p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(!p.flags.sign);
    }

    #[test]
    fn a_greater_than_b_immediate() {
        let mut p = Interpreter::new_test();

        p.compare_value(Operand::Immediate(Half::MAX), Operand::Immediate(0));

        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(p.flags.sign);
    }

    #[test]
    fn a_lesser_than_b_immediate() {
        let mut p = Interpreter::new_test();

        p.compare_value::<Half>(Operand::Immediate(0), Operand::Immediate(1));

        assert!(!p.flags.zero);
        assert!(p.flags.overflow);
        assert!(p.flags.sign);
    }

    #[test]
    fn a_and_b_equal_register() {
        let mut p = Interpreter::new_test();
        p.registers[Register::A] = Half::MAX as Word;
        p.registers[Register::B] = Half::MAX as Word;

        p.compare_value::<Half>(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        );

        assert!(p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(!p.flags.sign);
    }

    #[test]
    fn a_greater_than_b_register() {
        let mut p = Interpreter::new_test();
        p.registers[Register::A] = Half::MAX as Word;

        p.compare_value::<Half>(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        );

        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(p.flags.sign);
    }

    #[test]
    fn a_lesser_than_b_register() {
        let mut p = Interpreter::new_test();
        p.registers[Register::B] = 1;

        p.compare_value::<Half>(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        );

        assert!(!p.flags.zero);
        assert!(p.flags.overflow);
        assert!(p.flags.sign);
    }

    #[test]
    fn a_and_b_equal_mix() {
        let mut p = Interpreter::new_test();
        p.registers[Register::A] = Half::MAX as Word;

        p.compare_value(
            Operand::Register(Register::A),
            Operand::Immediate(Half::MAX),
        );

        assert!(p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(!p.flags.sign);
    }

    #[test]
    fn a_greater_than_b_mix() {
        let mut p = Interpreter::new_test();
        p.registers[Register::A] = Half::MAX as Word;

        p.compare_value::<Half>(Operand::Register(Register::A), Operand::Immediate(0));

        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(p.flags.sign);
    }

    #[test]
    fn a_lesser_than_b_mix() {
        let mut p = Interpreter::new_test();

        p.compare_value::<Half>(Operand::Register(Register::A), Operand::Immediate(1));

        assert!(!p.flags.zero);
        assert!(p.flags.overflow);
        assert!(p.flags.sign);
    }
}

#[cfg(test)]
mod word {
    use crate::{constant::Word, operand::Operand, register::Register, Interpreter};

    #[test]
    fn a_and_b_equal_immediate() {
        let mut p = Interpreter::new_test();

        p.compare_value(Operand::Immediate(Word::MAX), Operand::Immediate(Word::MAX));

        assert!(p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(!p.flags.sign);
    }

    #[test]
    fn a_greater_than_b_immediate() {
        let mut p = Interpreter::new_test();

        p.compare_value(Operand::Immediate(Word::MAX), Operand::Immediate(0));

        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(p.flags.sign);
    }

    #[test]
    fn a_lesser_than_b_immediate() {
        let mut p = Interpreter::new_test();

        p.compare_value::<Word>(Operand::Immediate(0), Operand::Immediate(1));

        assert!(!p.flags.zero);
        assert!(p.flags.overflow);
        assert!(p.flags.sign);
    }

    #[test]
    fn a_and_b_equal_register() {
        let mut p = Interpreter::new_test();
        p.registers[Register::A] = Word::MAX;
        p.registers[Register::B] = Word::MAX;

        p.compare_value::<Word>(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        );

        assert!(p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(!p.flags.sign);
    }

    #[test]
    fn a_greater_than_b_register() {
        let mut p = Interpreter::new_test();
        p.registers[Register::A] = Word::MAX;

        p.compare_value::<Word>(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        );

        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(p.flags.sign);
    }

    #[test]
    fn a_lesser_than_b_register() {
        let mut p = Interpreter::new_test();
        p.registers[Register::B] = 1;

        p.compare_value::<Word>(
            Operand::Register(Register::A),
            Operand::Register(Register::B),
        );

        assert!(!p.flags.zero);
        assert!(p.flags.overflow);
        assert!(p.flags.sign);
    }

    #[test]
    fn a_and_b_equal_mix() {
        let mut p = Interpreter::new_test();
        p.registers[Register::A] = Word::MAX;

        p.compare_value(
            Operand::Register(Register::A),
            Operand::Immediate(Word::MAX),
        );

        assert!(p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(!p.flags.sign);
    }

    #[test]
    fn a_greater_than_b_mix() {
        let mut p = Interpreter::new_test();
        p.registers[Register::A] = Word::MAX;

        p.compare_value::<Word>(Operand::Register(Register::A), Operand::Immediate(0));

        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(p.flags.sign);
    }

    #[test]
    fn a_lesser_than_b_mix() {
        let mut p = Interpreter::new_test();

        p.compare_value::<Word>(Operand::Register(Register::A), Operand::Immediate(1));

        assert!(!p.flags.zero);
        assert!(p.flags.overflow);
        assert!(p.flags.sign);
    }
}
