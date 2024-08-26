use crate::{error::ProcessorError, utils::Writer};
use clap::Parser;
use std::{fs::File, io::stdout, path::PathBuf};

pub struct Configuration {
    pub print_instructions_executed: bool,
    pub instructions_executed: u64,
    pub output: Box<dyn Writer>,
}

impl Configuration {
    #[cfg(test)]
    pub fn new_test() -> Self {
        Self {
            print_instructions_executed: false,
            instructions_executed: 0,
            output: Box::new(Vec::new()),
        }
    }
}

impl TryFrom<Arguments> for Configuration {
    type Error = ProcessorError;

    fn try_from(args: Arguments) -> Result<Self, Self::Error> {
        let output = match args.output {
            Some(path) => File::options()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path)
                .map(|f| Box::new(f) as Box<dyn Writer>)
                .map_err(|err| ProcessorError::FailedOutputFileCreation(err.to_string()))?,
            None => Box::new(stdout()),
        };

        let c = Self {
            print_instructions_executed: args.instructions,
            instructions_executed: 0,
            output,
        };
        Ok(c)
    }
}

#[derive(Debug, Parser)]
pub struct Arguments {
    /// Print the amount of executed instructions after the program is finished
    #[arg(long = "instructions", short = 'i', default_value = "false")]
    instructions: bool,

    /// Creates or uses the specified file as output for print instructions, otherwise stdout is used
    #[arg(long = "output", short = 'o', value_name = "FILE")]
    output: Option<PathBuf>,
}
