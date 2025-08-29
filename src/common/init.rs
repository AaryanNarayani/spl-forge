use crate::{commands::config::ConfigData, common::paths::path};
use anyhow::Result;
use colored::Colorize;
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
    println!("{}", "Welcome to spl-forge! Performing first-time setup...".bold().yellow());
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
        keypair_path.to_string_lossy().cyan()
    );
    println!(
        "   Your new public key is: {}",
        keypair.pubkey().to_string().bold()
    );

    config.save()?;
    println!(
        "Default configuration file created at: {}",
        config_path.to_string_lossy().cyan()
    );

    println!();
    println!("{}", "Tip: To use your existing Solana CLI wallet, run:".italic());
    println!("{}", "spl-forge config set --keypair solana-cli".green());
    println!();

    Ok(())
}