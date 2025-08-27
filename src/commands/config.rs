use crate::cli::{ConfigArgs, ConfigCommand};
use crate::common::solana_path::get_solana_keypair_path;
use anyhow::Result;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

// --- Data Structure & File Logic (Updated & Consolidated) ---

/// This struct represents the data in our rich config file.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigData {
    pub json_rpc_url: String,
    pub websocket_url: String,
    pub keypair_path: String,
    pub commitment: String,
    pub address_labels: HashMap<String, String>,
}

impl ConfigData {
    /// Defines the default values created by the `init` command.
    /// The `reset` command also uses this.
    pub fn default_values() -> Self {
        let default_keypair_path = dirs::home_dir()
            .map(|p| p.join(".config/spl-forge/id.json").to_string_lossy().to_string())
            .unwrap_or_else(|| "spl-forge-keypair.json".to_string());

        let mut labels = HashMap::new();
        labels.insert(
            "11111111111111111111111111111111".to_string(),
            "System Program".to_string(),
        );

        Self {
            json_rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
            websocket_url: "wss://api.mainnet-beta.solana.com/".to_string(),
            keypair_path: default_keypair_path,
            commitment: "confirmed".to_string(),
            address_labels: labels,
        }
    }

    /// A helper function to get the path to the config file.
    pub fn path() -> Result<PathBuf> {
        let home_dir =
            dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        let config_dir = home_dir.join(".config").join("spl-forge");
        std::fs::create_dir_all(&config_dir)?;
        Ok(config_dir.join("config.json"))
    }

    /// Loads config from file. Assumes the file was created by the `init` command.
    pub fn load() -> Result<Self> {
        let path = Self::path()?;
        let config_data = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&config_data)?)
    }

    /// Saves the current config to the file.
    pub fn save(&self) -> Result<()> {
        let path = Self::path()?;
        let config_data = serde_json::to_string_pretty(self)?;
        std::fs::write(path, config_data)?;
        Ok(())
    }
}

// --- Command Handlers (Updated) ---

/// Main handler that dispatches to the correct function.
pub async fn handle_config(args: ConfigArgs) -> Result<()> {
    match args.command {
        ConfigCommand::Get => handle_get().await,
        ConfigCommand::Reset => handle_reset().await,
        // Note: The fields here must match your updated cli.rs definition
        ConfigCommand::Set { url, keypair, commitment } => {
            handle_set(url, keypair, commitment).await
        }
    }
}

/// Logic for the "get" command, updated for the new fields.
async fn handle_get() -> Result<()> {
    let config = ConfigData::load()?;
    println!();
    println!("{}", "Current Configuration:".bold().yellow());
    println!("  {:<15} {}", "Config Path:".cyan(), ConfigData::path()?.to_string_lossy());
    println!("  {:<15} {}", "RPC URL:".cyan(), config.json_rpc_url);
    println!("  {:<15} {}", "Websocket URL:".cyan(), config.websocket_url);
    println!("  {:<15} {}", "Keypair Path:".cyan(), config.keypair_path);
    println!("  {:<15} {}", "Commitment:".cyan(), config.commitment);
    println!();
    Ok(())
}

/// Logic for the "reset" command, uses the new default values.
async fn handle_reset() -> Result<()> {
    let config = ConfigData::default_values();
    config.save()?;
    println!();
    println!("{}", "Configuration has been reset to default values.".green());
    handle_get().await?; // Show the newly reset config
    Ok(())
}

/// Logic for the "set" command, updated for new fields.
async fn handle_set(
    url: Option<String>,
    keypair: Option<PathBuf>,
    commitment: Option<String>,
) -> Result<()> {
    if url.is_none() && keypair.is_none() && commitment.is_none() {
        println!();
        println!("{}", "No values provided to set. Use --url, --keypair, or --commitment.".yellow());
        println!("Example: spl-forge config set --url https://api.devnet.solana.com");
        println!();
        return Ok(());
    }

    let mut config = ConfigData::load()?;

    if let Some(url) = url {
        config.json_rpc_url = url;
    }

    if let Some(keypair) = keypair {
        if keypair.to_string_lossy() == "solana-cli" {
            let solana_keypair_path = get_solana_keypair_path();
            config.keypair_path = solana_keypair_path.to_string_lossy().to_string();
        } else {
            config.keypair_path = keypair.to_string_lossy().to_string();
        }
    }

    if let Some(commitment) = commitment {
        config.commitment = commitment;
    }

    config.save()?;
    println!();
    println!("{}", "Configuration saved successfully.".green());
    handle_get().await?;
    Ok(())
}