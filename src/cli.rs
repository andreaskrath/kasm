use crate::{
    constant::{GIGA_BYTE, KILO_BYTE, MEGA_BYTE},
    error::{ArgumentError, InterpreterError},
    utils::Writer,
};
use clap::Parser;
use std::{fs::File, io::stdout, path::PathBuf};

pub struct Configuration {
    pub print_instructions_executed: bool,
    pub instructions_executed: u64,
    pub output: Box<dyn Writer>,
    pub debug: bool,
    pub stack_size: usize,
}

impl Configuration {
    #[cfg(test)]
    pub fn new_test() -> Self {
        use crate::constant::TEST_STACK_SIZE;

        Self {
            print_instructions_executed: false,
            instructions_executed: 0,
            output: Box::new(Vec::new()),
            debug: false,
            stack_size: TEST_STACK_SIZE,
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

        let stack_size = parse_stack_size(args.stack_size).map_err(InterpreterError::Argument)?;

        let c = Self {
            print_instructions_executed: args.instructions,
            instructions_executed: 0,
            output,
            debug: args.debug,
            stack_size,
        };
        Ok(c)
    }
}

#[derive(Debug, Parser, PartialEq)]
pub struct Arguments {
    /// Print the amount of executed instructions after the program is finished
    #[arg(long = "instructions", short = 'i')]
    instructions: bool,

    /// Creates or uses the specified file as output for print instructions, otherwise stdout is used
    #[arg(long = "output", short = 'o', value_name = "FILE")]
    output: Option<PathBuf>,

    /// Interprets the program in debug mode
    #[arg(long = "debug", short = 'd')]
    debug: bool,

    /// The size of the stack; requires a size suffix: b/B = byte, k/K = kilobyte, m/M = megabyte, g/G = gigabyte
    #[arg(long = "stack", short = 's', value_name = "SIZE", default_value = "4m")]
    stack_size: String,
}

/// Parses the indicated stack size by the stack size flag.
///
/// Ensures correct format and numeric values for the underlying architecture.
fn parse_stack_size(s: String) -> Result<usize, ArgumentError> {
    let (num, size_suffix) = s
        .split_at_checked(s.len() - 1)
        .ok_or(ArgumentError::CouldNotSplitSuffix)?;

    let parsed_num = num
        .parse::<usize>()
        .map_err(|_| ArgumentError::InvalidInitialStackSize(num.to_string()))?;

    let stack_size = match size_suffix {
        "b" | "B" => parsed_num,
        "k" | "K" => parsed_num
            .checked_mul(KILO_BYTE)
            .ok_or(ArgumentError::InvalidComputedStackSize)?,
        "m" | "M" => parsed_num
            .checked_mul(MEGA_BYTE)
            .ok_or(ArgumentError::InvalidComputedStackSize)?,
        "g" | "G" => parsed_num
            .checked_mul(GIGA_BYTE)
            .ok_or(ArgumentError::InvalidComputedStackSize)?,
        unknown => return Err(ArgumentError::InvalidStackSizeSuffix(unknown.to_string())),
    };

    Ok(stack_size)
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
                stack_size: String::from("4m"),
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
                stack_size: String::from("4m"),
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
                stack_size: String::from("4m"),
            };

            let actual = Arguments::parse_from(args);

