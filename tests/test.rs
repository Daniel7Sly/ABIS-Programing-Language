use std::fs;

use abis::{AbisError, Interpreter, Value};

fn test_script(test_file_name: &str) -> Result<Vec<Value>, AbisError> {
    let script = read_script(test_file_name);
    run_script(script)
}

fn read_script(test_file_name: &str) -> String {
    let script = fs::read_to_string(format!("abis_test_scripts\\{}", test_file_name)).expect(
        format!(
            "Unable to read test script abis_test_scripts\\{}",
            test_file_name
        )
        .as_str(),
    );
    script
}
fn run_script(script: String) -> Result<Vec<Value>, AbisError> {
    let mut interpreter = Interpreter::new();
    interpreter
        .load_script(script)
        .expect("Error Loading Script!");
    interpreter.run_scripts(true)
}

#[test]
/// Tests if can run hello world
/// If this test works it means the the lexer and the parser
/// should be working proprely and the interpreter could execute actions.
fn hello_world() {
    let test_results = test_script("test_hello_world.abis");
    assert!(test_results.is_ok());

    let test_results = test_results.unwrap();

    assert_eq!(test_results[0], Value::Text("Hello World!".to_string()));
}

#[test]
fn missing_main_flag() {
    assert_eq!(
        test_script("missing_main_flag.abis").unwrap_err(),
        AbisError::MainFlagNotFound
    );
}

#[test]
fn arithmetic_actions_test() {
    let test_results = test_script("arithmetic_actions_test.abis");
    assert!(test_results.is_ok());

    let test_results = test_results.unwrap();

    assert_eq!(test_results[0], Value::Numb(10f64));
    assert_eq!(test_results[1], Value::Numb(8f64));
    assert_eq!(test_results[2], Value::Numb(6f64));
    assert_eq!(test_results[3], Value::Numb(5f64));
    assert_eq!(test_results[4], Value::Numb(2f64));
}

fn comparation_actions_test() {
    let test_results = test_script("comparation_actions_test.abis");
    assert!(test_results.is_ok());

    let test_results = test_results.unwrap();

    assert_eq!(test_results[0], Value::Text("Hello World!".to_string()));
}
