pub(crate) mod lexer;

use std::collections::HashMap;

use crate::parser::lexer::{lexer, Token};

use crate::{
    AbisError, Action, ActionDef, FlagMap, ParseError, ParseStructError, Program, Struct,
    StructMap, TYPE_BOOL, TYPE_NUMB, TYPE_TEXT,
};

// KEYWORDS:
const KEYWORDS: &[&str] = &["proc", "struct", "is", "in", "out", "end" /*"const"*/];
const KW_PROC: &str = KEYWORDS[0];
const KW_STRUCT: &str = KEYWORDS[1];
const KW_IS: &str = KEYWORDS[2];
const KW_IN: &str = KEYWORDS[3];
const KW_OUT: &str = KEYWORDS[4];
const KW_END: &str = KEYWORDS[5];
// const KW_CONST: &str = KEYWORDS[6];

// if new keyword is added this will
// trigger some errors to account for the new keyword
pub(crate) const KEYWORDS_QUANT: usize = KEYWORDS.len();

type Name = String;
type Type = String;

pub(crate) fn parse_script(
    script: String,
    action_map: &HashMap<String, ActionDef>,
) -> Result<(Vec<Action>, FlagMap), ParseError> {
    let tokens: Vec<Token> = lexer(script);

    let (actions, flags) = parse_tokens(tokens, action_map)?;

    return Ok((actions, flags));
}

