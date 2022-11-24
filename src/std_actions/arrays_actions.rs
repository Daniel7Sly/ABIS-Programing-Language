use crate::{Procedure, Value, TYPE_NUMB, TYPE_VAR, TYPE_VAR_ARRAY, TYPE_VAR_NUMB};

pub(super) const ACTION_ARI: &str = "ari";
pub(super) const ACTION_ARI_ARGS: &[&str] = &[TYPE_VAR_ARRAY, TYPE_NUMB, TYPE_VAR];
pub(super) fn ari(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == ACTION_ARI_ARGS.len());

    let param1 = current_proc.get_value(&parameters[0]);
    let param2 = current_proc.get_value(&parameters[1]);
    let param3 = current_proc.get_variable_value_mutref(&parameters[2]);

    assert!(param1.is_array());
    assert!(param2.is_numb());
    assert!(param3.typee() == param1.typee());

    let i = param2.get_numb_value() as usize;

    *param3 = match param1 {
        Value::Array(_, vec) => vec[i].clone(),
        _ => unreachable!(),
    }
}

pub(super) const ACTION_ARL: &str = "ari";
pub(super) const ACTION_ARL_ARGS: &[&str] = &[TYPE_VAR_NUMB, TYPE_VAR_ARRAY];
pub(super) fn arl(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == ACTION_ARL_ARGS.len());

    let param2 = current_proc.get_value(&parameters[1]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.is_numb());
    assert!(param2.is_array());

    *param1 = match param2 {
        Value::Array(_, vec) => Value::Numb(vec.len() as f64),
        _ => unreachable!(),
    }
}
