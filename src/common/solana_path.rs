use std::path::PathBuf;

pub fn get_solana_keypair_path() -> PathBuf {
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    home_dir.join(".config/solana/id.json")
}