use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, disable_help_subcommand = true)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    Help,
    Config(ConfigArgs),
}

#[derive(Parser, Debug)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommand,
}

#[derive(clap::Subcommand, Debug)]
pub enum ConfigCommand {
    Get,
    Reset,
    Set {
        #[arg(long)]
        url: Option<String>,
        #[arg(long)]
        keypair: Option<PathBuf>,
    }
}