use std::collections::HashMap;

use crate::{Procedure, TYPE_BOOL};

//STANDART ACTIONS DEFINITION
const ACTIONCOUNT: u8 = 8;
///Creates
fn var(current_proc: &mut Procedure) {
    let parameters: &Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let name = parameters[1].clone();
    let typee = parameters[0].clone();

    current_proc.add_new_variable(name, typee);
}

fn giv(current_proc: &mut Procedure) {
    let parameters: &Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let param1 = current_proc.get_variable_value_mutref(&parameters[0], 0);
    let param2 = current_proc.get_value(&parameters[1], 1);

    assert!(param1.typee == param2.typee);

    *param1 = param2;
}

fn exe(current_proc: &mut Procedure) {
    let parameters: &Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 1);
    assert!(parameters[1].starts_with("@"));

    //this will simply execute the proc and ignore the returned value if it returned any.
    current_proc.get_value(&parameters[0], 0);
}

fn rtn(current_proc: &mut Procedure) {
    let parameters: &Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 1);

    let output_value = current_proc.get_value(&parameters[0], 0);

    assert!(current_proc.output_type.is_some());
    assert!(output_value.typee == current_proc.output_type.as_deref().unwrap());

    current_proc.output_value = Some(output_value);
}

fn jmp(current_proc: &mut Procedure) {
    let parameters: &Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 1);

    let param = &parameters[0];

    assert!(current_proc.flag_map.0.contains_key(param));

    current_proc.next_action_index = current_proc.flag_map.0[param];
}

fn iff(current_proc: &mut Procedure) {
    let parameters: &Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let param1 = current_proc.get_value(&parameters[0], 0);

    let param2 = &parameters[1];

    assert!(param1.typee == TYPE_BOOL);

    assert!(current_proc.flag_map.0.contains_key(param2));

    if param1.value.get_normal_bool_value() == true {
        current_proc.next_action_index = current_proc.flag_map.0[param2];
    }
}

fn ifn(current_proc: &mut Procedure) {
    let parameters: &Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let param1 = current_proc.get_value(&parameters[0], 0);

    let param2 = &parameters[1];

    assert!(param1.typee == TYPE_BOOL);

    assert!(current_proc.flag_map.contains_key(param2));

    if param1.value.get_normal_bool_value() == true {
        current_proc.next_action_index = current_proc.flag_map[param2];
    }
}

/*
Actions of previous version missing:
    PARSE
    SETARR
    ARRLENGHT
    JOINTEXT
    SPLITTEXT

Actions that could be added:
    LOG : text
    STP : (stops the interpreter)
    ASSERT : bool (same as STP but only stops if its false)
*/

pub(crate) fn hashmap_with_default_actions() -> HashMap<String, fn(&mut Procedure)> {
    let mut map = HashMap::<String, fn(&mut Procedure)>::new();

    map.insert("var".to_string(), var);
    map.insert("giv".to_string(), giv);
    map.insert("exe".to_string(), exe);
    map.insert("rtn".to_string(), rtn);
    map.insert("jmp".to_string(), jmp);
    map.insert("iff".to_string(), iff);
    map.insert("ifn".to_string(), ifn);

    assert!(map.len() == ACTIONCOUNT as usize);

    map
}
