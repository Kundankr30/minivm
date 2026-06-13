pub mod myargs;
mod dis;
mod asm;
mod run;
mod isa;
mod vm;

fn main() -> Result<(), String> {
    myargs::process_args()?;
    Ok(())
}