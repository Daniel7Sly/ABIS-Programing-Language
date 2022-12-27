use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    Program, Value, TYPE_BOOL, TYPE_FLAG, TYPE_NEUTRAL, TYPE_NUMB, TYPE_PROC, TYPE_TYPE, TYPE_VAR,
    TYPE_VAR_NUMB,
};

pub(super) const ACTION_VAR: &str = "var";
pub(super) const ACTION_VAR_ARGS: &[&str] = &[TYPE_TYPE, TYPE_NEUTRAL];
/// Creates a new variable
pub(super) fn var(program: &mut Program) {
    let parameters: Vec<String> = program.get_raw_parameters();

    assert!(parameters.len() == 2);

    let name = parameters[1].clone();
    let typee = parameters[0].clone();

    program.add_new_variable(name, typee);
}

pub(super) const ACTION_FRE: &str = "fre";
pub(super) const ACTION_FRE_ARGS: &[&str] = &[TYPE_VAR];
/// Frees the given variable
pub(super) fn fre(program: &mut Program) {
    let parameters: Vec<String> = program.get_raw_parameters();

    assert!(parameters.len() == ACTION_FRE_ARGS.len());

    let var_to_remove = &parameters[0].clone().trim_matches('$').to_string();

    program.var_map.remove(var_to_remove);
}

pub(super) const ACTION_GIV: &str = "giv";
pub(super) const ACTION_GIV_ARGS: &[&str] = &[TYPE_VAR, TYPE_NEUTRAL];
pub(super) fn giv(program: &mut Program) {
    let parameters: Vec<String> = program.get_raw_parameters();

    assert!(parameters.len() == 2);

    let param2 = program.get_value(&parameters[1]);
    let param1 = program.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee() == param2.typee());

    *param1 = param2;
}

pub(super) const ACTION_EXE: &str = "exe";
pub(super) const ACTION_EXE_ARGS: &[&str] = &[TYPE_PROC];
pub(super) fn exe(program: &mut Program) {
    let parameters: Vec<String> = program.get_raw_parameters();

    assert!(parameters.len() == 1);
    assert!(parameters[0].starts_with("@"));

    let flag = &parameters[0];

    assert!(program.flag_map.contains_key(flag));

    program.call_stack.push(program.current_action_index);

    program.next_action_index = program.flag_map[flag];
}

pub(super) const ACTION_RTN: &str = "rtn";
pub(super) const ACTION_RTN_ARGS: &[&str] = &[TYPE_NEUTRAL];
pub(super) fn rtn(program: &mut Program) {
    let parameters: Vec<String> = program.get_raw_parameters();

    assert!(parameters.len() == 1);

    if parameters[0] != "_".to_string() {
        let output_value = program.get_value(&parameters[0]);
        program.value_stack.push(output_value);
    }

    program.next_action_index = program
        .call_stack
        .pop()
        .expect("Hit RTN with callstack empty.")
        + 1;
}

pub(super) const ACTION_ARG: &str = "arg";
pub(super) const ACTION_ARG_ARGS: &[&str] = &[TYPE_NEUTRAL];
pub(super) fn arg(program: &mut Program) {
    let parameter: Vec<Value> = program.get_parameters_values();

    assert!(parameter.len() == 1);

    program.value_stack.push(parameter[0].clone());
}

pub(super) const ACTION_JMP: &str = "jmp";
pub(super) const ACTION_JMP_ARGS: &[&str] = &[TYPE_FLAG];
pub(super) fn jmp(program: &mut Program) {
    let parameters: Vec<String> = program.get_raw_parameters();

    assert!(parameters.len() == 1);

    let param = &parameters[0];

    println!("{:?}", program.flag_map);

    assert!(program.flag_map.0.contains_key(param), "{}", param);

    program.next_action_index = program.flag_map.0[param];
}

pub(super) const ACTION_IFT: &str = "ift";
pub(super) const ACTION_IFT_ARGS: &[&str] = &[TYPE_BOOL, TYPE_FLAG];
pub(super) fn ift(program: &mut Program) {
    let parameters: Vec<String> = program.get_raw_parameters();

    assert!(parameters.len() == 2);

    let param1 = program.get_value(&parameters[0]);

    let param2 = &parameters[1];

    assert!(param1.typee() == TYPE_BOOL);

    assert!(program.flag_map.0.contains_key(param2));

    if param1.get_bool_value() == true {
        program.next_action_index = program.flag_map.0[param2];
    }
}

pub(super) const ACTION_IFF: &str = "iff";
pub(super) const ACTION_IFF_ARGS: &[&str] = &[TYPE_BOOL, TYPE_FLAG];
pub(super) fn iff(program: &mut Program) {
    let parameters: Vec<String> = program.get_raw_parameters();

    assert!(parameters.len() == ACTION_IFF_ARGS.len());

    let param1 = program.get_value(&parameters[0]);

    let param2 = &parameters[1];

    assert!(param1.typee() == TYPE_BOOL);

    assert!(program.flag_map.contains_key(param2));

    if param1.get_bool_value() != true {
        program.next_action_index = program.flag_map[param2];
    }
}

pub(super) const ACTION_RND: &str = "rnd";
pub(super) const ACTION_RND_ARGS: &[&str] = &[];
/// Pushes a random number to the stack.
pub(super) fn rnd(program: &mut Program) {
    let parameters: Vec<String> = program.get_raw_parameters();

    assert!(parameters.len() == ACTION_RND_ARGS.len());

    // https://en.wikipedia.org/wiki/Linear_congruential_generator
    let mut seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos() as u64;

    const RAND_A: u64 = 6364136223846793005;
    const RAND_C: u64 = 1442695040888963407;

    seed = seed.overflowing_mul(RAND_A).0;
    seed = seed.overflowing_add(RAND_C).0;

    let rnd_number = seed >> 32;

    program.value_stack.push(Value::Numb(rnd_number as f64));
}
