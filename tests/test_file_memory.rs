use esolang::{
    memory::InMemoryMemory,
    types::{Float32, Instruction, Offset},
};

#[test]
fn can_create_and_run_basic_programs() {
    let mut builder = InMemoryMemory::builder()
        .data((
            Float32(1.0),
            Float32(2.0),
            Float32(0.0),
            Float32(3.0),
            Float32(0.0),
        ))
        .instruction(Instruction::AddFloat32, (Offset(-5 - 20), Offset(13)))
        .instruction(Instruction::PowerFloat32, (Offset(-10 - 12), Offset(10)))
        .build();
}
