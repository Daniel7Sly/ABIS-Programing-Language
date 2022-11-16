pub(crate) fn parse_script(&self, script: String) -> Result<(ProceduresMap, StructMap), AbisError> {
    let tokens: Vec<token> = lexer(script);

    let (struct_map, procedures_map) = parser(tokens, &self.action_map)?;

    return Ok((procedures_map, struct_map));
}

fn parser(
    tokens: Vec<token>,
    action_map: &HashMap<String, ActionDef>,
) -> Result<(StructMap, ProceduresMap), AbisError> {
    let mut struct_map: StructMap = StructMap::new();
    let mut proc_map: ProceduresMap = HashMap::new();

    // Struct parsing variables
    //Contains the struct that have to be parsed, contains the body of the struct.
    let mut struct_map_to_parse: HashMap<name, Vec<token>> = HashMap::new();
    //Used to know what struct we are reading the body.
    let mut current_struct_name: String = String::new();

    // Procedures parsing variables
    // Contains the procedures that have to be parsed, contains the input fields
    // the output type and the body of the of the procedure.
    let mut proc_map_to_parse: HashMap<
        name,
        // (body, input, output)
        (Vec<token>, Option<Vec<token>>, Option<token>),
    > = HashMap::new();
    //Used to know what procedure we are reading the body.
    let mut current_proc_name: String = String::new();

    //Used to keep track of the current contex
    let mut current_contex = MainParserContex::WaitingProcOrStructKW;

    //TODO: add verification for field types and names can not be action names and have special characters ("& $ # @ . = + * - / " | ? ( ) [ ] { }").

    const_assert!(KEYWORDS_QUANT == 6);
    for token in tokens {
        let word = token.0.as_str();
        match word {
            KW_STRUCT => match current_contex {
                MainParserContex::WaitingProcOrStructKW => {
                    current_contex = MainParserContex::ExpectingStructName;
                }
                _ => {
                    return Err(AbisError::InvalidKeyWordInCurrentContext(
                        token,
                        current_contex.into(),
                    ))
                }
            },
            KW_END => match current_contex {
                MainParserContex::ReadingStructBody => {
                    current_struct_name = String::new();
                    current_contex = MainParserContex::WaitingProcOrStructKW;
                }

                MainParserContex::ReadingProcedureBody => {
                    current_proc_name = String::new();

                    current_contex = MainParserContex::WaitingProcOrStructKW;
                }

                _ => {
                    return Err(AbisError::InvalidKeyWordInCurrentContext(
                        token,
                        current_contex,
                    ))
                }
            },
            KW_PROC => match current_contex {
                MainParserContex::WaitingProcOrStructKW => {
                    current_contex = MainParserContex::ExpectingProcName;
                }
                _ => {
                    return Err(AbisError::InvalidKeyWordInCurrentContext(
                        token,
                        current_contex,
                    ))
                }
            },

            KW_IN => match current_contex {
                MainParserContex::ExpectingIsOrInOrOutKW => {
                    proc_map_to_parse.get_mut(&current_proc_name).unwrap().1 = Some(Vec::new());

                    current_contex = MainParserContex::ReadingProcedureInputFields;
                }
                _ => {
                    return Err(AbisError::InvalidKeyWordInCurrentContext(
                        token,
                        current_contex,
                    ))
                }
            },

            KW_OUT => match current_contex {
                MainParserContex::ReadingProcedureInputFields
                | MainParserContex::ExpectingIsOrInOrOutKW => {
                    current_contex = MainParserContex::ExpectingOutputType;
                }
                _ => {
                    return Err(AbisError::InvalidKeyWordInCurrentContext(
                        token,
                        current_contex,
                    ))
                }
            },

            KW_IS => match current_contex {
                MainParserContex::ExpectingProcIsKW
                | MainParserContex::ExpectingIsOrInOrOutKW
                | MainParserContex::ReadingProcedureInputFields => {
                    current_contex = MainParserContex::ReadingProcedureBody;
                }
                MainParserContex::ExpectingStructIsKW => {
                    current_contex = MainParserContex::ReadingStructBody;
                }
                _ => {
                    return Err(AbisError::InvalidKeyWordInCurrentContext(
                        token,
                        current_contex,
                    ))
                }
            },

            _ => match current_contex {
                MainParserContex::WaitingProcOrStructKW => {
                    return Err(AbisError::ExpectedStructOrProcKWs(token));
                }

                MainParserContex::ExpectingStructName => {
                    if struct_map_to_parse.contains_key(&word.to_string()) {
                        return Err(AbisError::DuplicateStructName(token));
                    }
                    if contains_special_characters(&word) {
                        return Err(AbisError::ErrorParsingStruct(
                            ParseStructError::StructNameCanNotContainSpecialCharacters(token),
                        ));
                    }

                    current_struct_name = word.to_string();

                    struct_map_to_parse.insert(current_struct_name.clone(), vec![token]);

                    current_contex = MainParserContex::ExpectingStructIsKW;
                }

                MainParserContex::ReadingStructBody => {
                    assert!(struct_map_to_parse.contains_key(&current_struct_name));
                    (*struct_map_to_parse.get_mut(&current_struct_name).unwrap()).push(token);

                    //current_contex = Contex::ReadingStructBody;
                }

                //Procedures parsing--------------------------------------------------
                MainParserContex::ExpectingProcName => {
                    if proc_map_to_parse.contains_key(&word.to_string()) {
                        return Err(AbisError::DuplicateProcedureName(token));
                    }
                    if contains_special_characters(&word) {
                        return Err(AbisError::ErrorParsingProcedure(
                            ParseProcError::ProcedureNameCanNotContainSpecialCharacters(token),
                        ));
                    }

                    current_proc_name = word.to_string();

                    proc_map_to_parse.insert(current_proc_name.clone(), (vec![token], None, None));

                    current_contex = MainParserContex::ExpectingIsOrInOrOutKW;
                }

                //Contex::WaitingProcKW => continue,
                MainParserContex::ExpectingIsOrInOrOutKW => {
                    return Err(AbisError::ExpectedIsOrInOrOutKW(token));
                }

                MainParserContex::ExpectingProcIsKW | MainParserContex::ExpectingStructIsKW => {
                    return Err(AbisError::ExpectingIsKeyWord(token));
                }

                MainParserContex::ReadingProcedureInputFields => {
                    //This is "almost" equal to proc_map_to_parse[&current_proc_name].1.push(token);
                    proc_map_to_parse
                        .get_mut(&current_proc_name)
                        .unwrap()
                        .1
                        .as_mut()
                        .unwrap()
                        .push(token);
                }
                MainParserContex::ExpectingOutputType => {
                    proc_map_to_parse.get_mut(&current_proc_name).unwrap().2 = Some(token);
                    current_contex = MainParserContex::ExpectingProcIsKW;
                }
                MainParserContex::ReadingProcedureBody => {
                    proc_map_to_parse
                        .get_mut(&current_proc_name)
                        .unwrap()
                        .0
                        .push(token);
                }
            },
        }
    }

    // Parses all the structs
    for (k, v) in struct_map_to_parse.clone() {
        let s = parse_struct(&struct_map_to_parse, v);
        match s {
            Ok(x) => {
                struct_map.insert(k, x);
            }
            Err(e) => return Err(AbisError::ErrorParsingStruct(e)),
        }
    }

    for (k, (body, input, output)) in proc_map_to_parse.clone() {
        let s = parse_proc(body, input, output, &struct_map, action_map);
        match s {
            Ok(x) => {
                proc_map.insert(k, x);
            }
            Err(e) => return Err(AbisError::ErrorParsingProcedure(e)),
        }
    }

    return Ok((struct_map, proc_map));
}

