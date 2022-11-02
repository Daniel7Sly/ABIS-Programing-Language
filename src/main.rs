use std::fs;

use abis::{AbisError, ActionDef, Interpreter, Procedure, TYPE_TEXT};

fn main() -> Result<(), AbisError> {
    let mut interpreter = Interpreter::new();

    //TODO: Interpreter should have a single method to add actions, load the script, and run at the same time.

    interpreter.add_action(ACTION_PRINTLN, ActionDef::new(println, vec![TYPE_TEXT]));

    let script = fs::read_to_string("test.abis").expect("Unable to read file!");

    interpreter.load_script(script)?;

    interpreter.run_scripts()?;
    Ok(())
}

const ACTION_PRINTLN: &str = "println";
fn println(current_proc: &mut Procedure) {
    let parameters = current_proc.get_parameters_values();

    assert!(parameters.len() == 1);

    let text = parameters[0].value.get_normal_text_value();

    println!("{}", text);
}
