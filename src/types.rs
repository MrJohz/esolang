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
pub struct UnsignedInteger(u32);

impl From<u32> for UnsignedInteger {
    fn from(val: u32) -> Self {
        UnsignedInteger(val)
    }
}

impl From<UnsignedInteger> for u32 {
    fn from(val: UnsignedInteger) -> Self {
        val.0
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignedInteger(i32);

impl From<u32> for SignedInteger {
    fn from(val: u32) -> Self {
        SignedInteger(val as i32)
    }
}

impl From<SignedInteger> for u32 {
    fn from(val: SignedInteger) -> Self {
        val.0 as u32
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

    pub fn as_bytes(&self) -> [u8; 16] {
        let mut res = [0; 16];
        for (i, val) in self.data.iter().enumerate() {
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
    ($($a:literal => $name:ident($($argtype:ident $argname:ident),*),)+) => {
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
    };
}

instructions! {
    // Jumps
    0x01 => Jump(Address to),
    0x02 => JumpIfNotEqual(Address cmpleft, Address cmpright, Address to),
    0x03 => JumpIfLessThan(Address cmpleft, Address cmpright, Address to),

    // Maths on integers
    0x11 => AddIntegerUnsigned(Address left, Address right, Address to),
    0x12 => AddConstIntegerUnsigned(Address left, UnsignedInteger right, Address to),
    0x13 => AddIntegerSigned(Address left, Address right, Address to),
    0x14 => AddConstIntegerSigned(Address left, SignedInteger right, Address to),

    0x21 => SubtractIntegerUnsigned(Address left, Address right, Address to),
    0x22 => SubtractConstIntegerUnsigned(Address left, UnsignedInteger right, Address to),
    0x23 => SubtractIntegerSigned(Address left, Address right, Address to),
    0x24 => SubtractConstIntegerSigned(Address left, SignedInteger right, Address to),

    0x31 => MultiplyIntegerUnsigned(Address left, Address right, Address to),
    0x32 => MultiplyConstIntegerUnsigned(Address left, UnsignedInteger right, Address to),
    0x33 => MultiplyIntegerSigned(Address left, Address right, Address to),
    0x34 =>  MultiplyConstIntegerSigned(Address left, SignedInteger right, Address to),

    0x41 => DivideIntegerUnsigned(Address left, Address right, Address to),
    0x42 => DivideConstIntegerUnsigned(Address left, UnsignedInteger right, Address to),
    0x43 => DivideIntegerSigned(Address left, Address right, Address to),
    0x44 => DivideConstIntegerSigned(Address left, SignedInteger right, Address to),

    // Maths on floats
    0x51 => AddFloat(Address left, Address right, Address to),
    0x52 => AddConstFloat(Address left, Float right, Address to),

    0x53 => SubtractFloat(Address left, Address right, Address to),
    0x54 => SubtractConstFloat(Address left, Float right, Address to),

    0x55 => MultiplyFloat(Address left, Address right, Address to),
    0x56 => MultiplyConstFloat(Address left, Float right, Address to),

    0x57 => DivideFloat(Address left, Address right, Address to),
    0x58 => DivideConstFloat(Address left, Float right, Address to),

    // Memory movement
    0x61 => MoveStatic(Address from, Address to),
    0x62 => MoveIndirect(Address from, Address to),
    0x63 => MoveIntegerUnsigned(UnsignedInteger from, Address to),
    0x64 => MoveIntegerSigned(SignedInteger from, Address to),
    0x65 => MoveFloat(Float from, Address to),

    // Syscall
    0x71 => Syscall(UnsignedInteger syscall, Address argument),

    // NOP
    0x0 => NoOperation(),
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
}
