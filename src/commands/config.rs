use clap::ArgMatches;

use crate::utils::{config, prompt};

pub fn run(sub_matches: &ArgMatches) {
    config::init();

    match sub_matches.subcommand() {
        Some(("template", _sub_matches)) => {
            let placeholder = config::get_template();
            let template = prompt::ask_with_editor(&placeholder);

            config::update_template(&template);
        }
        _ => unreachable!(),
    }
}
