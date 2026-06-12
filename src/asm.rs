use std::fs;
pub fn assemble(file:&str){
    let contents = fs::read_to_string(file).expect("Count not real file");
    for line in contents.lines(){
        println!("{}",line);
    }
}