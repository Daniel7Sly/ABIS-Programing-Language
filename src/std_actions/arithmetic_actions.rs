use crate::{Procedure, Value, TYPE_NUMB};

// Arithmetic

pub(crate) fn add(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let param2 = current_proc.get_value(&parameters[1]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee() == TYPE_NUMB && param1.is_normal_numb());
    assert!(param2.typee() == TYPE_NUMB && param2.is_normal_numb());

    let numb1 = param1.get_normal_numb_value();
    let numb2 = param2.get_normal_numb_value();

    let rslt = numb1 + numb2;

    *param1 = Value::Numb(rslt);
}

pub(crate) fn sub(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let param2 = current_proc.get_value(&parameters[1]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee() == TYPE_NUMB && param1.is_normal_numb());
    assert!(param2.typee() == TYPE_NUMB && param2.is_normal_numb());

    let numb1 = param1.get_normal_numb_value();
    let numb2 = param2.get_normal_numb_value();

    let rslt = numb1 - numb2;

    *param1 = Value::Numb(rslt);
}

pub(crate) fn mul(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let param2 = current_proc.get_value(&parameters[1]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee() == TYPE_NUMB && param1.is_normal_numb());
    assert!(param2.typee() == TYPE_NUMB && param2.is_normal_numb());

    let numb1 = param1.get_normal_numb_value();
    let numb2 = param2.get_normal_numb_value();

    let rslt = numb1 * numb2;

    *param1 = Value::Numb(rslt);
}

pub(crate) fn div(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let param2 = current_proc.get_value(&parameters[1]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee() == TYPE_NUMB && param1.is_normal_numb());
    assert!(param2.typee() == TYPE_NUMB && param2.is_normal_numb());

    let numb1 = param1.get_normal_numb_value();
    let numb2 = param2.get_normal_numb_value();

    let rslt = numb1 / numb2;

    *param1 = Value::Numb(rslt);
}

pub(crate) fn mod_(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let param2 = current_proc.get_value(&parameters[1]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee() == TYPE_NUMB && param1.is_normal_numb());
    assert!(param2.typee() == TYPE_NUMB && param2.is_normal_numb());

    let numb1 = param1.get_normal_numb_value();
    let numb2 = param2.get_normal_numb_value();

    let rslt = numb1 % numb2;

    *param1 = Value::Numb(rslt);
}
