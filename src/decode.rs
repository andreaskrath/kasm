use std::str::SplitWhitespace;
use phf::phf_map;
use crate::{constant::DecodeTable, error::DecodeError, instruction::Instruction, operand::Operand, register::Register, Processor};

/// This compile time assertion ensures that the size of the decode table matches the amount of
/// enum variants in the Instruction enum.
///
/// In other words, it is a way to somewhat ensure that every instruction is represented in the
/// decode table.
const _: () = assert!(DECODE_TABLE.len() == Instruction::VARIANT_COUNT);

pub const DECODE_TABLE: DecodeTable = phf_map! {
    "stop" => Processor::decode_stop,
    "setb" => Processor::decode_set_byte,
    "setq" => Processor::decode_set_quarter,
    "seth" => Processor::decode_set_half,
    "setw" => Processor::decode_set_word,
    "addb" => Processor::decode_add_byte,
    "addq" => Processor::decode_add_quarter,
    "addh" => Processor::decode_add_half,
    "addw" => Processor::decode_add_word,
    "subb" => Processor::decode_sub_byte,
    "subq" => Processor::decode_sub_quarter,
    "subh" => Processor::decode_sub_half,
    "subw" => Processor::decode_sub_word,
    "mulb" => Processor::decode_mul_byte,
    "mulq" => Processor::decode_mul_quarter,
};

fn get_register_and_operand_str(mut iter: SplitWhitespace) -> Result<(&str, &str), DecodeError> {
    let (Some(s_register), Some(s_operand)) = (iter.next(), iter.next()) else {
        return Err(DecodeError::IncompleteInstruction);
    };

    Ok((s_register, s_operand))
}

impl Processor {
    /// A helper to decode instructions where:
    /// - Parameter 1 is a register.
    /// - Parameter 2 is a byte operand.
    fn decode_register_and_byte_operand(&mut self, iter: SplitWhitespace) -> Result<(), DecodeError> {
        let (s_register, s_operand) = get_register_and_operand_str(iter)?;
        let register = Register::try_from(s_register)?;
        let operand = Operand::try_from(s_operand)?;

        self.registers[Register::P1] = register.as_word();
        self.registers[Register::P2] = operand.byte_val_as_word(&self.registers);

        Ok(())
    }

    /// A helper to decode instructions where:
    /// - Parameter 1 is a register.
    /// - Parameter 2 is a quarter operand.
    fn decode_register_and_quarter_operand(&mut self, iter: SplitWhitespace) -> Result<(), DecodeError> {
        let (s_register, s_operand) = get_register_and_operand_str(iter)?;
        let register = Register::try_from(s_register)?;
        let operand = Operand::try_from(s_operand)?;

        self.registers[Register::P1] = register.as_word();
        self.registers[Register::P2] = operand.quarter_val_as_word(&self.registers);

        Ok(())
    }

    /// A helper to decode instructions where:
    /// - Parameter 1 is a register.
    /// - Parameter 2 is a half operand.
    fn decode_register_and_half_operand(&mut self, iter: SplitWhitespace) -> Result<(), DecodeError> {
        let (s_register, s_operand) = get_register_and_operand_str(iter)?;
        let register = Register::try_from(s_register)?;
        let operand = Operand::try_from(s_operand)?;

        self.registers[Register::P1] = register.as_word();
        self.registers[Register::P2] = operand.half_val_as_word(&self.registers);

        Ok(())
    }

    /// A helper to decode instructions where:
    /// - Parameter 1 is a register.
    /// - Parameter 2 is a word operand.
    fn decode_register_and_word_operand(&mut self, iter: SplitWhitespace) -> Result<(), DecodeError> {
        let (s_register, s_operand) = get_register_and_operand_str(iter)?;
        let register = Register::try_from(s_register)?;
        let operand = Operand::try_from(s_operand)?;

        self.registers[Register::P1] = register.as_word();
        self.registers[Register::P2] = operand.word_val(&self.registers);

        Ok(())
    }

    fn decode_stop(&mut self, _iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        Ok(Instruction::Stop)
    }

    fn decode_set_byte(&mut self, iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        self.decode_register_and_byte_operand(iter)?;

        Ok(Instruction::SetByte)
    }

    fn decode_set_quarter(&mut self, iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        self.decode_register_and_quarter_operand(iter)?;

        Ok(Instruction::SetQuarter)
    }

    fn decode_set_half(&mut self, iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        self.decode_register_and_half_operand(iter)?;

        Ok(Instruction::SetHalf)
    }