fn parse_struct(
    structs_to_parse_map: &HashMap<name, Vec<token>>,
    fields: Vec<token>,
) -> Result<Struct, ParseStructError> {
    assert!(fields.len() >= 3);
    //the first token is the name of the struct
    let name: token = fields[0].clone();

    if contains_special_characters(&name.0) {
        return Err(ParseStructError::StructNameCanNotContainSpecialCharacters(
            name,
        ));
    }

    // if structs_to_parse_map.contains_key(&name.0) {
    //     return Err(ParseStructError::DuplicateStructName(name));
    // }

    let mut new_struct = Struct {
        name: name.0,
        fields: HashMap::new(),
    };

    let mut field_type = String::new();
    for (i, token) in fields.into_iter().enumerate() {
        if i == 0 {
            //Skips the first one because is the name of the struct
            continue;
        }
        if i % 2 == 0 {
            //field name
            if is_basic_type(&token.0) || structs_to_parse_map.contains_key(&token.0) {
                return Err(ParseStructError::FieldNameCanNotBeNameOfType(token));
            } else if contains_special_characters(&token.0) {
                return Err(ParseStructError::StructFieldNameCanNotContainSpecialCharacters(token));
            } else if new_struct.fields.contains_key(&token.0) {
                return Err(ParseStructError::DuplicateStructFieldName(token));
            }
            assert!(!field_type.is_empty());
            new_struct.fields.insert(token.0, field_type.clone());
            field_type.clear();
        } else {
            //field type
            //TODO: check for recursive definition in structs like: struct aaa is bbb b end struct bbb is aaa a end
            if is_basic_type(&token.0) || structs_to_parse_map.contains_key(&token.0) {
                field_type = token.0
            } else {
                return Err(ParseStructError::TypeDoesNotExist(token));
            }
        }
    }

    Ok(new_struct)
}

