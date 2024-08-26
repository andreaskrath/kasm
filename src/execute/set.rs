use crate::{
    instruction::Set,
    operand::Operand,
    register::Register,
    utils::{FromBytes, ToWord},
    Interpreter,
};

impl Interpreter {
    pub fn set(&mut self, instruction: Set) {
        match instruction {
            Set::Byte(r, o) => self.set_value(r, o),
            Set::Quarter(r, o) => self.set_value(r, o),
            Set::Half(r, o) => self.set_value(r, o),
            Set::Word(r, o) => self.set_value(r, o),
        }
    }

    fn set_value<T>(&mut self, register: Register, operand: Operand<T>)
    where
        T: FromBytes + ToWord,
    {
        let value = self.get_operand_value(operand);
        self.registers[register] = value.to_word();
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
    fn set_from_immediate() {
        let mut p = Interpreter::new_test();
        let expected = Byte::MAX as Word;

        p.set_value(Register::A, Operand::Immediate(Byte::MAX));

        assert_eq!(p.registers[Register::A], expected);
    }

    #[test]
    fn set_from_register() {
        let mut p = Interpreter::new_test();
        p.registers[Register::B] = Byte::MAX as Word;
        let expected = Byte::MAX as Word;

        p.set_value::<Byte>(Register::A, Operand::Register(Register::B));

        assert_eq!(p.registers[Register::A], expected);
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
    fn set_from_immediate() {
        let mut p = Interpreter::new_test();
        let expected = Quarter::MAX as Word;

        p.set_value(Register::A, Operand::Immediate(Quarter::MAX));

        assert_eq!(p.registers[Register::A], expected);
    }

    #[test]
    fn set_from_register() {
        let mut p = Interpreter::new_test();
        p.registers[Register::B] = Quarter::MAX as Word;
        let expected = Quarter::MAX as Word;

        p.set_value::<Quarter>(Register::A, Operand::Register(Register::B));

        assert_eq!(p.registers[Register::A], expected);
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
    fn set_from_immediate() {
        let mut p = Interpreter::new_test();
        let expected = Half::MAX as Word;

        p.set_value(Register::A, Operand::Immediate(Half::MAX));

        assert_eq!(p.registers[Register::A], expected);
    }

    #[test]
    fn set_from_register() {
        let mut p = Interpreter::new_test();
        p.registers[Register::B] = Half::MAX as Word;
        let expected = Half::MAX as Word;

        p.set_value::<Half>(Register::A, Operand::Register(Register::B));

        assert_eq!(p.registers[Register::A], expected);
    }
}

#[cfg(test)]
mod word {
    use crate::{constant::Word, operand::Operand, register::Register, Interpreter};

    #[test]
    fn set_from_immediate() {
        let mut p = Interpreter::new_test();
        let expected = Word::MAX;

        p.set_value(Register::A, Operand::Immediate(Word::MAX));

        assert_eq!(p.registers[Register::A], expected);
    }

    #[test]
    fn set_from_register() {
        let mut p = Interpreter::new_test();
        p.registers[Register::B] = Word::MAX;
        let expected = Word::MAX;

        p.set_value::<Word>(Register::A, Operand::Register(Register::B));

        assert_eq!(p.registers[Register::A], expected);
    }
}
