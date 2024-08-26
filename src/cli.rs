use clap::Parser;

pub struct Configuration {
    pub print_instructions_executed: bool,
    pub instructions_executed: u64,
}

impl Configuration {
    #[cfg(test)]
    pub fn new_test() -> Self {
        Self {
            print_instructions_executed: false,
            instructions_executed: 0,
        }
    }
}

impl From<Arguments> for Configuration {
    fn from(args: Arguments) -> Self {
        Self {
            print_instructions_executed: args.instructions,
            instructions_executed: 0,
        }
    }
}

#[derive(Debug, Parser)]
pub struct Arguments {
    /// Print the amount of executed instructions after the program is finished
    #[arg(long = "instructions", short = 'i')]
    instructions: bool,
}
