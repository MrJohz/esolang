use crate::memory::Memory;

pub trait ReadWriteable {
    // NOTE: With full const generics, we can(?) replace
    // the &[u8] with [u8; NUM_BYTES] to ensure that the
    // bytes sizes are always statically correct
    const NUM_BYTES: usize;

    fn from_bytes(bytes: &[u8]) -> Self;
    fn into_bytes(self, bytes: &mut [u8]);
}

impl ReadWriteable for u8 {
    const NUM_BYTES: usize = 1;

    fn from_bytes(bytes: &[u8]) -> Self {
        bytes[0]
    }

    fn into_bytes(self, bytes: &mut [u8]) {
        bytes[0] = self;
    }
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

impl ReadWriteable for () {
    const NUM_BYTES: usize = 0;

    fn from_bytes(_bytes: &[u8]) -> Self {}

    fn into_bytes(self, _bytes: &mut [u8]) {}
}

impl<T1: ReadWriteable> ReadWriteable for (T1,) {
    const NUM_BYTES: usize = T1::NUM_BYTES;

    fn from_bytes(bytes: &[u8]) -> Self {
        (T1::from_bytes(bytes),)
    }

    fn into_bytes(self, bytes: &mut [u8]) {
        self.0.into_bytes(bytes);
    }
}

impl<T1: ReadWriteable, T2: ReadWriteable> ReadWriteable for (T1, T2) {
    const NUM_BYTES: usize = T1::NUM_BYTES + T2::NUM_BYTES;

    fn from_bytes(bytes: &[u8]) -> Self {
        (
            T1::from_bytes(&bytes[0..T1::NUM_BYTES]),
            T2::from_bytes(&bytes[T1::NUM_BYTES..]),
        )
    }

    fn into_bytes(self, bytes: &mut [u8]) {
        self.0.into_bytes(&mut bytes[0..T1::NUM_BYTES]);
        self.1.into_bytes(&mut bytes[T1::NUM_BYTES..]);
    }
}

impl<T1: ReadWriteable, T2: ReadWriteable, T3: ReadWriteable> ReadWriteable for (T1, T2, T3) {
    const NUM_BYTES: usize = T1::NUM_BYTES + T2::NUM_BYTES + T3::NUM_BYTES;

    fn from_bytes(bytes: &[u8]) -> Self {
        (
            T1::from_bytes(&bytes[0..T1::NUM_BYTES]),
            T2::from_bytes(&bytes[T1::NUM_BYTES..T1::NUM_BYTES + T2::NUM_BYTES]),
            T3::from_bytes(&bytes[T1::NUM_BYTES + T2::NUM_BYTES..]),
        )
    }

