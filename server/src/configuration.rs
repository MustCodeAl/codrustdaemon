#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Configuration {
    app_name: String,
    config_dir: String,
    data_dir: String,
    run_dir: String,
    home_dir: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            app_name: String::from(""),
            config_dir: String::from(""),
            data_dir: String::from(""),
            run_dir: String::from(""),
            home_dir: String::from(""),
        }
        // todo!()
    }
}

impl Configuration {
    fn get_completion_db_dir(&self) -> String {
        let path = format!("{}{}", self.data_dir, "completions");
        path
    }

    fn get_completions_sqlite_db(&self) -> String {
        let path = format!("{}{}", self.data_dir, "db.sqlite3");
        path
    }

    fn get_socket_file(&self) -> String {
        let path = format!("{}{}{}", self.run_dir, self.app_name, ".sock");
        path
    }

    fn get_lock_file(&self) -> String {
        let path = format!("{}{}{}", self.run_dir, self.app_name, ".lock");
        path
    }

    fn get_log_dir(&self) -> String {
        let path = format!("{}{}", self.data_dir, "log");
        path
    }

    fn get_pid_file(&self) -> String {
        let path = format!("{}{}{}", self.run_dir, self.app_name, ".pid");
        path
    }

    fn get_learn_blacklist_file(&self) -> String {
        let path = format!("{}{}", self.data_dir, "learn-blacklist.txt");
        path
    }

    fn get_known_commands_file(&self) -> String {
        let path = format!("{}{}", self.data_dir, "known-commands.toml");
        path
    }
    pub fn new(app_name: String, config_dir: String, data_dir: String, run_dir: String, home_dir: String) -> Self {
        let config: Self = confy::load("cod", "Cod").expect("failed to load config file");
        // Self { app_name, config_dir, data_dir, run_dir, home_dir }
        config
    }
}