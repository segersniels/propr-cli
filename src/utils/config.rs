use serde::{Deserialize, Serialize};
use std::process;

#[derive(Debug, Serialize, Deserialize)]
pub struct AssistantConfig {
    pub enabled: bool,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub model: String,
    pub prompt: String,
    pub template: String,
    pub generate_title: bool,
    pub assistant: AssistantConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            model: "gpt-3.5-turbo-1106".into(),
            prompt: r#"You will be asked to write a concise GitHub PR description based on a provided git diff.
Analyze the code changes and provide a concise explanation of the changes, their context and why they were made.

Don't reference file names or directories directly, instead give a general explanation of the changes made.
Do not treat imports and requires as changes or new features. If the provided message is not a diff respond with an appropriate message.
Don't surround your description in backticks but still write GitHub supported markdown."#.into(),
            template: r#"# Description"#.into(),
            generate_title: false,
            assistant: AssistantConfig {
                enabled: false,
                id: "".into(),
            },
        }
    }
}

impl Config {
    fn _get_info() -> (String, String) {
        let app_name = env!("CARGO_PKG_NAME");
        let config_name = "settings";

        (app_name.to_string(), config_name.to_string())
    }

    pub fn get_info(&self) -> (String, String) {
        Self::_get_info()
    }

    pub fn load() -> Self {
        let (app_name, config_name) = Self::_get_info();
        let result: Result<Config, confy::ConfyError> =
            confy::load(&app_name, config_name.as_str());

        match result {
            Ok(config) => config,
            Err(e) => {
                println!("{}", e);
                process::exit(1);
            }
        }
    }

    pub fn save(&self) {
        let (app_name, config_name) = Self::_get_info();
        let result = confy::store(&app_name, config_name.as_str(), self);

        match result {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e);
                process::exit(1);
            }
        }
    }
}
