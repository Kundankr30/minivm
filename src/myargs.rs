use std::env::args;
pub fn process_args(){
    let myargs:Vec<String>= args().collect();
    if myargs.len()<2{
        print!("asm,run,dis expected");
        return;
    }
    let file = myargs[2].as_str();
    match myargs[1].as_str(){
        "asm" => {
            println!("Assembler Mode");
            super::asm::assemble(file);
        }
        "run" =>{
            println!("VM Mode");
            super::run::run(file);
        }
        "dis" =>{
            println!("Disassembler Mode");
            super::dis::disassemble(file);
        }
        other=>{
            println!("Unknown command:{}",other)
        }

    }
 }
