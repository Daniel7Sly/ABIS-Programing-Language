use std::{collections::HashMap, vec};

use crate::{
    ActionDef, Procedure, ValueForm, TYPE_BOOL, TYPE_FLAG, TYPE_NEUTRAL, TYPE_NUMB, TYPE_PROC,
    TYPE_TEXT, TYPE_TYPE, TYPE_VAR, TYPE_VAR_BOOL, TYPE_VAR_NUMB, TYPE_VAR_TEXT,
};

//STANDART ACTIONS DEFINITION
const ACTIONS: &[&str] = &[
    "var", "giv", "exe", "rtn", "jmp", "iff", "ifn", "prs", "add", "sub", "mul", "div", "mod",
    "txt",
];

const ACTION_VAR: &str = ACTIONS[0];
const ACTION_GIV: &str = ACTIONS[1];
const ACTION_EXE: &str = ACTIONS[2];
const ACTION_RTN: &str = ACTIONS[3];
const ACTION_JMP: &str = ACTIONS[4];
const ACTION_IFF: &str = ACTIONS[5];
const ACTION_IFN: &str = ACTIONS[6];
const ACTION_PRS: &str = ACTIONS[7];
const ACTION_ADD: &str = ACTIONS[8];
const ACTION_SUB: &str = ACTIONS[9];
const ACTION_MUL: &str = ACTIONS[10];
const ACTION_DIV: &str = ACTIONS[11];
const ACTION_MOD: &str = ACTIONS[12];
const ACTION_TXT: &str = ACTIONS[13];

const ACTIONCOUNT: usize = ACTIONS.len();

///Creates a new variable to the procedure
fn var(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let name = parameters[1].clone();
    let typee = parameters[0].clone();

    current_proc.add_new_variable(name, typee);
}

fn giv(current_proc: &mut Procedure) {
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

fn exe(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 1);
    assert!(parameters[1].starts_with("@"));

    //this will simply execute the proc and ignore the returned value if it returned any.
    current_proc.get_value(&parameters[0]);
}

fn rtn(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 1);

    let output_value = current_proc.get_value(&parameters[0]);

    assert!(current_proc.output_type.is_some());
    assert!(output_value.typee == current_proc.output_type.as_deref().unwrap());

    current_proc.output_value = Some(output_value);
}

fn jmp(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 1);

    let param = &parameters[0];

    println!("{:?}", current_proc.flag_map);

    assert!(current_proc.flag_map.0.contains_key(param), "{}", param);

    current_proc.next_action_index = current_proc.flag_map.0[param];
}

