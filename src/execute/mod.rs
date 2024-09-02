use crate::{
    constant::Word, error::ExecuteError, instruction::Instruction, operand::Operand,
    registers::RegisterOperations, utils::FromBytes, Interpreter,
};

mod addition;
mod and;
mod compare;
mod division;
mod jump;
mod multiplication;
mod not;
mod or;
mod pop;
mod print_register;
mod print_stack;
mod push;
mod remainder;
mod set;
mod subtraction;
mod test;
mod xor;

impl Interpreter {
    fn get_operand_value<T>(&self, operand: Operand<T>) -> T
    where
        T: FromBytes,
    {
        match operand {
            Operand::Register(register) => self.registers.get::<T>(register),
            Operand::Immediate(value) => value,
        }
    }

    pub fn execute(&mut self, instruction: Instruction) -> Result<(), ExecuteError> {
        self.config.instructions_executed += 1;

        match instruction {
            Instruction::Stop => self.stop()?,
            Instruction::Set(set_ins) => self.set(set_ins),
            Instruction::Addition(add_ins) => self.add(add_ins),
            Instruction::Subtraction(sub_ins) => self.sub(sub_ins),
            Instruction::Multiplication(mul_ins) => self.mul(mul_ins),
            Instruction::Division(div_ins) => self.div(div_ins)?,
            Instruction::Remainder(rem_ins) => self.rem(rem_ins)?,
            Instruction::Push(push_ins) => self.push(push_ins)?,
            Instruction::Pop(pop_ins) => self.pop(pop_ins)?,
            Instruction::Call(operand) => self.call(operand)?,
            Instruction::Return => self.ret()?,
            Instruction::And(and_ins) => self.and(and_ins),
            Instruction::Or(or_ins) => self.or(or_ins),
            Instruction::Xor(xor_ins) => self.xor(xor_ins),
            Instruction::Not(not_ins) => self.not(not_ins),
            Instruction::Test(test_ins) => self.test(test_ins),
            Instruction::Compare(compare_ins) => self.compare(compare_ins),
            Instruction::Jump(jump_ins) => self.jump(jump_ins),
            Instruction::PrintRegister(print_reg_ins) => self.print_register(print_reg_ins)?,
            Instruction::PrintStack(print_stack_ins) => self.print_stack(print_stack_ins)?,
        }

        Ok(())
    }

    fn stop(&mut self) -> Result<(), ExecuteError> {
        self.running = false;

        if self.config.print_instructions_executed {
            writeln!(
                self.config.output,
                "Instructions Executed: {}",
                self.config.instructions_executed
            )
            .map_err(|err| ExecuteError::IO(err.to_string()))?;
        }

        Ok(())
    }

    fn call(&mut self, operand: Operand<Word>) -> Result<(), ExecuteError> {
        let destination = self.get_operand_value(operand);
        self.push_value(destination)?;

        Ok(())
    }

    fn ret(&mut self) -> Result<(), ExecuteError> {
        let destination = self.pop_value::<Word>()?;
        self.program_counter = destination;

        Ok(())
    }
}

#[cfg(test)]
mod stop {
    use crate::{error::ExecuteError, instruction::Instruction, Interpreter};

    #[test]
    fn sets_running_bool_to_false_and_prints_instructions_executed() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        i.config.print_instructions_executed = true;
        let instruction = Instruction::Stop;
        let expected = "Instructions Executed: 1\n";

        i.execute(instruction)?;
        let buffer = i.config.output.get_buffer().unwrap();

        assert!(!i.running);
        assert_eq!(i.config.instructions_executed, 1);
        assert_eq!(buffer, expected);

        Ok(())
    }
}

#[cfg(test)]
mod call {
    use crate::{
        constant::Word, error::ExecuteError, instruction::Instruction, operand::Operand,
        register::Register, Interpreter,
    };

    #[test]
    fn stack_overflow() {
        let mut i = Interpreter::new_test();
        i.stack_pointer = i.stack.len() as Word - 1;
        let instruction = Instruction::Call(Operand::Immediate(5));
        let expected = Err(ExecuteError::StackOverflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn call_from_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        i.registers[Register::A] = Word::MAX;
        let instruction = Instruction::Call(Operand::Register(Register::A));
        let expected = Word::MAX;

        i.execute(instruction)?;
        let mut bytes = [0; 8];
        bytes.copy_from_slice(&i.stack[0..i.sp()]);
        let actual = Word::from_le_bytes(bytes);

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn call_from_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Call(Operand::Immediate(Word::MAX));
        let expected = Word::MAX;

        i.execute(instruction)?;
        let mut bytes = [0; 8];
        bytes.copy_from_slice(&i.stack[0..i.sp()]);
        let actual = Word::from_le_bytes(bytes);

        assert_eq!(actual, expected);

        Ok(())
    }
}

#[cfg(test)]
mod ret {
    use crate::{constant::Word, error::ExecuteError, instruction::Instruction, Interpreter};

    #[test]
    fn stack_underflow() {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Return;
        let expected = Err(ExecuteError::StackUnderflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_return() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let bytes = Word::MAX.to_le_bytes();
        i.stack[0] = bytes[0];
        i.stack[1] = bytes[1];
        i.stack[2] = bytes[2];
        i.stack[3] = bytes[3];
        i.stack[4] = bytes[4];
        i.stack[5] = bytes[5];
        i.stack[6] = bytes[6];
        i.stack[7] = bytes[7];
        i.stack_pointer = 8;
        let instruction = Instruction::Return;
        let expected = Word::MAX;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }
}
