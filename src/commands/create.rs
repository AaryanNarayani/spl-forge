use crate::cli::{CreateArgs, CreateCommand};
use crate::client::SplForgeClient;
use crate::common::theme;
use anyhow::{Context, Result};
use solana_sdk::{pubkey::Pubkey, signer::Signer};
use std::str::FromStr;

/// Main handler for all `create` subcommands.
pub async fn handle_create(args: CreateArgs) -> Result<()> {
    let client = SplForgeClient::new().context("Failed to initialize client from config")?;

    match args.command {
        CreateCommand::Mint {
            mint_authority,
            freeze_authority,
            decimals,
            initial_supply,
        } => {
            let mint_authority =
                Pubkey::from_str(&mint_authority).context("Invalid --mint-authority public key")?;
            let freeze_authority = freeze_authority
                .map(|value| Pubkey::from_str(&value).context("Invalid --freeze-authority public key"))
                .transpose()?;

            println!("{}", theme::action("Creating mint..."));
            let mint_pubkey = client
                .create_mint_account(decimals, mint_authority, freeze_authority)
                .context("Failed to create mint account")?;

            println!("{} {}", theme::success("Mint Address:"), mint_pubkey);

            if initial_supply > 0 {
                if mint_authority != client.signer.pubkey() {
                    println!(
                        "{}",
                        theme::warning(
                            "Initial supply not minted because signer is not the mint authority."
                        )
                    );
                    println!(
                        "{}",
                        theme::warning(
                            "Use the mint authority keypair in config to mint initial supply."
                        )
                    );
                } else {
                    client
                        .mint_to(mint_pubkey, initial_supply)
                        .context("Failed to mint initial supply")?;
                    println!(
                        "{} {}",
                        theme::success("Minted initial supply:"),
                        initial_supply
                    );
                }
            }

            Ok(())
        }
        CreateCommand::Metadata {
            mint_address,
            name,
            symbol,
            uri,
            immutable,
        } => {
            let mint =
                Pubkey::from_str(&mint_address).context("Invalid --mint-address public key")?;

            println!("{}", theme::action("Creating metadata..."));
            client
                .create_metadata_account(mint, name, symbol, uri, !immutable)
                .context("Failed to create metadata account")?;

            println!("{} {}", theme::success("Metadata created for mint:"), mint);
            Ok(())
        }
        CreateCommand::Token {
            name,
            symbol,
            decimals,
            uri,
            initial_supply,
            freeze_authority,
            immutable,
        } => {
            let freeze_authority = freeze_authority
                .map(|value| Pubkey::from_str(&value).context("Invalid --freeze-authority public key"))
                .transpose()?;

            println!("{}", theme::action("[1/3] Creating token mint..."));
            let mint_pubkey = client
                .create_mint_account(decimals, client.signer.pubkey(), freeze_authority)
                .context("Failed to create token mint")?;

            println!("{} {}", theme::success("Mint Address:"), mint_pubkey);

            println!("{}", theme::action("[2/3] Creating token metadata..."));
            client
                .create_metadata_account(mint_pubkey, name, symbol, uri, !immutable)
                .context("Failed to create token metadata")?;

            if initial_supply > 0 {
                println!("{}", theme::action("[3/3] Minting initial supply..."));
                client
                    .mint_to(mint_pubkey, initial_supply)
                    .context("Failed to mint initial token supply")?;
                println!(
                    "{} {} {}",
                    theme::success("Minted"),
                    initial_supply,
                    theme::success("tokens to signer wallet.")
                );
            } else {
                println!(
                    "{}",
                    theme::warning(
                        "[3/3] Skipping initial supply mint because --initial-supply is 0."
                    )
                );
            }

            println!("{}", theme::success("Token creation complete."));
            Ok(())
        }
        CreateCommand::Nft {
            name,
            symbol,
            uri,
            immutable,
            freeze_authority,
            collection_mint,
        } => {
            let freeze_authority = freeze_authority
                .map(|value| Pubkey::from_str(&value).context("Invalid --freeze-authority public key"))
                .transpose()?;

            if let Some(collection) = collection_mint {
                println!(
                    "{} {}",
                    theme::warning("Collection mint support is not implemented yet. Ignoring:"),
                    collection
                );
            }

            println!("{}", theme::action("[1/4] Creating NFT mint..."));
            let mint_pubkey = client
                .create_mint_account(0, client.signer.pubkey(), freeze_authority)
                .context("Failed to create NFT mint")?;

            println!("{} {}", theme::success("NFT Mint Address:"), mint_pubkey);

            println!("{}", theme::action("[2/4] Creating NFT metadata..."));
            client
                .create_metadata_account(mint_pubkey, name, symbol, uri, !immutable)
                .context("Failed to create NFT metadata")?;

            println!("{}", theme::action("[3/4] Minting NFT to signer wallet..."));
            client
                .mint_to(mint_pubkey, 1)
                .context("Failed to mint NFT")?;

            println!("{}", theme::action("[4/4] Revoking mint authority..."));
            client
                .revoke_mint_authority(mint_pubkey)
                .context("Failed to revoke NFT mint authority")?;

            println!("{}", theme::success("NFT creation complete."));
            Ok(())
        }
        _ => {
            println!(
                "{}",
                theme::warning(
                    "This create subcommand is not implemented yet. Market, pool, and launch are coming later."
                )
            );
            Ok(())
        }
    }
}
