mod parser;
mod std_actions;

use std::collections::HashMap;

use parser::{parse_script, MainParserContex};

use crate::parser::lexer::Token;

// use crate::std_actions::ACTIONCOUNT;

// basic types
pub const TYPE_TEXT: &str = "text";
pub const TYPE_NUMB: &str = "numb";
pub const TYPE_BOOL: &str = "bool";

// Param types
const TYPE_VAR: &str = "VAR";
const TYPE_VAR_ARRAY: &str = "VAR_ARRAY";
const TYPE_TYPE: &str = "TYPE";
const TYPE_FLAG: &str = "FLAG";
const TYPE_PROC: &str = "PROC";
const TYPE_NEUTRAL: &str = "NEUTRAL";

const TYPE_VAR_TEXT: &str = "VAR_TEXT";
const TYPE_VAR_NUMB: &str = "VAR_NUMB";
const TYPE_VAR_BOOL: &str = "VAR_BOOL";

// Defualt Values
const DEF_TEXT_VALUE: String = String::new();
const DEF_NUMB_VALUE: f64 = 0.0;
const DEF_BOOL_VALUE: bool = false;

//static mut action_map: HashMap<String, fn(&mut Procedure)> = HashMap::new();

#[derive(Debug, Clone)]
pub enum Value {
    Numb(f64),
    Bool(bool),
    Text(String),
    Array(Type, Vec<Value>),
    Struct(Type, HashMap<String, Value>),
}
impl Value {
    pub fn new(typee: &str) -> Self {
        let value: Value = match typee {
            TYPE_TEXT => Value::Text(DEF_TEXT_VALUE),
            TYPE_NUMB => Value::Numb(DEF_NUMB_VALUE),
            TYPE_BOOL => Value::Bool(DEF_BOOL_VALUE),
            _ => {
                if typee.starts_with('#') {
                    let typee = typee.trim_matches('#').to_string();
                    Value::Array(typee, Vec::new())
                } else {
                    todo!("creation of values of structs is not implemented yet!");
                }
            }
        };
        value
    }
    ///Returns the type of the value.
    pub fn typee(&self) -> &str {
        match self {
            Value::Text(_) => TYPE_TEXT,
            Value::Numb(_) => TYPE_NUMB,
            Value::Bool(_) => TYPE_BOOL,
            Value::Array(t, _) => t,
            Value::Struct(t, _) => t,
        }
    }
    ///Returns a clone of the string value contained. It panics if the value is not NormalText
    pub fn get_text_value(&self) -> String {
        match self {
            Value::Text(x) => x.clone(),
            _ => panic!("atempted to get NormalText valueForm without being NormalText"),
        }
    }
    ///Returns a clone of the f64 value contained. It panics if the value is not NormalNumb
    pub fn get_numb_value(&self) -> f64 {
        match self {
            Value::Numb(x) => x.clone(),
            _ => panic!("atempted to get NormalNumb valueForm without being NormalNumb"),
        }
    }
    ///Returns a clone of the bool value contained. It panics if the value is not NormalBool
    pub fn get_bool_value(&self) -> bool {
        match self {
            Value::Bool(x) => x.clone(),
            _ => panic!("atempted to get NormalBool valueForm without being NormalBool"),
        }
    }

    pub fn is_text(&self) -> bool {
        match self {
            Value::Text(_) => true,
            _ => false,
        }
    }

