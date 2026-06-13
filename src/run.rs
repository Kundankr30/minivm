pub fn run(file:&str,trace:bool)->Result<(),String>{
    crate::vm::run(file,trace)?;
    Ok(())
}