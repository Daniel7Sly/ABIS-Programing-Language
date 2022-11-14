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

    let value = &parameters[0].value;
    if value.is_normal_text() {
        let text = parameters[0].value.get_normal_text_value();
        println!("{}", text);
    } else if value.is_normal_numb() {
        let text = parameters[0].value.get_normal_numb_value();
        println!("{}", text);
    } else if value.is_normal_bool() {
        let text = parameters[0].value.get_normal_bool_value();
        println!("{}", text);
    }
}
