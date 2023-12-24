use async_openai::{
    error::{ApiError, OpenAIError},
    types::{
        CreateMessageRequestArgs, CreateRunRequestArgs, CreateThreadRequestArgs, MessageContent,
        RunStatus,
    },
    Client,
};
use reqwest::Error;
use serde::Deserialize;
use std::process;
use tokio::time::sleep;

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

fn generate_user_message(diff: &str, template: &str) -> String {
    format!(
        r#"Use the following template to write your description, don't deviate from the template:
        {}

        The diff:
        ```diff
        {}
        ```
        "#,
        template,
        prepare_diff(diff),
    )
}

async fn get_assistant_response(
    assistant_id: &str,
    diff: &str,
    template: &str,
) -> Result<String, OpenAIError> {
    let client = Client::new();
    let thread_request = CreateThreadRequestArgs::default().build()?;
    let thread = client.threads().create(thread_request.clone()).await?;

    let user_message = generate_user_message(diff, template);
    let message = CreateMessageRequestArgs::default()
        .role("user")
        .content(user_message)
        .build()?;

    let _message_obj = client
        .threads()
        .messages(&thread.id)
        .create(message)
        .await?;

    let run_request = CreateRunRequestArgs::default()
        .assistant_id(assistant_id)
        .build()?;

    let run = client
        .threads()
        .runs(&thread.id)
        .create(run_request)
        .await?;

    loop {
        let run = client.threads().runs(&thread.id).retrieve(&run.id).await?;

        if run.status == RunStatus::Cancelled {
            break Err(OpenAIError::InvalidArgument("Run was cancelled".into()));
        } else if run.status == RunStatus::Cancelling {
            break Err(OpenAIError::InvalidArgument(
                "Run is in the process of being cancelled".into(),
            ));
        } else if run.status == RunStatus::Failed {
            break Err(OpenAIError::ApiError(ApiError {
                message: "Run has failed".into(),
                r#type: Some("run_failed".into()),
                param: None,
                code: None,
            }));
        } else if run.status == RunStatus::Expired {
            break Err(OpenAIError::InvalidArgument("Run has expired".into()));
        }

        match run.status {
            RunStatus::Completed => {
                let query = [("limit", "1")];
                let response = client.threads().messages(&thread.id).list(&query).await?;
                let message_id = response.data.get(0).unwrap().id.clone();
                let message = client
                    .threads()
                    .messages(&thread.id)
                    .retrieve(&message_id)
                    .await?;

                let content = message.content.get(0).unwrap();
                let text = match content {
                    MessageContent::Text(text) => text.text.value.clone(),
                    MessageContent::ImageFile(_) => {
                        panic!("images are not supported in the terminal")
                    }
                };

                return Ok(text);
            }
            _ => {
                // Wait for 1 second before checking the status again
                sleep(std::time::Duration::from_secs(1)).await;
            }
        }
    }
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
    prompt: &str,
    diff: &str,
    template: &str,
    model: &str,
) -> Result<String, Error> {
    let system_message = generate_system_message_for_diff(prompt, template);
    let body = create_payload(model, &system_message, &prepare_diff(diff));

    get_chat_completion(&body).await
}

/// Generate a PR description using the user specified assistant
pub async fn generate_description_through_assistant(
    assistant_id: &str,
    diff: &str,
    template: &str,
) -> Result<String, OpenAIError> {
    get_assistant_response(assistant_id, diff, template).await
}