fn iff(current_proc: &mut Procedure) {
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

fn ifn(current_proc: &mut Procedure) {
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

// PARSE
fn prs(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 3);

    let param2 = current_proc.get_value(&parameters[1]);
    assert!(param2.typee == TYPE_TEXT, "2º param is not of type TEXT");

    let text = param2.value.get_normal_text_value();

    let numb: Option<f64> = text.parse().ok();

    if let Some(numb) = numb {
        let param1 = current_proc.get_variable_value_mutref(&parameters[0]);
        assert!(param1.typee == TYPE_NUMB, "1º param is not of type NUMB");
        param1.value = ValueForm::NormalNumb(numb);

        let param3 = current_proc.get_variable_value_mutref(&parameters[3]);
        assert!(param3.typee == TYPE_BOOL, "3º param is not of type BOOL");
        param3.value = ValueForm::NormalBool(true);
    } else {
        let param3 = current_proc.get_variable_value_mutref(&parameters[3]);
        assert!(param3.typee == TYPE_BOOL, "3º param is not of type BOOL");
        param3.value = ValueForm::NormalBool(false);
    }
}

fn add(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let param2 = current_proc.get_value(&parameters[1]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee == TYPE_NUMB && param1.value.is_normal_numb());
    assert!(param2.typee == TYPE_NUMB && param2.value.is_normal_numb());

    let numb1 = param1.value.get_normal_numb_value();
    let numb2 = param2.value.get_normal_numb_value();

    let rslt = numb1 + numb2;

    param1.value = ValueForm::NormalNumb(rslt);
}

fn sub(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let param2 = current_proc.get_value(&parameters[1]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee == TYPE_NUMB && param1.value.is_normal_numb());
    assert!(param2.typee == TYPE_NUMB && param2.value.is_normal_numb());

    let numb1 = param1.value.get_normal_numb_value();
    let numb2 = param2.value.get_normal_numb_value();

    let rslt = numb1 - numb2;

    param1.value = ValueForm::NormalNumb(rslt);
}

fn mul(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let param2 = current_proc.get_value(&parameters[1]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee == TYPE_NUMB && param1.value.is_normal_numb());
    assert!(param2.typee == TYPE_NUMB && param2.value.is_normal_numb());

    let numb1 = param1.value.get_normal_numb_value();
    let numb2 = param2.value.get_normal_numb_value();

    let rslt = numb1 * numb2;

    param1.value = ValueForm::NormalNumb(rslt);
}

fn div(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let param2 = current_proc.get_value(&parameters[1]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee == TYPE_NUMB && param1.value.is_normal_numb());
    assert!(param2.typee == TYPE_NUMB && param2.value.is_normal_numb());

    let numb1 = param1.value.get_normal_numb_value();
    let numb2 = param2.value.get_normal_numb_value();

    let rslt = numb1 / numb2;

    param1.value = ValueForm::NormalNumb(rslt);
}

fn mod_(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let param2 = current_proc.get_value(&parameters[1]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee == TYPE_NUMB && param1.value.is_normal_numb());
    assert!(param2.typee == TYPE_NUMB && param2.value.is_normal_numb());

    let numb1 = param1.value.get_normal_numb_value();
    let numb2 = param2.value.get_normal_numb_value();

    let rslt = numb1 % numb2;

    param1.value = ValueForm::NormalNumb(rslt);
}

fn txt(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let param2 = current_proc.get_value(&parameters[1]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee == TYPE_TEXT);

    let text: String;
    match param2.value {
        ValueForm::Struct(v) => text = stringify!(v).to_string(),
        ValueForm::NormalText(v) => text = v,
        ValueForm::NormalNumb(v) => text = v.to_string(),
        ValueForm::NormalBool(v) => text = v.to_string(),
        ValueForm::Array(v) => text = stringify!(v).to_string(),
    }

    param1.value = ValueForm::NormalText(text);
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

    // VAR
    map.insert(
        ACTION_VAR.to_string(),
        ActionDef {
            method: var,
            parameters_types: vec![TYPE_TYPE, TYPE_NEUTRAL],
        },
    );
    // GIV
    map.insert(
        ACTION_GIV.to_string(),
        ActionDef {
            method: giv,
            parameters_types: vec![TYPE_VAR, TYPE_NEUTRAL],
        },
    );
    // EXE
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

    // PARSE
    map.insert(
        ACTION_PRS.to_string(),
        ActionDef {
            method: prs,
            parameters_types: vec![TYPE_VAR_NUMB, TYPE_TEXT, TYPE_VAR_BOOL],
        },
    );

    // ADD
    map.insert(
        ACTION_ADD.to_string(),
        ActionDef {
            method: add,
            parameters_types: vec![TYPE_VAR_NUMB, TYPE_NUMB],
        },
    );

    // SUB
    map.insert(
        ACTION_SUB.to_string(),
        ActionDef {
            method: sub,
            parameters_types: vec![TYPE_VAR_NUMB, TYPE_NUMB],
        },
    );

    // MUL
    map.insert(
        ACTION_MUL.to_string(),
        ActionDef {
            method: mul,
            parameters_types: vec![TYPE_VAR_NUMB, TYPE_NUMB],
        },
    );

    // DIV
    map.insert(
        ACTION_DIV.to_string(),
        ActionDef {
            method: div,
            parameters_types: vec![TYPE_VAR_NUMB, TYPE_NUMB],
        },
    );

    // MOD
    map.insert(
        ACTION_MOD.to_string(),
        ActionDef {
            method: mod_,
            parameters_types: vec![TYPE_VAR_NUMB, TYPE_NUMB],
        },
    );

    //TXT
    map.insert(
        ACTION_TXT.to_string(),
        ActionDef {
            method: txt,
            parameters_types: vec![TYPE_VAR_TEXT, TYPE_NEUTRAL],
        },
    );

    // If this is giving an error make sure to add the new action to the action map
    assert!(map.len() == ACTIONCOUNT as usize);

    map
}
