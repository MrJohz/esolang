mod memory;

use crate::types::{Instruction, Line};

pub use self::memory::{FileMemory, InMemoryMemory, Memory};

#[derive(Debug, Default)]
pub struct Machine<Mem: Memory> {
    pub memory: Mem,
}

impl<Mem: Memory> Machine<Mem> {
    pub fn with_memory(memory: Mem) -> Self {
        Machine { memory }
    }

    fn run_line(&mut self, line: Line) {
        let instruction: Instruction = line.into();
        instruction.execute(&mut self.memory);
    }

    pub fn run(&mut self) {
        while let Some(line) = self.memory.read_next() {
            self.run_line(line);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::machine::memory::InMemoryMemory;

    use super::*;

    #[test]
    fn running_empty_program_returns_successfully() {
        let mut machine = Machine::with_memory(InMemoryMemory::default());
        machine.run();
        assert_eq!(machine.memory, InMemoryMemory::default());
    }

    #[test]
    fn running_program_containing_noops_returns_successfully() {
        let mut machine = Machine::with_memory(InMemoryMemory::from(vec![Line::from(
            Instruction::NoOperation(),
        )]));
        machine.run();
        assert_eq!(
            machine.memory,
            InMemoryMemory::from(vec![Line::from(Instruction::NoOperation())]).with_offset(1)
        );
    }

    #[test]
    fn running_program_that_jumps_outside_of_program_bounds_runs_correctly() {
        let mut machine = Machine::with_memory(InMemoryMemory::from(vec![Line::from(
            Instruction::Jump(100.into()),
        )]));
        machine.run();
        assert_eq!(
            machine.memory,
            InMemoryMemory::from(vec![Line::from(Instruction::Jump(100.into()))]).with_offset(100)
        );
    }

    #[test]
    fn running_program_that_sets_data_runs_correctly() {
        let mut machine = Machine::with_memory(InMemoryMemory::from(vec![
            Line::new(0x99_99_u32, 0_u32, 0_u32, 0_u32),
            Line::new(0x99_00_u32, 0_u32, 0_u32, 0_u32),
            Line::from(Instruction::MoveStatic(0.into(), 1.into())),
        ]));
        machine.run();
        assert_eq!(
            machine.memory,
            InMemoryMemory::from(vec![
                Line::new(0x99_99_u32, 0_u32, 0_u32, 0_u32),
                Line::new(0x99_99_u32, 0_u32, 0_u32, 0_u32),
                Line::from(Instruction::MoveStatic(0.into(), 1.into())),
            ])
            .with_offset(3)
        );
    }

    #[test]
    fn setting_future_memory_is_possible() {
        let mut machine = Machine::with_memory(InMemoryMemory::from(vec![
            Line::new(0x99_99_u32, 0_u32, 0_u32, 0_u32),
            Line::from(Instruction::MoveStatic(0.into(), 5.into())),
        ]));
        machine.run();
        assert_eq!(
            machine.memory,
            InMemoryMemory::from(vec![
                Line::new(0x99_99_u32, 0_u32, 0_u32, 0_u32),
                Line::from(Instruction::MoveStatic(0.into(), 5.into())),
                Line::new(0_u32, 0_u32, 0_u32, 0_u32),
                Line::new(0_u32, 0_u32, 0_u32, 0_u32),
                Line::new(0_u32, 0_u32, 0_u32, 0_u32),
                Line::new(0x99_99_u32, 0_u32, 0_u32, 0_u32),
            ])
            .with_offset(6)
        );
    }

    #[test]
    fn jump_not_equal_jumps_if_the_memory_addresses_are_unequal() {
        let mut machine = Machine::with_memory(InMemoryMemory::from(vec![
            Line::from(Instruction::JumpIfNotEqual(1.into(), 2.into(), 4.into())),
            Line::new(1000_u32, 0_u32, 0_u32, 0_u32),
            Line::new(1001_u32, 0_u32, 0_u32, 0_u32),
            Line::from(Instruction::MoveStatic(1.into(), 0.into())),
        ]));
        machine.run();
        assert_eq!(
            machine.memory,
            InMemoryMemory::from(vec![
                Line::from(Instruction::JumpIfNotEqual(1.into(), 2.into(), 4.into())),
                Line::new(1000_u32, 0_u32, 0_u32, 0_u32),
                Line::new(1001_u32, 0_u32, 0_u32, 0_u32),
                Line::from(Instruction::MoveStatic(1.into(), 0.into())),
            ])
            .with_offset(4)
        );
    }

    #[test]
    fn jump_not_equal_does_not_jump_if_the_memory_addresses_are_the_same() {
        let mut machine = Machine::with_memory(InMemoryMemory::from(vec![
            Line::from(Instruction::JumpIfNotEqual(1.into(), 2.into(), 4.into())),
            Line::new(0x10_00_u32, 0_u32, 0_u32, 0_u32),
            Line::new(0x10_00_u32, 0_u32, 0_u32, 0_u32),
            Line::from(Instruction::MoveStatic(1.into(), 0.into())),
        ]));
        machine.run();
        assert_eq!(
            machine.memory,
            InMemoryMemory::from(vec![
                Line::new(0x10_00_u32, 0_u32, 0_u32, 0_u32),
                Line::new(0x10_00_u32, 0_u32, 0_u32, 0_u32),
                Line::new(0x10_00_u32, 0_u32, 0_u32, 0_u32),
                Line::from(Instruction::MoveStatic(1.into(), 0.into())),
            ])
            .with_offset(4)
        );
    }

    #[test]
    fn adding_unsigned_64_bit_integers() {
        let mut machine = Machine::with_memory(InMemoryMemory::from(vec![
            Line::from(Instruction::AddIntegerUnsigned(
                1.into(),
                2.into(),
                3.into(),
            )),
            Line::new(0x10_00_u32, 0x01_u32, 0x03_u32, 0x05_u32),
            Line::new(0x10_00_u32, 0x02_u32, 0x04_u32, 0x06_u32),
        ]));
        machine.run();
        assert_eq!(
            machine.memory,
            InMemoryMemory::from(vec![
                Line::from(Instruction::AddIntegerUnsigned(
                    1.into(),
                    2.into(),
                    3.into()
                )),
                Line::new(0x10_00_u32, 0x01_u32, 0x03_u32, 0x05_u32),
                Line::new(0x10_00_u32, 0x02_u32, 0x04_u32, 0x06_u32),
                Line::new(0x20_00_u32, 0x03_u32, 0x00_u32, 0x00_u32),
            ])
            .with_offset(4)
        );
    }

    #[test]
    fn adding_signed_64_bit_integers() {
        let mut machine = Machine::with_memory(InMemoryMemory::from(vec![
            Line::from(Instruction::AddIntegerSigned(1.into(), 2.into(), 3.into())),
            Line::new(0x10_00_u32, 0x01_u32, 0x03_u32, 0x05_u32),
            Line::new(0x10_00_u32, 0x02_u32, 0x04_u32, 0x06_u32),
        ]));
        machine.run();
        assert_eq!(
            machine.memory,
            InMemoryMemory::from(vec![
                Line::from(Instruction::AddIntegerSigned(1.into(), 2.into(), 3.into())),
                Line::new(0x10_00_u32, 0x01_u32, 0x03_u32, 0x05_u32),
                Line::new(0x10_00_u32, 0x02_u32, 0x04_u32, 0x06_u32),
                Line::new(0x20_00_u32, 0x03_u32, 0x00_u32, 0x00_u32),
            ])
            .with_offset(4)
        );
    }
}
