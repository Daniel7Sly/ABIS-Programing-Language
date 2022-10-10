fn main() {
    let line = "proc main is
    var num a giv a 20
    var num b giv b 10
    giv $a @sum_of $a $b
    giv $a ( $a + $a )
    println \"aaa asda asdasd asd as asd\"
    println $a
end"
    .to_string();
    let x: Vec<_> = lexer(line);
    println!("{:#?}", x);
}
type token = (String, usize, usize);
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
