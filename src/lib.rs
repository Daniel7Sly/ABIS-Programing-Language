use std::collections::{self, HashMap};

const TYPE_TEXT: &str = "TEXT";
const TYPE_NUM: &str = "NUM";
const TYPE_BOOL: &str = "BOOL";

#[derive(Debug, Clone)]
enum ValueValue {
    Struct(HashMap<String, Value>),
    Normal(String),
    Array(Vec<Value>),
}

//MAYBE
// #[derive(Debug, Clone)]
// enum Type {
//     Struct(String),
//     Text,
//     Num,
//     Bool,
//     Array(Type),
// }

#[derive(Debug, Clone)]
struct Value {
    typee: String,
    value: ValueValue,
}

impl Value {
    fn new(typee: &str) -> Self {
        let value: ValueValue = match typee {
            TYPE_TEXT => ValueValue::Normal("".to_string()),
            TYPE_NUM => ValueValue::Normal("0".to_string()),
            TYPE_BOOL => ValueValue::Normal("false".to_string()),
            _ => todo!(),
        };
        Value {
            typee: typee.to_string(),
            value: value,
        }
    }
}

#[derive(Debug, Clone)]
struct Variable {
    name: String,
    value: Value,
}
impl Variable {
    fn new(name: &String, typee: &String) -> Self {
        Variable {
            name: name.clone(),
            value: Value::new(typee),
        }
    }
}

#[derive(Debug, Clone)]
struct Action {
    name: String,
    parameters: Vec<String>,
    line: usize,
}
impl Action {
    fn new(name: String, parameters: Vec<String>, line: usize) -> Self {
        Self {
            name,
            parameters,
            line,
        }
    }
}

#[derive(Debug, Clone)]
struct Flag {
    name: String,
    position: usize,
    line: usize,
}
impl Flag {
    fn new(name: String, position: usize, line: usize) -> Self {
        Self {
            name,
            position,
            line,
        }
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
struct Block {
    name: String,
    line: usize,
    input_vars_and_types: Vec<(String, String)>,
    output_type: Option<String>,
    output_value: Option<Value>,

    action_vec: Vec<Action>,
    flag_vec: Vec<Flag>,
    var_vec: Vec<Variable>,

    next_action_index: usize,
}

impl Block {
    fn new(
        name: String,
        input_vars_and_types: Vec<(String, String)>,
        output_type: Option<String>,
        action_vec: Vec<Action>,
        flag_vec: Vec<Flag>,
        line: usize,
    ) -> Self {
        Self {
            name,
            input_vars_and_types,
            output_type,
            output_value: None,
            action_vec,
            flag_vec,
            var_vec: Vec::new(),
            next_action_index: 0,
            line,
        }
    }

    fn run_block(input_values: Option<Vec<String>>) {
        todo!()
    }
}

pub struct Interpreter {
    action_map: HashMap<String, fn(&mut Block)>,
    block_list: Vec<Block>,
    struct_list: Vec<Struct>,
    string_literals_list: Vec<String>,
    block_call_stack: Vec<Block>,
}
impl Interpreter {
    pub fn new() -> Self {
        Self {
            action_map: Self::hashmap_with_default_actions(),
            block_list: Vec::new(),
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

    fn parse_script(script: String) -> Result<(Vec<Block>, Vec<Struct>, Vec<String>), AbisError> {
        let mut block_list: Vec<Block> = Vec::new();
        let mut struct_list: Vec<Struct> = Vec::new();
        let mut string_literals_list: Vec<String> = Vec::new();

        'a: for (line_index, line) in script.lines().enumerate() {
            let line_words: Vec<&str> = line.split(' ').collect();

            for (word_index, word) in line_words.iter().enumerate() {}
        }

        return Ok((block_list, struct_list, string_literals_list));
        fn parse_block(block_script: &str) -> Block {
            todo!()
        }
        fn parse_struct() -> Struct {
            todo!()
        }
        fn parse_strings() -> String {
            todo!()
        }
    }

    fn hashmap_with_default_actions() -> HashMap<String, fn(&mut Block)> {
        let mut map = HashMap::<String, fn(&mut Block)>::new();

        map.insert("crt".to_string(), crt);
        map.insert("giv".to_string(), giv);
        map.insert("exe".to_string(), exe);
        map.insert("rtn".to_string(), rtn);
        map.insert("jmp".to_string(), jmp);
        map.insert("iff".to_string(), iff);
        map.insert("ifn".to_string(), ifn);

        assert!(map.len() == ACTIONCOUNT as usize);

        map
    }
}

//STANDART ACTIONS DEFINITION
const ACTIONCOUNT: u8 = 8;
fn crt(current_block: &mut Block) {
    todo!()
}

fn giv(current_block: &mut Block) {
    todo!()
}

fn exe(current_block: &mut Block) {
    todo!()
}

fn rtn(current_block: &mut Block) {
    todo!()
}

fn jmp(current_block: &mut Block) {
    todo!()
}

fn iff(current_block: &mut Block) {
    todo!()
}

fn ifn(current_block: &mut Block) {
    todo!()
}

/* Actions missing:
    PARSE
    SETARR
    ARRLENGHT
    JOINTEXT
    SPLITTEXT
*/

pub enum AbisError {
    NoLoadedScript,
}
