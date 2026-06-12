#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op{
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
}
pub fn encode(op:Op) ->Vec<u8>{
    match op{
        Op::Push(n) =>{
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
        Op::Load(s) => vec![0x40,s],
        Op::Store(s)=> vec![0x41,s],
        Op::Print => vec![0x60],
        Op::Halt => vec![0xFF],

    }
}
pub fn decode(bytes:&[u8],ip:usize) ->(Op,usize){
    match bytes[ip]{
        0x01=>{
            let n = i64::from_le_bytes(bytes[ip+1..ip+9].try_into().unwrap());
            (Op::Push(n),ip+9)
        }
        0x02 => (Op::Pop,ip+1),
        0x03 =>(Op::Dup,ip+1),
        0x04 =>(Op::Swap,ip+1),
        0x10 =>(Op::Add,ip+1),
        0x11 =>(Op::Sub,ip+1),
        0x12 =>(Op::Mul,ip+1),
        0x13 =>(Op::Div,ip+1),
        0x14 =>(Op::Mod,ip+1),
        0x15 =>(Op::Neg,ip+1),
        0x40 =>(Op::Load(bytes[ip+1]),ip+2),
        0x41=>(Op::Store(bytes[ip+1]),ip+2),
        0x60 =>(Op::Print,ip+1),
        0xFF => (Op::Halt,ip+1),
        other => panic!("unknown opcode: 0x{:02X} at ip={}", other, ip),
    }
}