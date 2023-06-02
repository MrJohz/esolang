use std::{
    fs::File,
    io::Read,
    io::{Result as IoResult, SeekFrom},
    io::{Seek, Write},
    path::Path,
};

use crate::types::ReadWriteable;

pub trait Memory {
    type Error;

    fn read<T: ReadWriteable>(&mut self) -> Result<T, Self::Error>;
    fn read_if_present<T: ReadWriteable>(&mut self) -> Result<Option<T>, Self::Error>;
    fn write<T: ReadWriteable>(&mut self, value: T) -> Result<(), Self::Error>;
    fn seek(&mut self, pos: i16) -> Result<(), Self::Error>;
}

#[derive(Debug)]
pub struct FileMemory {
    file: File,
}

impl FileMemory {
    pub fn new(file: impl AsRef<Path>) -> IoResult<Self> {
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
    type Error = std::io::Error;

    fn read<T: ReadWriteable>(&mut self) -> Result<T, Self::Error> {
        // NOTE: With full const generics, we should be able to define the
        // buffer size as exactly T::NUM_BYTES, which might(?) allow
        // the compiler to optimize this better and will be clearer

        let mut buffer = [0_u8; 8];
        self.file.read_exact(&mut buffer[0..T::NUM_BYTES])?;
        let value = T::from_bytes(&buffer);
        Ok(value)
    }

    fn read_if_present<T: ReadWriteable>(&mut self) -> Result<Option<T>, Self::Error> {
        let mut buffer = [0_u8; 8];
        match self.file.read_exact(&mut buffer[0..T::NUM_BYTES]) {
            Ok(_) => {
                let value = T::from_bytes(&buffer);
                Ok(Some(value))
            }
            Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn write<T: ReadWriteable>(&mut self, value: T) -> Result<(), Self::Error> {
        let mut buffer = [0_u8; 8];
        value.into_bytes(&mut buffer[0..T::NUM_BYTES]);
        self.file.write_all(&buffer[0..T::NUM_BYTES])?;
        Ok(())
    }

    fn seek(&mut self, pos: i16) -> Result<(), Self::Error> {
        self.file.seek(SeekFrom::Current(pos as i64))?;
        Ok(())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct InMemoryMemory {
    pub pc: usize,
    pub memory: Vec<u8>,
}

impl InMemoryMemory {
    pub fn from_vec(memory: Vec<u8>) -> Self {
        InMemoryMemory { pc: 0, memory }
    }
}

impl Memory for InMemoryMemory {
    type Error = ();

    fn read<T: ReadWriteable>(&mut self) -> Result<T, Self::Error> {
        let buffer = dbg!(&self.memory[dbg!(self.pc)..self.pc + dbg!(T::NUM_BYTES)]);
        self.pc += T::NUM_BYTES;
        Ok(T::from_bytes(buffer))
    }

    fn read_if_present<T: ReadWriteable>(&mut self) -> Result<Option<T>, Self::Error> {
        if (self.pc + T::NUM_BYTES) > self.memory.len() {
            return Ok(None);
        }
        let buffer = &self.memory[self.pc..self.pc + T::NUM_BYTES];
        self.pc += T::NUM_BYTES;
        Ok(Some(T::from_bytes(buffer)))
    }

    fn write<T: ReadWriteable>(&mut self, value: T) -> Result<(), Self::Error> {
        if self.pc + T::NUM_BYTES > self.memory.len() {
            self.memory.resize(self.pc + T::NUM_BYTES, 0);
        }

        let buffer = &mut self.memory[self.pc..self.pc + T::NUM_BYTES];
        value.into_bytes(buffer);
        Ok(())
    }

    fn seek(&mut self, pos: i16) -> Result<(), Self::Error> {
        self.pc = (self.pc as i16 + pos) as usize;
        Ok(())
    }
}
