use std::str::SplitWhitespace;
use phf::phf_map;
use crate::{constant::DecodeTable, error::DecodeError, instruction::Instruction, operand::Operand, register::Register, Processor};

pub const DECODE_TABLE: DecodeTable = phf_map! {
    "stop" => Processor::decode_stop,
    "setb" => Processor::decode_set_byte,
    "setq" => Processor::decode_set_quarter,
    "seth" => Processor::decode_set_half,
    "setw" => Processor::decode_set_word,
};

fn get_register_and_operand_str(mut iter: SplitWhitespace) -> Result<(&str, &str), DecodeError> {
    let (Some(s_register), Some(s_operand)) = (iter.next(), iter.next()) else {
        return Err(DecodeError::IncompleteInstruction);
    };

    Ok((s_register, s_operand))
}

impl Processor {
    fn decode_stop(&mut self, _iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        Ok(Instruction::Stop)
    }

    fn decode_set_byte(&mut self, iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (s_register, s_operand) = get_register_and_operand_str(iter)?;

        let register = Register::try_from(s_register)?;
        let operand = Operand::try_from(s_operand)?;

        self.registers[Register::P1] = register.as_word();
        self.registers[Register::P2] = operand.byte_val_as_word(&self.registers);

        Ok(Instruction::SetByte)
    }

    fn decode_set_quarter(&mut self, iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (s_register, s_operand) = get_register_and_operand_str(iter)?;

        let register = Register::try_from(s_register)?;
        let operand = Operand::try_from(s_operand)?;

        self.registers[Register::P1] = register.as_word();
        self.registers[Register::P2] = operand.quarter_val_as_word(&self.registers);

        Ok(Instruction::SetQuarter)
    }

    fn decode_set_half(&mut self, iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (s_register, s_operand) = get_register_and_operand_str(iter)?;

        let register = Register::try_from(s_register)?;
        let operand = Operand::try_from(s_operand)?;

        self.registers[Register::P1] = register.as_word();
        self.registers[Register::P2] = operand.half_val_as_word(&self.registers);

        Ok(Instruction::SetHalf)
    }

    fn decode_set_word(&mut self, iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (s_register, s_operand) = get_register_and_operand_str(iter)?;
        
        let register = Register::try_from(s_register)?;
        let operand = Operand::try_from(s_operand)?;

        self.registers[Register::P1] = register.as_word();
        self.registers[Register::P2] = operand.word_val(&self.registers);

        Ok(Instruction::SetWord)
    }
}

#[cfg(test)]
mod decode_set_byte {
    use crate::{error::DecodeError, instruction::Instruction, Processor};