    fn into_bytes(self, bytes: &mut [u8]) {
        self.0.into_bytes(&mut bytes[0..T1::NUM_BYTES]);
        self.1
            .into_bytes(&mut bytes[T1::NUM_BYTES..T1::NUM_BYTES + T2::NUM_BYTES]);
        self.2
            .into_bytes(&mut bytes[T1::NUM_BYTES + T2::NUM_BYTES..]);
    }
}

impl<T1: ReadWriteable, T2: ReadWriteable, T3: ReadWriteable, T4: ReadWriteable> ReadWriteable
    for (T1, T2, T3, T4)
{
    const NUM_BYTES: usize = T1::NUM_BYTES + T2::NUM_BYTES + T3::NUM_BYTES + T4::NUM_BYTES;

    fn from_bytes(bytes: &[u8]) -> Self {
        (
            T1::from_bytes(&bytes[0..T1::NUM_BYTES]),
            T2::from_bytes(&bytes[T1::NUM_BYTES..T1::NUM_BYTES + T2::NUM_BYTES]),
            T3::from_bytes(
                &bytes
                    [T1::NUM_BYTES + T2::NUM_BYTES..T1::NUM_BYTES + T2::NUM_BYTES + T3::NUM_BYTES],
            ),
            T4::from_bytes(&bytes[T1::NUM_BYTES + T2::NUM_BYTES + T3::NUM_BYTES..]),
        )
    }

    fn into_bytes(self, bytes: &mut [u8]) {
        self.0.into_bytes(&mut bytes[0..T1::NUM_BYTES]);
        self.1
            .into_bytes(&mut bytes[T1::NUM_BYTES..T1::NUM_BYTES + T2::NUM_BYTES]);
        self.2.into_bytes(
            &mut bytes
                [T1::NUM_BYTES + T2::NUM_BYTES..T1::NUM_BYTES + T2::NUM_BYTES + T3::NUM_BYTES],
        );
        self.3
            .into_bytes(&mut bytes[T1::NUM_BYTES + T2::NUM_BYTES + T3::NUM_BYTES..]);
    }
}

impl<
        T1: ReadWriteable,
        T2: ReadWriteable,
        T3: ReadWriteable,
        T4: ReadWriteable,
        T5: ReadWriteable,
    > ReadWriteable for (T1, T2, T3, T4, T5)
{
    const NUM_BYTES: usize =
        T1::NUM_BYTES + T2::NUM_BYTES + T3::NUM_BYTES + T4::NUM_BYTES + T5::NUM_BYTES;

    fn from_bytes(bytes: &[u8]) -> Self {
        (
            T1::from_bytes(&bytes[0..T1::NUM_BYTES]),
            T2::from_bytes(&bytes[T1::NUM_BYTES..T1::NUM_BYTES + T2::NUM_BYTES]),
            T3::from_bytes(
                &bytes
                    [T1::NUM_BYTES + T2::NUM_BYTES..T1::NUM_BYTES + T2::NUM_BYTES + T3::NUM_BYTES],
            ),
            T4::from_bytes(
                &bytes[T1::NUM_BYTES + T2::NUM_BYTES + T3::NUM_BYTES
                    ..T1::NUM_BYTES + T2::NUM_BYTES + T3::NUM_BYTES + T4::NUM_BYTES],
            ),
            T5::from_bytes(&bytes[T1::NUM_BYTES + T2::NUM_BYTES + T3::NUM_BYTES + T4::NUM_BYTES..]),
        )
    }

    fn into_bytes(self, bytes: &mut [u8]) {
        self.0.into_bytes(&mut bytes[0..T1::NUM_BYTES]);
        self.1
            .into_bytes(&mut bytes[T1::NUM_BYTES..T1::NUM_BYTES + T2::NUM_BYTES]);
        self.2.into_bytes(
            &mut bytes
                [T1::NUM_BYTES + T2::NUM_BYTES..T1::NUM_BYTES + T2::NUM_BYTES + T3::NUM_BYTES],
        );
        self.3.into_bytes(
            &mut bytes[T1::NUM_BYTES + T2::NUM_BYTES + T3::NUM_BYTES
                ..T1::NUM_BYTES + T2::NUM_BYTES + T3::NUM_BYTES + T4::NUM_BYTES],
        );
        self.4.into_bytes(
            &mut bytes[T1::NUM_BYTES + T2::NUM_BYTES + T3::NUM_BYTES + T4::NUM_BYTES..],
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Offset(pub i16);

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
pub struct OffsetPair(pub Offset, pub Offset);

impl ReadWriteable for OffsetPair {
    const NUM_BYTES: usize = <(Offset, Offset) as ReadWriteable>::NUM_BYTES;

    fn from_bytes(bytes: &[u8]) -> Self {
        let (left, right) = <_ as ReadWriteable>::from_bytes(bytes);
        Self(left, right)
    }

    fn into_bytes(self, bytes: &mut [u8]) {
        (self.0, self.1).into_bytes(bytes)
    }
}

impl ReadWriteable for bool {
    const NUM_BYTES: usize = 1;

    fn from_bytes(bytes: &[u8]) -> Self {
        bytes[0] != 0
    }

    fn into_bytes(self, bytes: &mut [u8]) {
        bytes[0] = self as u8;
    }
}

impl ReadWriteable for u64 {
    const NUM_BYTES: usize = 8;

    fn from_bytes(bytes: &[u8]) -> Self {
        u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ])
    }

    fn into_bytes(self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.to_le_bytes());
    }
}

impl ReadWriteable for u32 {
    const NUM_BYTES: usize = 4;

    fn from_bytes(bytes: &[u8]) -> Self {
        u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }

    fn into_bytes(self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.to_le_bytes());
    }
}

impl ReadWriteable for f32 {
    const NUM_BYTES: usize = 4;

    fn from_bytes(bytes: &[u8]) -> Self {
        f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }

