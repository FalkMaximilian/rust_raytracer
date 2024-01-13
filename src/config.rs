use std::{io::Error as IoError, process::Output};
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ConfigToml {
    output: Option<ConfigTomlOutput>
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlOutput {
    path: Option<String>
}

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
            ConfigToml {
                output: None,
            }
        });

        let img_path: String = match config_toml.output {
            Some(output) => {
                let path = output.path.unwrap_or_else(|| {
                    println!("Missing field 'path' in output section in config file.");
                    "./images/img.png".to_owned()
                });
                path
            },
            None => "./images/img.png".to_owned(),
        };

        Config { path: img_path }
    }
}