use esolang::{
    memory::InMemoryMemory,
    types::{Instruction, Offset},
};

#[test]
fn can_create_and_run_basic_programs() {
    let mut builder = InMemoryMemory::builder()
        .data((1.0_f32, 2.0_f32, 0.0_f32, 3.0_f32, 0.0_f32))
        .instruction(Instruction::AddFloat32, (Offset(-5 - 20), Offset(13)))
        .instruction(Instruction::PowerFloat32, (Offset(-10 - 12), Offset(10)))
        .build();
}