    fn into_bytes(self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.to_le_bytes());
    }
}

impl ReadWriteable for f64 {
    const NUM_BYTES: usize = 8;

    fn from_bytes(bytes: &[u8]) -> Self {
        f64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ])
    }

    fn into_bytes(self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.to_le_bytes());
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
        mem.seek(offset)?;
    },
    0x02 => JumpIf(cond: bool, offset: Offset) |mem| {
        if cond {
            mem.seek(offset)?;
        }
    },

    0x10 => AddInteger64(left: u64, right: u64, output: OffsetPair) |mem| {
        mem.seek(output.0)?;
        mem.write(left + right)?;
        mem.seek(output.1)?;
    },
    0x11 => SubtractInteger64(left: u64, right: u64, output: OffsetPair) |mem| {
        mem.seek(output.0)?;
        mem.write(left - right)?;
        mem.seek(output.1)?;
    },
    0x12 => MultiplyInteger64(left: u64, right: u64, output: OffsetPair) |mem| {
        mem.seek(output.0)?;
        mem.write(left * right)?;
        mem.seek(output.1)?;
    },
    0x13 => DivideUnsignedInteger64(left: u64, right: u64, output: OffsetPair) |mem| {
        mem.seek(output.0)?;
        mem.write(left / right)?;
        mem.seek(output.1)?;
    },
    0x14 => DivideSignedInteger64(left: u64, right: u64, output: OffsetPair) |mem| {
        mem.seek(output.0)?;
        mem.write((left as i64 / right as i64) as u64)?;
        mem.seek(output.1)?;
    },
    0x15 => ModuloUnsignedInteger64(left: u64, right: u64, output: OffsetPair) |mem| {
        mem.seek(output.0)?;
        mem.write(left % right)?;
        mem.seek(output.1)?;
    },
    0x16 => ModuloSignedInteger64(left: u64, right: u64, output: OffsetPair) |mem| {
        mem.seek(output.0)?;
        mem.write((left as i64 % right as i64) as u64)?;
        mem.seek(output.1)?;
    },

    0x20 => AddInteger32(left: u32, right: u32, output: OffsetPair) |mem| {
        mem.seek(output.0)?;
        mem.write(left + right)?;
        mem.seek(output.1)?;
    },
    0x21 => SubtractInteger32(left: u32, right: u32, output: OffsetPair) |mem| {
        mem.seek(output.0)?;
        mem.write(left - right)?;
        mem.seek(output.1)?;
    },
    0x22 => MultiplyInteger32(left: u32, right: u32, output: OffsetPair) |mem| {
        mem.seek(output.0)?;
        mem.write(left * right)?;
        mem.seek(output.1)?;
    },
    0x23 => DivideUnsignedInteger32(left: u32, right: u32, output: OffsetPair) |mem| {
        mem.seek(output.0)?;
        mem.write(left / right)?;
        mem.seek(output.1)?;
    },
    0x24 => DivideSignedInteger32(left: u32, right: u32, output: OffsetPair) |mem| {
        mem.seek(output.0)?;
        mem.write((left as i32 / right as i32) as u32)?;
        mem.seek(output.1)?;
    },
    0x25 => ModuloUnsignedInteger32(left: u32, right: u32, output: OffsetPair) |mem| {
        mem.seek(output.0)?;
        mem.write(left % right)?;
        mem.seek(output.1)?;
    },
    0x26 => ModuloSignedInteger32(left: u32, right: u32, output: OffsetPair) |mem| {
        mem.seek(output.0)?;
        mem.write((left as i32 % right as i32) as u32)?;
        mem.seek(output.1)?;
    },

    0x30 => AddFloat32(left: f32, right: f32, output: OffsetPair) |mem| {
        mem.seek(output.0)?;
        mem.write(left + right)?;
        mem.seek(output.1)?;
    },
    0x31 => SubtractFloat32(left: f32, right: f32, output: OffsetPair) |mem| {
        mem.seek(output.0)?;
        mem.write(left - right)?;
        mem.seek(output.1)?;
    },
    0x32 => MultiplyFloat32(left: f32, right: f32, output: OffsetPair) |mem| {
        mem.seek(output.0)?;
        mem.write(left * right)?;
        mem.seek(output.1)?;
    },
    0x33 => DivideFloat32(left: f32, right: f32, output: OffsetPair) |mem| {
        mem.seek(output.0)?;
        mem.write(left / right)?;
        mem.seek(output.1)?;
    },
    0x34 => ModuloFloat32(left: f32, right: f32, output: OffsetPair) |mem| {
        mem.seek(output.0)?;
        mem.write(left % right)?;
        mem.seek(output.1)?;
    },
    0x35 => PowerFloat32(left: f32, right: f32, output: OffsetPair) |mem| {
        mem.seek(output.0)?;
        mem.write(left.powf(right))?;
        mem.seek(output.1)?;
    },
    0x36 => AddFloat64(left: f64, right: f64, output: OffsetPair) |mem| {
        mem.seek(output.0)?;
        mem.write(left + right)?;
        mem.seek(output.1)?;
    },
    0x37 => SubtractFloat64(left: f64, right: f64, output: OffsetPair) |mem| {
        mem.seek(output.0)?;
        mem.write(left - right)?;
        mem.seek(output.1)?;
    },
    0x38 => MultiplyFloat64(left: f64, right: f64, output: OffsetPair) |mem| {
        mem.seek(output.0)?;
        mem.write(left * right)?;
        mem.seek(output.1)?;
    },
    0x39 => DivideFloat64(left: f64, right: f64, output: OffsetPair) |mem| {
        mem.seek(output.0)?;
        mem.write(left / right)?;
        mem.seek(output.1)?;
    },
    0x3A => ModuloFloat64(left: f64, right: f64, output: OffsetPair) |mem| {
        mem.seek(output.0)?;
        mem.write(left % right)?;
        mem.seek(output.1)?;
    },
    0x3B => PowerFloat64(left: f64, right: f64, output: OffsetPair) |mem| {
        mem.seek(output.0)?;
        mem.write(left.powf(right))?;
        mem.seek(output.1)?;
    },

    0x60 => Move1(from: Offset, to: Offset) |mem| {
        mem.seek(from)?;
        let value = mem.read::<[u8; 1]>()?;
        mem.seek(to)?;
        mem.write(value)?;
    },
    0x61 => Move2(from: Offset, to: Offset) |mem| {
        mem.seek(from)?;
        let value = mem.read::<[u8; 2]>()?;
        mem.seek(to)?;
        mem.write(value)?;
    },
    0x62 => Move4(from: Offset, to: Offset) |mem| {
        mem.seek(from)?;
        let value = mem.read::<[u8; 4]>()?;
        mem.seek(to)?;
        mem.write(value)?;
    },
    0x63 => Move8(from: Offset, to: Offset) |mem| {
        mem.seek(from)?;
        let value = mem.read::<[u8; 8]>()?;
        mem.seek(to)?;
        mem.write(value)?;
    },
    0x64 => MoveN(size: u8, from: Offset, to: Offset) |mem| {
        mem.seek(from)?;
        let mut value = Vec::with_capacity(size as usize);
        while value.len() < size as usize {
            value.push(mem.read::<u8>()?);
        }

        mem.seek(to)?;
        for byte in value {
            mem.write::<u8>(byte)?;
        }
    },
    0xA0 => PrintFloat() |_mem| {todo!()},
}

#[cfg(test)]
mod tests {
    use crate::memory::InMemoryMemory;

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
    fn test_executing_u32_add_on_zeroes() {
        let mut mem = InMemoryMemory::from_vec(vec![0x00; 12]);
        let instruction = Instruction::AddInteger32;
        instruction.execute(&mut mem).unwrap();

        assert_eq!(mem.pc, 12);
        assert_eq!(mem.read::<u32>().unwrap(), 0);
        assert_eq!(mem.memory.len(), 16);
    }

    #[test]
    fn test_executing_u32_add_on_nonzero_values() {
        let instruction = Instruction::AddInteger32;
        let mut mem = InMemoryMemory::builder()
            .data(5_u32)
            .data(15_u32)
            .data((Offset(0), Offset(0)))
            .build();
        instruction.execute(&mut mem).unwrap();

        assert_eq!(mem.pc, 12);
        assert_eq!(mem.read::<u32>().unwrap(), 20);
        assert_eq!(mem.memory.len(), 16);
    }
}
