use std::fs;

use abis::{AbisError, Interpreter};

fn main() -> Result<(), AbisError> {
    let mut interpreter = Interpreter::new();
    let script = fs::read_to_string("test.abis").expect("Unable to read file!");

    interpreter.load_script(script)?;
    Ok(())
}
