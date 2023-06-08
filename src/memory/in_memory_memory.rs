use crate::types::{Instruction, Offset, ReadWriteable};

use super::Memory;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct InMemoryMemory {
    pub pc: usize,
    pub memory: Vec<u8>,
}

impl InMemoryMemory {
    pub fn from_vec(memory: Vec<u8>) -> Self {
        InMemoryMemory { pc: 0, memory }
    }

    pub fn builder() -> InMemoryBuilder {
        InMemoryBuilder::new()
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

    fn seek(&mut self, pos: Offset) -> Result<(), Self::Error> {
        self.pc = (self.pc as i16 + pos.0) as usize;
        Ok(())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct InMemoryBuilder {
    memory: Vec<u8>,
}

impl InMemoryBuilder {
    fn new() -> Self {
        InMemoryBuilder { memory: Vec::new() }
    }

    pub fn instruction<Rw: ReadWriteable>(mut self, instruction: Instruction, args: Rw) -> Self {
        self.memory.push(instruction as u8);
        let mut buffer = vec![0_u8; Rw::NUM_BYTES];
        args.into_bytes(&mut buffer);
        self.memory.extend_from_slice(&buffer);
        self
    }

    pub fn data<Rw: ReadWriteable>(mut self, data: Rw) -> Self {
        let mut buffer = vec![0_u8; Rw::NUM_BYTES];
        data.into_bytes(&mut buffer);
        self.memory.extend_from_slice(&buffer);
        self
    }

    pub fn byte(mut self, byte: u8) -> Self {
        self.memory.push(byte);
        self
    }

    pub fn bytes(mut self, bytes: &[u8]) -> Self {
        self.memory.extend_from_slice(bytes);
        self
    }

    pub fn build(self) -> InMemoryMemory {
        InMemoryMemory::from_vec(self.memory)
    }
}
