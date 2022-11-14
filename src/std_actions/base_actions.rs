use crate::{Procedure, TYPE_BOOL};

///Creates a new variable to the procedure
pub(crate) fn var(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let name = parameters[1].clone();
    let typee = parameters[0].clone();

    current_proc.add_new_variable(name, typee);
}

pub(crate) fn giv(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let param2 = current_proc.get_value(&parameters[1]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(
        param1.typee == param2.typee,
        "{} {}",
        param1.typee,
        param2.typee
    );

    *param1 = param2;
}

pub(crate) fn exe(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 1);
    assert!(parameters[1].starts_with("@"));

    //this will simply execute the proc and ignore the returned value if it returned any.
    current_proc.get_value(&parameters[0]);
}

pub(crate) fn rtn(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 1);

    let output_value = current_proc.get_value(&parameters[0]);

    assert!(current_proc.output_type.is_some());
    assert!(output_value.typee == current_proc.output_type.as_deref().unwrap());

    current_proc.output_value = Some(output_value);
}

pub(crate) fn jmp(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 1);

    let param = &parameters[0];

    println!("{:?}", current_proc.flag_map);

    assert!(current_proc.flag_map.0.contains_key(param), "{}", param);

    current_proc.next_action_index = current_proc.flag_map.0[param];
}

pub(crate) fn iff(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let param1 = current_proc.get_value(&parameters[0]);

    let param2 = &parameters[1];

    assert!(param1.typee == TYPE_BOOL);

    assert!(current_proc.flag_map.0.contains_key(param2));

    if param1.value.get_normal_bool_value() == true {
        current_proc.next_action_index = current_proc.flag_map.0[param2];
    }
}

pub(crate) fn ifn(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let param1 = current_proc.get_value(&parameters[0]);

    let param2 = &parameters[1];

    assert!(param1.typee == TYPE_BOOL);

    assert!(current_proc.flag_map.contains_key(param2));

    if param1.value.get_normal_bool_value() == true {
        current_proc.next_action_index = current_proc.flag_map[param2];
    }
}