    fn decode_set_word(&mut self, iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        self.decode_register_and_word_operand(iter)?;

        Ok(Instruction::SetWord)
    }

    fn decode_add_byte(&mut self, iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        self.decode_register_and_byte_operand(iter)?;

        Ok(Instruction::AddByte)
    }

    fn decode_add_quarter(&mut self, iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        self.decode_register_and_quarter_operand(iter)?;

        Ok(Instruction::AddQuarter)
    }

    fn decode_add_half(&mut self, iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        self.decode_register_and_half_operand(iter)?;

        Ok(Instruction::AddHalf)
    }

    fn decode_add_word(&mut self, iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        self.decode_register_and_word_operand(iter)?;

        Ok(Instruction::AddWord)
    }

    fn decode_sub_byte(&mut self, iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        self.decode_register_and_word_operand(iter)?;

        Ok(Instruction::SubByte)
    }

    fn decode_sub_quarter(&mut self, iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        self.decode_register_and_word_operand(iter)?;

        Ok(Instruction::SubQuarter)
    }

    fn decode_sub_half(&mut self, iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        self.decode_register_and_word_operand(iter)?;

        Ok(Instruction::SubHalf)
    }

    fn decode_sub_word(&mut self, iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        self.decode_register_and_word_operand(iter)?;

        Ok(Instruction::SubWord)
    }

    fn decode_mul_byte(&mut self, iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        self.decode_register_and_byte_operand(iter)?;

        Ok(Instruction::MulByte)
    }

    fn decode_mul_quarter(&mut self, iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        self.decode_register_and_quarter_operand(iter)?;

        Ok(Instruction::MulQuarter)
    }
}

#[cfg(test)]
mod decode_register_and_byte_operand {
    use crate::{error::DecodeError, Processor};

