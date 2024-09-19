pub use cli::Arguments;
use cli::Configuration;
use constant::{Word, COMMENT, DEBUG_HELP, DEBUG_INITIAL};
pub use error::ArgumentError;
pub use error::InterpreterError;
use flags::Flags;
use instruction::Instruction;
use preprocess::expand_data_section;
use register::Register;
use registers::Registers;
use stack::Stack;
use std::io::stdin;

mod cli;
mod constant;
mod decode;
mod error;
mod execute;
mod flags;
mod instruction;
mod operand;
mod preprocess;
mod register;
mod registers;
mod stack;
mod utils;

pub struct Interpreter {
    registers: Registers,
    program_counter: Word,
    flags: Flags,
    running: bool,
    stack: Stack,
    config: Configuration,
}

impl Interpreter {
    pub fn try_new(args: Arguments) -> Result<Self, InterpreterError> {
        let config = Configuration::try_from(args)?;

        let p = Self {
            registers: [0; Register::VARIANT_COUNT],
            program_counter: 0,
            flags: Flags::new(),
            running: true,
            stack: Stack::new(config.stack_size),
            config,
        };
        Ok(p)
    }

    #[cfg(test)]
    pub fn new_test() -> Self {
        use constant::TEST_STACK_SIZE;

        Self {
            registers: [0; Register::VARIANT_COUNT],
            program_counter: 0,
            flags: Flags::new(),
            running: true,
            stack: Stack::new(TEST_STACK_SIZE),
            config: Configuration::new_test(),
        }
    }

    /// Gets the program counter as a usize.
    fn pc(&self) -> usize {
        self.program_counter as usize
    }

    pub fn run(&mut self, content: &str) -> Result<(), InterpreterError> {
        let data_expanded_program = expand_data_section(content).map_err(InterpreterError::Data)?;
        let program: Box<[&str]> = data_expanded_program.lines().collect();

        if self.config.debug {
            self.debug(program)?;
        } else {
            self.full(program)?;
        }

        Ok(())
    }

    fn advance(&mut self, program: &[&str]) -> Result<(), InterpreterError> {
        let line = program
            .get(self.pc())
            .ok_or(InterpreterError::InvalidProgramCounter(self.pc()))?;

        self.program_counter += 1;

        if line.starts_with(COMMENT) {
            return Ok(());
        }

        let instruction = self
            .decode(line)
            .map_err(|e| InterpreterError::Decode(self.pc() - 1, e))?;

        self.execute(instruction)
            .map_err(|e| InterpreterError::Execute(self.pc() - 1, e))?;

        Ok(())
    }

    fn full(&mut self, program: Box<[&str]>) -> Result<(), InterpreterError> {
        while self.running {
            self.advance(&program)?;
        }

        Ok(())
    }

    fn debug(&mut self, program: Box<[&str]>) -> Result<(), InterpreterError> {
        println!("{DEBUG_INITIAL}");

        while self.running {
            let mut action = String::new();
            stdin().read_line(&mut action).unwrap();

            match action.trim() {
                "help" | "h" => {
                    println!("{DEBUG_HELP}");
                }
                "next" | "n" => {
                    self.advance(&program)?;
                }
                "stop" | "s" => {
                    self.execute(Instruction::Stop)
                        .map_err(|err| InterpreterError::Execute(self.pc(), err))?;
                    break;
                }
                unknown => println!("unknown action: '{unknown}'"),
            }

            println!("pc: {}, sp: {}", self.pc(), self.stack.sp());
        }

        Ok(())
    }
}

#[cfg(test)]
mod integration {
    use crate::{
        constant::{Byte, Word, COMMENT},
        error::{DecodeError, ExecuteError, InterpreterError},
        register::Register,
        registers::RegisterOperations,
        Interpreter,
    };

