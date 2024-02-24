use toml::Value;

#[derive(Debug)]
pub struct Config {
    pub check_config_option: String,
}

pub fn parse_config(file_path: &str) -> Config {
    let config_str = match std::fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading config file: {}", err);
            String::new()
        }
    };

    let config_value: Value = match config_str.parse() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error parsing config file: {}", err);
            Value::default()
        }
    };

    let check_config_option = config_value["check_config_option"].as_str().unwrap_or_default().to_string();

    Config { check_config_option }
}
