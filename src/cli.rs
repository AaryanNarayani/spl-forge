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
    Create(CreateArgs),
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

        #[arg(long)]
        commitment: Option<String>,
    }
}

#[derive(Parser, Debug)]
pub struct CreateArgs {
    #[command(subcommand)]
    pub command: CreateCommand,
}

#[derive(clap::Subcommand, Debug)]
pub enum CreateCommand {
    Mint {
        #[arg(long)]
        mint_authority: String,

        #[arg(long)]
        freeze_authority: Option<String>,

        #[arg(long)]
        decimals: u8,

        #[arg(long)]
        initial_supply: u64,
    },
    Metadata {
        #[arg(long)]
        mint_address: String,

        #[arg(long)]
        name: String,

        #[arg(long)]
        symbol: String,

        #[arg(long)]
        uri: String,

        #[arg(long)]
        immutable: bool,
    },
    Token {
        #[arg(long)]
        name: String,

        #[arg(long)]
        symbol: String,

        #[arg(long)]
        decimals: u8,

        #[arg(long)]
        initial_supply: u64,

        #[arg(long)]
        uri: String,

        #[arg(long)]
        freeze_authority: Option<String>,

        #[arg(long)]
        immutable: bool,
    },
    Nft {
        #[arg(long)]
        name: String,

        #[arg(long)]
        symbol: String,

        #[arg(long)]
        uri: String,

        #[arg(long)]
        immutable: bool,

        #[arg(long)]
        freeze_authority: Option<String>,

        #[arg(long)]
        collection_mint: Option<String>,
    },
    Market {
        #[arg(long)]
        base_mint: String,

        #[arg(long)]
        quote_mint: String,
    },
    Pool {
        #[arg(long)]
        market_id: String,

        #[arg(long)]
        base_amount: String,

        #[arg(long)]
        quote_amount: String,
    },
    Launch {
        #[arg(long)]
        name: String,

        #[arg(long)]
        symbol: String,

        #[arg(long)]
        image_path: String,

        #[arg(long)]
        description: Option<String>,

        #[arg(long)]
        decimals: u8,

        #[arg(long)]
        initial_supply: u64,

        #[arg(long)]
        mint_authority: Option<String>,

        #[arg(long)]
        freeze_authority: Option<String>,

        #[arg(long)]
        initial_lp_base: u64,

        #[arg(long)]
        initial_lp_quote: u64,

        #[arg(long)]
        immutable: bool,

        #[arg(long)]
        burn_lp: bool,

        #[arg(long)]
        lock_lp_duration: u64,
    }
}