use std::{fs, path::PathBuf};

const DEFAULT_TEMPLATE: &str = r#"# Description"#;

/// Get the path to the settings file
pub fn get_file_path() -> PathBuf {
    let home_dir = dirs::home_dir().unwrap();
    home_dir.join(".propr/template")
}

/// Create the settings file if it doesn't exist
pub fn init() {
    let file_path = get_file_path();

    match file_path.parent() {
        Some(parent) => {
            if !parent.exists() {
                fs::create_dir_all(parent).expect("Unable to create directory");
                fs::write(&file_path, DEFAULT_TEMPLATE).expect("Unable to write file");
            }
        }
        None => {
            println!("Unable to get parent directory");
        }
    }
}

/// Get the template
pub fn get_template() -> String {
    let file_path = get_file_path();

    match fs::read_to_string(file_path) {
        Ok(template) => template,
        Err(_) => {
            println!("Unable to read file");
            String::from("")
        }
    }
}

/// Update the template
pub fn update_template(template: &str) {
    let file_path = get_file_path();

    fs::write(file_path, template).expect("Unable to write file");
}