    #[test]
    fn register_is_valid_and_immediate_value_is_max() {
        let mut p = Processor::new().unwrap();
        let input = "ra 255".split_whitespace();
        let expected = Ok(Instruction::SetByte);
        let actual = p.decode_set_byte(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn register_is_valid_and_immediate_value_is_min() {
        let mut p = Processor::new().unwrap();
        let input = "ra 0".split_whitespace();
        let expected = Ok(Instruction::SetByte);
        let actual = p.decode_set_byte(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn immediate_value_is_above_max_byte() {
        let mut p = Processor::new().unwrap();
        let input = "ra 256".split_whitespace();
        let expected = Err(DecodeError::InvalidOperand("256".to_string()));
        let actual = p.decode_set_byte(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn immediate_value_is_negative() {
        let mut p = Processor::new().unwrap();
        let input = "ra -1".split_whitespace();
        let expected = Err(DecodeError::InvalidOperand("-1".to_string()));
        let actual = p.decode_set_byte(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn register_is_invalid() {
        let mut p = Processor::new().unwrap();
        let input = "rx 0".split_whitespace();
        let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
        let actual = p.decode_set_byte(input);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod decode_set_quarter {
    use crate::{error::DecodeError, instruction::Instruction, Processor};

    #[test]
    fn register_is_valid_and_immediate_value_is_max() {
        let mut p = Processor::new().unwrap();
        let input = "ra 65535".split_whitespace();
        let expected = Ok(Instruction::SetQuarter);
        let actual = p.decode_set_quarter(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn register_is_valid_and_immediate_value_is_min() {
        let mut p = Processor::new().unwrap();
        let input = "ra 0".split_whitespace();
        let expected = Ok(Instruction::SetQuarter);
        let actual = p.decode_set_quarter(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn immediate_value_is_above_max_quarter() {
        let mut p = Processor::new().unwrap();
        let input = "ra 65536".split_whitespace();
        let expected = Err(DecodeError::InvalidOperand("65536".to_string()));
        let actual = p.decode_set_quarter(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn immediate_value_is_negative() {
        let mut p = Processor::new().unwrap();
        let input = "ra -1".split_whitespace();
        let expected = Err(DecodeError::InvalidOperand("-1".to_string()));
        let actual = p.decode_set_quarter(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn register_is_invalid() {
        let mut p = Processor::new().unwrap();
        let input = "rx 0".split_whitespace();
        let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
        let actual = p.decode_set_quarter(input);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod decode_set_half {
    use crate::{error::DecodeError, instruction::Instruction, Processor};

    #[test]
    fn register_is_valid_and_immediate_value_is_max() {
        let mut p = Processor::new().unwrap();
        let input = "ra 4294967295".split_whitespace();
        let expected = Ok(Instruction::SetHalf);
        let actual = p.decode_set_half(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn register_is_valid_and_immediate_value_is_min() {
        let mut p = Processor::new().unwrap();
        let input = "ra 0".split_whitespace();
        let expected = Ok(Instruction::SetHalf);
        let actual = p.decode_set_half(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn immediate_value_is_above_max_half() {
        let mut p = Processor::new().unwrap();
        let input = "ra 4294967296".split_whitespace();
        let expected = Err(DecodeError::InvalidOperand("4294967296".to_string()));
        let actual = p.decode_set_half(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn immediate_value_is_negative() {
        let mut p = Processor::new().unwrap();
        let input = "ra -1".split_whitespace();
        let expected = Err(DecodeError::InvalidOperand("-1".to_string()));
        let actual = p.decode_set_half(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn register_is_invalid() {
        let mut p = Processor::new().unwrap();
        let input = "rx 0".split_whitespace();
        let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
        let actual = p.decode_set_half(input);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod decode_set_word {
    use crate::{error::DecodeError, instruction::Instruction, Processor};

    #[test]
    fn register_is_valid_and_immediate_value_is_max() {
        let mut p = Processor::new().unwrap();
        let input = "ra 18446744073709551615".split_whitespace();
        let expected = Ok(Instruction::SetWord);
        let actual = p.decode_set_word(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn register_is_valid_and_immediate_value_is_min() {
        let mut p = Processor::new().unwrap();
        let input = "ra 0".split_whitespace();
        let expected = Ok(Instruction::SetWord);
        let actual = p.decode_set_word(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn immediate_value_is_above_max_word() {
        let mut p = Processor::new().unwrap();
        let input = "ra 18446744073709551616".split_whitespace();
        let expected = Err(DecodeError::InvalidOperand("18446744073709551616".to_string()));
        let actual = p.decode_set_word(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn immediate_value_is_negative() {
        let mut p = Processor::new().unwrap();
        let input = "ra -1".split_whitespace();
        let expected = Err(DecodeError::InvalidOperand("-1".to_string()));
        let actual = p.decode_set_word(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn register_is_invalid() {
        let mut p = Processor::new().unwrap();
        let input = "rx 0".split_whitespace();
        let expected = Err(DecodeError::InvalidRegister("rx".to_string()));
        let actual = p.decode_set_word(input);
        assert_eq!(actual, expected);
    }
}
