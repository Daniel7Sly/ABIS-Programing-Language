use std::fs;

use abis::{AbisError, ActionDef, Interpreter, Program, Value, TYPE_NEUTRAL};

#[test]
fn main_test() -> Result<(), AbisError> {
    test_script("test.abis")?;
    Ok(())
}

fn test_script(test_file_name: &str) -> Result<(), AbisError> {
    let script = read_script(test_file_name);
    run_script(script)?;
    Ok(())
}

fn read_script(test_file_name: &str) -> String {
    let script = fs::read_to_string(format!("abis_test_scripts\\{}", test_file_name))
        .expect("Unable to read file!");
    script
}
fn run_script(script: String) -> Result<(), AbisError> {
    let mut interpreter = Interpreter::new();
    interpreter.add_action(ACTION_TEST, ActionDef::new(test_action, ACTION_TEST_ARGS));
    interpreter
        .load_script(script)
        .expect("Error Loading Script!");
    interpreter.run_scripts()?;
    Ok(())
}

#[test]
/// Tests if can run hello world
/// If this test works it means the the lexer and the parser
/// should be working proprely
fn hello_world() {
    assert!(test_script("test_hello_world.abis").is_ok());
    assert_eq!(rtva(0), Value::Text("Hello World!".to_string()))
}
#[test]
fn missing_main_flag() {
    assert_eq!(
        test_script("missing_main_flag.abis").unwrap_err(),
        AbisError::MainFlagNotFound
    );
}

// This test action is used to test if adding new actions to the interpreter
// is working proprely and used in abis code to push values to TEST_VALUES
// then the values can be check in the main_test().
static mut TEST_VALUES: Vec<Value> = Vec::new();

const ACTION_TEST: &str = "test";
const ACTION_TEST_ARGS: &[&str] = &[TYPE_NEUTRAL];
fn test_action(current_proc: &mut Program) {
    let parameters = current_proc.get_parameters_values();

    assert!(parameters.len() == ACTION_TEST_ARGS.len());

    let value = parameters[0].get_pure_value();
    unsafe { TEST_VALUES.push(value) };
}
/// Stands for "Read Test_Values At"
fn rtva(i: usize) -> Value {
    unsafe { TEST_VALUES[i].clone() }
}
