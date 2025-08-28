use crate::cli::{CreateArgs, CreateCommand};


pub fn handle_create(args: CreateArgs) {
    match args.command {
        CreateCommand::Mint { mint_authority, freeze_authority, decimals, initial_supply } => {
            handler_in_progress()
        }
        CreateCommand::Launch { name, symbol, uri, decimals, initial_supply, mint_authority, freeze_authority, initial_lp_base, initial_lp_quote, immutable, burn_lp, lock_lp_duration } => {
            handler_in_progress()
        }
        CreateCommand::Market { base_mint, quote_mint } => {
            handler_in_progress()
        }
        CreateCommand::Pool { market_id, base_amount, quote_amount } => {
            handler_in_progress()
        }
        CreateCommand::Token { name, symbol, decimals, initial_supply, uri, freeze_authority, immutable } => {
            handler_in_progress()
        }
        CreateCommand::Metadata { mint_address, name, symbol, uri, immutable } => {
            handler_in_progress()
        }
        CreateCommand::Nft { name, symbol, uri, immutable, freeze_authority, collection_mint } => {
            handler_in_progress()
        }
    }

}

pub fn handler_in_progress() {
    println!("Handlers in progress...");
}