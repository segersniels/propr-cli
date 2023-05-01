use dialoguer::Editor;

pub fn ask_with_editor(placeholder: &str) -> String {
    match Editor::new().edit(placeholder).unwrap() {
        Some(template) => template,
        None => String::from(""),
    }
}
