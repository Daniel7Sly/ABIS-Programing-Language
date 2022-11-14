//Comparation and logic

fn bgt(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 3);

    let param2 = current_proc.get_value(&parameters[1]);
    let param3 = current_proc.get_value(&parameters[2]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee == TYPE_BOOL && param1.value.is_normal_bool());
    assert!(param2.typee == TYPE_NUMB && param2.value.is_normal_numb());
    assert!(param3.typee == TYPE_NUMB && param3.value.is_normal_numb());

    let numb1 = param2.value.get_normal_numb_value();
    let numb2 = param3.value.get_normal_numb_value();

    let rslt = numb1 > numb2;

    param1.value = ValueForm::NormalBool(rslt);
}

fn smt(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 3);

    let param2 = current_proc.get_value(&parameters[1]);
    let param3 = current_proc.get_value(&parameters[2]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee == TYPE_BOOL && param1.value.is_normal_bool());
    assert!(param2.typee == TYPE_NUMB && param2.value.is_normal_numb());
    assert!(param3.typee == TYPE_NUMB && param3.value.is_normal_numb());

    let numb1 = param2.value.get_normal_numb_value();
    let numb2 = param3.value.get_normal_numb_value();

    let rslt = numb1 < numb2;

    param1.value = ValueForm::NormalBool(rslt);
}

fn eql(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 3);

    let param2 = current_proc.get_value(&parameters[1]);
    let param3 = current_proc.get_value(&parameters[2]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee == TYPE_BOOL && param1.value.is_normal_bool());
    assert!(param2.typee == TYPE_NUMB && param2.value.is_normal_numb());
    assert!(param3.typee == TYPE_NUMB && param3.value.is_normal_numb());

    let numb1 = param2.value.get_normal_numb_value();
    let numb2 = param3.value.get_normal_numb_value();

    let rslt = numb1 == numb2;

    param1.value = ValueForm::NormalBool(rslt);
}

fn dif(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 3);

    let param2 = current_proc.get_value(&parameters[1]);
    let param3 = current_proc.get_value(&parameters[2]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee == TYPE_BOOL && param1.value.is_normal_bool());
    assert!(param2.typee == TYPE_NUMB && param2.value.is_normal_numb());
    assert!(param3.typee == TYPE_NUMB && param3.value.is_normal_numb());

    let numb1 = param2.value.get_normal_numb_value();
    let numb2 = param3.value.get_normal_numb_value();

    let rslt = numb1 != numb2;

    param1.value = ValueForm::NormalBool(rslt);
}

fn and(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 3);

    let param2 = current_proc.get_value(&parameters[1]);
    let param3 = current_proc.get_value(&parameters[2]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee == TYPE_BOOL && param1.value.is_normal_bool());
    assert!(param2.typee == TYPE_BOOL && param2.value.is_normal_bool());
    assert!(param3.typee == TYPE_BOOL && param3.value.is_normal_bool());

    let numb1 = param2.value.get_normal_bool_value();
    let numb2 = param3.value.get_normal_bool_value();

    let rslt = numb1 && numb2;

    param1.value = ValueForm::NormalBool(rslt);
}

fn orr(current_proc: &mut Procedure) {
    let parameters: Vec<String> = current_proc.get_raw_parameters();

    assert!(parameters.len() == 3);

    let param2 = current_proc.get_value(&parameters[1]);
    let param3 = current_proc.get_value(&parameters[2]);
    let param1 = current_proc.get_variable_value_mutref(&parameters[0]);

    assert!(param1.typee == TYPE_BOOL && param1.value.is_normal_bool());
    assert!(param2.typee == TYPE_BOOL && param2.value.is_normal_bool());
    assert!(param3.typee == TYPE_BOOL && param3.value.is_normal_bool());

    let numb1 = param2.value.get_normal_bool_value();
    let numb2 = param3.value.get_normal_bool_value();

    let rslt = numb1 || numb2;

    param1.value = ValueForm::NormalBool(rslt);
}
