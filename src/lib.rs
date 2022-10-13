mod std_actions;

use std::collections::{self, HashMap};

// use crate::std_actions::ACTIONCOUNT;

// KEYWORDS:
const KW_PROC: &str = "proc";
const KW_STRUCT: &str = "struct";
const KW_IS: &str = "is";
const KW_IN: &str = "in";
const KW_OUT: &str = "out";
const KW_END: &str = "end";

// update this number if new keyword is added this will
// trigger some errors to account for the new keyword
const KW_COUNT: u8 = 6;

// basic types
pub const TYPE_TEXT: &str = "TEXT";
pub const TYPE_NUMB: &str = "NUMB";
pub const TYPE_BOOL: &str = "BOOL";

#[derive(Debug, Clone)]
pub enum ValueForm {
    Struct(HashMap<String, Value>),
    Normal(String),
    Array(Vec<Value>),
}
impl ValueForm {
    ///Returns the normal value contained. It panics if the value is not Normal
    pub fn get_normal_value(&self) -> &String {
        match self {
            ValueForm::Normal(x) => x,
            _ => panic!("atempted to get normal valueForm without being normal"),
        }
    }

    pub fn is_normal(&self) -> bool {
        match self {
            ValueForm::Normal(_) => true,
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
            TYPE_TEXT => ValueForm::Normal("".to_string()),
            TYPE_NUMB => ValueForm::Normal("0".to_string()),
            TYPE_BOOL => ValueForm::Normal("false".to_string()),
            _ => todo!("creation of new values of structs are not implemented yet!"),
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

#[derive(Debug, Clone)]
struct StructMap(HashMap<String, Vec<(String, String)>>);
impl StructMap {
    fn new() -> Self {
        StructMap(HashMap::new())
    }
}
//This makes that you can access the map
// directly without the need of the ".0"
impl std::ops::Deref for StructMap {
    type Target = HashMap<String, Vec<(String, String)>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for StructMap {
    fn deref_mut(&mut self) -> &mut HashMap<String, Vec<(String, String)>> {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
pub struct Procedure {
    name: String,
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

    fn run_proc(input_values: Option<Vec<String>>) -> Option<Value> {
        todo!("run_proc is not implemented yet")
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
    proc_list: ProceduresMap,
    struct_map: StructMap,
    string_literals_list: Vec<String>,
    block_call_stack: Vec<Procedure>,
}
impl Interpreter {
    pub fn new() -> Self {
        Self {
            action_map: std_actions::hashmap_with_default_actions(),
            proc_list: HashMap::new(),
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
            //For creating structs
            let mut struct_map: StructMap = StructMap::new();
            let mut struct_name = String::new();
            let mut field_type = String::new();
            let mut fields: Vec<(String, String)> = Vec::new(); //(type, name)

            //For creating procedures
            let mut proc_map: ProceduresMap = HashMap::new();
            let mut proc_name: String = String::new();
            let mut proc_in_vars: Vec<(String, String)> = Vec::new();
            let mut output_type: Option<String> = None;
            let mut instructions: Vec<token> = Vec::new();

            let mut current_contex = Contex::WaitingProcOrStructKW;

            //TODO: add verification for field types and names can not be action names and have special caracters ("& $ # @").

            assert!(KW_COUNT == 6, "This paniked because a new key word was created. Pls update the parser and this assert operation for the new KW.");
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
                        Contex::ExpectingFieldTypeForStruct => {
                            if fields.len() < 1 {
                                return Err(AbisError::StructDefinitionEndedWithoutFields(token));
                            }

                            struct_map.insert(struct_name.clone(), fields.clone());

                            //Resets name fields and fields_names for new structs
                            struct_name.clear();
                            fields.clear();
                            //fields_names.clear();

                            current_contex = Contex::WaitingProcOrStructKW;
                        }

                        Contex::GettingActions => {
                            let action_vec = parse_instructions_to_actions(instructions.clone());

                            proc_map.insert(
                                proc_name.clone(),
                                Procedure::new(
                                    proc_name.clone(),
                                    if proc_in_vars.len() == 0 {
                                        None
                                    } else {
                                        Some(proc_in_vars.clone())
                                    },
                                    output_type.clone(),
                                    action_vec.0,
                                    action_vec.1,
                                ),
                            );

                            instructions.clear();
                            proc_name.clear();
                            proc_in_vars.clear();
                            output_type = None;

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
                            current_contex = Contex::ExpectingFieldTypeForProcedure;
                        }
                        _ => return Err(AbisError::InvalidKeyWordInCurrentContext(token)),
                    },

                    KW_OUT => match current_contex {
                        Contex::ExpectingFieldTypeForProcedure | Contex::ExpectingIsOrInOrOutKW => {
                            current_contex = Contex::ExpectingOutputType;
                        }
                        _ => return Err(AbisError::InvalidKeyWordInCurrentContext(token)),
                    },

                    KW_IS => match current_contex {
                        Contex::ExpectingProcIsKW => {
                            current_contex = Contex::GettingActions;
                        }
                        Contex::ExpectingStructIsKW => {
                            current_contex = Contex::ExpectingFieldTypeForStruct;
                        }
                        _ => return Err(AbisError::InvalidKeyWordInCurrentContext(token)),
                    },

                    _ => match current_contex {
                        Contex::WaitingProcOrStructKW => {
                            return Err(AbisError::ExpectedStructOrProcKWs(token));
                        }

                        Contex::ExpectingStructName => {
                            if struct_map.contains_key(&word.to_string()) {
                                return Err(AbisError::DuplicateStructName(token));
                            }

                            struct_name = word.to_string();

                            current_contex = Contex::ExpectingStructIsKW;
                        }

                        Contex::ExpectingFieldTypeForStruct => {
                            if word == TYPE_TEXT || word == TYPE_NUMB || word == TYPE_BOOL {
                                field_type = word.to_string();
                            } else if struct_map.contains_key(&word.to_string()) {
                                field_type = word.to_string();
                            } else {
                                return Err(AbisError::TypeNotDefined(token));
                            }

                            current_contex = Contex::ExpectingStructFieldName;
                        }

                        Contex::ExpectingStructFieldName => {
                            if fields.iter().any(|x| x.1 == word.to_string()) {
                                return Err(AbisError::DuplicateFieldName(token));
                            }

                            fields.push((field_type.clone(), word.to_string()));

                            field_type = String::new();

                            current_contex = Contex::ExpectingFieldTypeForStruct;
                        }
                        //Procedures parsing--------------------------------------------------
                        Contex::ExpectingProcName => {
                            if proc_map.contains_key(&word.to_string()) {
                                return Err(AbisError::DuplicateProcedureName(token));
                            }

                            proc_name = word.to_string();

                            current_contex = Contex::ExpectingIsOrInOrOutKW;
                        }

                        //Contex::WaitingProcKW => continue,
                        Contex::ExpectingIsOrInOrOutKW => {
                            return Err(AbisError::ExpectedIsOrInOrOutKW(token))
                        }
                        Contex::ExpectingProcIsKW | Contex::ExpectingStructIsKW => {
                            return Err(AbisError::ExpectingIsKeyWord(token))
                        }
                        Contex::ExpectingProcedureFieldName => {
                            todo!("parsing input fields are not implemented yet")
                        }
                        Contex::ExpectingFieldTypeForProcedure => {
                            todo!("parsing input fields are not implemented yet")
                        }
                        Contex::ExpectingOutputType => {
                            todo!("output type is not implemented yet");
                            //TODO: Validate type
                            output_type = Some(word.to_string());
                        }
                        Contex::GettingActions => {
                            instructions.push(token.clone());
                        }
                    },
                }
            }

            return Ok((struct_map, proc_map));

            enum Contex {
                WaitingProcOrStructKW,
                ExpectingProcName,
                ExpectingIsOrInOrOutKW,
                ExpectingStructIsKW,
                ExpectingProcIsKW,
                ExpectingStructFieldName,
                ExpectingProcedureFieldName,
                ExpectingFieldTypeForStruct,
                ExpectingFieldTypeForProcedure,
                ExpectingOutputType,
                ExpectingStructName,
                GettingActions,
            }
        }

        fn parse_instructions_to_actions(instructions: Vec<token>) -> (Vec<Action>, FlagMap) {
            todo!("parse_instructions_to_actions is not implemented yet")
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
}
