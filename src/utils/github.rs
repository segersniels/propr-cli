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

fn get_merge_base_commit_id(branch: &str) -> String {
    let merge_output = execute_command("git", &["merge-base", "origin/HEAD", branch]);

    String::from_utf8(merge_output.stdout)
        .unwrap()
        .trim()
        .to_owned()
}

pub fn get_base_branch(branch: &str) -> String {
    let commit_id = get_merge_base_commit_id(branch);
    let output = execute_command("git", &["branch", "--contains", &commit_id]);

    match String::from_utf8(output.stdout) {
        Ok(base_branch) => base_branch
            .lines()
            .find(|line| !line.contains('*') && !line.trim().is_empty())
            .expect("could not find base branch")
            .trim()
            .to_owned(),
        Err(_) => {
            println!("Could not determine base branch");
            process::exit(0);
        }
    }
}

pub fn get_diff() -> String {
    let output = execute_command("git", &["diff", "origin/HEAD...HEAD"]);

    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}
