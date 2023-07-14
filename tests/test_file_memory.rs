use esolang::{
    machine::Machine,
    memory::{FileMemory, InMemoryMemory},
    types::{Instruction, Offset},
};

#[test]
fn can_create_and_run_basic_programs() {
    let memory = FileMemory::with_file(
        InMemoryMemory::builder()
            .instruction(
                Instruction::AddFloat32,
                (1.0_f32, 2.0_f32, (Offset(5), Offset(-7))),
            )
            .instruction(
                Instruction::PowerFloat32,
                (3.0_f32, 0.0_f32, (Offset(0), (Offset(0)))),
            )
            .to_tmp_file()
            .unwrap(),
    );
    let mut machine = Machine::with_memory(memory);
    machine.run().unwrap();
    // TODO: assertions
}
