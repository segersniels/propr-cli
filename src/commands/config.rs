use std::process;

use clap::ArgMatches;
use serde::{Deserialize, Serialize};

use crate::utils::{config, prompt};

#[derive(Serialize, Deserialize)]
struct Config {
    model: String,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            model: "gpt-3.5-turbo".into(),
        }
    }
}

fn get_info() -> (String, String) {
    let app_name = env!("CARGO_PKG_NAME");
    let config_name = "settings";

    (app_name.to_string(), config_name.to_string())
}

fn load_config() -> Config {
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

fn save_config(config: Config) {
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

pub fn run(sub_matches: &ArgMatches) {
    config::init();

    match sub_matches.subcommand() {
        Some(("template", _sub_matches)) => {
            let placeholder = config::get_template();
            let template = prompt::ask_with_editor(&placeholder);

            // Don't bother overriding if we didn't do anything
            if template.is_empty() {
                return;
            }

            config::update_template(&template);
        }
        Some(("model", _sub_matches)) => {
            let mut config = load_config();
            config.model = prompt::ask_with_prompt(
                vec!["gpt-3.5-turbo", "gpt-4"],
                &format!("Select the model to use (current: {})", &config.model),
                &config.model,
            );

            save_config(config);
        }
        _ => unreachable!(),
    }
}
