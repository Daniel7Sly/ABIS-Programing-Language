use crate::{Program, Value, TYPE_BOOL, TYPE_NEUTRAL, TYPE_NUMB};

//Comparation and logic

pub(super) const ACTION_BGT: &str = "bgt";
pub(super) const ACTION_BGT_ARGS: &[&str] = &[TYPE_NUMB, TYPE_NUMB];
/// Pushes true to value stack if the first number is bigger than the second.
/// Otherwise pushes false.
pub(super) fn bgt(program: &mut Program) {
    let parameters: Vec<String> = program.get_raw_parameters();

    assert!(parameters.len() == ACTION_BGT_ARGS.len());

    let param2 = program.get_value(&parameters[0]);
    let param3 = program.get_value(&parameters[1]);

    assert!(param2.typee() == TYPE_NUMB && param2.is_numb());
    assert!(param3.typee() == TYPE_NUMB && param3.is_numb());

    let numb1 = param2.get_numb_value();
    let numb2 = param3.get_numb_value();

    let rslt = numb1 > numb2;

    program.value_stack.push(Value::Bool(rslt));
}

pub(super) const ACTION_SMT: &str = "smt";
pub(super) const ACTION_SMT_ARGS: &[&str] = &[TYPE_NUMB, TYPE_NUMB];
/// Pushes true to value stack if the first number is smaller than the second.
/// Otherwise pushes false.
pub(super) fn smt(program: &mut Program) {
    let parameters: Vec<String> = program.get_raw_parameters();

    assert!(parameters.len() == ACTION_SMT_ARGS.len());

    let param2 = program.get_value(&parameters[0]);
    let param3 = program.get_value(&parameters[1]);

    assert!(param2.typee() == TYPE_NUMB && param2.is_numb());
    assert!(param3.typee() == TYPE_NUMB && param3.is_numb());

    let numb1 = param2.get_numb_value();
    let numb2 = param3.get_numb_value();

    let rslt = numb1 < numb2;

    program.value_stack.push(Value::Bool(rslt));
}

pub(super) const ACTION_EQL: &str = "eql";
pub(super) const ACTION_EQL_ARGS: &[&str] = &[TYPE_NEUTRAL, TYPE_NEUTRAL];
/// Pushes true to value stack if the first number is equal to the second.
/// Otherwise pushes false.
pub(super) fn eql(program: &mut Program) {
    let parameters: Vec<String> = program.get_raw_parameters();

    assert!(parameters.len() == ACTION_EQL_ARGS.len());

    let param1 = program.get_value(&parameters[0]);
    let param2 = program.get_value(&parameters[1]);

    assert!(param1.typee() == param2.typee());

    let rslt = param1 == param2;

    program.value_stack.push(Value::Bool(rslt));
}

pub(super) const ACTION_DIF: &str = "dif";
pub(super) const ACTION_DIF_ARGS: &[&str] = &[TYPE_NEUTRAL, TYPE_NEUTRAL];
/// Pushes true to value stack if the first number is different than the second.
/// Otherwise pushes false.
pub(super) fn dif(program: &mut Program) {
    let parameters: Vec<String> = program.get_raw_parameters();

    assert!(parameters.len() == ACTION_DIF_ARGS.len());

    let param1 = program.get_value(&parameters[0]);
    let param2 = program.get_value(&parameters[1]);

    assert!(param1.typee() == param2.typee());

    let rslt = param1 != param2;

    program.value_stack.push(Value::Bool(rslt));
}

pub(super) const ACTION_AND: &str = "and";
pub(super) const ACTION_AND_ARGS: &[&str] = &[TYPE_BOOL, TYPE_BOOL];
/// Pushes true to value stack if both parameters are true.
/// Otherwise pushes false.
pub(super) fn and(program: &mut Program) {
    let parameters: Vec<String> = program.get_raw_parameters();

    assert!(parameters.len() == ACTION_AND_ARGS.len());

    let param2 = program.get_value(&parameters[0]);
    let param3 = program.get_value(&parameters[1]);

    assert!(param2.typee() == TYPE_BOOL && param2.is_bool());
    assert!(param3.typee() == TYPE_BOOL && param3.is_bool());

    let numb1 = param2.get_bool_value();
    let numb2 = param3.get_bool_value();

    let rslt = numb1 && numb2;

    program.value_stack.push(Value::Bool(rslt));
}

pub(super) const ACTION_ORR: &str = "orr";
pub(super) const ACTION_ORR_ARGS: &[&str] = &[TYPE_BOOL, TYPE_BOOL];
/// Pushes true to value stack if one or both of parameters are true.
/// Otherwise pushes false.
pub(super) fn orr(program: &mut Program) {
    let parameters: Vec<String> = program.get_raw_parameters();

    assert!(parameters.len() == ACTION_ORR_ARGS.len());

    let param2 = program.get_value(&parameters[0]);
    let param3 = program.get_value(&parameters[1]);

    assert!(param2.typee() == TYPE_BOOL && param2.is_bool());
    assert!(param3.typee() == TYPE_BOOL && param3.is_bool());

    let numb1 = param2.get_bool_value();
    let numb2 = param3.get_bool_value();

    let rslt = numb1 || numb2;

    program.value_stack.push(Value::Bool(rslt));
}
