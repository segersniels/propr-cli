use dialoguer::{Editor, Select};

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
