use reqwest::Error;
use serde::Deserialize;
use std::process;

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

fn prepare_diff(diff: &str) -> String {
    let chunks = split_diff_into_chunks(diff);
    remove_lock_files(chunks).join("\n")
}

fn generate_prompt(diff: &str, template: &str) -> String {
    format!("Generate a concise PR description from the provided git diff according to a provided template.
        The PR description should be a good summary of the changes made.
        Do not reference each file and function added but rather give a general explanation of the changes made.
        Do not treat imports and requires as changes or new features.
        You can leave out the entire heading of the template if no applicable changes are found.

        The PR description should be structured as follows: \"\"\"
        {}
        \"\"\"

        Here is the diff: \"\"\"
        {}
        \"\"\"
        ",
        template,
        prepare_diff(diff)
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
struct Response {
    choices: [Choice; 1],
}

pub async fn generate_description(
    diff: &str,
    template: &str,
    model: &str,
) -> Result<String, Error> {
    let api_key = std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
        println!("Error: OPENAI_API_KEY environment variable not set");
        process::exit(0);
    });

    let prompt = generate_prompt(diff, template);
    let body = serde_json::json!({
        "model": model,
        "messages": [{ "role": "user", "content": prompt }],
        "temperature": 0.7,
        "top_p": 1,
        "frequency_penalty": 0,
        "presence_penalty": 0,
        "max_tokens": MAX_TOKEN_LENGTH,
    });

    let url = "https://api.openai.com/v1/chat/completions".to_string();
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", api_key),
        )
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&body).unwrap())
        .send()
        .await?;

    let result = response.text().await;
    match result {
        Ok(response) => {
            let data: Response = serde_json::from_str(response.as_str()).unwrap();

            Ok(data.choices[0].message.content.clone())
        }
        Err(_) => {
            println!("Error: Could not fetch response from OpenAI");
            std::process::exit(1);
        }
    }
}
