use std::process;

use clap::ArgMatches;
use octocrab::Octocrab;

use crate::utils::{config, github, loader, openai, prompt};

/// Depending on `config.generate_title` we either ask the user for a title or generate one
async fn get_title(body: &str) -> String {
    let config = config::load();

    if config.generate_title {
        openai::generate_title(body).await.unwrap_or_else(|err| {
            println!("{}", err);
            process::exit(1);
        })
    } else {
        prompt::ask_with_input("Provide a title for your pull request")
    }
}

pub async fn run(sub_matches: &ArgMatches) {
    let token = std::env::var("GITHUB_TOKEN").unwrap_or_else(|_| {
        println!("Error: GITHUB_TOKEN environment variable not set");
        process::exit(0);
    });

    let config = config::load();
    let octocrab = Octocrab::builder()
        .personal_token(token)
        .build()
        .unwrap_or_else(|err| {
            println!("{}", err);
            process::exit(1);
        });

    let head = github::get_current_branch();
    let default_branch = github::get_default_branch();
    let base = sub_matches
        .get_one::<String>("branch")
        .unwrap_or(&default_branch);

    let (repo_name, owner) = github::get_repo_name_and_owner();
    let diff = github::get_diff(base);

    if diff.is_empty() {
        println!("Error: No diff found");
        process::exit(0);
    }

    let mut body = String::new();
    while body.is_empty() {
        let mut loader = loader::create_loader("Generating");
        let model = sub_matches
            .get_one::<String>("model")
            .unwrap_or(&config.model);
        let description =
            openai::generate_description(&config.prompt, &diff, &config.template, model)
                .await
                .unwrap_or_else(|err| {
                    println!("{}", err);
                    process::exit(1);
                });
        loader.stop_with_message("✅ Done\n".into());

        // Print the description
        println!("{}\n", description);

        // Ask if the user wants to keep the description
        let should_keep = prompt::ask_for_confirmation("Keep this description?");
        if should_keep {
            body = description;
            break;
        }
    }

    let title = get_title(&body).await;
    let result = octocrab
        .pulls(owner, repo_name)
        .create(&title, &head, base)
        .body(&body)
        .send()
        .await;

    match result {
        Ok(pull_request) => {
            println!("Pull request created at {}", pull_request.html_url.unwrap());
        }
        Err(err) => match err {
            octocrab::Error::GitHub { source, .. } => {
                match source.errors {
                    Some(errors) => {
                        let message = errors[0]["message"].as_str().unwrap_or_else(|| {
                            println!("{:?}", errors);
                            process::exit(1);
                        });

                        println!("{}", message);
                    }
                    None => {
                        println!("{}", source);
                    }
                }
                process::exit(1);
            }
            _ => {
                println!("{}", err);
                process::exit(1);
            }
        },
    }
}
