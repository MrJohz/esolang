use esolang::{
    machine::Machine,
    memory::InMemoryMemory,
    types::{Instruction, Offset},
};

#[test]
fn can_create_and_run_basic_programs() {
    let memory = InMemoryMemory::builder()
        .instruction(
            Instruction::AddFloat32,
            (1.0_f32, 2.0_f32, (Offset(5), Offset(-7))),
        )
        .instruction(
            Instruction::PowerFloat32,
            (3.0_f32, 0.0_f32, (Offset(0), (Offset(0)))),
        )
        .build();
    let mut machine = Machine::with_memory(memory);
    machine.run().unwrap();
    assert_eq!(
        machine.memory.memory,
        InMemoryMemory::builder()
            .instruction(
                Instruction::AddFloat32,
                (1.0_f32, 2.0_f32, (Offset(5), Offset(-7))),
            )
            .instruction(
                Instruction::PowerFloat32,
                (3.0_f32, 3.0_f32, (Offset(0), (Offset(0)))),
            )
            .data((1.0_f32 + 2.0).powf(3.0))
            .build()
            .memory
    )
}
