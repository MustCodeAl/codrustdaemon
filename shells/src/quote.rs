

// todo make this use env args for the program instead
fn quote(args: &Vec<String>) -> String {

    let mut quoted = Vec::<String>::new();

    for arg in args {
        // if char is one of these, then we need to quote it
        match arg.chars().rev().next_back().expect("not valid char") {
             '\''| '|'| '&'| ';'| '<'| '>'| '('| ')'|
            '$'| '`'| '\\'| '"'| ' '| '\t'| '\n'|
            '*'| '?'| '['| ']'| '#'| '~'| '='| '%' => {
                quoted.push(format!("\"{arg}\"" ));
                 //todo finish this

            }
            _ => {
                continue
            }
        }
    }
    quoted.join(" ")
}