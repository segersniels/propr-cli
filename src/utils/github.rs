use std::process::{self, Command};

fn execute_command(cmd: &str, args: &[&str]) -> std::process::Output {
    Command::new(cmd)
        .args(args)
        .output()
        .expect("Failed to execute command")
}

fn fetch_remote_origin() -> (String, String) {
    let output = execute_command("git", &["config", "--get", "remote.origin.url"]);
    match String::from_utf8(output.stdout) {
        Ok(remote_origin) => {
            let mut split = remote_origin.split('/');
            let repo_name = split
                .next_back()
                .unwrap()
                .trim()
                .trim_end_matches(".git")
                .to_string();
            let owner = split.next_back().unwrap().to_string();

            (repo_name, owner)
        }
        Err(_) => {
            println!("Could not fetch remote origin");
            process::exit(0);
        }
    }
}

pub fn get_repo_name_and_owner() -> (String, String) {
    let (repo_name, owner) = fetch_remote_origin();

    (repo_name, owner)
}

pub fn get_current_branch() -> String {
    let output = execute_command("git", &["branch", "--show-current"]);
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

pub fn get_default_branch() -> String {
    let output = execute_command("git", &["remote", "show", "origin"]);
    let stdout = String::from_utf8(output.stdout).expect("stdout is not valid UTF-8");

    stdout
        .lines()
        .find(|line| line.starts_with("  HEAD branch:"))
        .and_then(|line| line.split(": ").nth(1))
        .expect("could not find default branch")
        .to_string()
}

pub fn get_diff(base_branch: &str) -> String {
    let output = execute_command("git", &["diff", base_branch]);

    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}
