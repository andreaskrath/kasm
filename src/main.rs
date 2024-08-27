use clap::Parser;
use kasm::{Arguments, Interpreter};

fn main() {
    let args = Arguments::parse();
    let mut p = match Interpreter::try_new(args) {
        Ok(p) => p,
        Err(err) => {
            eprintln!("{err}");
            return;
        }
    };
    // this is just an example program for now
    let program = include_str!("../program.kasm");

    if let Err(err) = p.run(program) {
        eprintln!("{err}");
    }
}
