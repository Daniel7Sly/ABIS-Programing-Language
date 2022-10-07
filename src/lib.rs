mod std_actions;

use std::collections::{self, HashMap};

// use crate::std_actions::ACTIONCOUNT;

// keywords
const KW_ARR: &[&str] = &[KW_PROC, KW_STRUCT];
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
pub const TYPE_NUM: &str = "NUM";
pub const TYPE_BOOL: &str = "BOOL";

#[derive(Debug, Clone)]
pub enum ValueForm {
    Struct(HashMap<String, Value>),
    Normal(String),
    Array(Vec<Value>),
}
impl ValueForm {
    pub fn get_normal_value(&self) -> &String {
        match self {
            ValueForm::Normal(x) => x,
            _ => panic!("atempted to get normal value without being normal"),
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
            TYPE_NUM => ValueForm::Normal("0".to_string()),
            TYPE_BOOL => ValueForm::Normal("false".to_string()),
            _ => todo!("creation of new values of structs are not implemented yet!"),
        };
        Value {
            typee: typee.to_string(),
            value: value,
        }
    }
}

// #[derive(Debug, Clone)]
// struct Variable {
//     name: String,
//     value: Value,
// }
// impl Variable {
//     fn new(name: &String, typee: &String) -> Self {
//         Variable {
//             name: name.clone(),
//             value: Value::new(typee),
//         }
//     }
// }

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
// directly wothout the neec of the ".0"
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
// directly wothout the neec of the ".0"
impl std::ops::Deref for FlagMap {
    type Target = HashMap<String, usize>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
struct Struct {
    name: String,
    fields: Vec<(String, String)>, //(type, name)
}
impl Struct {
    fn new(name: String, fields: Vec<(String, String)>) -> Self {
        Self { name, fields }
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

pub struct Interpreter {
    action_map: HashMap<String, fn(&mut Procedure)>,
    proc_list: Vec<Procedure>,
    struct_list: Vec<Struct>,
    string_literals_list: Vec<String>,
    block_call_stack: Vec<Procedure>,
}
impl Interpreter {
    pub fn new() -> Self {
        Self {
            action_map: std_actions::hashmap_with_default_actions(),
            proc_list: Vec::new(),
            struct_list: Vec::new(),
            string_literals_list: Vec::new(),
            block_call_stack: Vec::new(),
        }
    }

    pub fn add_action(&mut self) {
        todo!("adding actions is not implemented yet!")
    }

    ///Loads the current script reciving it as a string to the interpreter.
    ///
    /// Returns a true representing if the script was parsed sucsesfully.
    ///
    /// The String should contain all the text of a .abis file.
    fn load_script(&mut self, script: String) -> Result<(), AbisError> {
        let (block_list, struct_list, string_literals_list) = Self::parse_script(script)?;
        todo!()
    }

    pub fn run_script(&mut self) -> Result<(), AbisError> {
        todo!()
    }

    fn parse_script(
        script: String,
    ) -> Result<(Vec<Procedure>, Vec<Struct>, Vec<String>), AbisError> {
        let mut procedure_list: Vec<Procedure> = Vec::new();
        let mut struct_list: Vec<Struct> = Vec::new();
        let mut string_literals_list: Vec<String> = Vec::new();

        //TODO implement line position for erros.

        let mut uncommented_script: String = script
            .lines()
            .into_iter()
            .filter(|x| !x.trim().starts_with("#"))
            .collect();

        //stores and replaces the string literals
        {
            let double_quotes_positions: Vec<usize> =
                uncommented_script.match_indices('"').map(|x| x.0).collect();

            if double_quotes_positions.len() % 2 != 0 {
                return Err(AbisError::StringDeclarationWithoutEnding);
            }

            let mut count: usize = 0;
            for (i, pos1) in double_quotes_positions.iter().enumerate().step_by(2).rev() {
                let pos2 = double_quotes_positions[i + 1];
                string_literals_list.push(uncommented_script.clone()[*pos1 + 1..pos2].to_string());
                uncommented_script.replace_range(*pos1..pos2 + 1, format!("${}", count).as_str());
                count += 1;
            }
        }

        // Spaces parentheses
        uncommented_script = uncommented_script.replace("(", " ( ");
        uncommented_script = uncommented_script.replace(")", " ) ");

        //Removes white spaces
        uncommented_script.retain(|c| c != '\t' || c != '\n');

        // word processing
        {
            let words: Vec<&str> = uncommented_script.split(' ').collect();
            let words: Vec<&str> = words.into_iter().filter(|x| *x != "").collect();

            if words.len() < 4 {
                return Err(AbisError::InvalidScript);
            }

            struct_list = parse_structs(words.clone())?;
            procedure_list = parse_procs(words)?;
        }

        return Ok((procedure_list, struct_list, string_literals_list));

        fn parse_structs(words: Vec<&str>) -> Result<Vec<Struct>, AbisError> {
            //The final list containing the structs
            let mut struct_list: Vec<Struct> = Vec::new();
            //List containing the names of already created structs
            let mut struct_names: Vec<String> = Vec::new();
            //List containing the names of already created fields
            let mut fields_names: Vec<String> = Vec::new();

            let mut curr_contex: Contex = Contex::WaitingStructKW;

            let mut name = String::new();
            let mut field_type = String::new();

            let mut fields: Vec<(String, String)> = Vec::new(); //(type, name)

            //let mut field_name = String::new();

            for word in words.into_iter() {
                match word {
                    KW_STRUCT => match curr_contex {
                        Contex::WaitingStructKW => {
                            curr_contex = Contex::ExpectingStructName;
                        }
                        _ => return Err(AbisError::InvalidKeyWordInCurrentContext),
                    },
                    KW_IS => match curr_contex {
                        Contex::ExpectingIsKw => {
                            curr_contex = Contex::ExpectingFieldType;
                        }
                        _ => return Err(AbisError::InvalidKeyWordInCurrentContext),
                    },
                    KW_END => match curr_contex {
                        Contex::ExpectingFieldType => {
                            if fields.len() < 1 {
                                return Err(AbisError::StructDefinitionEndedWithoutFields);
                            }

                            struct_list.push(Struct::new(name.clone(), fields.clone()));

                            struct_names.push(name.clone());

                            //Resets name fields and fields_names for new structs
                            name.clear();
                            fields.clear();
                            fields_names.clear();

                            curr_contex = Contex::WaitingStructKW;
                        }
                        _ => return Err(AbisError::InvalidKeyWordInCurrentContext),
                    },
                    _ => match curr_contex {
                        Contex::WaitingStructKW => {
                            continue;
                        }

                        Contex::ExpectingStructName => {
                            if struct_names.contains(&word.to_string()) {
                                return Err(AbisError::DuplicateStructName);
                            }

                            struct_names.push(word.to_string());
                            name = word.to_string();
                        }

                        Contex::ExpectingFieldType => {
                            if word == TYPE_TEXT || word == TYPE_NUM || word == TYPE_BOOL {
                                field_type = word.to_string();
                            } else if struct_names.contains(&word.to_string()) {
                                field_type = word.to_string();
                            } else {
                                return Err(AbisError::TypeNotDefined);
                            }

                            curr_contex = Contex::ExpectingFieldName;
                        }

                        Contex::ExpectingFieldName => {
                            if fields_names.contains(&word.to_string()) {
                                return Err(AbisError::DuplicateFieldName);
                            }

                            fields.push((field_type.clone(), word.to_string()));

                            field_type = String::new();

                            curr_contex = Contex::ExpectingFieldType;
                        }

                        Contex::ExpectingIsKw => {
                            return Err(AbisError::ExpectingIsKeyWord);
                        }
                    },
                }
            }

            return Ok(struct_list);

            enum Contex {
                WaitingStructKW,
                ExpectingStructName,
                ExpectingIsKw,
                ExpectingFieldType,
                ExpectingFieldName,
            }
        }

        fn parse_procs(words: Vec<&str>) -> Result<Vec<Procedure>, AbisError> {
            let mut proc_list: Vec<Procedure> = Vec::new();
            //Contains the names of already created procedures
            let mut proc_names: Vec<&str> = Vec::new();

            //Things of the procedure
            let mut proc_name: String = String::new();
            let mut proc_in_vars: Vec<(String, String)> = Vec::new();
            let mut output_type: Option<String> = None;
            let mut instructions: Vec<&str> = Vec::new();

            let mut current_contex = Contex::WaitingProcKW;

            for word in words.into_iter() {
                match word {
                    KW_PROC => match current_contex {
                        Contex::WaitingProcKW => {
                            current_contex = Contex::ExpectingProcName;
                        }
                        _ => return Err(AbisError::InvalidKeyWordInCurrentContext),
                    },

                    KW_IN => match current_contex {
                        Contex::ExpectingIsOrInOrOutKW => {
                            current_contex = Contex::ExpectingFieldType;
                        }
                        _ => return Err(AbisError::InvalidKeyWordInCurrentContext),
                    },

                    KW_OUT => match current_contex {
                        Contex::ExpectingFieldType | Contex::ExpectingIsOrInOrOutKW => {
                            current_contex = Contex::ExpectingOutputType;
                        }
                        _ => return Err(AbisError::InvalidKeyWordInCurrentContext),
                    },

                    KW_IS => match current_contex {
                        Contex::ExpectingIsKw => {
                            current_contex = Contex::GettingActions;
                        }
                        _ => return Err(AbisError::InvalidKeyWordInCurrentContext),
                    },

                    KW_END => match current_contex {
                        Contex::GettingActions => {
                            let action_vec = parse_instructions_to_actions(instructions.clone());

                            proc_list.push(Procedure::new(
                                proc_name.clone(),
                                if proc_in_vars.len() == 0 {
                                    None
                                } else {
                                    Some(proc_in_vars.clone())
                                },
                                output_type.clone(),
                                action_vec.0,
                                action_vec.1,
                            ));

                            instructions.clear();
                            proc_name.clear();
                            proc_in_vars.clear();
                            output_type = None;

                            current_contex = Contex::WaitingProcKW;
                        }
                        _ => return Err(AbisError::InvalidKeyWordInCurrentContext),
                    },

                    _ => match current_contex {
                        Contex::ExpectingProcName => {
                            if proc_names.contains(&word) {
                                return Err(AbisError::DuplicateProcedureName);
                            }

                            proc_name = word.to_string();
                            proc_names.push(word);
                        }

                        Contex::WaitingProcKW => continue,
                        Contex::ExpectingIsOrInOrOutKW => {
                            return Err(AbisError::ExpectedIsOrInOrOutKW)
                        }
                        Contex::ExpectingIsKw => return Err(AbisError::ExpectingIsKeyWord),
                        Contex::ExpectingFieldName => {
                            todo!("parsing input fields are not implementd yet")
                        }
                        Contex::ExpectingFieldType => {
                            todo!("parsing input fields are not implementd yet")
                        }
                        Contex::ExpectingOutputType => {
                            todo!("output type is not implemented yet");
                            //TODO: Validate type
                            output_type = Some(word.to_string());
                        }
                        Contex::GettingActions => {
                            instructions.push(word);
                        }
                    },
                }
            }

            return Ok(proc_list);

            enum Contex {
                WaitingProcKW,
                ExpectingProcName,
                ExpectingIsOrInOrOutKW,
                ExpectingIsKw,
                ExpectingFieldName,
                ExpectingFieldType,
                ExpectingOutputType,
                GettingActions,
            }
        }

        fn parse_instructions_to_actions(instructions: Vec<&str>) -> (Vec<Action>, FlagMap) {
            todo!("parse_instructions_to_actions is not implemented yet")
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AbisError {
    TypeNotDefined,
    NoLoadedScript,
    StringDeclarationWithoutEnding,
    InvalidBlockStructure,
    InvalidScript,
    InvalidKeyWordInCurrentContext,
    DuplicateStructName,
    StructDefinitionEndedWithoutFields,
    DuplicateFieldName,
    ExpectingIsKeyWord,
    StructDefinitionEndedIncompletly,
    NameOfStructFieldCanNotHaveNameOfAType,

    //Proc Errors
    DuplicateProcedureName,
    ExpectedIsOrInOrOutKW,
}
