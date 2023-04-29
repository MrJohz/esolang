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
        match line.into() {
            Instruction::NoOperation() => {}
            Instruction::Jump(address) => {
                self.memory.set_offset(address);
            }
            Instruction::JumpIfNotEqual(left, right, address) => {
                if self.memory.get(left) != self.memory.get(right) {
                    self.memory.set_offset(address);
                }
            }
            Instruction::JumpIfLessThan(_, _, _) => todo!(),
            Instruction::AddIntegerUnsigned(_, _, _) => todo!(),
            Instruction::AddConstIntegerUnsigned(_, _, _) => todo!(),
            Instruction::AddIntegerSigned(_, _, _) => todo!(),
            Instruction::AddConstIntegerSigned(_, _, _) => todo!(),
            Instruction::SubtractIntegerUnsigned(_, _, _) => todo!(),
            Instruction::SubtractConstIntegerUnsigned(_, _, _) => todo!(),
            Instruction::SubtractIntegerSigned(_, _, _) => todo!(),
            Instruction::SubtractConstIntegerSigned(_, _, _) => todo!(),
            Instruction::MultiplyIntegerUnsigned(_, _, _) => todo!(),
            Instruction::MultiplyConstIntegerUnsigned(_, _, _) => todo!(),
            Instruction::MultiplyIntegerSigned(_, _, _) => todo!(),
            Instruction::MultiplyConstIntegerSigned(_, _, _) => todo!(),
            Instruction::DivideIntegerUnsigned(_, _, _) => todo!(),
            Instruction::DivideConstIntegerUnsigned(_, _, _) => todo!(),
            Instruction::DivideIntegerSigned(_, _, _) => todo!(),
            Instruction::DivideConstIntegerSigned(_, _, _) => todo!(),
            Instruction::AddFloat(_, _, _) => todo!(),
            Instruction::AddConstFloat(_, _, _) => todo!(),
            Instruction::SubtractFloat(_, _, _) => todo!(),
            Instruction::SubtractConstFloat(_, _, _) => todo!(),
            Instruction::MultiplyFloat(_, _, _) => todo!(),
            Instruction::MultiplyConstFloat(_, _, _) => todo!(),
            Instruction::DivideFloat(_, _, _) => todo!(),
            Instruction::DivideConstFloat(_, _, _) => todo!(),
            Instruction::MoveStatic(_, _) => todo!(),
            Instruction::MoveIndirect(_, _) => todo!(),
            Instruction::MoveIntegerUnsigned(int, address) => self
                .memory
                .set(address, Line::new(int, 0_u32, 0_u32, 0_u32)),
            Instruction::MoveIntegerSigned(_, _) => todo!(),
            Instruction::MoveFloat(_, _) => todo!(),
            Instruction::Syscall(_, _) => todo!(),
        }
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
        let mut machine = Machine::with_memory(InMemoryMemory::from(vec![Line::from(
            Instruction::MoveIntegerUnsigned(123.into(), 0.into()),
        )]));
        machine.run();
        assert_eq!(
            machine.memory,
            InMemoryMemory::from(vec![Line::new(123_u32, 0_u32, 0_u32, 0_u32)]).with_offset(1)
        );
    }

    #[test]
    fn setting_future_memory_is_possible() {
        let mut machine = Machine::with_memory(InMemoryMemory::from(vec![Line::from(
            Instruction::MoveIntegerUnsigned(999.into(), 5.into()),
        )]));
        machine.run();
        assert_eq!(
            machine.memory,
            InMemoryMemory::from(vec![
                Line::from(Instruction::MoveIntegerUnsigned(999.into(), 5.into())),
                Line::new(0_u32, 0_u32, 0_u32, 0_u32),
                Line::new(0_u32, 0_u32, 0_u32, 0_u32),
                Line::new(0_u32, 0_u32, 0_u32, 0_u32),
                Line::new(0_u32, 0_u32, 0_u32, 0_u32),
                Line::new(999_u32, 0_u32, 0_u32, 0_u32),
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
            Line::from(Instruction::MoveIntegerUnsigned(999.into(), 0.into())),
        ]));
        machine.run();
        assert_eq!(
            machine.memory,
            InMemoryMemory::from(vec![
                Line::from(Instruction::JumpIfNotEqual(1.into(), 2.into(), 4.into())),
                Line::new(1000_u32, 0_u32, 0_u32, 0_u32),
                Line::new(1001_u32, 0_u32, 0_u32, 0_u32),
                Line::from(Instruction::MoveIntegerUnsigned(999.into(), 0.into())),
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
            Line::from(Instruction::MoveIntegerUnsigned(0x99_00.into(), 0.into())),
        ]));
        machine.run();
        assert_eq!(
            machine.memory,
            InMemoryMemory::from(vec![
                Line::new(0x99_00_u32, 0_u32, 0_u32, 0_u32),
                Line::new(0x10_00_u32, 0_u32, 0_u32, 0_u32),
                Line::new(0x10_00_u32, 0_u32, 0_u32, 0_u32),
                Line::from(Instruction::MoveIntegerUnsigned(0x99_00.into(), 0.into())),
            ])
            .with_offset(4)
        );
    }
}
