use crate::{Program, Value, TYPE_NUMB, TYPE_VAR, TYPE_VAR_ARRAY, TYPE_VAR_NUMB};

pub(super) const ACTION_ARI: &str = "ari";
pub(super) const ACTION_ARI_ARGS: &[&str] = &[TYPE_VAR, TYPE_VAR_ARRAY, TYPE_NUMB];
pub(super) fn ari(current_proc: &mut Program) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == ACTION_ARI_ARGS.len());

    let param2 = current_proc.get_value(&parameters[1]);
    let param3 = current_proc.get_value(&parameters[2]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param2.is_array());
    assert!(param3.is_numb());
    assert!(param1.typee() == param2.typee());

    let i = param1.get_numb_value() as usize;

    *param1 = match param2 {
        Value::Array(_, vec) => vec[i].clone(),
        _ => unreachable!(),
    }
}

pub(super) const ACTION_ARL: &str = "arl";
pub(super) const ACTION_ARL_ARGS: &[&str] = &[TYPE_VAR_NUMB, TYPE_VAR_ARRAY];
pub(super) fn arl(current_proc: &mut Program) {
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

//TODO: PSH -> pushes new elements to the array
