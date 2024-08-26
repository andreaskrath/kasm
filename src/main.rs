use kasm::Interpreter;

fn main() {
    let mut p = Interpreter::default();
    // this is just an example program for now
    let program = String::from("set ra 2500\nstp");

    if let Err(err) = p.run(program) {
        eprintln!("{}", err)
    }
}
