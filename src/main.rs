use processor::Processor;

mod processor;

fn main() {
    let mut p = match Processor::new() {
        Ok(p) => p,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    // this is just an example program for now
    let program = String::from("set ra 2500\nstp");

    if let Err(err) = p.start(&program) {
        eprintln!("{}", err)
    }
}
