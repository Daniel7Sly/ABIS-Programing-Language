// word row col
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub word: String,
    pub row: usize,
    pub col: usize,
    //pub file_name: String,
}

pub(crate) fn lexer(script_content: String) -> Vec<Token> {
    let mut token_vec: Vec<Token> = Vec::new();

    let lines: Vec<&str> = script_content.lines().collect();

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
            if word.starts_with("\"") && !word.ends_with('\"') {
                reading_string_literal = true;
                string_literal.push_str(word);
            } else if reading_string_literal {
                string_literal.push_str(format!(" {}", word).as_str());

                if word.ends_with("\"") {
                    reading_string_literal = false;

                    token_vec.push(Token {
                        word: string_literal.clone(),
                        row: line_pos + 1,
                        col: col + 1,
                    });
                    string_literal.clear();
                }
            } else {
                token_vec.push(Token {
                    word: word.to_string(),
                    row: line_pos + 1,
                    col: col + 1,
                });
            }
        }
    }

    return token_vec;
}

// credit: https://stackoverflow.com/a/67098851
fn split_whitespace_indices(s: &str) -> impl Iterator<Item = (usize, &str)> {
    return s
        .split_whitespace()
        .map(move |sub| (addr_of(sub) - addr_of(s), sub));
    fn addr_of(s: &str) -> usize {
        s.as_ptr() as usize
    }
}
