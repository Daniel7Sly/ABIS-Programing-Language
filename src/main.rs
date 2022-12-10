use std::fs;

use abis::{AbisError, ActionDef, Interpreter, Program, TYPE_TEXT};

fn main() -> Result<(), AbisError> {
    let mut interpreter = Interpreter::new();

    //TODO: Interpreter should have a single method to add actions, load the script, and run at the same time.

    interpreter.add_action(ACTION_PRINTLN, ActionDef::new(println, ACTION_PRINTLN_ARGS));

    let script = fs::read_to_string("test.abis").expect("Unable to read file!");

    interpreter.load_script(script)?;

    interpreter.run_scripts()?;
    Ok(())
}

const ACTION_PRINTLN: &str = "println";
const ACTION_PRINTLN_ARGS: &[&str] = &[TYPE_TEXT];
fn println(current_proc: &mut Program) {
    let parameters = current_proc.get_parameters_values();

    assert!(parameters.len() == 1);

    let value = &parameters[0];
    if value.is_text() {
        let text = parameters[0].get_text_value();
        println!("{}", text);
    } else if value.is_numb() {
        let text = parameters[0].get_numb_value();
        println!("{}", text);
    } else if value.is_bool() {
        let text = parameters[0].get_bool_value();
        println!("{}", text);
    }
}
