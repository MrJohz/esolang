use crate::types::ReadWriteable;

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
