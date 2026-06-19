#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Push(i64),
    Pop,
    Dup,
    Swap,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,
    Load(u8),
    Store(u8),
    Print,
    Halt,
    EQ,
    LT,
    GT,
    JMP(u32),
    JZ(u32),
    JNZ(u32),
}
pub fn encode(op: Op) -> Vec<u8> {
    match op {
        Op::Push(n) => {
            let mut bytes = vec![0x01];
            //little endian - le
            //[opcode , rest bytes for number]
            bytes.extend_from_slice(&n.to_le_bytes());
            bytes
        }
        Op::Pop => vec![0x02],
        Op::Dup => vec![0x03],
        Op::Swap => vec![0x04],
        Op::Add => vec![0x10],
        Op::Sub => vec![0x11],
        Op::Mul => vec![0x12],
        Op::Div => vec![0x13],
        Op::Mod => vec![0x14],
        Op::Neg => vec![0x15],
        Op::Load(s) => vec![0x40, s],
        Op::Store(s) => vec![0x41, s],
        Op::Print => vec![0x60],
        Op::Halt => vec![0xFF],
        Op::EQ => vec![0x20],
        Op::LT => vec![0x21],
        Op::GT => vec![0x22],
        Op::JMP(addr) => {
            let mut bytes = vec![0x30];
            bytes.extend_from_slice(&addr.to_le_bytes());
            bytes
        }
        Op::JZ(addr) => {
            let mut bytes = vec![0x31];
            bytes.extend_from_slice(&addr.to_le_bytes());
            bytes
        }
        Op::JNZ(addr) => {
            let mut bytes = vec![0x32];
            bytes.extend_from_slice(&addr.to_le_bytes());
            bytes
        }
    }
}
pub fn decode(bytes: &[u8], ip: usize) -> Result<(Op, usize), String> {
    match bytes[ip] {
        0x01 => {
            let n = i64::from_le_bytes(bytes[ip + 1..ip + 9].try_into().unwrap());
            Ok((Op::Push(n), ip + 9))
        }
        0x02 => Ok((Op::Pop, ip + 1)),
        0x03 => Ok((Op::Dup, ip + 1)),
        0x04 => Ok((Op::Swap, ip + 1)),
        0x10 => Ok((Op::Add, ip + 1)),
        0x11 => Ok((Op::Sub, ip + 1)),
        0x12 => Ok((Op::Mul, ip + 1)),
        0x13 => Ok((Op::Div, ip + 1)),
        0x14 => Ok((Op::Mod, ip + 1)),
        0x15 => Ok((Op::Neg, ip + 1)),
        0x40 => Ok((Op::Load(bytes[ip + 1]), ip + 2)),
        0x41 => Ok((Op::Store(bytes[ip + 1]), ip + 2)),
        0x60 => Ok((Op::Print, ip + 1)),
        0xFF => Ok((Op::Halt, ip + 1)),
        0x20 => Ok((Op::EQ, ip + 1)),
        0x21 => Ok((Op::LT, ip + 1)),
        0x22 => Ok((Op::GT, ip + 1)),
        0x30 => {
            let addr = u32::from_le_bytes(bytes[ip + 1..ip + 5].try_into().unwrap());
            Ok((Op::JMP(addr), ip + 5))
        }
        0x31 => {
            let addr = u32::from_le_bytes(bytes[ip + 1..ip + 5].try_into().unwrap());
            Ok((Op::JZ(addr), ip + 5))
        }
        0x32 => {
            let addr = u32::from_le_bytes(bytes[ip + 1..ip + 5].try_into().unwrap());
            Ok((Op::JNZ(addr), ip + 5))
        }
        other => panic!("unknown opcode: 0x{:02X} at ip={}", other, ip),
    }
}
