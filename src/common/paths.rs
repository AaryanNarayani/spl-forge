use std::path::PathBuf;

use anyhow::Result;

pub fn get_solana_keypair_path() -> PathBuf {
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    home_dir.join(".config/solana/id.json")
}

pub fn path() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    let config_dir = home_dir.join(".config").join("spl-forge");
    std::fs::create_dir_all(&config_dir)?;
    Ok(config_dir.join("config.json"))
}