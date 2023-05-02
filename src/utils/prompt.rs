use dialoguer::{Confirm, Editor, Input, Select};

pub fn ask_with_editor(placeholder: &str) -> String {
    match Editor::new().edit(placeholder).unwrap() {
        Some(template) => template,
        None => String::from(""),
    }
}

pub fn ask_with_prompt(choices: Vec<&str>, message: &str, _default: &str) -> String {
    let choice = Select::new()
        .with_prompt(message)
        .default(0)
        .items(&choices)
        .interact()
        .unwrap();

    choices[choice].to_string()
}

pub fn ask_with_input(message: &str) -> String {
    Input::new().with_prompt(message).interact().unwrap()
}

pub fn ask_for_confirmation(message: &str) -> bool {
    Confirm::new().with_prompt(message).interact().unwrap()
}