            assert_eq!(actual, expected);
        }
    }

    mod instructions {
        use crate::Arguments;
        use clap::Parser;

        #[test]
        fn undefined() {
            let args = [""];
            let expected = Arguments {
                instructions: false,
                output: None,
                debug: false,
                stack_size: String::from("4m"),
            };

            let actual = Arguments::parse_from(args);

            assert_eq!(actual, expected);
        }

        #[test]
        fn long() {
            let args = ["", "--instructions"];
            let expected = Arguments {
                instructions: true,
                output: None,
                debug: false,
                stack_size: String::from("4m"),
            };

            let actual = Arguments::parse_from(args);

            assert_eq!(actual, expected);
        }

        #[test]
        fn short() {
            let args = ["", "-i"];
            let expected = Arguments {
                instructions: true,
                output: None,
                debug: false,
                stack_size: String::from("4m"),
            };

            let actual = Arguments::parse_from(args);

            assert_eq!(actual, expected);
        }
    }

    mod output {
        use std::path::PathBuf;

        use crate::Arguments;
        use clap::Parser;

        #[test]
        fn undefined() {
            let args = [""];
            let expected = Arguments {
                instructions: false,
                output: None,
                debug: false,
                stack_size: String::from("4m"),
            };

            let actual = Arguments::parse_from(args);

            assert_eq!(actual, expected);
        }

        #[test]
        fn long() {
            let args = ["", "--output", "file.txt"];
            let expected = Arguments {
                instructions: false,
                output: Some(PathBuf::from("file.txt")),
                debug: false,
                stack_size: String::from("4m"),
            };

            let actual = Arguments::parse_from(args);

            assert_eq!(actual, expected);
        }

        #[test]
        fn short() {
            let args = ["", "-o", "file.txt"];
            let expected = Arguments {
                instructions: false,
                output: Some(PathBuf::from("file.txt")),
                debug: false,
                stack_size: String::from("4m"),
            };

            let actual = Arguments::parse_from(args);

            assert_eq!(actual, expected);
        }
    }
}

#[cfg(test)]
mod parse_stack_size {
    use super::parse_stack_size;
    use crate::{
        constant::{GIGA_BYTE, KILO_BYTE, MEGA_BYTE},
        error::ArgumentError,
    };

    #[test]
    fn could_not_split_suffix_error() {
        let input = String::from("500ø");
        let expected = Err(ArgumentError::CouldNotSplitSuffix);

        let actual = parse_stack_size(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn invalid_stack_size_suffix_error() {
        let input = String::from("500l");
        let expected = Err(ArgumentError::InvalidStackSizeSuffix("l".to_string()));

        let actual = parse_stack_size(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn invalid_initial_stack_size_error() {
        let input = format!("{}0b", usize::MAX);
        let expected = Err(ArgumentError::InvalidInitialStackSize(format!(
            "{}0",
            usize::MAX
        )));

        let actual = parse_stack_size(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn invalid_computed_stack_size_error() {
        let input = format!("{}k", usize::MAX);
        let expected = Err(ArgumentError::InvalidComputedStackSize);

        let actual = parse_stack_size(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn lowercase_byte() -> Result<(), ArgumentError> {
        let input = String::from("2b");
        let expected = 2;

        let actual = parse_stack_size(input)?;

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn uppercase_byte() -> Result<(), ArgumentError> {
        let input = String::from("2B");
        let expected = 2;

        let actual = parse_stack_size(input)?;

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn lowercase_kilo_byte() -> Result<(), ArgumentError> {
        let input = String::from("2k");
        let expected = 2 * KILO_BYTE;

        let actual = parse_stack_size(input)?;

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn uppercase_kilo_byte() -> Result<(), ArgumentError> {
        let input = String::from("2K");
        let expected = 2 * KILO_BYTE;

        let actual = parse_stack_size(input)?;

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn lowercase_mega_byte() -> Result<(), ArgumentError> {
        let input = String::from("2m");
        let expected = 2 * MEGA_BYTE;

        let actual = parse_stack_size(input)?;

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn uppercase_mega_byte() -> Result<(), ArgumentError> {
        let input = String::from("2M");
        let expected = 2 * MEGA_BYTE;

        let actual = parse_stack_size(input)?;

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn lowercase_giga_byte() -> Result<(), ArgumentError> {
        let input = String::from("2g");
        let expected = 2 * GIGA_BYTE;

        let actual = parse_stack_size(input)?;

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn uppercase_giga_byte() -> Result<(), ArgumentError> {
        let input = String::from("2G");
        let expected = 2 * GIGA_BYTE;

        let actual = parse_stack_size(input)?;

        assert_eq!(actual, expected);
        Ok(())
    }
}
