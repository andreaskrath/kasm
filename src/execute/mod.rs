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
