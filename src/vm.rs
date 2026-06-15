use std::convert::TryInto;
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
            isa::Op::Push(n) => stack.push(n),
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
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a);
                stack.push(b);
            }
            isa::Op::Add => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a + b);
            }
            isa::Op::Sub => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a - b);
            }
            isa::Op::Mul => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a * b);
            }
            isa::Op::Div => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                if b==0{
                    return Err(format!("trap at ip=0x{:04X}: division by zero", ip));
                }
                stack.push(a / b);
            }
            isa::Op::Mod => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                if b == 0 {
                    return Err(format!("trap at ip=0x{:04X}: modulo by zero", ip));
                }
                stack.push(a % b);
            }
            isa::Op::Neg => {
                let a = stack.pop().unwrap();
                stack.push(-a);
            }
            isa::Op::Load(s) => {
                stack.push(globals[s as usize]);
            }
            isa::Op::Store(s) => {
                let a = stack.pop().unwrap();
                globals[s as usize] = a;
            }
            isa::Op::Print => {
                let ans = stack.pop().unwrap();
                println!("{}", ans);
            }
            isa::Op::Halt => break,
        }
    }
    Ok(())
}