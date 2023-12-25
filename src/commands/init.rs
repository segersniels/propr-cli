use crate::utils::{config::Config, openai::ALLOWED_MODELS, prompt};

pub fn run() {
    let mut config = Config::load();

    let should_ask_for_template =
        prompt::ask_for_confirmation("Would you like to configure a custom template?");

    if should_ask_for_template {
        // Ask user for desired template
        let template = prompt::ask_with_editor(&config.template);
        if !template.is_empty() {
            config.template = template;
        }
    }

    // Ask user which model they wants to use
    config.model = prompt::ask_with_prompt(
        ALLOWED_MODELS.to_vec(),
        &format!("Select the model to use (current: {})", &config.model),
    );

    // Ask if user wants to automatically generate a title
    config.generate_title =
        prompt::ask_for_confirmation("Would you like propr to generate a title for you?");

    // Ask user whether they want to use a custom assistant
    config.assistant = prompt::configure_assistant(&config);

    config.save();
}
