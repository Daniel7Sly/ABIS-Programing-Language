use std::{collections::HashMap, vec};

use crate::{
    ActionDef, Procedure, TYPE_BOOL, TYPE_FLAG, TYPE_NEUTRAL, TYPE_PROC, TYPE_TYPE, TYPE_VAR,
};

//STANDART ACTIONS DEFINITION
const ACTIONS: &[&str] = &["var", "giv", "exe", "rtn", "jmp", "iff", "ifn"];

const ACTION_VAR: &str = ACTIONS[0];
const ACTION_GIV: &str = ACTIONS[1];
const ACTION_EXE: &str = ACTIONS[2];
const ACTION_RTN: &str = ACTIONS[3];
const ACTION_JMP: &str = ACTIONS[4];
const ACTION_IFF: &str = ACTIONS[5];
const ACTION_IFN: &str = ACTIONS[6];

const ACTIONCOUNT: usize = ACTIONS.len();
///Creates a new variable to the procedure
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

pub(crate) fn hashmap_with_default_actions() -> HashMap<String, ActionDef> {
    let mut map = HashMap::<String, ActionDef>::new();

    map.insert(
        ACTION_VAR.to_string(),
        ActionDef {
            method: var,
            parameters_types: vec![TYPE_TYPE, TYPE_NEUTRAL],
        },
    );
    map.insert(
        ACTION_GIV.to_string(),
        ActionDef {
            method: giv,
            parameters_types: vec![TYPE_VAR, TYPE_NEUTRAL],
        },
    );
    map.insert(
        ACTION_EXE.to_string(),
        ActionDef {
            method: exe,
            parameters_types: vec![TYPE_PROC],
        },
    );
    map.insert(
        ACTION_RTN.to_string(),
        ActionDef {
            method: rtn,
            parameters_types: vec![TYPE_NEUTRAL],
        },
    );
    map.insert(
        ACTION_JMP.to_string(),
        ActionDef {
            method: jmp,
            parameters_types: vec![TYPE_FLAG],
        },
    );
    map.insert(
        ACTION_IFF.to_string(),
        ActionDef {
            method: iff,
            parameters_types: vec![TYPE_BOOL, TYPE_FLAG],
        },
    );
    map.insert(
        ACTION_IFN.to_string(),
        ActionDef {
            method: ifn,
            parameters_types: vec![TYPE_BOOL, TYPE_FLAG],
        },
    );

    assert!(
        map.len() == ACTIONCOUNT as usize,
        "If this fails add the new action here!"
    );

    map
}