    pub fn is_numb(&self) -> bool {
        match self {
            Value::Numb(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Value::Bool(_) => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            Value::Array(_, _) => true,
            _ => false,
        }
    }

    pub fn is_struct(&self) -> bool {
        match self {
            Value::Struct(_, _) => true,
            _ => false,
        }
    }
}

// #[derive(Debug, Clone)]
// pub struct Value {
//     typee: String,
//     pub value: ValueForm,
// }

// impl Value {
//     fn new(typee: &str) -> Self {
//         let value_form: ValueForm = match typee {
//             TYPE_TEXT => ValueForm::NormalText(DEF_TEXT_VALUE),
//             TYPE_NUMB => ValueForm::NormalNumb(DEF_NUMB_VALUE),
//             TYPE_BOOL => ValueForm::NormalBool(DEF_BOOL_VALUE),
//             _ => todo!("creation of new values of structs/arrays, are not implemented yet!"),
//         };
//         Value {
//             typee: typee.to_string(),
//             value: value_form,
//         }
//     }
// }

#[derive(Clone)]
pub struct ActionDef {
    method: fn(&mut Procedure),
    // parameters_types: Vec<&'static str>,
    parameters_types: &'static [&'static str],
}
impl ActionDef {
    pub fn new(method: fn(&mut Procedure), parameters_types: &'static [&'static str]) -> Self {
        ActionDef {
            method,
            parameters_types,
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
impl std::ops::DerefMut for VariableMap {
    fn deref_mut(&mut self) -> &mut HashMap<String, Value> {
        &mut self.0
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

type Name = String;
type Type = String;
type Fields = HashMap<Name, Type>;

#[derive(Debug, Clone)]
struct Struct {
    name: String,
    fields: Fields,
}

#[derive(Debug, Clone)]
struct StructMap(HashMap<Name, Struct>);
impl StructMap {
    fn new() -> Self {
        StructMap(HashMap::new())
    }
}
//This makes that you can access the map
// directly without the need of the ".0"
impl std::ops::Deref for StructMap {
    type Target = HashMap<Name, Struct>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for StructMap {
    fn deref_mut(&mut self) -> &mut HashMap<Name, Struct> {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
pub struct Procedure {
    name: String,
    //                                   type  name
    input_args_and_types: Option<HashMap<Name, Type>>,
    output_type: Option<String>,
    output_value: Option<Value>,

    proc_exe_args: Vec<Value>,

    action_list: Vec<Action>,
    flag_map: FlagMap,
    var_map: VariableMap,

    next_action_index: usize,
    current_action_index: usize,
}

impl Procedure {
    fn new(
        name: String,
        input_args_and_types: Option<HashMap<Name, Type>>,
        output_type: Option<String>,
        action_vec: Vec<Action>,
        flag_map: FlagMap,
        //line: usize,
    ) -> Self {
        Self {
            name,
            input_args_and_types,
            output_type,
            output_value: None,
            proc_exe_args: Vec::new(),
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
            .for_each(|param| param_values.push(self.get_value(param)));
        return param_values;
    }

    /// Returns a clone of the raw parameters
    fn get_raw_parameters(&self) -> Vec<String> {
        self.action_list[self.current_action_index]
            .parameters
            .clone()
    }

    ///Gets the value of the given parameter. A parameter can be a string, number, boolean, variable
    fn get_value(&self, param: &String) -> Value {
        if param.starts_with("$") {
            let var_name = param.trim_start_matches('$').to_string();
            assert!(self.var_map.contains_key(&var_name));

            self.var_map[&var_name].clone()
        }
        // else if param.starts_with('@') {
        //     let proc_name = param.trim_start_matches('@');
        //     interpreter.proc_map[proc_name].run_proc(input_values, actions_def)
        // }
        else if param.starts_with('\"') {
            Value::Text(param.trim_matches('\"').to_string())
        } else if let Some(boolean) = param.parse::<bool>().ok() {
            Value::Bool(boolean)
        } else if let Some(number) = param.parse::<f64>().ok() {
            Value::Numb(number)
        } else {
            unreachable!("param was not of the expecting values. It must be some error in the error_checker.rs.")
        }
    }

    ///Returns a mutable reference of the value of the variable found by the param
    fn get_variable_value_mutref<'a>(&'a mut self, param: &String) -> &'a mut Value {
        assert!(param.starts_with('$'));

        let k = param.trim_matches('$');

        assert!(self.var_map.contains_key(k));

        let val_ref = self.var_map.get_mut(k).unwrap();

        val_ref
    }

    fn add_new_variable(&mut self, name: String, typee: String) {
        assert!(!self.var_map.0.contains_key(&name),);
        let value = Value::new(&typee);
        self.var_map.insert(name, value);
    }

    fn add_new_variable_with_value(&mut self, name: String, typee: String, value: Value) {
        assert!(!self.var_map.0.contains_key(&name),);
        assert!(value.typee() == typee);
        self.var_map.insert(name, value);
    }

    fn run_proc(
        &mut self,
        input_values: Option<Vec<Value>>,
        actions_def: &HashMap<String, ActionDef>,
    ) -> Option<Value> {
        if input_values.is_some() {
            assert!(self.input_args_and_types.is_some());
            assert!(
                self.input_args_and_types.as_ref().unwrap().len()
                    == input_values.as_ref().unwrap().len()
            );
        }
        //Creates the variables with the input values
        match input_values {
            Some(iv) => {
                for (i, (name, typee)) in self
                    .input_args_and_types
                    .as_ref()
                    .unwrap()
                    .clone()
                    .iter()
                    .enumerate()
                {
                    let input_value = &iv[i];
                    //assert!(input_value.typee == *typee);
                    self.add_new_variable_with_value(
                        name.clone(),
                        typee.clone(),
                        input_value.clone(),
                    );
                }
            }
            None => {}
        }

        //Run procedure
        let mut i = 0;
        while i < self.action_list.len() {
            //println!("index = {}", i);
            self.current_action_index = i;
            self.next_action_index = i + 1;
            let action = self.action_list[i].clone();
            // Runs the action
            (actions_def[&action.name].method)(self);

            i = self.next_action_index;
        }

        return None;
    }
}

type ProceduresMap = HashMap<String, Procedure>;

pub struct Interpreter {
    action_map: HashMap<String, ActionDef>,
    proc_map: ProceduresMap,
    struct_map: StructMap,
    //string_literals_list: Vec<String>,
    block_call_stack: Vec<Procedure>,
}
impl Interpreter {
    pub fn new() -> Self {
        Self {
            action_map: std_actions::hashmap_with_default_actions(),
            proc_map: HashMap::new(),
            struct_map: StructMap::new(),
            //string_literals_list: Vec::new(),
            block_call_stack: Vec::new(),
        }
    }

    /// Adds a new action to the interpreter
    pub fn add_action(&mut self, action_name: &str, definition: ActionDef) {
        self.action_map.insert(action_name.to_string(), definition);
    }

    ///Loads the current script reciving it as a string to the interpreter.
    ///
    /// Returns a AbisError if the scrript could not be parsed.
    ///
    /// The String should contain all the text of a .abis file.
    pub fn load_script(&mut self, script: String) -> Result<(), AbisError> {
        let (proc_map, struct_map) = parse_script(script, &self.action_map)?;
        self.struct_map = struct_map;
        self.proc_map = proc_map;
        Ok(())
    }

    //Runs the current loaded script.
    pub fn run_scripts(&mut self) -> Result<(), AbisError> {
        if !self.proc_map.contains_key("main") {
            return Err(AbisError::MainProcedureNotFound);
        }

        let _ = &self
            .proc_map
            .get_mut("main")
            .unwrap()
            .run_proc(None, &self.action_map);

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum AbisError {
    MainProcedureNotFound,
    TypeNotDefined(Token),
    NoLoadedScript(Token),
    StringDeclarationWithoutEnding(Token),
    InvalidBlockStructure(Token),
    InvalidScript(Token),
    InvalidKeyWordInCurrentContext(Token, MainParserContex),
    DuplicateStructName(Token),
    StructDefinitionEndedWithoutFields(Token),
    DuplicateFieldName(Token),
    ExpectingIsKeyWord(Token),
    StructDefinitionEndedIncompletly(Token),
    NameOfStructFieldCanNotHaveNameOfAType(Token),

    ExpectedStructOrProcKWs(Token),

    //Proc Errors
    DuplicateProcedureName(Token),
    ExpectedIsOrInOrOutKW(Token),
    ErrorParsingProcedure(ParseProcError),

    //StructErrors
    ErrorParsingStruct(ParseStructError),
}

#[derive(Debug, PartialEq)]
pub enum ParseStructError {
    StructNameCanNotContainSpecialCharacters(Token),
    DuplicateStructName(Token),
    TypeDoesNotExist(Token),
    FieldNameCanNotBeNameOfType(Token),
    StructFieldNameCanNotContainSpecialCharacters(Token),
    DuplicateStructFieldName(Token),
}

#[derive(Debug, PartialEq)]
pub enum ParseProcError {
    ProcedureNameCanNotContainSpecialCharacters(Token),
    FieldTypeNotDefined(Token),
    DuplicateFieldName(Token),
    OutputTypeNotDefined(Token),
    UnknownAction(Token),
    ExpectedParamFoundAction(Token),
}