fn _parser(
    tokens: Vec<Token>,
    action_map: &HashMap<String, ActionDef>,
) -> Result<Program, AbisError> {
    unimplemented!();
    // let mut struct_map: StructMap = StructMap::new();
    // //let mut proc_map: ProceduresMap = HashMap::new();

    // // Struct parsing variables
    // //Contains the struct that have to be parsed, contains the body of the struct.
    // let mut struct_map_to_parse: HashMap<Name, Vec<Token>> = HashMap::new();
    // //Used to know what struct we are reading the body.
    // let mut current_struct_name: String = String::new();

    // // Procedures parsing variables
    // // Contains the procedures that have to be parsed, contains the input fields
    // // the output type and the body of the of the procedure.
    // let mut proc_map_to_parse: HashMap<
    //     Name,
    //     // (body, input, output)
    //     (Vec<Token>, Option<Vec<Token>>, Option<Token>),
    // > = HashMap::new();
    // //Used to know what procedure we are reading the body.
    // let mut current_proc_name: String = String::new();

    // //Used to keep track of the current contex
    // let mut current_contex = MainParserContex::WaitingProcOrStructKW;

    // //TODO: add verification for field types and names can not be action names and have special characters ("& $ # @ . = + * - / " | ? ( ) [ ] { }").

    // assert!(KEYWORDS_QUANT == 6);

    // for token in tokens {
    //     let word = token.word.as_str();
    //     match word {
    //         KW_STRUCT => match current_contex {
    //             MainParserContex::WaitingProcOrStructKW => {
    //                 current_contex = MainParserContex::ExpectingStructName;
    //             }
    //             _ => {
    //                 return Err(AbisError::InvalidKeyWordInCurrentContext(
    //                     token,
    //                     current_contex.into(),
    //                 ))
    //             }
    //         },
    //         KW_END => match current_contex {
    //             MainParserContex::ReadingStructBody => {
    //                 current_struct_name = String::new();
    //                 current_contex = MainParserContex::WaitingProcOrStructKW;
    //             }

    //             MainParserContex::ReadingProcedureBody => {
    //                 current_proc_name = String::new();

    //                 current_contex = MainParserContex::WaitingProcOrStructKW;
    //             }

    //             _ => {
    //                 return Err(AbisError::InvalidKeyWordInCurrentContext(
    //                     token,
    //                     current_contex,
    //                 ))
    //             }
    //         },
    //         KW_PROC => match current_contex {
    //             MainParserContex::WaitingProcOrStructKW => {
    //                 current_contex = MainParserContex::ExpectingProcName;
    //             }
    //             _ => {
    //                 return Err(AbisError::InvalidKeyWordInCurrentContext(
    //                     token,
    //                     current_contex,
    //                 ))
    //             }
    //         },

    //         KW_IN => match current_contex {
    //             MainParserContex::ExpectingIsOrInOrOutKW => {
    //                 proc_map_to_parse.get_mut(&current_proc_name).unwrap().1 = Some(Vec::new());

    //                 current_contex = MainParserContex::ReadingProcedureInputFields;
    //             }
    //             _ => {
    //                 return Err(AbisError::InvalidKeyWordInCurrentContext(
    //                     token,
    //                     current_contex,
    //                 ))
    //             }
    //         },

    //         KW_OUT => match current_contex {
    //             MainParserContex::ReadingProcedureInputFields
    //             | MainParserContex::ExpectingIsOrInOrOutKW => {
    //                 current_contex = MainParserContex::ExpectingOutputType;
    //             }
    //             _ => {
    //                 return Err(AbisError::InvalidKeyWordInCurrentContext(
    //                     token,
    //                     current_contex,
    //                 ))
    //             }
    //         },

    //         KW_IS => match current_contex {
    //             MainParserContex::ExpectingProcIsKW
    //             | MainParserContex::ExpectingIsOrInOrOutKW
    //             | MainParserContex::ReadingProcedureInputFields => {
    //                 current_contex = MainParserContex::ReadingProcedureBody;
    //             }
    //             MainParserContex::ExpectingStructIsKW => {
    //                 current_contex = MainParserContex::ReadingStructBody;
    //             }
    //             _ => {
    //                 return Err(AbisError::InvalidKeyWordInCurrentContext(
    //                     token,
    //                     current_contex,
    //                 ))
    //             }
    //         },

    //         _ => match current_contex {
    //             MainParserContex::WaitingProcOrStructKW => {
    //                 return Err(AbisError::ExpectedStructOrProcKWs(token));
    //             }

    //             MainParserContex::ExpectingStructName => {
    //                 if struct_map_to_parse.contains_key(&word.to_string()) {
    //                     return Err(AbisError::DuplicateStructName(token));
    //                 }
    //                 if contains_special_characters(&word) {
    //                     return Err(AbisError::ErrorParsingStruct(
    //                         ParseStructError::StructNameCanNotContainSpecialCharacters(token),
    //                     ));
    //                 }

    //                 current_struct_name = word.to_string();

    //                 struct_map_to_parse.insert(current_struct_name.clone(), vec![token]);

    //                 current_contex = MainParserContex::ExpectingStructIsKW;
    //             }

    //             MainParserContex::ReadingStructBody => {
    //                 assert!(struct_map_to_parse.contains_key(&current_struct_name));
    //                 (*struct_map_to_parse.get_mut(&current_struct_name).unwrap()).push(token);

    //                 //current_contex = Contex::ReadingStructBody;
    //             }

    //             //Procedures parsing--------------------------------------------------
    //             MainParserContex::ExpectingProcName => {
    //                 if proc_map_to_parse.contains_key(&word.to_string()) {
    //                     return Err(AbisError::DuplicateProcedureName(token));
    //                 }
    //                 if contains_special_characters(&word) {
    //                     return Err(AbisError::ErrorParsingProcedure(
    //                         ParseProcError::ProcedureNameCanNotContainSpecialCharacters(token),
    //                     ));
    //                 }

    //                 current_proc_name = word.to_string();

    //                 proc_map_to_parse.insert(current_proc_name.clone(), (vec![token], None, None));

    //                 current_contex = MainParserContex::ExpectingIsOrInOrOutKW;
    //             }

    //             //Contex::WaitingProcKW => continue,
    //             MainParserContex::ExpectingIsOrInOrOutKW => {
    //                 return Err(AbisError::ExpectedIsOrInOrOutKW(token));
    //             }

    //             MainParserContex::ExpectingProcIsKW | MainParserContex::ExpectingStructIsKW => {
    //                 return Err(AbisError::ExpectingIsKeyWord(token));
    //             }

    //             MainParserContex::ReadingProcedureInputFields => {
    //                 //This is "almost" equal to proc_map_to_parse[&current_proc_name].1.push(token);
    //                 proc_map_to_parse
    //                     .get_mut(&current_proc_name)
    //                     .unwrap()
    //                     .1
    //                     .as_mut()
    //                     .unwrap()
    //                     .push(token);
    //             }
    //             MainParserContex::ExpectingOutputType => {
    //                 proc_map_to_parse.get_mut(&current_proc_name).unwrap().2 = Some(token);
    //                 current_contex = MainParserContex::ExpectingProcIsKW;
    //             }
    //             MainParserContex::ReadingProcedureBody => {
    //                 proc_map_to_parse
    //                     .get_mut(&current_proc_name)
    //                     .unwrap()
    //                     .0
    //                     .push(token);
    //             }
    //         },
    //     }
    // }

    // // Parses all the structs
    // for (k, v) in struct_map_to_parse.clone() {
    //     let s = parse_struct(&struct_map_to_parse, v);
    //     match s {
    //         Ok(x) => {
    //             struct_map.insert(k, x);
    //         }
    //         Err(e) => return Err(AbisError::ErrorParsingStruct(e)),
    //     }
    // }

    // for (k, (body, input, output)) in proc_map_to_parse.clone() {
    //     let s = parse_proc(body, input, output, &struct_map, action_map);
    //     match s {
    //         Ok(x) => {
    //             proc_map.insert(k, x);
    //         }
    //         Err(e) => return Err(AbisError::ErrorParsingProcedure(e)),
    //     }
    // }

    // return Ok((struct_map, proc_map));
}

