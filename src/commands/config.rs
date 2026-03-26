use crate::cli::{ConfigArgs, ConfigCommand};
use crate::common::paths::{get_solana_keypair_path, path};
use crate::common::theme;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigData {
    pub json_rpc_url: String,
    pub websocket_url: String,
    pub keypair_path: String,
    pub commitment: String,
    pub address_labels: HashMap<String, String>,
}

impl ConfigData {
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

    pub fn load() -> Result<Self> {
        let path = path()?;
        let config_data = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&config_data)?)
    }

    pub fn save(&self) -> Result<()> {
        let path = path()?;
        let config_data = serde_json::to_string_pretty(self)?;
        std::fs::write(path, config_data)?;
        Ok(())
    }
}

pub async fn handle_config(args: ConfigArgs) -> Result<()> {
    match args.command {
        ConfigCommand::Get => handle_get().await,
        ConfigCommand::Reset => handle_reset().await,
        ConfigCommand::Set {
            network,
            url,
            keypair,
            commitment,
        } => {
            handle_set(network, url, keypair, commitment).await
        }
    }
}

fn apply_network_preset(config: &mut ConfigData, network: &str) -> Result<()> {
    match network.to_ascii_lowercase().as_str() {
        "devnet" => {
            config.json_rpc_url = "https://api.devnet.solana.com".to_string();
            config.websocket_url = "wss://api.devnet.solana.com/".to_string();
            config.commitment = "confirmed".to_string();
        }
        "mainnet" | "mainnet-beta" => {
            config.json_rpc_url = "https://api.mainnet-beta.solana.com".to_string();
            config.websocket_url = "wss://api.mainnet-beta.solana.com/".to_string();
            config.commitment = "confirmed".to_string();
        }
        "localhost" | "localnet" => {
            config.json_rpc_url = "http://127.0.0.1:8899".to_string();
            config.websocket_url = "ws://127.0.0.1:8900/".to_string();
            config.commitment = "confirmed".to_string();
        }
        _ => {
            anyhow::bail!(
                "Invalid network preset '{}'. Use one of: devnet, mainnet, localhost",
                network
            );
        }
    }

    Ok(())
}

async fn handle_get() -> Result<()> {
    let config = ConfigData::load()?;
    println!();
    println!("{}", theme::heading("Current Configuration:"));
    println!("  {:<15} {}", theme::label("Config Path:"), path()?.to_string_lossy());
    println!("  {:<15} {}", theme::label("RPC URL:"), config.json_rpc_url);
    println!("  {:<15} {}", theme::label("Websocket URL:"), config.websocket_url);
    println!("  {:<15} {}", theme::label("Keypair Path:"), config.keypair_path);
    println!("  {:<15} {}", theme::label("Commitment:"), config.commitment);
    println!();
    Ok(())
}

async fn handle_reset() -> Result<()> {
    let config = ConfigData::default_values();
    config.save()?;
    println!();
    println!("{}", theme::success("Configuration has been reset to default values."));
    handle_get().await?; // Show the newly reset config
    Ok(())
}

async fn handle_set(
    network: Option<String>,
    url: Option<String>,
    keypair: Option<PathBuf>,
    commitment: Option<String>,
) -> Result<()> {
    if network.is_none() && url.is_none() && keypair.is_none() && commitment.is_none() {
        println!();
        println!("{}", theme::warning("No values provided to set. Use --url, --keypair, or --commitment."));
        println!(
            "{}",
            theme::muted(
                "Examples: spl-forge config set devnet | spl-forge config set localhost | spl-forge config set --url https://api.devnet.solana.com"
            )
        );
        println!();
        return Ok(());
    }

    let mut config = ConfigData::load()?;

    if let Some(network) = network {
        apply_network_preset(&mut config, &network)?;
    }

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
    println!("{}", theme::success("Configuration saved successfully."));
    handle_get().await?;
    Ok(())
}