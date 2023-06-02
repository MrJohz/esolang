use crate::machine::Memory;

pub trait ReadWriteable {
    // NOTE: With full const generics, we can(?) replace
    // the &[u8] with [u8; NUM_BYTES] to ensure that the
    // bytes sizes are always statically correct
    const NUM_BYTES: usize;

    fn from_bytes(bytes: &[u8]) -> Self;
    fn into_bytes(self, bytes: &mut [u8]);
}

impl<const N: usize> ReadWriteable for [u8; N] {
    const NUM_BYTES: usize = N;

    fn from_bytes(bytes: &[u8]) -> Self {
        let mut result = [0_u8; N];
        result.copy_from_slice(&bytes[0..N]);
        result
    }

    fn into_bytes(self, bytes: &mut [u8]) {
        bytes[0..N].copy_from_slice(&self);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Offset(i16);

impl ReadWriteable for Offset {
    const NUM_BYTES: usize = 2;

    fn from_bytes(bytes: &[u8]) -> Self {
        Offset(i16::from_le_bytes([bytes[0], bytes[1]]))
    }

    fn into_bytes(self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.0.to_le_bytes());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Boolean(bool);

impl ReadWriteable for Boolean {
    const NUM_BYTES: usize = 1;

    fn from_bytes(bytes: &[u8]) -> Self {
        Boolean(bytes[0] != 0)
    }

    fn into_bytes(self, bytes: &mut [u8]) {
        bytes[0] = self.0 as u8;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Integer64(u64);

impl ReadWriteable for Integer64 {
    const NUM_BYTES: usize = 8;

    fn from_bytes(bytes: &[u8]) -> Self {
        Integer64(u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }

    fn into_bytes(self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.0.to_le_bytes());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Integer32(u32);

impl ReadWriteable for Integer32 {
    const NUM_BYTES: usize = 4;

    fn from_bytes(bytes: &[u8]) -> Self {
        Integer32(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    fn into_bytes(self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.0.to_le_bytes());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Float32(f32);

impl ReadWriteable for Float32 {
    const NUM_BYTES: usize = 4;

    fn from_bytes(bytes: &[u8]) -> Self {
        Float32(f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    fn into_bytes(self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.0.to_le_bytes());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Float64(f64);

impl ReadWriteable for Float64 {
    const NUM_BYTES: usize = 8;

    fn from_bytes(bytes: &[u8]) -> Self {
        Float64(f64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }

    fn into_bytes(self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.0.to_le_bytes());
    }
}

macro_rules! instructions {
    ($($a:literal => $name:ident($($argname:ident: $argtype:ty),*) |$mem:ident| $block:expr,)+) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Instruction {
            $($name = $a,)*
        }

        impl Instruction {
            pub fn execute<Mem: Memory>(&self, mem: &mut Mem) -> Result<(), Mem::Error> {
                match *self {
                    $(Instruction::$name => {
                        $(let $argname = mem.read::<$argtype>()?;)*
                        let $mem = mem;
                        $block;
                    })*
                }
                Ok(())
            }
        }

        impl ReadWriteable for Instruction {
            const NUM_BYTES: usize = 1;


            fn from_bytes(bytes: &[u8]) -> Self {
                match bytes[0] {
                    $(x if x == $a => Instruction::$name,)*
                    _ => Instruction::Noop,
                }
            }


            fn into_bytes(self, bytes: &mut [u8]) {
                bytes[0] = self as u8;
            }
        }
    };
}

instructions! {
    0x00 => Noop() |_mem| {},
    0x01 => Jump(offset: Offset) |mem| {
        mem.seek(offset.0)?;
    },
    0x02 => JumpIf(cond: Boolean, offset: Offset) |mem| {
        if cond.0 {
            mem.seek(offset.0)?;
        }
    },

    0x10 => AddInteger64(a: Integer64, b: Integer64, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Integer64(a.0 + b.0))?;
    },
    0x11 => SubtractInteger64(a: Integer64, b: Integer64, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Integer64(a.0 - b.0))?;
    },
    0x12 => MultiplyInteger64(a: Integer64, b: Integer64, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Integer64(a.0 * b.0))?;
    },
    0x13 => DivideUnsignedInteger64(a: Integer64, b: Integer64, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Integer64(a.0 / b.0))?;
    },
    0x14 => DivideSignedInteger64(a: Integer64, b: Integer64, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Integer64((a.0 as i64 / b.0 as i64) as u64))?;
    },
    0x15 => ModuloUnsignedInteger64(a: Integer64, b: Integer64, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Integer64(a.0 % b.0))?;
    },
    0x16 => ModuloSignedInteger64(a: Integer64, b: Integer64, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Integer64((a.0 as i64 % b.0 as i64) as u64))?;
    },

    0x20 => AddInteger32(a: Integer32, b: Integer32, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Integer32(a.0 + b.0))?;
    },
    0x21 => SubtractInteger32(a: Integer32, b: Integer32, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Integer32(a.0 - b.0))?;
    },
    0x22 => MultiplyInteger32(a: Integer32, b: Integer32, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Integer32(a.0 * b.0))?;
    },
    0x23 => DivideUnsignedInteger32(a: Integer32, b: Integer32, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Integer32(a.0 / b.0))?;
    },
    0x24 => DivideSignedInteger32(a: Integer32, b: Integer32, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Integer32((a.0 as i32 / b.0 as i32) as u32))?;
    },
    0x25 => ModuloUnsignedInteger32(a: Integer32, b: Integer32, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Integer32(a.0 % b.0))?;
    },
    0x26 => ModuloSignedInteger32(a: Integer32, b: Integer32, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Integer32((a.0 as i32 % b.0 as i32) as u32))?;
    },

    0x30 => AddFloat32(a: Float32, b: Float32, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Float32(a.0 + b.0))?;
    },
    0x31 => SubtractFloat32(a: Float32, b: Float32, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Float32(a.0 - b.0))?;
    },
    0x32 => MultiplyFloat32(a: Float32, b: Float32, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Float32(a.0 * b.0))?;
    },
    0x33 => DivideFloat32(a: Float32, b: Float32, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Float32(a.0 / b.0))?;
    },
    0x34 => ModuloFloat32(a: Float32, b: Float32, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Float32(a.0 % b.0))?;
    },
    0x35 => PowerFloat32(a: Float32, b: Float32, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Float32(a.0.powf(b.0)))?;
    },
    0x36 => AddFloat64(a: Float64, b: Float64, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Float64(a.0 + b.0))?;
    },
    0x37 => SubtractFloat64(a: Float64, b: Float64, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Float64(a.0 - b.0))?;
    },
    0x38 => MultiplyFloat64(a: Float64, b: Float64, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Float64(a.0 * b.0))?;
    },
    0x39 => DivideFloat64(a: Float64, b: Float64, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Float64(a.0 / b.0))?;
    },
    0x3A => ModuloFloat64(a: Float64, b: Float64, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Float64(a.0 % b.0))?;
    },
    0x3B => PowerFloat64(a: Float64, b: Float64, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Float64(a.0.powf(b.0)))?;
    },

    0x40 => BitwiseAndInteger64(a: Integer64, b: Integer64, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Integer64(a.0 & b.0))?;
    },
    0x41 => BitwiseOrInteger64(a: Integer64, b: Integer64, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Integer64(a.0 | b.0))?;
    },
    0x42 => BitwiseXorInteger64(a: Integer64, b: Integer64, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Integer64(a.0 ^ b.0))?;
    },
    0x43 => BitwiseNotInteger64(a: Integer64, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Integer64(!a.0))?;
    },
    0x44 => BitwiseShiftLeftInteger64(a: Integer64, b: Integer64, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Integer64(a.0 << b.0))?;
    },
    0x45 => BitwiseShiftRightInteger64(a: Integer64, b: Integer64, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Integer64(a.0 >> b.0))?;
    },
    0x50 => BitwiseAndInteger32(a: Integer32, b: Integer32, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Integer32(a.0 & b.0))?;
    },
    0x51 => BitwiseOrInteger32(a: Integer32, b: Integer32, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Integer32(a.0 | b.0))?;
    },
    0x52 => BitwiseXorInteger32(a: Integer32, b: Integer32, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Integer32(a.0 ^ b.0))?;
    },
    0x53 => BitwiseNotInteger32(a: Integer32, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Integer32(!a.0))?;
    },
    0x54 => BitwiseShiftLeftInteger32(a: Integer32, b: Integer32, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Integer32(a.0 << b.0))?;
    },
    0x55 => BitwiseShiftRightInteger32(a: Integer32, b: Integer32, offset: Offset) |mem| {
        mem.seek(offset.0)?;
        mem.write(Integer32(a.0 >> b.0))?;
    },

    0x60 => Move1(from: Offset, to: Offset) |mem| {
        mem.seek(from.0)?;
        let value = mem.read::<[u8; 1]>()?;
        mem.seek(to.0)?;
        mem.write(value)?;
    },
    0x61 => Move2(from: Offset, to: Offset) |mem| {
        mem.seek(from.0)?;
        let value = mem.read::<[u8; 2]>()?;
        mem.seek(to.0)?;
        mem.write(value)?;
    },
    0x62 => Move4(from: Offset, to: Offset) |mem| {
        mem.seek(from.0)?;
        let value = mem.read::<[u8; 4]>()?;
        mem.seek(to.0)?;
        mem.write(value)?;
    },
    0x63 => Move8(from: Offset, to: Offset) |mem| {
        mem.seek(from.0)?;
        let value = mem.read::<[u8; 8]>()?;
        mem.seek(to.0)?;
        mem.write(value)?;
    },
    0x64 => MoveN(size: [u8; 1], from: Offset, to: Offset) |mem| {
        mem.seek(from.0)?;
        let mut value = Vec::with_capacity(size[0] as usize);
        while value.len() < size[0] as usize {
            value.push(mem.read::<[u8; 1]>()?);
        }

        mem.seek(to.0)?;
        for byte in value {
            mem.write::<[u8; 1]>(byte)?;
        }
    },
}

