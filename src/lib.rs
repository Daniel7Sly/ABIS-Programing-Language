mod std_actions;

use std::collections::{self, HashMap};

use static_assertions::const_assert;
use std_actions::hashmap_with_default_actions;

// use crate::std_actions::ACTIONCOUNT;

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
const KEYWORDS_QUANT: usize = KEYWORDS.len();

// basic types
pub const TYPE_TEXT: &str = "TEXT";
pub const TYPE_NUMB: &str = "NUMB";
pub const TYPE_BOOL: &str = "BOOL";

const DEF_TEXT_VALUE: String = String::new();
const DEF_NUMB_VALUE: f64 = 0.0;
const DEF_BOOL_VALUE: bool = false;

//static mut action_map: HashMap<String, fn(&mut Procedure)> = HashMap::new();

#[derive(Debug, Clone)]
pub enum ValueForm {
    Struct(HashMap<String, Value>),
    NormalText(String),
    NormalNumb(f64),
    NormalBool(bool),
    Array(Vec<Value>),
}
impl ValueForm {
    ///Returns a clone of the string value contained. It panics if the value is not NormalText
    pub fn get_normal_text_value(&self) -> String {
        match self {
            ValueForm::NormalText(x) => x.clone(),
            _ => panic!("atempted to get NormalText valueForm without being NormalText"),
        }
    }
    ///Returns a clone of the f64 value contained. It panics if the value is not NormalNumb
    pub fn get_normal_numb_value(&self) -> f64 {
        match self {
            ValueForm::NormalNumb(x) => x.clone(),
            _ => panic!("atempted to get NormalNumb valueForm without being NormalNumb"),
        }
    }
    ///Returns a clone of the bool value contained. It panics if the value is not NormalBool
    pub fn get_normal_bool_value(&self) -> bool {
        match self {
            ValueForm::NormalBool(x) => x.clone(),
            _ => panic!("atempted to get NormalBool valueForm without being NormalBool"),
        }
    }

    pub fn is_normal_text(&self) -> bool {
        match self {
            ValueForm::NormalText(_) => true,
            _ => false,
        }
    }

    pub fn is_normal_numb(&self) -> bool {
        match self {
            ValueForm::NormalNumb(_) => true,
            _ => false,
        }
    }

    pub fn is_normal_bool(&self) -> bool {
        match self {
            ValueForm::NormalBool(_) => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            ValueForm::Array(_) => true,
            _ => false,
        }
    }

