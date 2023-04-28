use std::{fs::File, io::Read, io::Seek};

use crate::types::{Address, Line};

pub trait Memory {
    fn read_next(&mut self) -> Option<Line>;
    fn get(&mut self, address: impl Into<Address>) -> Option<Line>;
    fn set(&mut self, address: impl Into<Address>, line: impl Into<Line>);
    fn set_offset(&mut self, address: impl Into<Address>);
    fn as_bytes(&self) -> Vec<u8>;
}

#[derive(Debug)]
pub struct FileMemory {
    file: File,
}

impl FileMemory {
    pub fn new(file: File) -> Self {
        FileMemory { file }
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

        self.file
            .seek(std::io::SeekFrom::Start(u64::from(u32::from(
                address.into(),
            ))))
            .ok()?;
        let mut buffer = [0_u8; 16];
        self.file.read_exact(&mut buffer).ok()?;
        let line = Line::from(buffer);

        self.file
            .seek(std::io::SeekFrom::Start(cursor))
            .expect("Failed to seek back to original position");

        Some(line)
    }

    fn set(&mut self, address: impl Into<Address>, line: impl Into<Line>) {
        todo!()
    }

    fn set_offset(&mut self, address: impl Into<Address>) {
        todo!()
    }

    fn as_bytes(&self) -> Vec<u8> {
        todo!()
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

    fn as_bytes(&self) -> Vec<u8> {
        self.memory
            .iter()
            .flat_map(|line| line.as_bytes().to_vec())
            .collect()
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
