use std::{
    fs::File,
    io::{Read, Seek, Write},
    path::PathBuf,
};

use clap::Parser;
use types::{Instruction, Line};

mod machine;
mod types;

#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    name: PathBuf,
}

fn main() {
    let args = Args::parse();

    let mut file = File::options()
        .read(true)
        .write(true)
        .open(args.name)
        .expect("Failed to open file");
    let mut memory = Vec::new();
    loop {
        let mut buffer = [0_u8; 16];
        match file.read_exact(&mut buffer) {
            Ok(_) => memory.push(Line::from(buffer)),
            Err(_) => break,
        }
    }
    let memory = machine::InMemoryMemory::from_vec(memory);
    let mut machine = machine::Machine::with_memory(memory);

    machine.run();

    let machine = machine::Machine::with_memory(machine::InMemoryMemory::from_vec(vec![
        Line::from(Instruction::JumpIfNotEqual(1.into(), 2.into(), 4.into())),
        Line::new(0x10_99_u32, 0_u32, 0_u32, 0_u32),
        Line::new(0x10_99_u32, 0_u32, 0_u32, 0_u32),
        Line::from(Instruction::MoveIntegerUnsigned(0x99_99.into(), 0.into())),
    ]));
    file.seek(std::io::SeekFrom::Start(0))
        .expect("seeking failed");
    file.write_all(&machine.as_bytes()).expect("writing failed");
}
