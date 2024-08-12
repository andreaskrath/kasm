use std::str::SplitWhitespace;
use phf::phf_map;
use crate::{constant::DecodeTable, error::DecodeError, instruction::Instruction, operand::Operand, register::Register, Processor};

pub const DECODE_TABLE: DecodeTable = phf_map! {
    "stop" => Processor::decode_stop,
    "setb" => Processor::decode_set_byte,
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
}
