mod arithmetic_actions;
mod base_actions;
mod comparation_actions;
mod parse_actions;

use std::{collections::HashMap, vec};

use crate::{
    ActionDef, TYPE_BOOL, TYPE_FLAG, TYPE_NEUTRAL, TYPE_NUMB, TYPE_PROC, TYPE_TEXT, TYPE_TYPE,
    TYPE_VAR, TYPE_VAR_BOOL, TYPE_VAR_NUMB, TYPE_VAR_TEXT,
};

//STANDART ACTIONS DEFINITION
const ACTIONS: &[&str] = &[
    "var", "giv", "exe", "rtn", "jmp", "iff", "ifn", "prs", "add", "sub", "mul", "div", "mod",
    "txt", "bgt", "smt", "eql", "dif", "and", "orr",
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
const ACTION_BGT: &str = ACTIONS[14];
const ACTION_SMT: &str = ACTIONS[15];
const ACTION_EQL: &str = ACTIONS[16];
const ACTION_DIF: &str = ACTIONS[17];
const ACTION_AND: &str = ACTIONS[18];
const ACTION_ORR: &str = ACTIONS[19];

const ACTIONCOUNT: usize = ACTIONS.len();

/*
    Actions of previous version missing:
        SETARR
        ARRLENGHT
        JOINTEXT
        SPLITTEXT

    Actions that could be added:
        LOG : text
        STP : (stops the interpreter)
        ASSERT : bool (same as STP but only stops if its false)
*/

/// Inserts each action definition function into a map
pub(crate) fn hashmap_with_default_actions() -> HashMap<String, ActionDef> {
    let mut map = HashMap::<String, ActionDef>::new();

    // VAR
    map.insert(
        ACTION_VAR.to_string(),
        ActionDef {
            method: base_actions::var,
            parameters_types: vec![TYPE_TYPE, TYPE_NEUTRAL],
        },
    );
    // GIV
    map.insert(
        ACTION_GIV.to_string(),
        ActionDef {
            method: base_actions::giv,
            parameters_types: vec![TYPE_VAR, TYPE_NEUTRAL],
        },
    );
    // EXE
    map.insert(
        ACTION_EXE.to_string(),
        ActionDef {
            method: base_actions::exe,
            parameters_types: vec![TYPE_PROC],
        },
    );
    map.insert(
        ACTION_RTN.to_string(),
        ActionDef {
            method: base_actions::rtn,
            parameters_types: vec![TYPE_NEUTRAL],
        },
    );
    map.insert(
        ACTION_JMP.to_string(),
        ActionDef {
            method: base_actions::jmp,
            parameters_types: vec![TYPE_FLAG],
        },
    );
    map.insert(
        ACTION_IFF.to_string(),
        ActionDef {
            method: base_actions::iff,
            parameters_types: vec![TYPE_BOOL, TYPE_FLAG],
        },
    );
    map.insert(
        ACTION_IFN.to_string(),
        ActionDef {
            method: base_actions::ifn,
            parameters_types: vec![TYPE_BOOL, TYPE_FLAG],
        },
    );

    // PARSE
    map.insert(
        ACTION_PRS.to_string(),
        ActionDef {
            method: parse_actions::prs,
            parameters_types: vec![TYPE_VAR_NUMB, TYPE_TEXT, TYPE_VAR_BOOL],
        },
    );

    //TXT
    map.insert(
        ACTION_TXT.to_string(),
        ActionDef {
            method: parse_actions::txt,
            parameters_types: vec![TYPE_VAR_TEXT, TYPE_NEUTRAL],
        },
    );
    // ADD
    map.insert(
        ACTION_ADD.to_string(),
        ActionDef {
            method: arithmetic_actions::add,
            parameters_types: vec![TYPE_VAR_NUMB, TYPE_NUMB],
        },
    );

    // SUB
    map.insert(
        ACTION_SUB.to_string(),
        ActionDef {
            method: arithmetic_actions::sub,
            parameters_types: vec![TYPE_VAR_NUMB, TYPE_NUMB],
        },
    );

    // MUL
    map.insert(
        ACTION_MUL.to_string(),
        ActionDef {
            method: arithmetic_actions::mul,
            parameters_types: vec![TYPE_VAR_NUMB, TYPE_NUMB],
        },
    );

    // DIV
    map.insert(
        ACTION_DIV.to_string(),
        ActionDef {
            method: arithmetic_actions::div,
            parameters_types: vec![TYPE_VAR_NUMB, TYPE_NUMB],
        },
    );

    // MOD
    map.insert(
        ACTION_MOD.to_string(),
        ActionDef {
            method: arithmetic_actions::mod_,
            parameters_types: vec![TYPE_VAR_NUMB, TYPE_NUMB],
        },
    );

    // Bigger than
    map.insert(
        ACTION_BGT.to_string(),
        ActionDef {
            method: comparation_actions::bgt,
            parameters_types: vec![TYPE_VAR_BOOL, TYPE_NUMB, TYPE_NUMB],
        },
    );

    // Smaller than
    map.insert(
        ACTION_SMT.to_string(),
        ActionDef {
            method: comparation_actions::smt,
            parameters_types: vec![TYPE_VAR_BOOL, TYPE_NUMB, TYPE_NUMB],
        },
    );

    // equal
    map.insert(
        ACTION_EQL.to_string(),
        ActionDef {
            method: comparation_actions::eql,
            parameters_types: vec![TYPE_VAR_BOOL, TYPE_NUMB, TYPE_NUMB],
        },
    );

    // different
    map.insert(
        ACTION_DIF.to_string(),
        ActionDef {
            method: comparation_actions::dif,
            parameters_types: vec![TYPE_VAR_BOOL, TYPE_NUMB, TYPE_NUMB],
        },
    );

    // And
    map.insert(
        ACTION_AND.to_string(),
        ActionDef {
            method: comparation_actions::and,
            parameters_types: vec![TYPE_VAR_BOOL, TYPE_BOOL, TYPE_BOOL],
        },
    );

    // or
    map.insert(
        ACTION_ORR.to_string(),
        ActionDef {
            method: comparation_actions::orr,
            parameters_types: vec![TYPE_VAR_BOOL, TYPE_BOOL, TYPE_BOOL],
        },
    );

    // If this is giving an error make sure to add the new action to the action map
    assert!(map.len() == ACTIONCOUNT as usize);

    map
}