    pub fn is_struct(&self) -> bool {
        match self {
            ValueForm::Struct(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Value {
    typee: String,
    value: ValueForm,
}

impl Value {
    fn new(typee: &str) -> Self {
        let value: ValueForm = match typee {
            TYPE_TEXT => ValueForm::NormalText(DEF_TEXT_VALUE),
            TYPE_NUMB => ValueForm::NormalNumb(DEF_NUMB_VALUE),
            TYPE_BOOL => ValueForm::NormalBool(DEF_BOOL_VALUE),
            _ => todo!("creation of new values of structs/arrays are not implemented yet!"),
        };
        Value {
            typee: typee.to_string(),
            value: value,
        }
    }
}

#[derive(Debug, Clone)]
struct Action {
    name: String,
    parameters: Vec<String>,
}
impl Action {
    fn new(name: String, parameters: Vec<String>) -> Self {
        Self { name, parameters }
    }
}

#[derive(Debug, Clone)]
struct VariableMap(HashMap<String, Value>);
impl VariableMap {
    fn new() -> Self {
        Self(HashMap::new())
    }
}
//This makes that you can access the map
// directly without the need of the ".0"
impl std::ops::Deref for VariableMap {
    type Target = HashMap<String, Value>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
struct FlagMap(HashMap<String, usize>);
impl FlagMap {
    fn new(flag_map: HashMap<String, usize>) -> Self {
        Self(flag_map)
    }
}
//This makes that you can access the map
// directly without the need of the ".0"
impl std::ops::Deref for FlagMap {
    type Target = HashMap<String, usize>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

type name = String;
type typee = String;
type fields = HashMap<name, typee>;

#[derive(Debug, Clone)]
struct Struct {
    name: String,
    fields: fields,
}

#[derive(Debug, Clone)]
struct StructMap(HashMap<name, Struct>);
impl StructMap {
    fn new() -> Self {
        StructMap(HashMap::new())
    }
}
//This makes that you can access the map
// directly without the need of the ".0"
impl std::ops::Deref for StructMap {
    type Target = HashMap<name, Struct>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for StructMap {
    fn deref_mut(&mut self) -> &mut HashMap<name, Struct> {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
pub struct Procedure {
    name: String,
    //                                type    name
    input_vars_and_types: Option<Vec<(String, String)>>,
    output_type: Option<String>,
    output_value: Option<Value>,

    action_list: Vec<Action>,
    flag_map: FlagMap,
    var_map: VariableMap,

    next_action_index: usize,
    current_action_index: usize,
}

impl Procedure {
    fn new(
        name: String,
        input_vars_and_types: Option<Vec<(String, String)>>,
        output_type: Option<String>,
        action_vec: Vec<Action>,
        flag_map: FlagMap,
        //line: usize,
    ) -> Self {
        Self {
            name,
            input_vars_and_types,
            output_type,
            output_value: None,
            action_list: action_vec,
            flag_map,
            var_map: VariableMap::new(),
            next_action_index: 0,
            current_action_index: 0,
            //line,
        }
    }

    pub fn get_parameters_values(&self) -> Vec<Value> {
        let parameters = self.get_raw_parameters();
        let mut param_values: Vec<Value> = Vec::new();

        //Gets the value of each parameter
        parameters
            .iter()
            .for_each(|param| param_values.push(self.get_value(param, self.current_action_index)));
        return param_values;
    }

    fn get_raw_parameters(&self) -> &Vec<String> {
        let parameters = &self.action_list[self.current_action_index].parameters;
        parameters
    }

    ///Gets the value of the given parameter
    fn get_value(&self, param: &String, param_index: usize) -> Value {
        todo!("get_value is not implemented yet")
    }

    ///Returns a mutable reference of the value of the variable found by the param
    fn get_variable_value_mutref<'a>(&self, param: &String, param_index: usize) -> &'a mut Value {
        todo!("get_variable is not implemented yet")
    }

    fn add_new_variable(&mut self, name: String, typee: String) {
        assert!(!self.var_map.0.contains_key(&name));
        let value = get_default_value_of_type(typee);
        self.var_map.0.insert(name, value);
    }

    fn add_new_variable_with_value(&mut self, name: String, typee: String, value: Value) {
        todo!()
    }

    fn run_proc(&mut self, input_values: Option<Vec<Value>>) -> Option<Value> {
        if input_values.is_some() {
            assert!(self.input_vars_and_types.is_some());
            assert!(
                self.input_vars_and_types.clone().unwrap().len()
                    == input_values.as_ref().unwrap().len()
            );
        }
        match input_values {
            Some(iv) => iv.iter().enumerate().for_each(|(i, input_value)| {
                let (typee, name) = &self.input_vars_and_types.clone().unwrap()[i];
                self.add_new_variable_with_value(name.clone(), typee.clone(), input_value.clone());
            }),
            None => {}
        }

        //Run procedure
        let mut i: usize = 0;
        while i < self.action_list.len() {
            let action = self.action_list[i].clone();
            todo!("run_proc is not implemnted yet");
            i += 1;
        }

        return None;
    }
}

fn get_default_value_of_type(typee: String) -> Value {
    todo!("get_default_value_of_type is not implemented yet");
}

type ProceduresMap = HashMap<String, Procedure>;

// word row col
type token = (String, usize, usize);

pub struct Interpreter {
    action_map: HashMap<String, fn(&mut Procedure)>,
    proc_map: ProceduresMap,
    struct_map: StructMap,
    string_literals_list: Vec<String>,
    block_call_stack: Vec<Procedure>,
}
impl Interpreter {
    pub fn new() -> Self {
        Self {
            action_map: std_actions::hashmap_with_default_actions(),
            proc_map: HashMap::new(),
            struct_map: StructMap::new(),
            string_literals_list: Vec::new(),
            block_call_stack: Vec::new(),
        }
    }

    pub fn add_action(&mut self) {
        todo!("add_action is not implemented yet!")
    }

    ///Loads the current script reciving it as a string to the interpreter.
    ///
    /// Returns a AbisError if the scrript could not be parsed.
    ///
    /// The String should contain all the text of a .abis file.
    fn load_script(&mut self, script: String) -> Result<(), AbisError> {
        let (block_list, struct_list) = Self::parse_script(script)?;
        todo!("load_script is not implemented yet");
    }

    pub fn run_script(&mut self) -> Result<(), AbisError> {
        todo!("run_script is not implemented yet");
    }

    fn parse_script(script: String) -> Result<(ProceduresMap, StructMap), AbisError> {
        let tokens: Vec<token> = lexer(script);

        let (struct_map, procedures_map) = parser(tokens)?;

        return Ok((procedures_map, struct_map));

        fn lexer(script: String) -> Vec<token> {
            let mut token_vec: Vec<token> = Vec::new();

            let lines: Vec<&str> = script.lines().collect();

            for (line_pos, line) in lines.iter().enumerate() {
                // skip line if comment
                if line.trim_start().starts_with("#") {
                    continue;
                }

                // col word
                let words: Vec<(usize, &str)> = split_whitespace_indices(line).collect();

                let mut reading_string_literal: bool = false;
                let mut string_literal = String::new();

                for (col, word) in words {
                    if word.starts_with("\"") {
                        reading_string_literal = true;
                        string_literal.push_str(word);
                    } else if reading_string_literal {
                        string_literal.push_str(format!(" {}", word).as_str());

                        if word.ends_with("\"") {
                            reading_string_literal = false;

                            token_vec.push((string_literal.clone(), line_pos + 1, col + 1));
                            string_literal.clear();
                        }
                    } else {
                        token_vec.push((word.to_string(), line_pos + 1, col + 1));
                    }
                }
            }

            return token_vec;

            // credit: https://stackoverflow.com/a/67098851
            fn split_whitespace_indices(s: &str) -> impl Iterator<Item = (usize, &str)> {
                return s
                    .split_whitespace()
                    .map(move |sub| (addr_of(sub) - addr_of(s), sub));
                fn addr_of(s: &str) -> usize {
                    s.as_ptr() as usize
                }
            }
        }

        fn parser(tokens: Vec<token>) -> Result<(StructMap, ProceduresMap), AbisError> {
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
            let mut current_contex = Contex::WaitingProcOrStructKW;

            //TODO: add verification for field types and names can not be action names and have special caracters ("& $ # @ . = + * - / " | ? ( ) [ ] { }").

            const_assert!(KEYWORDS_QUANT == 6);
            for token in tokens {
                let word = token.0.as_str();
                match word {
                    KW_STRUCT => match current_contex {
                        Contex::WaitingProcOrStructKW => {
                            current_contex = Contex::ExpectingStructName;
                        }
                        _ => return Err(AbisError::InvalidKeyWordInCurrentContext(token)),
                    },
                    KW_END => match current_contex {
                        Contex::ReadingStructBody => {
                            current_struct_name = String::new();
                            current_contex = Contex::WaitingProcOrStructKW;
                        }

                        Contex::ReadingProcedureBody => {
                            current_proc_name = String::new();

                            current_contex = Contex::WaitingProcOrStructKW;
                        }

                        _ => return Err(AbisError::InvalidKeyWordInCurrentContext(token)),
                    },
                    KW_PROC => match current_contex {
                        Contex::WaitingProcOrStructKW => {
                            current_contex = Contex::ExpectingProcName;
                        }
                        _ => return Err(AbisError::InvalidKeyWordInCurrentContext(token)),
                    },

                    KW_IN => match current_contex {
                        Contex::ExpectingIsOrInOrOutKW => {
                            proc_map_to_parse.get_mut(&current_proc_name).unwrap().1 =
                                Some(Vec::new());

                            current_contex = Contex::ReadingProcedureInputFields;
                        }
                        _ => return Err(AbisError::InvalidKeyWordInCurrentContext(token)),
                    },

                    KW_OUT => match current_contex {
                        Contex::ReadingProcedureInputFields | Contex::ExpectingIsOrInOrOutKW => {
                            current_contex = Contex::ExpectingOutputType;
                        }
                        _ => return Err(AbisError::InvalidKeyWordInCurrentContext(token)),
                    },

                    KW_IS => match current_contex {
                        Contex::ExpectingProcIsKW | Contex::ReadingProcedureInputFields => {
                            current_contex = Contex::ReadingProcedureBody;
                        }
                        Contex::ExpectingStructIsKW => {
                            current_contex = Contex::ReadingStructBody;
                        }
                        _ => return Err(AbisError::InvalidKeyWordInCurrentContext(token)),
                    },

                    _ => match current_contex {
                        Contex::WaitingProcOrStructKW => {
                            return Err(AbisError::ExpectedStructOrProcKWs(token));
                        }

                        Contex::ExpectingStructName => {
                            if struct_map_to_parse.contains_key(&word.to_string()) {
                                return Err(AbisError::DuplicateStructName(token));
                            }
                            if contains_special_characters(&word) {
                                return Err(AbisError::ErrorParsingStruct(
                                    ParseStructError::StructNameCanNotContainSpecialCharacters(
                                        token,
                                    ),
                                ));
                            }

                            current_struct_name = word.to_string();

                            struct_map_to_parse.insert(current_struct_name.clone(), Vec::new());

                            current_contex = Contex::ExpectingStructIsKW;
                        }

                        Contex::ReadingStructBody => {
                            assert!(struct_map_to_parse.contains_key(&current_struct_name));
                            (*struct_map_to_parse.get_mut(&current_struct_name).unwrap())
                                .push(token);

                            //current_contex = Contex::ReadingStructBody;
                        }

                        //Procedures parsing--------------------------------------------------
                        Contex::ExpectingProcName => {
                            if proc_map_to_parse.contains_key(&word.to_string()) {
                                return Err(AbisError::DuplicateProcedureName(token));
                            }
                            if contains_special_characters(&word) {
                                return Err(AbisError::ErrorParsingProcedure(
                                    ParseProcError::ProcedureNameCanNotContainSpecialCharacters(
                                        token,
                                    ),
                                ));
                            }

                            current_proc_name = word.to_string();

                            proc_map_to_parse
                                .insert(current_proc_name.clone(), (Vec::new(), None, None));

                            current_contex = Contex::ExpectingIsOrInOrOutKW;
                        }

                        //Contex::WaitingProcKW => continue,
                        Contex::ExpectingIsOrInOrOutKW => {
                            return Err(AbisError::ExpectedIsOrInOrOutKW(token));
                        }

                        Contex::ExpectingProcIsKW | Contex::ExpectingStructIsKW => {
                            return Err(AbisError::ExpectingIsKeyWord(token));
                        }

                        Contex::ReadingProcedureInputFields => {
                            //This is "almost" equal to proc_map_to_parse[&current_proc_name].1.push(token);
                            proc_map_to_parse
                                .get_mut(&current_proc_name)
                                .unwrap()
                                .1
                                .as_mut()
                                .unwrap()
                                .push(token);
                        }
                        Contex::ExpectingOutputType => {
                            proc_map_to_parse.get_mut(&current_proc_name).unwrap().2 = Some(token);
                            current_contex = Contex::ExpectingProcIsKW;
                        }
                        Contex::ReadingProcedureBody => {
                            proc_map_to_parse
                                .get_mut(&current_proc_name)
                                .unwrap()
                                .0
                                .push(token);
                        }
                    },
                }
            }

            return Ok((struct_map, proc_map));

            enum Contex {
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
        }

        fn parse_proc(
            body: Vec<token>,
            input_vars: Option<Vec<token>>,
            output_type: Option<token>,
        ) -> (Vec<Action>, FlagMap) {
            todo!("parse_proc is not implemented yet")
        }

        fn parse_struct(
            structs_to_parse_map: &HashMap<name, Vec<token>>,
            name: token,
            fields: Vec<token>,
        ) -> Result<Struct, ParseStructError> {
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
                if i % 2 == 0 {
                    //field name
                    if is_basic_type(&token.0) || structs_to_parse_map.contains_key(&token.0) {
                        return Err(ParseStructError::FieldNameCanNotBeNameOfType(token));
                    } else if contains_special_characters(&token.0) {
                        return Err(
                            ParseStructError::StructFieldNameCanNotContainSpecialCharacters(token),
                        );
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

        fn is_basic_type(word: &String) -> bool {
            if word == TYPE_TEXT || word == TYPE_NUMB || word == TYPE_BOOL {
                true
            } else {
                false
            }
        }

        fn contains_special_characters(string: &str) -> bool {
            string.chars().all(char::is_alphanumeric)
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AbisError {
    TypeNotDefined(token),
    NoLoadedScript(token),
    StringDeclarationWithoutEnding(token),
    InvalidBlockStructure(token),
    InvalidScript(token),
    InvalidKeyWordInCurrentContext(token),
    DuplicateStructName(token),
    StructDefinitionEndedWithoutFields(token),
    DuplicateFieldName(token),
    ExpectingIsKeyWord(token),
    StructDefinitionEndedIncompletly(token),
    NameOfStructFieldCanNotHaveNameOfAType(token),

    ExpectedStructOrProcKWs(token),

    //Proc Errors
    DuplicateProcedureName(token),
    ExpectedIsOrInOrOutKW(token),
    ErrorParsingProcedure(ParseProcError),

    //StructErrors
    ErrorParsingStruct(ParseStructError),
}

#[derive(Debug, PartialEq)]
pub enum ParseStructError {
    StructNameCanNotContainSpecialCharacters(token),
    DuplicateStructName(token),
    TypeDoesNotExist(token),
    FieldNameCanNotBeNameOfType(token),
    StructFieldNameCanNotContainSpecialCharacters(token),
    DuplicateStructFieldName(token),
}

#[derive(Debug, PartialEq)]
pub enum ParseProcError {
    ProcedureNameCanNotContainSpecialCharacters(token),
}
