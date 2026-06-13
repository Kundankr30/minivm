use crate::isa;
use std::convert::TryInto;

pub fn disassemble(file: &str) -> Result<(), String> {
    let bytes = std::fs::read(file)
        .map_err(|e| format!("Could not read file: {}", e))?;

    if &bytes[0..4] != &[0x4D, 0x56, 0x4D, 0x00] {
        return Err("not a valid .tbc file".to_string());
    }

    if bytes[4] != 0x01 {
        return Err("wrong version".to_string());
    }

    let code_len = u32::from_le_bytes(
        bytes[5..9]
            .try_into()
            .map_err(|_| "invalid code length".to_string())?,
    ) as usize;

    let code = &bytes[9..9 + code_len];

    let mut ip = 0;

    while ip < code.len() {
        let (op, new_ip) = isa::decode(code, ip)?;
        println!("{}", op_to_string(op));
        ip = new_ip;
    }

    Ok(())
}

fn op_to_string(op: isa::Op) -> String {
    match op {
        isa::Op::Push(n) => format!("PUSH {}", n),
        isa::Op::Pop => "POP".to_string(),
        isa::Op::Dup => "DUP".to_string(),
        isa::Op::Swap => "SWAP".to_string(),
        isa::Op::Add => "ADD".to_string(),
        isa::Op::Sub => "SUB".to_string(),
        isa::Op::Mul => "MUL".to_string(),
        isa::Op::Div => "DIV".to_string(),
        isa::Op::Mod => "MOD".to_string(),
        isa::Op::Neg => "NEG".to_string(),
        isa::Op::Load(s) => format!("LOAD {}", s),
        isa::Op::Store(s) => format!("STORE {}", s),
        isa::Op::Print => "PRINT".to_string(),
        isa::Op::Halt => "HALT".to_string(),
    }
}