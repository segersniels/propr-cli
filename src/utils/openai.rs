use reqwest::Error;
use serde::Deserialize;
use std::process;

pub const ALLOWED_MODELS: [&str; 6] = [
    "gpt-3.5-turbo",
    "gpt-3.5-turbo-16k",
    "gpt-3.5-turbo-1106",
    "gpt-4",
    "gpt-4-32k",
    "gpt-4-1106-preview",
];

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

fn generate_system_message_for_diff(system_message: &str, template: &str) -> String {
    format!(
        r#"{}

        Follow this exact template to write your description:
        """
        {}
        """
        "#,
        system_message, template,
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

fn create_payload(model: &str, system_message: &str, content: &str) -> serde_json::Value {
    serde_json::json!({
        "model": model,
        "messages": [
            { "role": "system", "content": system_message },
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
async fn get_chat_completion(body: &serde_json::Value) -> Result<String, Error> {
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
        .body(serde_json::to_string(body).unwrap())
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
    let system_message = "Generate a concise PR title from the provided description prefixed with a suitable gitmoji";

    /*
    Generate the title using gpt-3.5-turbo since it is the fastest model
    and we don't want to spend too many tokens on this
    */
    let body = create_payload("gpt-3.5-turbo-1106", system_message, description);

    get_chat_completion(&body).await
}

/// Generate a concise PR description
pub async fn generate_description(
    system_message: &str,
    diff: &str,
    template: &str,
    model: &str,
) -> Result<String, Error> {
    let system_message = generate_system_message_for_diff(system_message, template);
    let body = create_payload(model, &system_message, &prepare_diff(diff));

    get_chat_completion(&body).await
}
