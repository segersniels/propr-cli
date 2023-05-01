use clap::ArgMatches;

use crate::utils::{config, prompt};

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
        _ => unreachable!(),
    }
}