    #[test]
    fn param1_register_and_param2_register_are_valid() {
        let mut p = Processor::new().unwrap();
        let input = "ra rb".split_whitespace();
        let expected = Ok(());
        let actual = p.decode_register_and_byte_operand(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn param1_register_is_valid_and_param2_immediate_value_is_max() {
        let mut p = Processor::new().unwrap();
        let input = "ra 255".split_whitespace();
        let expected = Ok(());
        let actual = p.decode_register_and_byte_operand(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn param1_register_is_valid_and_param2_immediate_value_is_min() {
        let mut p = Processor::new().unwrap();
        let input = "ra 0".split_whitespace();
        let expected = Ok(());
        let actual = p.decode_register_and_byte_operand(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn param2_immediate_value_is_above_max_byte() {
        let mut p = Processor::new().unwrap();
        let input = "ra 256".split_whitespace();
        let expected = Err(DecodeError::InvalidOperand("256".to_string()));
        let actual = p.decode_register_and_byte_operand(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn param2_immediate_value_is_negative() {
        let mut p = Processor::new().unwrap();
        let input = "ra -1".split_whitespace();
        let expected = Err(DecodeError::InvalidOperand("-1".to_string()));
        let actual = p.decode_register_and_byte_operand(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn param1_register_is_invalid() {
        let mut p = Processor::new().unwrap();
        let input = "rx 0".split_whitespace();
        let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
        let actual = p.decode_register_and_byte_operand(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn param2_register_is_invalid() {
        let mut p = Processor::new().unwrap();
        let input = "ra rx".split_whitespace();
        let expected = Err(DecodeError::InvalidOperand("rx".to_string()));
        let actual = p.decode_register_and_byte_operand(input);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod decode_register_and_quarter_operand {
    use crate::{error::DecodeError, Processor};

    #[test]
    fn param1_register_and_param2_register_are_valid() {
        let mut p = Processor::new().unwrap();
        let input = "ra rb".split_whitespace();
        let expected = Ok(());
        let actual = p.decode_register_and_quarter_operand(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn param1_register_is_valid_and_param2_immediate_value_is_max() {
        let mut p = Processor::new().unwrap();
        let input = "ra 65535".split_whitespace();
        let expected = Ok(());
        let actual = p.decode_register_and_quarter_operand(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn param1_register_is_valid_and_param2_immediate_value_is_min() {
        let mut p = Processor::new().unwrap();
        let input = "ra 0".split_whitespace();
        let expected = Ok(());
        let actual = p.decode_register_and_quarter_operand(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn param2_immediate_value_is_above_max_quarter() {
        let mut p = Processor::new().unwrap();
        let input = "ra 65536".split_whitespace();
        let expected = Err(DecodeError::InvalidOperand("65536".to_string()));
        let actual = p.decode_register_and_quarter_operand(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn param2_immediate_value_is_negative() {
        let mut p = Processor::new().unwrap();
        let input = "ra -1".split_whitespace();
        let expected = Err(DecodeError::InvalidOperand("-1".to_string()));
        let actual = p.decode_register_and_quarter_operand(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn param1_register_is_invalid() {
        let mut p = Processor::new().unwrap();
        let input = "rx 0".split_whitespace();
        let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
        let actual = p.decode_register_and_quarter_operand(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn param2_register_is_invalid() {
        let mut p = Processor::new().unwrap();
        let input = "ra rx".split_whitespace();
        let expected = Err(DecodeError::InvalidOperand("rx".to_string()));
        let actual = p.decode_register_and_quarter_operand(input);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod decode_register_and_half_operand {
    use crate::{error::DecodeError, Processor};

    #[test]
    fn param1_register_and_param2_register_are_valid() {
        let mut p = Processor::new().unwrap();
        let input = "ra rb".split_whitespace();
        let expected = Ok(());
        let actual = p.decode_register_and_half_operand(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn param1_register_is_valid_and_param2_immediate_value_is_max() {
        let mut p = Processor::new().unwrap();
        let input = "ra 4294967295".split_whitespace();
        let expected = Ok(());
        let actual = p.decode_register_and_half_operand(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn param1_register_is_valid_and_param2_immediate_value_is_min() {
        let mut p = Processor::new().unwrap();
        let input = "ra 0".split_whitespace();
        let expected = Ok(());
        let actual = p.decode_register_and_half_operand(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn param2_immediate_value_is_above_max_half() {
        let mut p = Processor::new().unwrap();
        let input = "ra 4294967296".split_whitespace();
        let expected = Err(DecodeError::InvalidOperand("4294967296".to_string()));
        let actual = p.decode_register_and_half_operand(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn param2_immediate_value_is_negative() {
        let mut p = Processor::new().unwrap();
        let input = "ra -1".split_whitespace();
        let expected = Err(DecodeError::InvalidOperand("-1".to_string()));
        let actual = p.decode_register_and_half_operand(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn param1_register_is_invalid() {
        let mut p = Processor::new().unwrap();
        let input = "rx 0".split_whitespace();
        let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
        let actual = p.decode_register_and_half_operand(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn param2_register_is_invalid() {
        let mut p = Processor::new().unwrap();
        let input = "ra rx".split_whitespace();
        let expected = Err(DecodeError::InvalidOperand("rx".to_string()));
        let actual = p.decode_register_and_half_operand(input);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod decode_register_and_word_operand {
    use crate::{error::DecodeError, Processor};

    #[test]
    fn param1_register_and_param2_register_are_valid() {
        let mut p = Processor::new().unwrap();
        let input = "ra rb".split_whitespace();
        let expected = Ok(());
        let actual = p.decode_register_and_word_operand(input);
        assert_eq!(actual, expected);
    }   

    #[test]
    fn param1_register_is_valid_and_param2_immediate_value_is_max() {
        let mut p = Processor::new().unwrap();
        let input = "ra 18446744073709551615".split_whitespace();
        let expected = Ok(());
        let actual = p.decode_register_and_word_operand(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn param1_register_is_valid_and_param2_immediate_value_is_min() {
        let mut p = Processor::new().unwrap();
        let input = "ra 0".split_whitespace();
        let expected = Ok(());
        let actual = p.decode_register_and_word_operand(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn param2_immediate_value_is_above_max_word() {
        let mut p = Processor::new().unwrap();
        let input = "ra 18446744073709551616".split_whitespace();
        let expected = Err(DecodeError::InvalidOperand("18446744073709551616".to_string()));
        let actual = p.decode_register_and_word_operand(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn param2_immediate_value_is_negative() {
        let mut p = Processor::new().unwrap();
        let input = "ra -1".split_whitespace();
        let expected = Err(DecodeError::InvalidOperand("-1".to_string()));
        let actual = p.decode_register_and_word_operand(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn param1_register_is_invalid() {
        let mut p = Processor::new().unwrap();
        let input = "rx 0".split_whitespace();
        let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
        let actual = p.decode_register_and_word_operand(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn param2_register_is_invalid() {
        let mut p = Processor::new().unwrap();
        let input = "ra rx".split_whitespace();
        let expected = Err(DecodeError::InvalidOperand("rx".to_string()));
        let actual = p.decode_register_and_word_operand(input);
        assert_eq!(actual, expected);
    }
}