    #[test]
    fn push_word_pop_four_bytes_into_registers() -> Result<(), InterpreterError> {
        let mut i = Interpreter::new_test();
        let program = [
            format!("pshw {}", Word::MAX).as_ref(),
            "popb ra",
            "popb rb",
            "popb rc",
            "popb rd",
            "stop",
        ]
        .join("\n");

        i.run(&program)?;

        assert_eq!(i.registers.get::<Byte>(Register::A), Byte::MAX);
        assert_eq!(i.registers.get::<Byte>(Register::B), Byte::MAX);
        assert_eq!(i.registers.get::<Byte>(Register::C), Byte::MAX);
        assert_eq!(i.registers.get::<Byte>(Register::D), Byte::MAX);

        Ok(())
    }

    #[test]
    fn min_sub_max_compare_edge_case_jump_if_lesser() -> Result<(), InterpreterError> {
        let mut i = Interpreter::new_test();
        let program = [
            format!("cmpb 0 {}", Byte::MAX).as_ref(), // this is 1 after wrapping around
            "jil 3",
            "setb ra 10",
            "stop",
        ]
        .join("\n");

        i.run(&program)?;

        assert_eq!(i.registers.get::<Byte>(Register::A), 0);
        assert!(i.flags.overflow);
        assert!(!i.flags.zero);

        Ok(())
    }

    #[test]
    fn pop_empty_stack() {
        let mut i = Interpreter::new_test();
        let program = ["popb ra"].join("\n");
        let expected = Err(InterpreterError::Execute(0, ExecuteError::StackUnderflow));

        let actual = i.run(&program);

        assert_eq!(actual, expected);
    }

    #[test]
    fn push_until_stack_overflow() {
        let mut i = Interpreter::new_test();
        let program = ["pshw 10", "jmp 0"].join("\n");
        let expected = Err(InterpreterError::Execute(0, ExecuteError::StackOverflow));

        let actual = i.run(&program);

        assert_eq!(actual, expected);
    }

    #[test]
    fn compute_fibonacci_number_10() -> Result<(), InterpreterError> {
        let mut i = Interpreter::new_test();
        let program = [
            "setb ra 0",
            "setb rb 1",
            "setb rc 2",
            "addb ra rb",
            "pshb ra",
            "setb ra rb",
            "popb rb",
            "addb rc 1",
            "cmpb rc 10",
            "jnz 3",
            "prrb rb",
            "stop",
        ]
        .join("\n");
        let expected_value = 34;
        let expected_print = "rb: 34\n";

        i.run(&program)?;
        let actual_value = i.registers.get::<Byte>(Register::B);
        let actual_print = i
            .config
            .output
            .get_buffer()
            .expect("interpreter test instance should return buffer");

        assert_eq!(actual_value, expected_value);
        assert_eq!(actual_print, expected_print);
        assert!(!i.flags.sign);
        assert!(!i.flags.overflow);
        assert!(i.flags.zero);

        Ok(())
    }

    #[test]
    fn comment_has_no_effect() -> Result<(), InterpreterError> {
        let mut i = Interpreter::new_test();
        let program = [format!("{} setb rb 5", COMMENT).as_ref(), "stop"].join("\n");
        let expected = 0;

        i.run(&program)?;
        let actual = i.registers.get::<Byte>(Register::B);

        assert_eq!(actual, expected);
        assert_eq!(i.config.instructions_executed, 1);

        Ok(())
    }

    #[test]
    fn decode_error_on_expected_line() {
        let mut i = Interpreter::new_test();
        let program = "hello";
        let expected = Err(InterpreterError::Decode(
            0,
            DecodeError::UnknownInstruction("hello".to_string()),
        ));

        let actual = i.run(program);

        assert_eq!(actual, expected);
    }

    #[test]
    fn execute_error_on_expected_line() {
        let mut i = Interpreter::new_test();
        let program = "divb ra 0";
        let expected = Err(InterpreterError::Execute(0, ExecuteError::DivideByZero));

        let actual = i.run(program);

        assert_eq!(actual, expected);
    }
}
