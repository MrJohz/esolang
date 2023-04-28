#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address(u64);

impl Address {
    pub fn incr(&mut self) {
        self.0 += 1;
    }
}

impl From<u64> for Address {
    fn from(val: u64) -> Self {
        Address(val)
    }
}

impl From<Address> for u64 {
    fn from(val: Address) -> Self {
        val.0
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnsignedInteger(u64);

impl From<u64> for UnsignedInteger {
    fn from(val: u64) -> Self {
        UnsignedInteger(val)
    }
}

impl From<UnsignedInteger> for u64 {
    fn from(val: UnsignedInteger) -> Self {
        val.0
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignedInteger(i64);

impl From<u64> for SignedInteger {
    fn from(val: u64) -> Self {
        SignedInteger(val as i64)
    }
}

impl From<SignedInteger> for u64 {
    fn from(val: SignedInteger) -> Self {
        val.0 as u64
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Float(f64);

impl From<u64> for Float {
    fn from(val: u64) -> Self {
        Float(f64::from_ne_bytes(val.to_ne_bytes()))
    }
}

impl From<Float> for u64 {
    fn from(val: Float) -> Self {
        u64::from_ne_bytes(val.0.to_ne_bytes())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Line {
    data: [u64; 4],
}

impl Line {
    pub fn new(
        op0: impl Into<u64>,
        op1: impl Into<u64>,
        op2: impl Into<u64>,
        op3: impl Into<u64>,
    ) -> Self {
        Line {
            data: [op0.into(), op1.into(), op2.into(), op3.into()],
        }
    }
}

impl From<[u64; 4]> for Line {
    fn from(val: [u64; 4]) -> Self {
        Line { data: val }
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
    1 => Jump(Address to),
    2 => JumpIfNotEqual(Address cmpleft, Address cmpright, Address to),
    3 => JumpIfLessThan(Address cmpleft, Address cmpright, Address to),

    // Maths on integers
    101 => AddIntegerUnsigned(Address left, Address right, Address to),
    102 => AddConstIntegerUnsigned(Address left, UnsignedInteger right, Address to),
    103 => AddIntegerSigned(Address left, Address right, Address to),
    104 => AddConstIntegerSigned(Address left, SignedInteger right, Address to),

    111 => SubtractIntegerUnsigned(Address left, Address right, Address to),
    112 => SubtractConstIntegerUnsigned(Address left, UnsignedInteger right, Address to),
    113 => SubtractIntegerSigned(Address left, Address right, Address to),
    114 => SubtractConstIntegerSigned(Address left, SignedInteger right, Address to),

    121 => MultiplyIntegerUnsigned(Address left, Address right, Address to),
    122 => MultiplyConstIntegerUnsigned(Address left, UnsignedInteger right, Address to),
    123 => MultiplyIntegerSigned(Address left, Address right, Address to),
    124 =>  MultiplyConstIntegerSigned(Address left, SignedInteger right, Address to),

    131 => DivideIntegerUnsigned(Address left, Address right, Address to),
    132 => DivideConstIntegerUnsigned(Address left, UnsignedInteger right, Address to),
    133 => DivideIntegerSigned(Address left, Address right, Address to),
    134 => DivideConstIntegerSigned(Address left, SignedInteger right, Address to),

    // Maths on floats
    201 => AddFloat(Address left, Address right, Address to),
    202 => AddConstFloat(Address left, Float right, Address to),

    203 => SubtractFloat(Address left, Address right, Address to),
    204 => SubtractConstFloat(Address left, Float right, Address to),

    205 => MultiplyFloat(Address left, Address right, Address to),
    206 => MultiplyConstFloat(Address left, Float right, Address to),

    207 => DivideFloat(Address left, Address right, Address to),
    208 => DivideConstFloat(Address left, Float right, Address to),

    // Memory movement
    301 => MoveStatic(Address from, Address to),
    302 => MoveIndirect(Address from, Address to),
    303 => MoveIntegerUnsigned(UnsignedInteger from, Address to),
    304 => MoveIntegerSigned(SignedInteger from, Address to),
    305 => MoveFloat(Float from, Address to),

    // Syscall
    401 => Syscall(UnsignedInteger syscall, Address argument),

    // NOP
    000 => NoOperation(),
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
