use reqwest::Error;
use serde::Deserialize;
use std::process;

pub const ALLOWED_MODELS: [&str; 4] = ["gpt-3.5-turbo", "gpt-3.5-turbo-16k", "gpt-4", "gpt-4-32k"];
const MAX_TOKEN_LENGTH: i32 = 500;
const FILES_TO_IGNORE: [&str; 11] = [
    "package-lock.json",
    "yarn.lock",
    "npm-debug.log",
    "yarn-debug.log",
    "yarn-error.log",
    ".pnpm-debug.log",
    "Cargo.lock",
    "Gemfile.lock",
    "mix.lock",
    "Pipfile.lock",
    "composer.lock",
];

fn split_diff_into_chunks(diff: &str) -> Vec<String> {
    diff.split("diff --git")
        .skip(1)
        .map(|chunk| chunk.trim_start().to_owned())
        .collect::<Vec<String>>()
}

fn remove_lock_files(chunks: Vec<String>) -> Vec<String> {
    chunks
        .into_iter()
        .filter(|chunk| {
            let first_line = chunk.lines().next().unwrap_or_default();
            !FILES_TO_IGNORE.iter().any(|file| first_line.contains(file))
        })
        .collect()
}

/// Split the diff in chunks and remove any lock files to save on tokens
fn prepare_diff(diff: &str) -> String {
    let chunks = split_diff_into_chunks(diff);
    remove_lock_files(chunks).join("\n")
}

fn generate_context(template: &str) -> String {
    format!("Given the git diff below, please create a concise GitHub PR description using the provided template.
        Analyze the code changes and provide a detailed yet concise explanation of the changes, their context,
        why they were made, and their potential impact. If a section from the template does not apply
        (no significant changes in that category), omit that section from your final output.
        Avoid referencing file names directly, instead focus on explaining the changes in a broader context.

        The template: \"\"\"
        {}
        \"\"\"
    ",
    template,
    )
}

#[derive(Deserialize, Debug)]
struct Message {
    content: String,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
}

#[derive(Deserialize, Debug)]
struct ResponseError {
    message: String,
    code: String,
}

#[derive(Deserialize, Debug)]
struct Response {
    choices: Option<[Choice; 1]>,
    error: Option<ResponseError>,
}

fn create_payload(model: &str, context: &str, content: &str) -> serde_json::Value {
    serde_json::json!({
        "model": model,
        "messages": [
            { "role": "system", "content": context },
            { "role": "user", "content": content }
        ],
        "temperature": 0.7,
        "top_p": 1,
        "frequency_penalty": 0,
        "presence_penalty": 0,
        "max_tokens": MAX_TOKEN_LENGTH,
    })
}

/// Perform a completion request to the OpenAI API
async fn get_chat_completion(body: String) -> Result<String, Error> {
    let api_key = std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
        println!("Error: OPENAI_API_KEY environment variable not set");
        process::exit(0);
    });

    let response = reqwest::Client::new()
        .post("https://api.openai.com/v1/chat/completions")
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", api_key),
        )
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(body)
        .send()
        .await?;

    let result: Result<String, Error> = response.text().await;
    match result {
        Ok(response) => {
            let data: Response = serde_json::from_str(response.as_str()).unwrap();

            if data.error.is_some() {
                let error = data.error.unwrap();

                match error.code.as_str() {
                    "context_length_exceeded" => {
                        println!("Error: The provided diff is too large. Try using a different model that supports a higher token count or reduce the size of the diff.");
                        std::process::exit(1);
                    }
                    _ => {
                        println!("Error: {}", error.message);
                        std::process::exit(1);
                    }
                }
            }

            if let Some(choice) = data.choices {
                Ok(choice[0].message.content.clone())
            } else {
                println!("Error: {}", response.as_str());
                std::process::exit(1);
            }
        }
        Err(_) => {
            println!("Error: Could not fetch response from OpenAI");
            std::process::exit(1);
        }
    }
}

/// Generate a concise PR title
pub async fn generate_title(description: &str) -> Result<String, Error> {
    let context = "Generate a concise PR title from the provided description prefixed with a suitable gitmoji";

    /*
    Generate the title using gpt-3.5-turbo since it is the fastest model
    and we don't want to spend too many tokens on this
    */
    let body = create_payload("gpt-3.5-turbo", context, description);

    get_chat_completion(serde_json::to_string(&body).unwrap()).await
}

/// Generate a concise PR description
pub async fn generate_description(
    diff: &str,
    template: &str,
    model: &str,
) -> Result<String, Error> {
    let context = generate_context(template);
    let body = create_payload(model, &context, &prepare_diff(diff));

    get_chat_completion(serde_json::to_string(&body).unwrap()).await
}
