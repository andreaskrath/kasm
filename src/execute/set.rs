use crate::{instruction::Set, operand::Operand, register::Register, utils::FromBytes, Processor};

impl Processor {
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
        T: FromBytes,
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
        Processor,
    };

    #[test]
    fn set_byte() {
        let mut p = Processor::new().unwrap();
        let expected = Byte::MAX;

        p.set_value(Register::A, Operand::Immediate(Byte::MAX));

        assert_eq!(p.registers[Register::A], expected as Word);
    }
}

#[cfg(test)]
mod quarter {
    use crate::{
        constant::{Quarter, Word},
        operand::Operand,
        register::Register,
        Processor,
    };

    #[test]
    fn set_quarter() {
        let mut p = Processor::new().unwrap();
        let expected = Quarter::MAX;

        p.set_value(Register::A, Operand::Immediate(Quarter::MAX));

        assert_eq!(p.registers[Register::A], expected as Word);
    }
}

#[cfg(test)]
mod half {
    use crate::{
        constant::{Half, Word},
        operand::Operand,
        register::Register,
        Processor,
    };

    #[test]
    fn set_half() {
        let mut p = Processor::new().unwrap();
        let expected = Half::MAX;

        p.set_value(Register::A, Operand::Immediate(Half::MAX));

        assert_eq!(p.registers[Register::A], expected as Word);
    }
}

#[cfg(test)]
mod word {
    use crate::{constant::Word, operand::Operand, register::Register, Processor};

    #[test]
    fn set_word() {
        let mut p = Processor::new().unwrap();
        let expected = Word::MAX;

        p.set_value(Register::A, Operand::Immediate(Word::MAX));

        assert_eq!(p.registers[Register::A], expected);
    }
}
