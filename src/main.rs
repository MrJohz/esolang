use std::path::PathBuf;

use clap::Parser;

mod machine;
mod types;

#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    name: PathBuf,
}

fn main() {
    let args = Args::parse();

    let memory = machine::FileMemory::new(args.name).unwrap();
    let mut machine = machine::Machine::with_memory(memory);

    machine.run();
}
