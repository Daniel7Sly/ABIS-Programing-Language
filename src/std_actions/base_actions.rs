use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    Procedure, Value, TYPE_BOOL, TYPE_FLAG, TYPE_NEUTRAL, TYPE_NUMB, TYPE_PROC, TYPE_TYPE,
    TYPE_VAR, TYPE_VAR_NUMB,
};

pub(super) const ACTION_VAR: &str = "var";
pub(super) const ACTION_VAR_ARGS: &[&str] = &[TYPE_TYPE, TYPE_NEUTRAL];
///Creates a new variable to the procedure
pub(super) fn var(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let name = parameters[1].clone();
    let typee = parameters[0].clone();

    current_proc.add_new_variable(name, typee);
}

pub(super) const ACTION_GIV: &str = "giv";
pub(super) const ACTION_GIV_ARGS: &[&str] = &[TYPE_VAR, TYPE_NEUTRAL];
pub(super) fn giv(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let param2 = current_proc.get_value(&parameters[1]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee() == param2.typee());

    *param1 = param2;
}

pub(super) const ACTION_EXE: &str = "exe";
pub(super) const ACTION_EXE_ARGS: &[&str] = &[TYPE_PROC];
pub(super) fn exe(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 1);
    assert!(parameters[0].starts_with("@"));

    //this will simply execute the proc and ignore the returned value if it returned any.
    current_proc.get_value(&parameters[0]);
}

pub(super) const ACTION_RTN: &str = "rtn";
pub(super) const ACTION_RTN_ARGS: &[&str] = &[TYPE_NEUTRAL];
pub(super) fn rtn(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 1);

    let output_value = current_proc.get_value(&parameters[0]);

    assert!(current_proc.output_type.is_some());
    assert!(output_value.typee() == current_proc.output_type.as_deref().unwrap());

    current_proc.output_value = Some(output_value);
}

pub(super) const ACTION_JMP: &str = "jmp";
pub(super) const ACTION_JMP_ARGS: &[&str] = &[TYPE_FLAG];
pub(super) fn jmp(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 1);

    let param = &parameters[0];

    println!("{:?}", current_proc.flag_map);

    assert!(current_proc.flag_map.0.contains_key(param), "{}", param);

    current_proc.next_action_index = current_proc.flag_map.0[param];
}

pub(super) const ACTION_IFT: &str = "ift";
pub(super) const ACTION_IFT_ARGS: &[&str] = &[TYPE_BOOL, TYPE_FLAG];
pub(super) fn ift(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let param1 = current_proc.get_value(&parameters[0]);

    let param2 = &parameters[1];

    assert!(param1.typee() == TYPE_BOOL);

    assert!(current_proc.flag_map.0.contains_key(param2));

    if param1.get_bool_value() == true {
        current_proc.next_action_index = current_proc.flag_map.0[param2];
    }
}

pub(super) const ACTION_IFF: &str = "iff";
pub(super) const ACTION_IFF_ARGS: &[&str] = &[TYPE_BOOL, TYPE_FLAG];
pub(super) fn iff(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let param1 = current_proc.get_value(&parameters[0]);

    let param2 = &parameters[1];

    assert!(param1.typee() == TYPE_BOOL);

    assert!(current_proc.flag_map.contains_key(param2));

    if param1.get_bool_value() != true {
        current_proc.next_action_index = current_proc.flag_map[param2];
    }
}

pub(super) const ACTION_RND: &str = "rnd";
pub(super) const ACTION_RND_ARGS: &[&str] = &[TYPE_VAR_NUMB];
pub(super) fn rnd(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 1);

    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee() == TYPE_NUMB);

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

    *param1 = Value::Numb(rnd_number as f64);
}
