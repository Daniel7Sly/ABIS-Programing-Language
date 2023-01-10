use crate::{get_default_value_of_type, Program, Value, TYPE_NEUTRAL, TYPE_NUMB, TYPE_VAR_ARRAY};

pub(super) const ACTION_ARI: &str = "ari";
pub(super) const ACTION_ARI_ARGS: &[&str] = &[TYPE_VAR_ARRAY, TYPE_NUMB];
/// Pushes to the stack the value of the array at the given index.
/// Returns the Default value of the type of the array if the array is empty.
pub(super) fn ari(current_proc: &mut Program) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == ACTION_ARI_ARGS.len());

    let param1 = current_proc.get_value(&parameters[0]);
    let param2 = current_proc.get_value(&parameters[1]);

    assert!(param1.is_array());
    assert!(param2.is_numb());

    let i = param2.get_numb_value() as usize;

    let value = match param1 {
        Value::Array(typee, vec) => {
            if vec.len() > 0 {
                vec[i % vec.len()].clone()
            } else {
                get_default_value_of_type(&typee)
            }
        }
        _ => unreachable!(),
    };
    current_proc.value_stack.push(value);
}

pub(super) const ACTION_ARL: &str = "arl";
pub(super) const ACTION_ARL_ARGS: &[&str] = &[TYPE_VAR_ARRAY];
/// Pushes the Length of the array to the stack.
pub(super) fn arl(current_proc: &mut Program) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == ACTION_ARL_ARGS.len());

    let param1 = current_proc.get_value(&parameters[0]);

    assert!(param1.is_array());

    let value = match param1 {
        Value::Array(_, vec) => Value::Numb(vec.len() as f64),
        _ => unreachable!(),
    };
    current_proc.value_stack.push(value);
}

pub(super) const ACTION_ARP: &str = "arp";
pub(super) const ACTION_ARP_ARGS: &[&str] = &[TYPE_VAR_ARRAY, TYPE_NEUTRAL];
/// Pushes an value to the array.
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
