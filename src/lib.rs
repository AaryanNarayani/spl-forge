pub mod cli;
pub mod commands;
pub mod common;

use cli::{Args, Command};

use crate::common::init;

pub async fn run(args: Args) -> anyhow::Result<()> {
    init::ensure_config_exists()?;
    match args.command {
        Command::Help => commands::help::Help::help_log().await?,
        Command::Config(args) => commands::config::handle_config(args).await?,
        Command::Create(args) => commands::create::handle_create(args),
    }
    Ok(())
}