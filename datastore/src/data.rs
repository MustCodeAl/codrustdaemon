use clap::Parser;

#[derive(Parser)]
pub struct Command {
    args: Vec<String>,
    env: Vec<String>,
    dir: String,
}


pub struct Completion {
    flag: String,
    context: FlagContext,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct FlagContext {
    sub_command: Vec<String>,
    // `json:"sub-command,omitempty"`
    framework: String, //`json:"framework,omitempty"`
}


struct HelpPage {
    executable_path: String,
    completions: Vec<Completion>,
    check_sum: String,
    command: Command,
}

struct Policy(String);


enum PolicyType {
    PolicyUnknown(Policy),
    PolicyAsk(Policy),
    PolicyTrust(Policy),
    PolicyIgnore(Policy),
}

struct AddHelpPageStatus(String);

enum AddHelpPageStatusType {
    AddHelpPageStatusNew(AddHelpPageStatus),
    AddHelpPageStatusUpdated(AddHelpPageStatus),
}


fn check_executable_path(executable_path: String) -> bool {
    todo!()
}

fn check_help_page(help_page: HelpPage) -> bool {
    todo!()
}

fn canonize_executable_path(executable_path: String) -> String {
    todo!()
}

// Function to check if a command matches a given context
fn is_command_matching_context(command: Vec<String>, context: FlagContext) -> bool {
    // Get the sub_command from the context
    let sub_command = &context.sub_command;

    // Create an iterator over the command, skipping the first element
    let mut command_iter = command.iter().skip(1);

    // Iterate over each element in sub_command
    for sub in sub_command {
        // For each sub_command, iterate over the remaining elements in command
        for cmd in command_iter.by_ref() {
            // If we find a match, break out of the inner loop
            if cmd == sub {
                break;
            }
        }

        // If we've exhausted the command iterator without finding a match, return false
        if command_iter.len() == 0 {
            return false;
        }
    }

    // If we've successfully iterated over all elements in sub_command, return true
    true
}





fn main() {
    // println!("Hello, world!");

    // Command {
    //     args: std::env::args(),
        // env: vec![],
        // dir: String::from(""),
    // };
}
