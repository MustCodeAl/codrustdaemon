trait ShellScriptGenerator {
    fn get_preamble(&self) -> Option<Vec<String>>;
    // tood pass in completitons instead or data store
    fn generate_completions(&self) -> Vec<String>;
    fn reset_command(&mut self) -> Vec<String>;
}
// pub use crate::datastore::data::completion;
struct Completion;
struct DataStore {
    completions: Vec<Completion>,
    shell: String,
    pid: u32,
}

enum ShellType {
    Bash,
    Zsh,
    Fish,
    // Powershell,
    // Cmd,
}


struct ExecutablePath(std::path::PathBuf);

struct Shell {
    shell_type: ShellType,
    pid: u32,
    executable_path0: ExecutablePath,
    completions: Vec<Completion>,
}

impl Shell {
    pub fn new(shell_type: ShellType, pid: u32, executable_path0: ExecutablePath, completions: Vec<Completion>) -> Self {
        Self { shell_type, pid, executable_path0, completions }
    }
}


impl ShellScriptGenerator for Shell {
    fn get_preamble(&self) -> Option<Vec<String>> {
        match self.shell_type {
            ShellType::Bash => {
                std::fs::read_to_string("bash_preamble.sh").unwrap()
            }
            ShellType::Zsh => {
                std::fs::read_to_string("zsh_preamble.sh").unwrap()
            }
            ShellType::Fish => {
                std::fs::read_to_string("fish_preamble.sh").unwrap()
            }
        //     ShellType::Powershell => {
        //         std::fs::read_to_string("powershell_preamble.sh").unwrap()
        //     }
        //     ShellType::Cmd => {
        //         std::fs::read_to_string("cmd_preamble.sh").unwrap()
        //     }
        };
        todo!()
    }

    fn generate_completions(&self) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();

        // should return cod command path
        lines.push(format!("__cod_clear_completions \n{}", self.executable_path0.0.file_name().unwrap().to_str().unwrap()));
        lines

        // todo!()
    }
    fn reset_command(&mut self) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();

        match self.shell_type {
            ShellType::Bash => {
                // should return cod command path
                lines.push(format!("__cod_clear_completions \n{}", self.executable_path0.0.file_name().unwrap().to_str().unwrap()));
                lines
            }
            ShellType::Zsh => {
                // should return cod command path
                lines.push(format!("__cod_clear_completions \n{}", self.executable_path0.0.file_name().unwrap().to_str().unwrap()));
                lines
            }
            ShellType::Fish => {
                // should return cod command path
                lines.push(format!("complete --command %{} --erase\n", self.executable_path0.0.file_name().unwrap().to_str().unwrap()));
                lines
            }
            // ShellType::Powershell => {
            //     // should return cod command path
            //     lines.push(format!("__cod_clear_completions \n{}", self.executable_path0.executable_path.file_name().unwrap().to_str().unwrap()));
            //     lines
            // }
            // ShellType::Cmd => {
            //     // should return cod command path
            //     lines.push(format!("__cod_clear_completions \n{}", self.executable_path0.executable_path.file_name().unwrap().to_str().unwrap()));
            //     lines
            // }
        }



        // todo!()
    }
}