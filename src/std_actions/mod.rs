mod arithmetic_actions;
mod arrays_actions;
mod base_actions;
mod comparation_actions;
mod parse_actions;

use std::collections::HashMap;

use crate::ActionDef;

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
        base_actions::ACTION_VAR.to_string(),
        ActionDef {
            method: base_actions::var,
            parameters_types: base_actions::ACTION_VAR_ARGS,
        },
    );
    // GIV
    map.insert(
        base_actions::ACTION_GIV.to_string(),
        ActionDef {
            method: base_actions::giv,
            parameters_types: base_actions::ACTION_GIV_ARGS,
        },
    );
    // EXE
    map.insert(
        base_actions::ACTION_EXE.to_string(),
        ActionDef {
            method: base_actions::exe,
            parameters_types: base_actions::ACTION_EXE_ARGS,
        },
    );
    // RTN
    map.insert(
        base_actions::ACTION_RTN.to_string(),
        ActionDef {
            method: base_actions::rtn,
            parameters_types: base_actions::ACTION_RTN_ARGS,
        },
    );
    // ARG
    map.insert(
        base_actions::ACTION_ARG.to_string(),
        ActionDef {
            method: base_actions::arg,
            parameters_types: base_actions::ACTION_ARG_ARGS,
        },
    );
    // JMP
    map.insert(
        base_actions::ACTION_JMP.to_string(),
        ActionDef {
            method: base_actions::jmp,
            parameters_types: base_actions::ACTION_JMP_ARGS,
        },
    );
    map.insert(
        base_actions::ACTION_IFT.to_string(),
        ActionDef {
            method: base_actions::ift,
            parameters_types: base_actions::ACTION_IFT_ARGS,
        },
    );
    map.insert(
        base_actions::ACTION_IFF.to_string(),
        ActionDef {
            method: base_actions::iff,
            parameters_types: base_actions::ACTION_IFF_ARGS,
        },
    );

    // Random
    map.insert(
        base_actions::ACTION_RND.to_string(),
        ActionDef {
            method: base_actions::rnd,
            parameters_types: base_actions::ACTION_RND_ARGS,
        },
    );

    // PARSE
    map.insert(
        parse_actions::ACTION_PRS.to_string(),
        ActionDef {
            method: parse_actions::prs,
            parameters_types: parse_actions::ACTION_PRS_ARGS,
        },
    );

    //TXT
    map.insert(
        parse_actions::ACTION_TXT.to_string(),
        ActionDef {
            method: parse_actions::txt,
            parameters_types: parse_actions::ACTION_TXT_ARGS,
        },
    );
    // ADD
    map.insert(
        arithmetic_actions::ACTION_ADD.to_string(),
        ActionDef {
            method: arithmetic_actions::add,
            parameters_types: arithmetic_actions::ACTION_ADD_ARGS,
        },
    );

    // SUB
    map.insert(
        arithmetic_actions::ACTION_SUB.to_string(),
        ActionDef {
            method: arithmetic_actions::sub,
            parameters_types: arithmetic_actions::ACTION_SUB_ARGS,
        },
    );

    // MUL
    map.insert(
        arithmetic_actions::ACTION_MUL.to_string(),
        ActionDef {
            method: arithmetic_actions::mul,
            parameters_types: arithmetic_actions::ACTION_MUL_ARGS,
        },
    );

    // DIV
    map.insert(
        arithmetic_actions::ACTION_DIV.to_string(),
        ActionDef {
            method: arithmetic_actions::div,
            parameters_types: arithmetic_actions::ACTION_DIV_ARGS,
        },
    );

    // MOD
    map.insert(
        arithmetic_actions::ACTION_MOD.to_string(),
        ActionDef {
            method: arithmetic_actions::mod_,
            parameters_types: arithmetic_actions::ACTION_MOD_ARGS,
        },
    );

    // Bigger than
    map.insert(
        comparation_actions::ACTION_BGT.to_string(),
        ActionDef {
            method: comparation_actions::bgt,
            parameters_types: comparation_actions::ACTION_BGT_ARGS,
        },
    );

    // Smaller than
    map.insert(
        comparation_actions::ACTION_SMT.to_string(),
        ActionDef {
            method: comparation_actions::smt,
            parameters_types: comparation_actions::ACTION_SMT_ARGS,
        },
    );

    // equal
    map.insert(
        comparation_actions::ACTION_EQL.to_string(),
        ActionDef {
            method: comparation_actions::eql,
            parameters_types: comparation_actions::ACTION_EQL_ARGS,
        },
    );

    // different
    map.insert(
        comparation_actions::ACTION_DIF.to_string(),
        ActionDef {
            method: comparation_actions::dif,
            parameters_types: comparation_actions::ACTION_DIF_ARGS,
        },
    );

    // And
    map.insert(
        comparation_actions::ACTION_AND.to_string(),
        ActionDef {
            method: comparation_actions::and,
            parameters_types: comparation_actions::ACTION_AND_ARGS,
        },
    );

    // or
    map.insert(
        comparation_actions::ACTION_ORR.to_string(),
        ActionDef {
            method: comparation_actions::orr,
            parameters_types: comparation_actions::ACTION_ORR_ARGS,
        },
    );

    // array Index
    map.insert(
        arrays_actions::ACTION_ARI.to_string(),
        ActionDef {
            method: arrays_actions::ari,
            parameters_types: arrays_actions::ACTION_ARI_ARGS,
        },
    );

    // array Lenght
    map.insert(
        arrays_actions::ACTION_ARL.to_string(),
        ActionDef {
            method: arrays_actions::arl,
            parameters_types: arrays_actions::ACTION_ARL_ARGS,
        },
    );

    // If this is giving an error make sure to add the new action to the action map
    // assert!(map.len() == ACTIONCOUNT as usize);

    map
}
