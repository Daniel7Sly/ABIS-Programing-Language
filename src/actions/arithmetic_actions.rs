// Arithmetic

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
