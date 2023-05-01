use clap::{self, arg, Arg, ArgAction, Command};

mod commands;
mod utils;

fn cli() -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
        .about("Generate your PRs from the command line with AI")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("create")
                .about("Creates a PR with a generated description")
                .arg(
                    Arg::new("gpt-4")
                        .action(ArgAction::SetTrue)
                        .long("gpt-4")
                        .help("Use gpt-4 instead of gpt-3.5-turbo model"),
                ),
        )
        .subcommand(
            Command::new("generate")
                .about("Generates a PR description and outputs it")
                .arg(
                    Arg::new("gpt-4")
                        .action(ArgAction::SetTrue)
                        .long("gpt-4")
                        .help("Use gpt-4 instead of gpt-3.5-turbo model"),
                ),
        )
        .subcommand(
            Command::new("config")
                .about("Configure propr to your liking")
                .arg_required_else_help(true)
                .subcommand(Command::new("template").about(format!(
                    "Adjust the template used by propr at `{}`",
                    utils::config::get_file_path().to_string_lossy()
                ))),
        )
}

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("create", sub_matches)) => commands::create::run(sub_matches).await,
        Some(("generate", sub_matches)) => commands::generate::run(sub_matches).await,
        Some(("config", sub_matches)) => commands::config::run(sub_matches),
        _ => unreachable!(),
    }
}
