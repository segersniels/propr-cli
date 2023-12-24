use std::process;

use clap::ArgMatches;

use crate::utils::{config, github, loader, openai};

pub async fn run(sub_matches: &ArgMatches) {
    let default_branch = github::get_default_branch();
    let base = sub_matches
        .get_one::<String>("branch")
        .unwrap_or(&default_branch);

    let diff = github::get_diff(base);
    if diff.is_empty() {
        println!("No diff found");
        return;
    }

    let config = config::load();
    let mut loader = loader::create_loader("Generating");
    let model = sub_matches
        .get_one::<String>("model")
        .unwrap_or(&config.model);

    if let Some(assistant_id) = sub_matches.get_one::<String>("assistant-id") {
        match openai::generate_description_through_assistant(assistant_id, &diff, &config.template)
            .await
        {
            Ok(description) => {
                loader.stop_with_message("✅ Done\n".into());
                println!("{}", description);
            }
            Err(e) => {
                println!("{}", e);
                process::exit(1);
            }
        }
    } else {
        match openai::generate_description(&config.prompt, &diff, &config.template, model).await {
            Ok(description) => {
                loader.stop_with_message("✅ Done\n".into());
                println!("{}", description);
            }
            Err(e) => {
                println!("{}", e);
                process::exit(1);
            }
        }
    }
}
