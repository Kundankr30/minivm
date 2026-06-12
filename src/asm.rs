use std::fs;
pub fn assemble(file:&str){
    let contents = fs::read_to_string(file).expect("Count not real file");
    for line in contents.lines(){
        let line = line.trim();
        if line.is_empty(){continue;}
        if line.starts_with(';'){continue;}
        //let line = match line.split_once(';'){
        //    Some((code,_))=>code.trim(),
        //     None=>line,
        // };
        //handling comment lines;
        let op = parse_line(line);
        println!("{:?}",op);
    }
}
fn parse_line(line:&str) ->crate::isa::Op{
    let mut parts = line.split_whitespace();
    let memonics = parts.next().unwrap().to_uppercase();
    match memonics.as_str(){
        "PUSH" =>{
            let n = parts.next().unwrap().parse().unwrap();
            crate::isa::Op::Push(n)
        }
                "POP"   => crate::isa::Op::Pop,
        "DUP"   => crate::isa::Op::Dup,
        "SWAP"  => crate::isa::Op::Swap,
        "ADD"   => crate::isa::Op::Add,
        "SUB"   => crate::isa::Op::Sub,
        "MUL"   => crate::isa::Op::Mul,
        "DIV"   => crate::isa::Op::Div,
        "MOD"   => crate::isa::Op::Mod,
        "NEG"   => crate::isa::Op::Neg,
        "LOAD"  => {
            let s: u8 = parts.next().unwrap().parse().unwrap();
            crate::isa::Op::Load(s)
        }
        "STORE" => {
            let s: u8 = parts.next().unwrap().parse().unwrap();
            crate::isa::Op::Store(s)
        }
        "PRINT" => crate::isa::Op::Print,
        "HALT"  => crate::isa::Op::Halt,
        other   => panic!("unknown mnemonic: {}", other)
    }
}   