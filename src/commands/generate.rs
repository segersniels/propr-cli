use std::process;

use crate::utils::{config, github, loader, openai};

pub async fn run() {
    let diff = github::get_diff();
    if diff.is_empty() {
        println!("No diff found");
        return;
    }

    let mut loader = loader::create_loader("Generating");
    let template = config::get_template();
    match openai::generate_description(&diff, &template).await {
        Ok(description) => {
            loader.stop_with_message("✅ Generated description\n".into());
            println!("{}", description);
        }
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    }
}
