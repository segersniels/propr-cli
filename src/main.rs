use clap::{self, Command};

mod commands;
mod utils;

fn cli() -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
        .about("Generate your PRs from the command line with AI")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("create").about("Creates a PR with a generated description"))
        .subcommand(Command::new("generate").about("Generates a PR description and outputs it"))
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
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("create", _sub_matches)) => commands::create::run().await,
        Some(("generate", _sub_matches)) => commands::generate::run().await,
        Some(("config", sub_matches)) => commands::config::run(sub_matches),
        _ => unreachable!(),
    }
}