#[cfg(test)]
mod tests {
    use crate::machine::InMemoryMemory;

    use super::*;

    #[test]
    fn test_executing_jump() {
        let mut mem = InMemoryMemory::from_vec(vec![0x03, 0x02]);
        let instruction = Instruction::Jump;
        instruction.execute(&mut mem).unwrap();
        assert_eq!(mem.pc, 0x0205);
    }

    #[test]
    fn test_executing_jump_if_true() {
        let mut mem = InMemoryMemory::from_vec(vec![0x01, 0x03, 0x02]);
        let instruction = Instruction::JumpIf;
        instruction.execute(&mut mem).unwrap();
        assert_eq!(mem.pc, 0x0206);
    }

    #[test]
    fn test_executing_jump_if_false() {
        let mut mem = InMemoryMemory::from_vec(vec![0x00, 0x03, 0x02]);
        let instruction = Instruction::JumpIf;
        instruction.execute(&mut mem).unwrap();
        assert_eq!(mem.pc, 0x03);
    }

    #[test]
    fn test_executing_i32_add() {
        let mut mem = InMemoryMemory::from_vec(vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
        let instruction = Instruction::AddInteger32;
        instruction.execute(&mut mem).unwrap();

        assert_eq!(mem.pc, 10);
        assert_eq!(mem.read::<Integer32>().unwrap(), Integer32(0));
        assert_eq!(mem.memory.len(), 14);
    }
}
