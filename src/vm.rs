use std::{convert::TryInto};
use crate::isa;

pub fn run(file: &str, trace: bool) -> Result<(), String> {
    let bytes = std::fs::read(file)
        .map_err(|e| format!("could not read file: {}", e))?;
    if &bytes[0..4] != &[0x4D, 0x56, 0x4D, 0x00] {
        return Err("Invalid tbc file".to_string());
    }
    if bytes[4] != 0x01 {
        return Err("Wrong version".to_string());
    }
    let code_len = u32::from_le_bytes(
        bytes[5..9]
            .try_into()
            .map_err(|_| "Invalid code length".to_string())?
    ) as usize;
    let code = &bytes[9..9 + code_len];
    println!("File valid! code Length: {} bytes", code_len);
    let mut stack: Vec<i64> = Vec::new();
    let mut globals = [0i64; 256];
    let mut ip = 0;
    loop {
        let (op, new_ip) = isa::decode(code, ip)?;
        if trace {
            println!("ip=0x{:04X} {:?} stack:{:?}", ip, op, stack);
        }
        ip = new_ip;
        match op {
            isa::Op::Push(n) => {
                if stack.len()>=1024{
                    return Err(format!("trap at ip=0x{:04X}:stack overflow (stack full)",ip));
                }
                stack.push(n);
            }
            isa::Op::Pop => {
                //tack.pop();
                match stack.pop() {
                    Some(_) => {},
                    None => return Err(format!("trap at ip=0x{:04X}: stack underflow", ip)),
                }
            }
            isa::Op::Dup => {
                let top = *stack.last().unwrap();
                stack.push(top);
            }
            isa::Op::Swap => {
                let a = safe_pop(&mut stack,ip)?;
                let b = safe_pop(&mut stack,ip)?;
                stack.push(a);
                stack.push(b);
            }
            isa::Op::Add => {
                let b = safe_pop(&mut stack,ip)?;
                let a = safe_pop(&mut stack,ip)?;
                stack.push(a + b);
            }
            isa::Op::Sub => {
                let b = safe_pop(&mut stack,ip)?;
                let a = safe_pop(&mut stack,ip)?;
                stack.push(a - b);
            }
            isa::Op::Mul => {
                let b = safe_pop(&mut stack,ip)?;
                let a = safe_pop(&mut stack,ip)?;
                stack.push(a * b);
            }
            isa::Op::Div => {
                let b = safe_pop(&mut stack,ip)?;
                let a = safe_pop(&mut stack,ip)?;
                if b==0{
                    return Err(format!("trap at ip=0x{:04X}: division by zero", ip));
                }
                stack.push(a / b);
            }
            isa::Op::Mod => {
                let b = safe_pop(&mut stack,ip)?;
                let a = safe_pop(&mut stack,ip)?;
                if b == 0 {
                    return Err(format!("trap at ip=0x{:04X}: modulo by zero", ip));
                }
                stack.push(a % b);
            }
            isa::Op::Neg => {
                let a = safe_pop(&mut stack,ip)?;
                stack.push(-a);
            }
            isa::Op::Load(s) => {
                stack.push(globals[s as usize]);
            }
            isa::Op::Store(s) => {
                let a = safe_pop(&mut stack,ip)?;
                globals[s as usize] = a;
            }
            isa::Op::Print => {
                let ans = safe_pop(&mut stack,ip)?;
                println!("{}", ans);
            }
            isa::Op::Halt => break,
        }
    }
    Ok(())
}
 fn safe_pop(stack:&mut Vec<i64>,ip:usize) ->Result<i64,String>{
        match stack.pop(){
            Some(v) =>Ok(v),
            None => return Err(format!("trap at ip=0x{:04X}:stack underflow",ip)),
        }
}