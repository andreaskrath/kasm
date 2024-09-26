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

    pub(super) fn execute(&mut self, instruction: Instruction) -> Result<(), ExecuteError> {
        self.config.instructions_executed += 1;

        let mut increment_pc = instruction.increment();

        match instruction {
            Instruction::Addition(add_ins) => self.add(add_ins),
            Instruction::And(and_ins) => self.and(and_ins),
            Instruction::Call(operand) => self.call(operand)?,
            Instruction::Compare(compare_ins) => self.compare(compare_ins),
            Instruction::Division(div_ins) => self.div(div_ins)?,
            Instruction::Jump(jump_ins, operand, relative) => {
                increment_pc = !self.jump(jump_ins, operand, relative)?;
            }
            Instruction::Multiplication(mul_ins) => self.mul(mul_ins),
            Instruction::Not(not_ins) => self.not(not_ins),
            Instruction::Or(or_ins) => self.or(or_ins),
            Instruction::Pop(pop_ins) => self.pop(pop_ins)?,
            Instruction::PrintRegister(print_reg_ins) => self.print_register(print_reg_ins)?,
            Instruction::PrintStack(print_stack_ins) => self.print_stack(print_stack_ins)?,
            Instruction::Push(push_ins) => self.push(push_ins)?,
            Instruction::Remainder(rem_ins) => self.rem(rem_ins)?,
            Instruction::Return => self.ret()?,
            Instruction::Set(set_ins) => self.set(set_ins),
            Instruction::Stop => self.stop()?,
            Instruction::Subtraction(sub_ins) => self.sub(sub_ins),
            Instruction::Test(test_ins) => self.test(test_ins),
            Instruction::Xor(xor_ins) => self.xor(xor_ins),
        }

        if increment_pc {
            self.program_counter += 1;
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
        let return_address = self.program_counter + 1;
        self.stack.push(return_address)?;

        let destination = self.get_operand_value(operand);
        self.program_counter = destination;

        Ok(())
    }

    fn ret(&mut self) -> Result<(), ExecuteError> {
        let destination = self.stack.pop::<Word>()?;
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
        constant::{Byte, Word, TEST_STACK_SIZE},
        error::ExecuteError,
        instruction::Instruction,
        operand::Operand,
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn stack_overflow() {
        let mut i = Interpreter::new_test();
        // filling the stack with values
        for _ in 0..TEST_STACK_SIZE {
            i.stack
                .push::<Byte>(0)
                .expect("should be able to fill stack");
        }
        let instruction = Instruction::Call(Operand::Immediate(5));
        let expected = Err(ExecuteError::StackOverflow);

        let actual = i.execute(instruction);

        assert_eq!(actual, expected);
    }

    #[test]
    fn call_from_register() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        i.registers.set(Register::A, 500);
        let instruction = Instruction::Call(Operand::Register(Register::A));
        let expected_return_address = 2;
        let expected_program_counter = 500;

        i.execute(instruction)?;
        let actual_return_address: Word = i
            .stack
            .pop()
            .expect("should be able to pop value from stack");

        assert_eq!(actual_return_address, expected_return_address);
        assert_eq!(i.program_counter, expected_program_counter);

        Ok(())
    }

    #[test]
    fn call_from_immediate() -> Result<(), ExecuteError> {
        let mut i = Interpreter::new_test();
        let instruction = Instruction::Call(Operand::Immediate(500));
        let expected_return_address = 2;
        let expected_program_counter = 500;

        i.execute(instruction)?;
        let actual_return_address: Word = i
            .stack
            .pop()
            .expect("should be able to pop value from stack");

        assert_eq!(actual_return_address, expected_return_address);
        assert_eq!(i.program_counter, expected_program_counter);

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
        i.stack
            .push(Word::MAX)
            .expect("should be able to push value onto stack");
        let instruction = Instruction::Return;
        let expected = Word::MAX;

        i.execute(instruction)?;

        assert_eq!(i.program_counter, expected);

        Ok(())
    }
}
