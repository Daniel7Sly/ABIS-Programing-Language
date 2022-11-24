use crate::{Procedure, Value, TYPE_BOOL, TYPE_NUMB, TYPE_VAR_BOOL};

//Comparation and logic

pub(super) const ACTION_BGT: &str = "bgt";
pub(super) const ACTION_BGT_ARGS: &[&str] = &[TYPE_VAR_BOOL, TYPE_NUMB, TYPE_NUMB];
pub(super) fn bgt(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 3);

    let param2 = current_proc.get_value(&parameters[1]);
    let param3 = current_proc.get_value(&parameters[2]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee() == TYPE_BOOL && param1.is_bool());
    assert!(param2.typee() == TYPE_NUMB && param2.is_numb());
    assert!(param3.typee() == TYPE_NUMB && param3.is_numb());

    let numb1 = param2.get_numb_value();
    let numb2 = param3.get_numb_value();

    let rslt = numb1 > numb2;

    *param1 = Value::Bool(rslt);
}

pub(super) const ACTION_SMT: &str = "smt";
pub(super) const ACTION_SMT_ARGS: &[&str] = &[TYPE_VAR_BOOL, TYPE_NUMB, TYPE_NUMB];
pub(super) fn smt(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 3);

    let param2 = current_proc.get_value(&parameters[1]);
    let param3 = current_proc.get_value(&parameters[2]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee() == TYPE_BOOL && param1.is_bool());
    assert!(param2.typee() == TYPE_NUMB && param2.is_numb());
    assert!(param3.typee() == TYPE_NUMB && param3.is_numb());

    let numb1 = param2.get_numb_value();
    let numb2 = param3.get_numb_value();

    let rslt = numb1 < numb2;

    *param1 = Value::Bool(rslt);
}

pub(super) const ACTION_EQL: &str = "eql";
pub(super) const ACTION_EQL_ARGS: &[&str] = &[TYPE_VAR_BOOL, TYPE_NUMB, TYPE_NUMB];
pub(super) fn eql(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 3);

    let param2 = current_proc.get_value(&parameters[1]);
    let param3 = current_proc.get_value(&parameters[2]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee() == TYPE_BOOL && param1.is_bool());
    assert!(param2.typee() == TYPE_NUMB && param2.is_numb());
    assert!(param3.typee() == TYPE_NUMB && param3.is_numb());

    let numb1 = param2.get_numb_value();
    let numb2 = param3.get_numb_value();

    let rslt = numb1 == numb2;

    *param1 = Value::Bool(rslt);
}

pub(super) const ACTION_DIF: &str = "dif";
pub(super) const ACTION_DIF_ARGS: &[&str] = &[TYPE_VAR_BOOL, TYPE_NUMB, TYPE_NUMB];
pub(super) fn dif(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 3);

    let param2 = current_proc.get_value(&parameters[1]);
    let param3 = current_proc.get_value(&parameters[2]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee() == TYPE_BOOL && param1.is_bool());
    assert!(param2.typee() == TYPE_NUMB && param2.is_numb());
    assert!(param3.typee() == TYPE_NUMB && param3.is_numb());

    let numb1 = param2.get_numb_value();
    let numb2 = param3.get_numb_value();

    let rslt = numb1 != numb2;

    *param1 = Value::Bool(rslt);
}

pub(super) const ACTION_AND: &str = "and";
pub(super) const ACTION_AND_ARGS: &[&str] = &[TYPE_VAR_BOOL, TYPE_BOOL, TYPE_BOOL];
pub(super) fn and(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 3);

    let param2 = current_proc.get_value(&parameters[1]);
    let param3 = current_proc.get_value(&parameters[2]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee() == TYPE_BOOL && param1.is_bool());
    assert!(param2.typee() == TYPE_BOOL && param2.is_bool());
    assert!(param3.typee() == TYPE_BOOL && param3.is_bool());

    let numb1 = param2.get_bool_value();
    let numb2 = param3.get_bool_value();

    let rslt = numb1 && numb2;

    *param1 = Value::Bool(rslt);
}

pub(super) const ACTION_ORR: &str = "orr";
pub(super) const ACTION_ORR_ARGS: &[&str] = &[TYPE_VAR_BOOL, TYPE_BOOL, TYPE_BOOL];
pub(super) fn orr(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 3);

    let param2 = current_proc.get_value(&parameters[1]);
    let param3 = current_proc.get_value(&parameters[2]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee() == TYPE_BOOL && param1.is_bool());
    assert!(param2.typee() == TYPE_BOOL && param2.is_bool());
    assert!(param3.typee() == TYPE_BOOL && param3.is_bool());

    let numb1 = param2.get_bool_value();
    let numb2 = param3.get_bool_value();

    let rslt = numb1 || numb2;

    *param1 = Value::Bool(rslt);
}
