use dialoguer::{Confirm, Editor, Input, Select};

use super::config::{AssistantConfig, Config};

pub fn ask_with_editor(placeholder: &str) -> String {
    match Editor::new().edit(placeholder).unwrap() {
        Some(template) => template,
        None => String::from(""),
    }
}

pub fn ask_with_prompt(choices: Vec<&str>, message: &str) -> String {
    let choice = Select::new()
        .with_prompt(message)
        .default(0)
        .items(&choices)
        .interact()
        .unwrap();

    choices[choice].to_string()
}

pub fn ask_with_input(message: &str, default: Option<String>) -> String {
    let mut input = Input::new();
    input.with_prompt(message);

    if let Some(default) = default {
        input.default(default);
    }

    input.interact().unwrap()
}

pub fn ask_for_confirmation(message: &str) -> bool {
    Confirm::new().with_prompt(message).interact().unwrap()
}

pub fn configure_assistant(config: &Config) -> AssistantConfig {
    let enabled = ask_for_confirmation("Would you like to use an assistant?");
    let assistant_id = if enabled {
        ask_with_input(
            "Provide the assistant's id",
            Some(config.assistant.id.clone()),
        )
    } else {
        String::from("")
    };

    AssistantConfig {
        enabled,
        id: if enabled {
            assistant_id
        } else {
            config.assistant.id.to_owned()
        },
    }
}
