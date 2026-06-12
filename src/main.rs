pub mod myargs;
mod dis;
mod asm;
mod run;
mod isa;

fn main() {
    myargs::process_args();
}
