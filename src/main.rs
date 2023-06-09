use std::path::PathBuf;

use clap::Parser;

use esolang::machine;
use esolang::memory;

#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    name: PathBuf,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    let memory = memory::FileMemory::with_path(args.name).unwrap();
    let mut machine = machine::Machine::with_memory(memory);

    machine.run()?;
    Ok(())
}
