use std::env::args;

pub fn process_args() -> Result<(), String> {
    let myargs: Vec<String> = args().collect();
    if myargs.len() < 3 {
        return Err("Usage: <asm|run|dis> <file>".to_string());
    }
    let file = myargs[2].as_str();

    match myargs[1].as_str() {
        "asm" => {
            println!("Assembler Mode");
            super::asm::assemble(file)?;
        }
        "run" => {
            let trace = myargs.iter().any(|a| a == "--trace");
            super::run::run(file, trace)?;
        }
        "dis" => {
            super::dis::disassemble(file)?;
        }
        other => {
            return Err(format!("Unknown command: {}", other));
        }
    }
    Ok(())
}