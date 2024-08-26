use kasm::Interpreter;

fn main() {
    let mut p = Interpreter::default();
    // this is just an example program for now
    let program = include_str!("../program.kasm");

    if let Err(err) = p.run(program) {
        eprintln!("{}", err)
    }
}
