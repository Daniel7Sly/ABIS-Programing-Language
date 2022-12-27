use crate::{Program, Value, TYPE_NUMB};

// Arithmetic

pub(super) const ACTION_ADD: &str = "add";
pub(super) const ACTION_ADD_ARGS: &[&str] = &[TYPE_NUMB, TYPE_NUMB];
/// Adds the two numbers, and pushes the result to the value stack.
pub(super) fn add(program: &mut Program) {
    let parameters: Vec<String> = program.get_raw_parameters();

    assert!(parameters.len() == ACTION_ADD_ARGS.len());

    let param1 = program.get_value(&parameters[0]);
    let param2 = program.get_value(&parameters[1]);

    assert!(param1.typee() == TYPE_NUMB && param1.is_numb());
    assert!(param2.typee() == TYPE_NUMB && param2.is_numb());

    let numb1 = param1.get_numb_value();
    let numb2 = param2.get_numb_value();

    let rslt = numb1 + numb2;

    program.value_stack.push(Value::Numb(rslt));
}

pub(super) const ACTION_SUB: &str = "sub";
pub(super) const ACTION_SUB_ARGS: &[&str] = &[TYPE_NUMB, TYPE_NUMB];
/// Subtracts the two numbers, and pushes the result to the value stack.
pub(super) fn sub(program: &mut Program) {
    let parameters: Vec<String> = program.get_raw_parameters();

    assert!(parameters.len() == ACTION_SUB_ARGS.len());

    let param1 = program.get_value(&parameters[0]);
    let param2 = program.get_value(&parameters[1]);

    assert!(param1.typee() == TYPE_NUMB && param1.is_numb());
    assert!(param2.typee() == TYPE_NUMB && param2.is_numb());

    let numb1 = param1.get_numb_value();
    let numb2 = param2.get_numb_value();

    let rslt = numb1 - numb2;
    current_proc.value_stack.push(Value::Numb(rslt));

    program.value_stack.push(Value::Numb(rslt));
}

pub(super) const ACTION_MUL: &str = "mul";
pub(super) const ACTION_MUL_ARGS: &[&str] = &[TYPE_NUMB, TYPE_NUMB];
/// Multiplies the two numbers, and pushes the result to the value stack.
pub(super) fn mul(program: &mut Program) {
    let parameters: Vec<String> = program.get_raw_parameters();

    assert!(parameters.len() == ACTION_MUL_ARGS.len());

    let param1 = program.get_value(&parameters[0]);
    let param2 = program.get_value(&parameters[1]);

    assert!(param1.typee() == TYPE_NUMB && param1.is_numb());
    assert!(param2.typee() == TYPE_NUMB && param2.is_numb());

    let numb1 = param1.get_numb_value();
    let numb2 = param2.get_numb_value();

    let rslt = numb1 * numb2;
    current_proc.value_stack.push(Value::Numb(rslt));

    program.value_stack.push(Value::Numb(rslt));
}

pub(super) const ACTION_DIV: &str = "div";
pub(super) const ACTION_DIV_ARGS: &[&str] = &[TYPE_NUMB, TYPE_NUMB];
/// Divides the two numbers, and pushes the result to the value stack.
pub(super) fn div(program: &mut Program) {
    let parameters: Vec<String> = program.get_raw_parameters();

    assert!(parameters.len() == ACTION_DIV_ARGS.len());

    let param2 = program.get_value(&parameters[1]);
    let param1 = program.get_value(&parameters[0]);

    assert!(param1.typee() == TYPE_NUMB && param1.is_numb());
    assert!(param2.typee() == TYPE_NUMB && param2.is_numb());

    let numb1 = param1.get_numb_value();
    let numb2 = param2.get_numb_value();

    let rslt = numb1 / numb2;
    current_proc.value_stack.push(Value::Numb(rslt));

    program.value_stack.push(Value::Numb(rslt));
}

pub(super) const ACTION_MOD: &str = "mod";
pub(super) const ACTION_MOD_ARGS: &[&str] = &[TYPE_NUMB, TYPE_NUMB];
/// Divides the two numbers, and pushes the remainder to the value stack.
pub(super) fn mod_(program: &mut Program) {
    let parameters: Vec<String> = program.get_raw_parameters();

    assert!(parameters.len() == ACTION_MOD_ARGS.len());

    let param2 = program.get_value(&parameters[1]);
    let param1 = program.get_value(&parameters[0]);

    assert!(param1.typee() == TYPE_NUMB && param1.is_numb());
    assert!(param2.typee() == TYPE_NUMB && param2.is_numb());

    let numb1 = param1.get_numb_value();
    let numb2 = param2.get_numb_value();

    let rslt = numb1 % numb2;

    program.value_stack.push(Value::Numb(rslt));
}
