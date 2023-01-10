use crate::{
    Program, Value, TYPE_BOOL, TYPE_NEUTRAL, TYPE_NUMB, TYPE_TEXT, TYPE_VAR_BOOL, TYPE_VAR_NUMB,
    TYPE_VAR_TEXT,
};

pub(super) const ACTION_PRS: &str = "prs";
pub(super) const ACTION_PRS_ARGS: &[&str] = &[TYPE_VAR_NUMB, TYPE_TEXT, TYPE_VAR_BOOL];
/// This action parses text into a number.
/// The 3rd param is a boolean representing if the parsing
/// was done sucssedfully or not.
///
/// PRS $numb text $bool
pub(crate) fn prs(current_proc: &mut Program) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == ACTION_PRS_ARGS.len());

    let param2 = current_proc.get_value(&parameters[1]);
    assert!(param2.typee() == TYPE_TEXT, "2º param is not of type TEXT");

    let text = param2.get_text_value();

    let numb: Option<f64> = text.parse().ok();

    if let Some(numb) = numb {
        let param1 = current_proc.get_variable_value_mutref(&parameters[0]);
        assert!(param1.typee() == TYPE_NUMB, "1º param is not of type NUMB");
        *param1 = Value::Numb(numb);

        let param3 = current_proc.get_variable_value_mutref(&parameters[2]);
        assert!(param3.typee() == TYPE_BOOL, "3º param is not of type BOOL");
        *param3 = Value::Bool(true);
    } else {
        let param3 = current_proc.get_variable_value_mutref(&parameters[2]);
        assert!(param3.typee() == TYPE_BOOL, "3º param is not of type BOOL");
        *param3 = Value::Bool(false);
    }
}

pub(super) const ACTION_TXT: &str = "txt";
pub(super) const ACTION_TXT_ARGS: &[&str] = &[TYPE_NEUTRAL];
/// Parses any given value into text pushing it to the stack.
/// TXT anything
pub(crate) fn txt(current_proc: &mut Program) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == ACTION_TXT_ARGS.len());

    let param2 = current_proc.get_value(&parameters[0]);

    let text: String;
    match param2 {
        Value::Struct(_, _v) => todo!(),
        Value::Text(v) => text = v,
        Value::Numb(v) => text = v.to_string(),
        Value::Bool(v) => text = v.to_string(),
        Value::Array(_, _v) => todo!(),
    }

    current_proc.value_stack.push(Value::Text(text));
}
