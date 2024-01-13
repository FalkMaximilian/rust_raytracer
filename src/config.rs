use std::io::Error as IoError;
use std::fs;

pub struct Config {
    pub path: String
}

impl Config {
    pub fn new() -> Config {

        let config_filepaths: [&str; 2] = [
            "./config.toml",
            "./Config.toml"
        ];

        let mut content = "".to_owned();

        for filepath in config_filepaths {
            let result: Result<String, IoError> = fs::read_to_string(filepath);

            if result.is_ok() {
                content = result.unwrap();
                break;
            }
        }


        let config_toml = toml::from_str(&content).unwrap_or_else(|_| {
            println!("Failed to create ConfigToml Object from config file.");
        });

        println!("{}", content);

        Config { path: "".to_owned() }
    }
}