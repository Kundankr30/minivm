pub mod myargs;
mod dis;
mod asm;
mod run;
mod isa;
mod vm;

fn main() {
    myargs::process_args();
}
