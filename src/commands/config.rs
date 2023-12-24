use clap::ArgMatches;

use crate::utils::{config, openai::ALLOWED_MODELS, prompt};

pub fn run(sub_matches: &ArgMatches) {
    let mut config = config::load();

    match sub_matches.subcommand() {
        Some(("prompt", _sub_matches)) => {
            let prompt = prompt::ask_with_editor(&config.prompt);
            if prompt.is_empty() {
                return;
            }

            config.prompt = prompt;

            config::save(config);
        }
        Some(("template", _sub_matches)) => {
            let template = prompt::ask_with_editor(&config.template);
            if template.is_empty() {
                return;
            }

            config.template = template;

            config::save(config);
        }
        Some(("model", _sub_matches)) => {
            config.model = prompt::ask_with_prompt(
                ALLOWED_MODELS.to_vec(),
                &format!("Select the model to use (current: {})", &config.model),
            );

            config::save(config);
        }
        Some(("generate-title", _sub_matches)) => {
            config.generate_title =
                prompt::ask_for_confirmation("Would you like propr to generate a title for you?");

            config::save(config);
        }
        Some(("list", _sub_matches)) => {
            let (app_name, config_name) = config::get_info();

            println!(
                "Config located at: {:?}",
                confy::get_configuration_file_path(&app_name, config_name.as_str()).unwrap()
            );

            println!("{:?}", config);
        }
        _ => unreachable!(),
    }
}
