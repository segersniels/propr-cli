use std::process;

use octocrab::Octocrab;

use crate::utils::{config, github, loader, openai, prompt};

pub async fn run() {
    let token = std::env::var("GITHUB_TOKEN").unwrap_or_else(|_| {
        println!("Error: GITHUB_TOKEN environment variable not set");
        process::exit(0);
    });

    let octocrab = Octocrab::builder()
        .personal_token(token)
        .build()
        .unwrap_or_else(|err| {
            println!("{}", err);
            process::exit(1);
        });

    let head = github::get_current_branch();
    let base = github::get_default_branch();
    let (repo_name, owner) = github::get_repo_name_and_owner();
    let diff = github::get_diff();

    if diff.is_empty() {
        println!("Error: No diff found");
        process::exit(0);
    }

    let mut body = String::new();
    while body.is_empty() {
        let mut loader = loader::create_loader("Generating");
        let config = config::load();
        let description = openai::generate_description(&diff, &config.template, &config.model)
            .await
            .unwrap_or_else(|err| {
                println!("{}", err);
                process::exit(1);
            });
        loader.stop_with_message("✅ Generated description\n".into());

        // Print the description
        println!("{}\n", description);

        // Ask if the user wants to keep the description
        let should_keep = prompt::ask_for_confirmation("Keep this description?");
        if should_keep {
            body = description;
            break;
        }
    }

    let title = openai::generate_title(&body).await.unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    let result = octocrab
        .pulls(owner, repo_name)
        .create(&title, &head, &base)
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
