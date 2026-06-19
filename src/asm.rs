use crate::isa;
use std::fs;
pub fn assemble(file: &str) -> Result<(), String> {
    let contents = fs::read_to_string(file).expect("Count not real file");
    let mut code: Vec<u8> = Vec::new();
    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with(';') {
            continue;
        }
        //let line = match line.split_once(';'){
        //    Some((code,_))=>code.trim(),
        //     None=>line,
        // };
        //handling comment lines;
        let op = parse_line(line);
        code.extend_from_slice(&isa::encode(op));
        println!("{:?}", op);
    }
    let mut output: Vec<u8> = Vec::new();
    output.extend_from_slice(&[0x4D, 0x56, 0x4D, 0x00]);
    output.push(0x01);
    output.extend_from_slice(&(code.len() as u32).to_le_bytes());
    output.extend_from_slice(&code);
    let out_file = file.replace(".tasm", ".tbc");
    fs::write(&out_file, &output).expect("could not write file");
    println!("written to {}", out_file);
    Ok(())
}
fn parse_line(line: &str) -> crate::isa::Op {
    let mut parts = line.split_whitespace();
    let memonics = parts.next().unwrap().to_uppercase();
    match memonics.as_str() {
        "PUSH" => {
            // push "7" ->some("push") some("7" ) --->unwarap "7"-->unwrap we get 7
            let n = parts.next().unwrap().parse().unwrap();
            crate::isa::Op::Push(n)
        }
        "POP" => crate::isa::Op::Pop,
        "DUP" => crate::isa::Op::Dup,
        "SWAP" => crate::isa::Op::Swap,
        "ADD" => crate::isa::Op::Add,
        "SUB" => crate::isa::Op::Sub,
        "MUL" => crate::isa::Op::Mul,
        "DIV" => crate::isa::Op::Div,
        "MOD" => crate::isa::Op::Mod,
        "NEG" => crate::isa::Op::Neg,
        "LOAD" => {
            let s: u8 = parts.next().unwrap().parse().unwrap();
            crate::isa::Op::Load(s)
        }
        "STORE" => {
            let s: u8 = parts.next().unwrap().parse().unwrap();
            crate::isa::Op::Store(s)
        }
        "PRINT" => crate::isa::Op::Print,
        "HALT" => crate::isa::Op::Halt,
        "EQ" => crate::isa::Op::EQ,
        "LT" => crate::isa::Op::LT,
        "GT" => crate::isa::Op::GT,
        other => panic!("unknown mnemonic: {}", other),
    }
}