fn parse_proc(
    body: Vec<token>,
    input_vars: Option<Vec<token>>,
    output_type: Option<token>,
    structs: &StructMap,
    action_map: &HashMap<String, ActionDef>,
) -> Result<Procedure, ParseProcError> {
    assert!(body.len() > 2);
    let proc_name = body[0].0.clone();
    let input_vars_and_types: Option<HashMap<name, typee>> = match input_vars {
        Some(tokens) => {
            assert!(tokens.len() % 2 == 0);
            let mut map: HashMap<name, typee> = HashMap::new();
            let mut i = 0;
            while i < tokens.len() {
                let typee = tokens[i].0.clone();
                let name = tokens[i + 1].0.clone();

                if is_basic_type(&typee) || structs.contains_key(&typee) {
                    if map.contains_key(&name) {
                        return Err(ParseProcError::DuplicateFieldName(tokens[i].clone()));
                    }
                    map.insert(name, typee);
                } else {
                    return Err(ParseProcError::FieldTypeNotDefined(tokens[i].clone()));
                }

                i += 2;
            }
            Some(map)
        }
        None => None,
    };

    let output_type: Option<typee> = if let Some(t) = output_type {
        if is_basic_type(&t.0) || structs.contains_key(&t.0) {
            Some(t.0)
        } else {
            return Err(ParseProcError::OutputTypeNotDefined(t));
        }
    } else {
        None
    };

    let (action_vec, flag_map) = parse_proc_body(body, structs, action_map)?;

    let new_proc = Procedure::new(
        proc_name,
        input_vars_and_types,
        output_type,
        action_vec,
        flag_map,
    );

    return Ok(new_proc);

    fn parse_proc_body(
        body: Vec<token>,
        _map: &StructMap,
        action_map: &HashMap<String, ActionDef>,
    ) -> Result<(Vec<Action>, FlagMap), ParseProcError> {
        //removes first token because is the name of the procedure.
        let mut body = body;
        body.remove(0);

        let mut action_vec = Vec::new();
        let mut flag_map: HashMap<String, usize> = HashMap::new();

        let mut current_action_name = String::new();
        let mut current_action_params = Vec::<String>::new();
        let mut current_action_param_counter = 0;

        let mut context: Context = Context::ExpectingActionNameOrFlag;

        let mut action_counter = 0;

        for (_i, token) in body.into_iter().enumerate() {
            let word = token.0.clone();
            match context {
                Context::ExpectingActionNameOrFlag => {
                    if word.ends_with(':') {
                        let flag = word.trim_end_matches(':').to_string();
                        flag_map.insert(flag, action_counter);
                    } else {
                        if !action_map.contains_key(&word) {
                            return Err(ParseProcError::UnknownAction(token));
                        }

                        current_action_name = word.clone();
                        current_action_param_counter = action_map[&word].parameters_types.len();
                        context = Context::ReadingActionArgs;
                    }
                }
                Context::ReadingActionArgs => {
                    if action_map.contains_key(&word) {
                        return Err(ParseProcError::ExpectedParamFoundAction(token));
                    }
                    current_action_params.push(word);
                    current_action_param_counter -= 1;

                    if current_action_param_counter == 0 {
                        action_vec.push(Action::new(
                            current_action_name.clone(),
                            current_action_params.clone(),
                        ));

                        current_action_name.clear();
                        current_action_params.clear();

                        action_counter += 1;

                        context = Context::ExpectingActionNameOrFlag;
                    }
                }
            }
        }

        return Ok((action_vec, FlagMap::new(flag_map)));

        enum Context {
            ExpectingActionNameOrFlag,
            ReadingActionArgs,
        }
    }
}

fn contains_special_characters(string: &str) -> bool {
    !string.chars().all(char::is_alphanumeric)
}

fn is_basic_type(word: &String) -> bool {
    if word == TYPE_TEXT || word == TYPE_NUMB || word == TYPE_BOOL {
        true
    } else {
        false
    }
}
