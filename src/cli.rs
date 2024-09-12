use crate::{error::InterpreterError, utils::Writer};
use clap::Parser;
use std::{fs::File, io::stdout, path::PathBuf};

pub struct Configuration {
    pub print_instructions_executed: bool,
    pub instructions_executed: u64,
    pub output: Box<dyn Writer>,
    pub debug: bool,
}

impl Configuration {
    #[cfg(test)]
    pub fn new_test() -> Self {
        Self {
            print_instructions_executed: false,
            instructions_executed: 0,
            output: Box::new(Vec::new()),
            debug: false,
        }
    }
}

impl TryFrom<Arguments> for Configuration {
    type Error = InterpreterError;

    fn try_from(args: Arguments) -> Result<Self, Self::Error> {
        let output = match args.output {
            Some(path) => File::options()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path)
                .map(|f| Box::new(f) as Box<dyn Writer>)
                .map_err(|err| InterpreterError::FailedOutputFileCreation(err.to_string()))?,
            None => Box::new(stdout()),
        };

        let c = Self {
            print_instructions_executed: args.instructions,
            instructions_executed: 0,
            output,
            debug: args.debug,
        };
        Ok(c)
    }
}

#[derive(Debug, Parser, PartialEq)]
pub struct Arguments {
    /// Print the amount of executed instructions after the program is finished
    #[arg(long = "instructions", short = 'i', default_value = "false")]
    instructions: bool,

    /// Creates or uses the specified file as output for print instructions, otherwise stdout is used
    #[arg(long = "output", short = 'o', value_name = "FILE")]
    output: Option<PathBuf>,

    /// Interprets the program in debug mode
    #[arg(long = "debug", short = 'd', default_value = "false")]
    debug: bool,
}

#[cfg(test)]
mod regression {
    mod debug {
        use crate::Arguments;
        use clap::Parser;

        #[test]
        fn undefined() {
            let args = [""];
            let expected = Arguments {
                instructions: false,
                output: None,
                debug: false,
            };

            let actual = Arguments::parse_from(args);

            assert_eq!(actual, expected);
        }

        #[test]
        fn long() {
            let args = ["", "--debug"];
            let expected = Arguments {
                instructions: false,
                output: None,
                debug: true,
            };

            let actual = Arguments::parse_from(args);

            assert_eq!(actual, expected);
        }

        #[test]
        fn short() {
            let args = ["", "-d"];
            let expected = Arguments {
                instructions: false,
                output: None,
                debug: true,
            };

            let actual = Arguments::parse_from(args);

            assert_eq!(actual, expected);
        }
    }
}
