fn quote(args: &Vec<String>) -> String {
    let mut quoted = Vec::<String>::new();

    for arg in args {
        // if arg is an empty string, skip this iteration
        if arg.is_empty() {
            continue;
        }

        // if char is one of these, then we need to quote it
        match arg.chars().rev().next_back().unwrap() {
            '\'' | '|' | '&' | ';' | '<' | '>' | '(' | ')' |
            '$' | '`' | '\\' | '"' | ' ' | '\t' | '\n' |
            '*' | '?' | '[' | ']' | '#' | '~' | '=' | '%' => {
                quoted.push(format!("\"{}\"", arg)); // corrected string formatting
            }
            _ =>	continue,
        }
    }
    quoted.join(" ")
}



