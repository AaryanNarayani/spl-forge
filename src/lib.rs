pub mod cli;
pub mod commands;
pub mod common;

use cli::{Args, Command};

pub async fn run(args: Args) -> anyhow::Result<()> {
    match args.command {
        Command::Help => commands::help::Help::help_log().await?,
        Command::Config(args) => commands::config::handle_config(args).await?,
    }
    Ok(())
}