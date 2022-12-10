use crate::{Program, Value, TYPE_NUMB, TYPE_VAR_NUMB};

// Arithmetic

pub(super) const ACTION_ADD: &str = "add";
pub(super) const ACTION_ADD_ARGS: &[&str] = &[TYPE_VAR_NUMB, TYPE_NUMB];
pub(super) fn add(current_proc: &mut Program) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let param2 = current_proc.get_value(&parameters[1]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee() == TYPE_NUMB && param1.is_numb());
    assert!(param2.typee() == TYPE_NUMB && param2.is_numb());

    let numb1 = param1.get_numb_value();
    let numb2 = param2.get_numb_value();

    let rslt = numb1 + numb2;

    *param1 = Value::Numb(rslt);
}

pub(super) const ACTION_SUB: &str = "sub";
pub(super) const ACTION_SUB_ARGS: &[&str] = &[TYPE_VAR_NUMB, TYPE_NUMB];
pub(super) fn sub(current_proc: &mut Program) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let param2 = current_proc.get_value(&parameters[1]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee() == TYPE_NUMB && param1.is_numb());
    assert!(param2.typee() == TYPE_NUMB && param2.is_numb());

    let numb1 = param1.get_numb_value();
    let numb2 = param2.get_numb_value();

    let rslt = numb1 - numb2;

    *param1 = Value::Numb(rslt);
}

pub(super) const ACTION_MUL: &str = "mul";
pub(super) const ACTION_MUL_ARGS: &[&str] = &[TYPE_VAR_NUMB, TYPE_NUMB];
pub(super) fn mul(current_proc: &mut Program) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let param2 = current_proc.get_value(&parameters[1]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee() == TYPE_NUMB && param1.is_numb());
    assert!(param2.typee() == TYPE_NUMB && param2.is_numb());

    let numb1 = param1.get_numb_value();
    let numb2 = param2.get_numb_value();

    let rslt = numb1 * numb2;

    *param1 = Value::Numb(rslt);
}

pub(super) const ACTION_DIV: &str = "div";
pub(super) const ACTION_DIV_ARGS: &[&str] = &[TYPE_VAR_NUMB, TYPE_NUMB];
pub(super) fn div(current_proc: &mut Program) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let param2 = current_proc.get_value(&parameters[1]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee() == TYPE_NUMB && param1.is_numb());
    assert!(param2.typee() == TYPE_NUMB && param2.is_numb());

    let numb1 = param1.get_numb_value();
    let numb2 = param2.get_numb_value();

    let rslt = numb1 / numb2;

    *param1 = Value::Numb(rslt);
}

pub(super) const ACTION_MOD: &str = "mod";
pub(super) const ACTION_MOD_ARGS: &[&str] = &[TYPE_VAR_NUMB, TYPE_NUMB];
pub(super) fn mod_(current_proc: &mut Program) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let param2 = current_proc.get_value(&parameters[1]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee() == TYPE_NUMB && param1.is_numb());
    assert!(param2.typee() == TYPE_NUMB && param2.is_numb());

    let numb1 = param1.get_numb_value();
    let numb2 = param2.get_numb_value();

    let rslt = numb1 % numb2;

    *param1 = Value::Numb(rslt);
}
