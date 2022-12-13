use crate::{
    get_default_value_of_type, Program, Value, TYPE_NEUTRAL, TYPE_NUMB, TYPE_VAR, TYPE_VAR_ARRAY,
    TYPE_VAR_NUMB,
};

pub(super) const ACTION_ARI: &str = "ari";
pub(super) const ACTION_ARI_ARGS: &[&str] = &[TYPE_VAR, TYPE_VAR_ARRAY, TYPE_NUMB];
/// Get the value of the array at the given index.
/// Returns the Default value of the type of the array if the array is empty.
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
        Value::Array(typee, vec) => {
            if vec.len() > 0 {
                vec[i % vec.len()].clone()
            } else {
                get_default_value_of_type(&typee)
            }
        }
        _ => unreachable!(),
    }
}

pub(super) const ACTION_ARL: &str = "arl";
pub(super) const ACTION_ARL_ARGS: &[&str] = &[TYPE_VAR_NUMB, TYPE_VAR_ARRAY];
/// Returns the Length of the array.
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

pub(super) const ACTION_ARP: &str = "arp";
pub(super) const ACTION_ARP_ARGS: &[&str] = &[TYPE_VAR_ARRAY, TYPE_NEUTRAL];
/// Returns the Length of the array.
pub(super) fn arp(current_proc: &mut Program) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == ACTION_ARP_ARGS.len());

    let param2 = current_proc.get_value(&parameters[1]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.is_array());
    assert!(param1.typee() == param2.typee());

    match param1 {
        Value::Array(_, vec) => vec.push(param2),
        _ => unreachable!(),
    }
}
