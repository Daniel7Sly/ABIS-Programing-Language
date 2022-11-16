use crate::{Procedure, ValueForm, TYPE_BOOL, TYPE_NUMB, TYPE_TEXT};

// PARSE

/// This action parses text into a number.
/// The 3rd param is a boolean representing if the parsing
/// was done sucssedfully or not.
///
/// PRS $numb text $bool
pub(crate) fn prs(current_proc: &mut Procedure) {
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

/// Parses any given value into text.
///
/// TXT $text anything
pub(crate) fn txt(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 2);

    let param2 = current_proc.get_value(&parameters[1]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee == TYPE_TEXT);

    let text: String;
    match param2.value {
        ValueForm::Struct(_v) => text = stringify!(v).to_string(),
        ValueForm::NormalText(v) => text = v,
        ValueForm::NormalNumb(v) => text = v.to_string(),
        ValueForm::NormalBool(v) => text = v.to_string(),
        ValueForm::Array(_v) => text = stringify!(v).to_string(),
    }

    param1.value = ValueForm::NormalText(text);
}