fn _parse_struct(
    structs_to_parse_map: &HashMap<Name, Vec<Token>>,
    fields: Vec<Token>,
) -> Result<Struct, ParseStructError> {
    assert!(fields.len() >= 3);
    //the first token is the name of the struct
    let name: Token = fields[0].clone();

    if contains_special_characters(&name.word) {
        return Err(ParseStructError::StructNameCanNotContainSpecialCharacters(
            name,
        ));
    }

    // if structs_to_parse_map.contains_key(&name.0) {
    //     return Err(ParseStructError::DuplicateStructName(name));
    // }

    let mut new_struct = Struct {
        name: name.word,
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
            if is_basic_type(&token.word) || structs_to_parse_map.contains_key(&token.word) {
                return Err(ParseStructError::FieldNameCanNotBeNameOfType(token));
            } else if contains_special_characters(&token.word) {
                return Err(ParseStructError::StructFieldNameCanNotContainSpecialCharacters(token));
            } else if new_struct.fields.contains_key(&token.word) {
                return Err(ParseStructError::DuplicateStructFieldName(token));
            }
            assert!(!field_type.is_empty());
            new_struct.fields.insert(token.word, field_type.clone());
            field_type.clear();
        } else {
            //field type
            //TODO: check for recursive definition in structs like: struct aaa is bbb b end struct bbb is aaa a end
            if is_basic_type(&token.word) || structs_to_parse_map.contains_key(&token.word) {
                field_type = token.word
            } else {
                return Err(ParseStructError::TypeDoesNotExist(token));
            }
        }
    }

    Ok(new_struct)
}

fn _parse_proc(
    body: Vec<Token>,
    input_vars: Option<Vec<Token>>,
    output_type: Option<Token>,
    structs: &StructMap,
    action_map: &HashMap<String, ActionDef>,
) -> Result<Program, ParseError> {
    unimplemented!()
    // assert!(body.len() > 2);
    // let proc_name = body[0].word.clone();
    // let input_vars_and_types: Option<HashMap<Name, Type>> = match input_vars {
    //     Some(tokens) => {
    //         assert!(tokens.len() % 2 == 0);
    //         let mut map: HashMap<Name, Type> = HashMap::new();
    //         let mut i = 0;
    //         while i < tokens.len() {
    //             let typee = tokens[i].word.clone();
    //             let name = tokens[i + 1].word.clone();

    //             if is_basic_type(&typee) || structs.contains_key(&typee) {
    //                 if map.contains_key(&name) {
    //                     return Err(ParseProcError::DuplicateFieldName(tokens[i].clone()));
    //                 }
    //                 map.insert(name, typee);
    //             } else {
    //                 return Err(ParseProcError::FieldTypeNotDefined(tokens[i].clone()));
    //             }

    //             i += 2;
    //         }
    //         Some(map)
    //     }
    //     None => None,
    // };

    // let output_type: Option<Type> = if let Some(t) = output_type {
    //     if is_basic_type(&t.word) || structs.contains_key(&t.word) {
    //         Some(t.word)
    //     } else {
    //         return Err(ParseProcError::OutputTypeNotDefined(t));
    //     }
    // } else {
    //     None
    // };

    // let (action_vec, flag_map) = parse_proc_body(body, structs, action_map)?;

    // let new_proc = Program::new(action_vec, flag_map);

    // return Ok(new_proc);
}

// The previous name of this fn was parse_proc_body
fn parse_tokens(
    tokens: Vec<Token>,
    action_map: &HashMap<String, ActionDef>,
) -> Result<(Vec<Action>, FlagMap), ParseError> {
    let body = tokens;

    let mut action_vec = Vec::new();
    let mut flag_map: HashMap<String, usize> = HashMap::new();

    let mut current_action_name = String::new();
    let mut current_action_params = Vec::<String>::new();
    let mut current_action_param_counter = 0;

    let mut context: Context = Context::ExpectingActionNameOrFlag;

    let mut action_counter = 0;

    for (_i, token) in body.into_iter().enumerate() {
        let word = token.word.clone();
        match context {
            Context::ExpectingActionNameOrFlag => {
                if word.ends_with(':') {
                    let flag = word.trim_end_matches(':').to_string();
                    flag_map.insert(flag, action_counter);
                } else {
                    if !action_map.contains_key(&word) {
                        return Err(ParseError::UnknownAction(token));
                    }

                    current_action_name = word.clone();
                    current_action_param_counter = action_map[&word].parameters_types.len();
                    context = Context::ReadingActionArgs;
                }
            }
            Context::ReadingActionArgs => {
                if action_map.contains_key(&word) {
                    return Err(ParseError::ExpectedParamFoundAction(token));
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

#[derive(Debug, PartialEq)]
pub enum MainParserContex {
    WaitingProcOrStructKW,
    ExpectingProcName,
    ExpectingIsOrInOrOutKW,
    ExpectingProcIsKW,
    ReadingProcedureInputFields,
    ExpectingOutputType,
    ReadingProcedureBody,
    //For Structs
    ExpectingStructName,
    ExpectingStructIsKW,
    //----------
    ReadingStructBody,
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
