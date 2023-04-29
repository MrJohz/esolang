use std::{
    fs::File,
    io::Read,
    io::{Result, SeekFrom},
    io::{Seek, Write},
    path::Path,
};

use crate::types::{Address, Instruction, Line};

pub trait Memory {
    fn read_next(&mut self) -> Option<Line>;
    fn get(&mut self, address: impl Into<Address>) -> Option<Line>;
    fn set(&mut self, address: impl Into<Address>, line: impl Into<Line>);
    fn set_offset(&mut self, address: impl Into<Address>);
}

#[derive(Debug)]
pub struct FileMemory {
    file: File,
}

impl FileMemory {
    pub fn new(file: impl AsRef<Path>) -> Result<Self> {
        Ok(FileMemory {
            file: File::options()
                .read(true)
                .write(true)
                .create(true)
                .open(file.as_ref())?,
        })
    }
}

impl Memory for FileMemory {
    fn read_next(&mut self) -> Option<Line> {
        let mut buffer = [0_u8; 16];
        self.file.read_exact(&mut buffer).ok()?;
        let line = Line::from(buffer);
        Some(line)
    }

    fn get(&mut self, address: impl Into<Address>) -> Option<Line> {
        let cursor = self.file.stream_position().unwrap();

        self.set_offset(address);
        let mut buffer = [0_u8; 16];
        self.file.read_exact(&mut buffer).ok()?;
        let line = Line::from(buffer);

        self.file
            .seek(std::io::SeekFrom::Start(cursor))
            .expect("Failed to seek back to original position");

        Some(line)
    }

    fn set(&mut self, address: impl Into<Address>, line: impl Into<Line>) {
        let cursor = self.file.stream_position().unwrap();
        self.set_offset(address);
        let line = line.into();
        self.file
            .write_all(&line.as_bytes())
            .expect("Failed to write line");
        self.file
            .seek(std::io::SeekFrom::Start(cursor))
            .expect("Failed to seek back to original position");
    }

    fn set_offset(&mut self, address: impl Into<Address>) {
        let address = u64::from(u32::from(address.into()));
        let position = SeekFrom::Start(address * 16);
        self.file.seek(position).expect("Failed to seek to address");
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct InMemoryMemory {
    pc: Address,
    memory: Vec<Line>,
}

impl InMemoryMemory {
    pub fn from_vec(memory: Vec<Line>) -> Self {
        InMemoryMemory {
            pc: Address::from(0),
            memory,
        }
    }

    pub fn with_offset(mut self, address: impl Into<Address>) -> Self {
        self.pc = address.into();
        self
    }
}

impl Memory for InMemoryMemory {
    fn read_next(&mut self) -> Option<Line> {
        let line = self.get(self.pc);
        if line.is_some() {
            self.pc.incr();
        }
        line
    }

    fn set_offset(&mut self, address: impl Into<Address>) {
        self.pc = address.into();
    }

    fn get(&mut self, address: impl Into<Address>) -> Option<Line> {
        self.memory.get(u32::from(address.into()) as usize).copied()
    }

    fn set(&mut self, address: impl Into<Address>, line: impl Into<Line>) {
        let address = u32::from(address.into()) as usize;
        if (address + 1) > self.memory.len() {
            self.memory.resize(address + 1, Line::default());
        }

        self.memory[address] = line.into();
    }
}

impl From<Vec<Line>> for InMemoryMemory {
    fn from(memory: Vec<Line>) -> Self {
        InMemoryMemory {
            pc: Address::from(0),
            memory,
        }
    }
}
