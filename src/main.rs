use clap::{self, ArgAction, Command};
use human_panic::setup_panic;

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
                    clap::Arg::new("branch")
                        .short('b')
                        .long("branch")
                        .help("The base branch to point your changes to")
                        .action(ArgAction::Set),
                ),
        )
        .subcommand(
            Command::new("generate")
                .about("Generates a PR description and outputs it")
                .arg(
                    clap::Arg::new("branch")
                        .short('b')
                        .long("branch")
                        .help("The base branch to point your changes to")
                        .action(ArgAction::Set),
                ),
        )
        .subcommand(
            Command::new("config")
                .about("Configure propr to your liking")
                .arg_required_else_help(true)
                .subcommand(Command::new("template").about("Adjust the template used by propr"))
                .subcommand(Command::new("model").about("Adjust the model used by propr"))
                .subcommand(
                    Command::new("generate-title")
                        .about("Configure whether to generate a title or not"),
                )
                .subcommand(Command::new("list").about("List the current configuration")),
        )
}

#[tokio::main]
async fn main() {
    setup_panic!();

    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("create", sub_matches)) => commands::create::run(sub_matches).await,
        Some(("generate", sub_matches)) => commands::generate::run(sub_matches).await,
        Some(("config", sub_matches)) => commands::config::run(sub_matches),
        _ => unreachable!(),
    }
}
