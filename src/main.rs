use clap::Parser;
use kasm::{ArgumentError, Arguments, Interpreter, InterpreterError};
use std::{fs::File, io::Read};

fn main() {
    let args = Arguments::parse();
    let file_name = args.file_name.clone();
    if !file_name.is_file() {
        eprintln!(
            "{}",
            InterpreterError::Argument(ArgumentError::NotAFile(file_name))
        );
        return;
    }

    let mut i = match Interpreter::try_new(args) {
        Ok(p) => p,
        Err(err) => {
            eprintln!("{err}");
            return;
        }
    };

    let mut file = match File::options()
        .read(true)
        .write(false)
        .truncate(false)
        .create(false)
        .open(file_name.clone())
    {
        Ok(f) => f,
        Err(err) => {
            let error_msg = match err.kind() {
                std::io::ErrorKind::NotFound => {
                    InterpreterError::Argument(ArgumentError::FileNotFound(file_name))
                }
                std::io::ErrorKind::PermissionDenied => {
                    InterpreterError::Argument(ArgumentError::LackingPermissions(
                        "open the specified program file".to_string(),
                    ))
                }
                _ => InterpreterError::Argument(ArgumentError::UnknownProgramFileIssue(
                    err.to_string(),
                )),
            };
            eprintln!("{error_msg}");
            return;
        }
    };

    let mut content = String::new();
    if file.read_to_string(&mut content).is_err() {
        eprintln!(
            "{}",
            InterpreterError::Argument(ArgumentError::ProgramFileInvalidEncoding)
        );
        return;
    }

    if let Err(err) = i.run(&content) {
        eprintln!("{err}");
    }
}
