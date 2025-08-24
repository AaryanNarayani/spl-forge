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
}