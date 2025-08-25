use crate::cli::{ConfigArgs, ConfigCommand};
use anyhow::Result;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::path::{PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ConfigData {
    pub rpc_url: String,
    pub keypair_path: String,
}

impl ConfigData {
    fn default_values() -> Self {
        Self {
            rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
            keypair_path: dirs::home_dir()
                .map(|p| p.join(".config/solana/id.json").to_string_lossy().to_string())
                .unwrap_or_else(|| "".to_string()),
        }
    }

    fn path() -> Result<PathBuf> {
        let home_dir = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        let config_dir = home_dir.join(".config").join("spl-forge");
        std::fs::create_dir_all(&config_dir)?;
        Ok(config_dir.join("config.json"))
    }

    fn load() -> Result<Self> {
        let path = Self::path()?;
        if path.exists() {
            let config_data = std::fs::read_to_string(path)?;
            Ok(serde_json::from_str(&config_data)?)
        } else {
            Ok(Self::default_values())
        }
    }

    fn save(&self) -> Result<()> {
        let path = Self::path()?;
        let config_data = serde_json::to_string_pretty(self)?;
        std::fs::write(path, config_data)?;
        Ok(())
    }
}

pub async fn handle_config(args: ConfigArgs) -> Result<()> {
    match args.command {
        ConfigCommand::Get => handle_get().await,
        ConfigCommand::Reset => handle_reset().await,
        ConfigCommand::Set { url, keypair } => handle_set(url, keypair).await,
    }
}

async fn handle_get() -> Result<()> {
    let config = ConfigData::load()?;
    println!();
    println!("{}", "Current Configuration:".bold().yellow());
    println!("  {:<12} {}", "RPC URL:".cyan(), config.rpc_url);
    println!("  {:<12} {}", "Keypair Path:".cyan(), config.keypair_path);
    println!();
    Ok(())
}

async fn handle_reset() -> Result<()> {
    let config = ConfigData::default_values();
    config.save()?;
    println!();
    println!("{}", "Configuration has been reset to default values.".green());
    handle_get().await?; // Show the newly reset config
    Ok(())
}

async fn handle_set(url: Option<String>, keypair: Option<PathBuf>) -> Result<()> {
    if url.is_none() && keypair.is_none() {
        println!();
        println!("{}", "No values provided to set. Use --url or --keypair.".yellow());
        println!("Example: spl-forge config set --url https://api.devnet.solana.com");
        println!();
        return Ok(());
    }

    let mut config = ConfigData::load()?;

    if let Some(url) = url {
        config.rpc_url = url;
    }

    if let Some(keypair) = keypair {
        config.keypair_path = keypair.to_string_lossy().to_string();
    }

    config.save()?;
    println!();
    println!("{}", "Configuration saved successfully.".green());
    handle_get().await?;
    Ok(())
}