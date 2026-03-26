use crate::{commands::config::ConfigData, common::paths::path};
use crate::common::theme;
use anyhow::Result;
use solana_sdk::signer::{keypair::Keypair, Signer};
use std::path::PathBuf;

pub fn ensure_config_exists() -> Result<()> {
    let config_path = path()?;
    if !config_path.exists() {
        run_first_time_setup(&config_path)?;
    }
    Ok(())
}

fn run_first_time_setup(config_path: &PathBuf) -> Result<()> {
    println!();
    println!("{}", theme::heading("Welcome to spl-forge! Performing first-time setup..."));
    println!();

    let config = ConfigData::default_values();
    let keypair = Keypair::new();
    let keypair_path = PathBuf::from(&config.keypair_path);

    if let Some(parent) = keypair_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    solana_sdk::signer::keypair::write_keypair_file(
        &keypair,
        &keypair_path,
    )
    .map_err(|e| anyhow::anyhow!("Failed to write new keypair file: {}", e))?;

    println!(
        "A new keypair has been created for you at: {}",
        theme::label(&keypair_path.to_string_lossy())
    );
    println!(
        "   Your new public key is: {}",
        theme::success(&keypair.pubkey().to_string())
    );

    config.save()?;
    println!(
        "Default configuration file created at: {}",
        theme::label(&config_path.to_string_lossy())
    );

    println!();
    println!("{}", theme::tip("Tip: To use your existing Solana CLI wallet, run:"));
    println!("{}", theme::command("spl-forge config set --keypair solana-cli"));
    println!();

    Ok(())
}