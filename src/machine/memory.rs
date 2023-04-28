use std::ops::Index;

use crate::types::{Address, Line};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Memory {
    pc: Address,
    memory: Vec<Line>,
}

impl Memory {
    pub fn from_vec(memory: Vec<Line>) -> Self {
        Memory {
            pc: Address::from(0),
            memory,
        }
    }

    pub fn with_offset(mut self, address: impl Into<Address>) -> Self {
        self.pc = address.into();
        self
    }

    pub fn read_next(&mut self) -> Option<Line> {
        let line = self.get(self.pc);
        if line.is_some() {
            self.pc.incr();
        }
        line
    }

    pub fn set_offset(&mut self, address: impl Into<Address>) {
        self.pc = address.into();
    }

    pub fn get(&self, address: impl Into<Address>) -> Option<Line> {
        self.memory.get(u64::from(address.into()) as usize).copied()
    }

    pub fn set(&mut self, address: impl Into<Address>, line: impl Into<Line>) {
        let address = u64::from(address.into()) as usize;
        if (address + 1) > self.memory.len() {
            self.memory.resize(address + 1, Line::default());
        }

        self.memory[address] = line.into();
    }
}

impl From<Vec<Line>> for Memory {
    fn from(memory: Vec<Line>) -> Self {
        Memory {
            pc: Address::from(0),
            memory,
        }
    }
}
