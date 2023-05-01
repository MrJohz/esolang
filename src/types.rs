use std::ops::Add;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address(u32);

impl Address {
    pub fn incr(&mut self) {
        self.0 += 1;
    }
}

impl From<u32> for Address {
    fn from(val: u32) -> Self {
        Address(val)
    }
}

impl From<Address> for u32 {
    fn from(val: Address) -> Self {
        val.0
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnsignedInteger(u64);

impl From<u32> for UnsignedInteger {
    fn from(val: u32) -> Self {
        UnsignedInteger(val as u64)
    }
}

impl From<UnsignedInteger> for u32 {
    fn from(val: UnsignedInteger) -> Self {
        val.0 as u32
    }
}

impl From<u64> for UnsignedInteger {
    fn from(val: u64) -> Self {
        UnsignedInteger(val)
    }
}

impl From<Line> for UnsignedInteger {
    fn from(val: Line) -> Self {
        UnsignedInteger(u64::from_ne_bytes(val.as_bytes()))
    }
}

impl From<UnsignedInteger> for Line {
    fn from(val: UnsignedInteger) -> Self {
        let bytes = val.0.to_ne_bytes();

        Line::new(
            u32::from_ne_bytes(bytes[0..4].try_into().unwrap()),
            u32::from_ne_bytes(bytes[4..8].try_into().unwrap()),
            0_u32,
            0_u32,
        )
    }
}

impl Add for UnsignedInteger {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        UnsignedInteger(self.0 + rhs.0)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignedInteger(i64);

impl From<i64> for SignedInteger {
    fn from(val: i64) -> Self {
        SignedInteger(val)
    }
}

impl From<Line> for SignedInteger {
    fn from(val: Line) -> Self {
        Self(i64::from_ne_bytes(val.as_bytes()))
    }
}

impl From<SignedInteger> for Line {
    fn from(val: SignedInteger) -> Self {
        let bytes = val.0.to_ne_bytes();

        Line::new(
            u32::from_ne_bytes(bytes[0..4].try_into().unwrap()),
            u32::from_ne_bytes(bytes[4..8].try_into().unwrap()),
            0_u32,
            0_u32,
        )
    }
}

impl Add for SignedInteger {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Float(f32);

impl From<u32> for Float {
    fn from(val: u32) -> Self {
        Float(f32::from_ne_bytes(val.to_ne_bytes()))
    }
}

impl From<Float> for u32 {
    fn from(val: Float) -> Self {
        u32::from_ne_bytes(val.0.to_ne_bytes())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Line {
    data: [u32; 4],
}

impl Line {
    pub fn new(
        op0: impl Into<u32>,
        op1: impl Into<u32>,
        op2: impl Into<u32>,
        op3: impl Into<u32>,
    ) -> Self {
        Line {
            data: [op0.into(), op1.into(), op2.into(), op3.into()],
        }
    }

    pub fn as_bytes<const N: usize>(&self) -> [u8; N] {
        let mut res = [0; N];
        for (i, val) in self.data.iter().enumerate() {
            if i == N / 4 {
                break;
            }

            res[i * 4..(i + 1) * 4].copy_from_slice(&val.to_ne_bytes());
        }
        res
    }
}

impl From<[u32; 4]> for Line {
    fn from(val: [u32; 4]) -> Self {
        Line { data: val }
    }
}

impl From<&[u32; 4]> for Line {
    fn from(val: &[u32; 4]) -> Self {
        Line { data: *val }
    }
}

impl From<[u8; 16]> for Line {
    fn from(val: [u8; 16]) -> Self {
        Line {
            data: [
                u32::from_ne_bytes([val[0], val[1], val[2], val[3]]),
                u32::from_ne_bytes([val[4], val[5], val[6], val[7]]),
                u32::from_ne_bytes([val[8], val[9], val[10], val[11]]),
                u32::from_ne_bytes([val[12], val[13], val[14], val[15]]),
            ],
        }
    }
}

impl From<&[u8; 16]> for Line {
    fn from(val: &[u8; 16]) -> Self {
        Line {
            data: [
                u32::from_ne_bytes([val[0], val[1], val[2], val[3]]),
                u32::from_ne_bytes([val[4], val[5], val[6], val[7]]),
                u32::from_ne_bytes([val[8], val[9], val[10], val[11]]),
                u32::from_ne_bytes([val[12], val[13], val[14], val[15]]),
            ],
        }
    }
}

macro_rules! instructions {
    ($($a:literal => $name:ident($($argtype:ident $argname:ident),*) |$mem:ident| $block:expr,)+) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum Instruction {
            $($name($($argtype,)*),)*
        }

        impl From<Line> for Instruction {
            fn from(line: Line) -> Self {
                let mut idx = 0;
                match line.data[0] {
                    $($a => Instruction::$name($({
                        idx += 1;
                        $argtype::from(line.data[idx])
                },)*),)*
                    _ => Instruction::NoOperation(),
                }
            }
        }

        impl From<Instruction> for Line {
            fn from(instr: Instruction) -> Self {
                match instr {
                    $(Instruction::$name($($argname,)*) => {
                        let mut line = Line::default();
                        line.data[0] = $a;

                        let mut _idx = 0;
                        $(
                            _idx += 1;
                            line.data[_idx] = $argname.into();
                        )*
                        line
                    })*
                }
            }
        }

        impl Instruction {
            pub fn execute(&self, mem: &mut impl crate::machine::Memory) {
                match *self {
                    $(Instruction::$name($($argname,)*) => {
                        let $mem = mem;
                        $block;
                    })*
                }
            }
        }
    };
}

instructions! {
    // Jumps
    0x01 => Jump(Address to) |mem| mem.set_offset(to),
    0x02 => JumpIfNotEqual(Address cmpleft, Address cmpright, Address to) |mem| {
        if mem.get(cmpleft) != mem.get(cmpright) {
            mem.set_offset(to);
        }
    },
    0x03 => JumpIfLessThan(Address cmpleft, Address cmpright, Address to) |mem| {
        if mem.get(cmpleft) < mem.get(cmpright) {
            mem.set_offset(to);
        }
    },

    // Maths on integers
    0x11 => AddIntegerUnsigned(Address left, Address right, Address to) |mem| {
        let left: UnsignedInteger = mem.get(left).unwrap_or_default().into();
        let right: UnsignedInteger = mem.get(right).unwrap_or_default().into();
        mem.set(to, left + right);
    },
    0x12 => AddIntegerSigned(Address left, Address right, Address to) |mem| {
        let left: SignedInteger = mem.get(left).unwrap_or_default().into();
        let right: SignedInteger = mem.get(right).unwrap_or_default().into();
        mem.set(to, left + right);
    },

    0x21 => SubtractIntegerUnsigned(Address _left, Address _right, Address _to) |_mem| todo!(),
    0x22 => SubtractIntegerSigned(Address _left, Address _right, Address _to) |_mem| todo!(),

    0x31 => MultiplyIntegerUnsigned(Address _left, Address _right, Address _to) |_mem| todo!(),
    0x32 => MultiplyIntegerSigned(Address _left, Address _right, Address _to) |_mem| todo!(),

    0x41 => DivideIntegerUnsigned(Address _left, Address _right, Address _to) |_mem| todo!(),
    0x42 => DivideIntegerSigned(Address _left, Address _right, Address _to) |_mem| todo!(),

    // Maths on floats
    0x51 => AddFloat(Address _left, Address _right, Address _to) |_mem| todo!(),
    0x52 => SubtractFloat(Address _left, Address _right, Address _to) |_mem| todo!(),
    0x53 => MultiplyFloat(Address _left, Address _right, Address _to) |_mem| todo!(),
    0x54 => DivideFloat(Address _left, Address _right, Address _to) |_mem| todo!(),

    // Memory movement
    0x61 => MoveStatic(Address from, Address to) |mem| {
        let line = mem.get(from).unwrap_or_default();
        mem.set(to, line);
    },
    0x62 => MoveIndirect(Address _from, Address _to) |_mem| todo!(),

    // Syscall
    0x71 => Syscall(UnsignedInteger _syscall, Address _argument) |_mem| todo!(),

    // NOP
    0x0 => NoOperation() |_mem| {},
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_create_noop_from_line() {
        let line = Line::from([0, 0, 0, 0]);
        let instr = Instruction::from(line);
        assert_eq!(instr, Instruction::NoOperation());

        let line = Line::from([999, 0, 0, 0]);
        let instr = Instruction::from(line);
        assert_eq!(instr, Instruction::NoOperation());
    }

    #[test]
    fn can_create_jump_from_line() {
        let line = Line::from([1, 10, 0, 0]);
        let instr = Instruction::from(line);
        assert_eq!(instr, Instruction::Jump(10.into()));
    }

    #[test]
    fn can_create_extended_jump_from_line() {
        let line = Line::from([2, 10, 11, 12]);
        let instr = Instruction::from(line);
        assert_eq!(
            instr,
            Instruction::JumpIfNotEqual(Address::from(10), Address::from(11), Address::from(12))
        );
    }

    #[test]
    fn can_create_line_from_byte_array() {
        let line = Line::from([
            0x63_u8, 0x00, 0x00, 0x00, 0x99, 0x99, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ]);
        assert_eq!(line, Line::from([0x63, 0x9999, 0x06, 0x00]))
    }
}
