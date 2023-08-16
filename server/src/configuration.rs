use path::PathBuf;
use std::path;

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
    pub fn completion_db_dir(&self) -> PathBuf {
        format!("{}{}", self.data_dir, "completions").into()
    }

    pub fn completions_sqlite_db(&self) -> PathBuf {
        format!("{}{}", self.data_dir, "db.sqlite3").into()
    }

    pub fn socket_file(&self) -> PathBuf {
        format!("{}{}{}", self.run_dir, self.app_name, ".sock").into()
    }

    pub fn lock_file(&self) -> PathBuf {
        format!("{}{}{}", self.run_dir, self.app_name, ".lock").into()
    }

    pub fn log_dir(&self) -> PathBuf {
        format!("{}{}", self.data_dir, "log").into()
    }

    pub fn pid_file(&self) -> PathBuf {
        format!("{}{}{}", self.run_dir, self.app_name, ".pid").into()
    }

    pub fn learn_blacklist_file(&self) -> PathBuf {
        format!("{}{}", self.data_dir, "learn-blacklist.txt").into()
    }

    pub fn known_commands_file(&self) -> PathBuf {
        format!("{}{}", self.data_dir, "known-commands.toml").into()
    }
    pub fn new() -> Self {
        let config: Self = confy::load("cod", "Cod").expect("failed to load config file");
        // Self { app_name, config_dir, data_dir, run_dir, home_dir }
        config
    }
}