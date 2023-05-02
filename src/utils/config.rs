use serde::{Deserialize, Serialize};
use std::process;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub model: String,
    pub template: String,
    pub generate_title: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            model: "gpt-3.5-turbo".into(),
            template: r#"# Description"#.into(),
            generate_title: false,
        }
    }
}

pub fn get_info() -> (String, String) {
    let app_name = env!("CARGO_PKG_NAME");
    let config_name = "settings";

    (app_name.to_string(), config_name.to_string())
}

pub fn load() -> Config {
    let (app_name, config_name) = get_info();
    let result: Result<Config, confy::ConfyError> = confy::load(&app_name, config_name.as_str());

    match result {
        Ok(config) => config,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    }
}

pub fn save(config: Config) {
    let (app_name, config_name) = get_info();
    let result = confy::store(&app_name, config_name.as_str(), config);

    match result {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    }
}
