use crate::cli::{WalletArgs, WalletCommand};
use crate::client::SplForgeClient;
use crate::commands::config::ConfigData;
use crate::common::theme;
use anyhow::{Context, Result};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

const LAMPORTS_PER_SOL: f64 = 1_000_000_000.0;

pub async fn handle_wallet(args: WalletArgs) -> Result<()> {
    let client = SplForgeClient::new().context("Failed to initialize client from config")?;

    match args.command {
        WalletCommand::Address => {
            println!("{} {}", theme::label("Address:"), client.signer.pubkey());
            Ok(())
        }
        WalletCommand::Balance { address } => {
            let address = match address {
                Some(value) => Pubkey::from_str(&value).context("Invalid --address public key")?,
                None => client.signer.pubkey(),
            };

            let lamports = client
                .client
                .get_balance(&address)
                .context("Failed to fetch wallet balance")?;
            let sol = lamports as f64 / LAMPORTS_PER_SOL;

            println!("{} {}", theme::label("Address:"), address);
            println!("{} {:.9} SOL", theme::label("Balance:"), sol);
            Ok(())
        }
        WalletCommand::Status => {
            let config = ConfigData::load().context("Failed to load config")?;
            let address = client.signer.pubkey();
            let lamports = client
                .client
                .get_balance(&address)
                .context("Failed to fetch wallet balance")?;
            let sol = lamports as f64 / LAMPORTS_PER_SOL;

            println!();
            println!("{}", theme::heading("Wallet Status"));
            println!("  {:<12} {}", theme::label("Address:"), address);
            println!("  {:<12} {:.9} SOL", theme::label("Balance:"), sol);
            println!("  {:<12} {}", theme::label("RPC URL:"), config.json_rpc_url);
            println!("  {:<12} {}", theme::label("Commitment:"), config.commitment);
            println!();

            Ok(())
        }
        WalletCommand::Airdrop { amount } => {
            let config = ConfigData::load().context("Failed to load config")?;
            let rpc = config.json_rpc_url.to_ascii_lowercase();

            let allowed = rpc.contains("devnet")
                || rpc.contains("127.0.0.1")
                || rpc.contains("localhost");

            if !allowed {
                anyhow::bail!(
                    "Airdrop is only allowed on devnet or localnet. Current RPC: {}",
                    config.json_rpc_url
                );
            }

            if amount <= 0.0 {
                anyhow::bail!("Airdrop amount must be greater than 0");
            }

            let lamports = (amount * LAMPORTS_PER_SOL).round() as u64;
            let address = client.signer.pubkey();
            let before = client
                .client
                .get_balance(&address)
                .context("Failed to fetch pre-airdrop balance")?;

            println!(
                "{} {:.9} SOL {}",
                theme::action("Requesting airdrop of"),
                amount,
                theme::muted("to active wallet...")
            );

            let sig = client
                .client
                .request_airdrop(&address, lamports)
                .context("Airdrop request failed")?;

            let mut after = before;
            for _ in 0..30 {
                let _ = client.client.get_signature_status(&sig);
                after = client.client.get_balance(&address).unwrap_or(before);
                if after >= before + lamports {
                    break;
                }
                thread::sleep(Duration::from_millis(500));
            }

            if after < before + lamports {
                anyhow::bail!(
                    "Airdrop did not finalize in time. Requested {:.9} SOL",
                    amount
                );
            }

            println!("{} {}", theme::success("Airdrop signature:"), sig);
            println!(
                "{} {:.9} SOL",
                theme::success("New balance:"),
                (after as f64) / LAMPORTS_PER_SOL
            );
            Ok(())
        }
    }
}
