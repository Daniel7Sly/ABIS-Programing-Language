use std::{fmt::Display, fs};

use abis::{AbisError, ActionDef, Interpreter, Program, TYPE_TEXT};

fn main() -> Result<(), AbisError> {
    let mut interpreter = Interpreter::new();
    //TODO: Interpreter should have a single method to add actions, load the script, and run at the same time.

    interpreter.add_action(ACTION_PRINTLN, ActionDef::new(println, ACTION_PRINTLN_ARGS));

    let script = fs::read_to_string("experiment.abis").expect("Unable to read file!");

    interpreter
        .load_script(script)
        .expect("Error Loading Script!");

    interpreter.run_scripts()?;
    Ok(())
}

const ACTION_PRINTLN: &str = "println";
const ACTION_PRINTLN_ARGS: &[&str] = &[TYPE_TEXT];
fn println(current_proc: &mut Program) {
    let parameters = current_proc.get_parameters_values();

    assert!(parameters.len() == 1);

    let text = parameters[0].get_pure_value().to_string();
    println!("{}", text);
}
