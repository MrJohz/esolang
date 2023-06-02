use crate::memory::Memory;
use crate::types::Instruction;

#[derive(Debug, Default)]
pub struct Machine<Mem: Memory> {
    pub memory: Mem,
}

impl<Mem: Memory> Machine<Mem> {
    pub fn with_memory(memory: Mem) -> Self {
        Machine { memory }
    }

    pub fn run(&mut self) -> Result<(), Mem::Error> {
        while let Some(instruction) = self.memory.read_if_present::<Instruction>()? {
            instruction.execute(&mut self.memory)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::memory::InMemoryMemory;

    use super::*;

    fn machine(mem: Vec<u8>) -> Machine<InMemoryMemory> {
        Machine::with_memory(InMemoryMemory::from_vec(mem))
    }

    #[test]
    fn running_empty_program_returns_successfully() {
        let mut machine = Machine::with_memory(InMemoryMemory::default());
        machine.run().unwrap();
        assert_eq!(machine.memory, InMemoryMemory::default());
    }

    #[test]
    fn running_program_containing_noops_returns_successfully() {
        let mut machine = machine(vec![0x00, 0x00, 0x00, 0x00]);
        machine.run().unwrap();
        assert_eq!(machine.memory.memory, vec![0x00, 0x00, 0x00, 0x00]);
        assert_eq!(machine.memory.pc, 4);
    }

    #[test]
    fn running_program_that_jumps_outside_of_program_bounds_runs_correctly() {
        let mut machine = machine(vec![Instruction::Jump as u8, 0x11, 0x22, 0x00]);
        machine.run().unwrap();
        assert_eq!(
            machine.memory.memory,
            vec![Instruction::Jump as u8, 0x11, 0x22, 0x00]
        );
        assert_eq!(machine.memory.pc, 0x2214);
    }

    #[test]
    fn running_program_that_sets_data_runs_correctly() {
        let mut machine = machine(vec![Instruction::Move2 as u8, 0xFC, 0xFF, 0x00, 0x00]);
        machine.run().unwrap();
        assert_eq!(
            machine.memory.memory,
            vec![Instruction::Move2 as u8, 0xFC, 0xFF, 0xFC, 0xFF]
        );
        assert_eq!(machine.memory.pc, 5);
    }

    #[test]
    fn setting_future_memory_is_possible() {
        let mut machine = machine(vec![Instruction::Move2 as u8, 0xFC, 0xFF, 0x64, 0x00]);
        machine.run().unwrap();
        assert_eq!(machine.memory.pc, 100 + 5);
        assert_eq!(machine.memory.memory.len(), 100 + 5);
        assert_eq!(machine.memory.memory[101..105], [0x00, 0x00, 0xFC, 0xFF]);
    }
}
