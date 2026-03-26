use std::fs;

use crate::common::paths::path;
use anyhow::{Result, anyhow};
use serde::Deserialize;
use solana_client::rpc_client::RpcClient;

use solana_sdk::{
    program_pack::Pack,
    pubkey::Pubkey,
    signer::{
        Signer,
        keypair::{Keypair, read_keypair_file},
    },
    system_instruction,
    transaction::Transaction,
};

use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use spl_token::state::Mint;

#[derive(Deserialize)]
struct AppConfig {
    json_rpc_url: String,
    keypair_path: String,
}

pub struct SplForgeClient {
    pub client: RpcClient,
    pub signer: Keypair,
}

impl SplForgeClient {
    pub fn new() -> Result<Self> {
        let config_path = path().map_err(|e| anyhow!("Failed to get config path: {}", e))?;
        let config_contents = fs::read_to_string(&config_path)
            .map_err(|e| anyhow!("Failed to read config file at {:?}: {}", config_path, e))?;
        let app_config: AppConfig = serde_json::from_str(&config_contents)
            .map_err(|e| anyhow!("Failed to parse config file: {}", e))?;
        let client = RpcClient::new(app_config.json_rpc_url);
        let signer = read_keypair_file(&app_config.keypair_path).map_err(|e| {
            anyhow!(
                "Failed to read keypair file at {}: {}",
                app_config.keypair_path,
                e
            )
        })?;
        Ok(Self { client, signer })
    }

    pub fn create_mint_account(
        &self,
        decimals: u8,
        mint_authority: Pubkey,
        freeze_authority: Option<Pubkey>,
    ) -> Result<Pubkey> {
        let mint_account = Keypair::new();
        println!("Creating mint account: {}", mint_account.pubkey());
        // SPL token mints must use Pack length, not Rust struct size.
        let mint_size = Mint::LEN;
        println!("Mint account size: {}", mint_size);
        let rent = self
            .client
            .get_minimum_balance_for_rent_exemption(mint_size)?;
        println!("Rent exemption amount: {}", rent);

        let create_account_ix = system_instruction::create_account(
            &self.signer.pubkey(),
            &mint_account.pubkey(),
            rent,
            mint_size as u64,
            &spl_token::id(),
        );

        let initialize_mint_ix = spl_token::instruction::initialize_mint(
            &spl_token::id(),
            &mint_account.pubkey(),
            &mint_authority,
            freeze_authority.as_ref(),
            decimals,
        )?;

        let recent_blockhash = self.client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &[create_account_ix, initialize_mint_ix],
            Some(&self.signer.pubkey()),
            &[&self.signer, &mint_account],
            recent_blockhash,
        );
        self.client.send_and_confirm_transaction(&transaction)?;
        Ok(mint_account.pubkey())
    }

    pub fn create_metadata_account(
        &self,
        mint: Pubkey,
        name: String,
        symbol: String,
        uri: String,
        is_mutable: bool,
    ) -> Result<()> {
        let _ = (mint, name, symbol, uri, is_mutable);
        Err(anyhow!(
            "metadata creation is temporarily disabled while dependency versions are being stabilized"
        ))
    }

    pub fn mint_to(&self, mint: Pubkey, amount: u64) -> Result<()> {
        let signer_pubkey = self.signer.pubkey();
        let destination_ata = get_associated_token_address(&signer_pubkey, &mint);
        let mut instructions = vec![];

        // Check if ATA exists
        if self.client.get_account(&destination_ata).is_err() {
            instructions.push(create_associated_token_account(
                &signer_pubkey,
                &signer_pubkey,
                &mint,
                &spl_token::id(),
            ));
        }

        // Get mint info to calculate scaled amount
        let mint_account = self.client.get_account(&mint)?;
        let mint_data = Mint::unpack(&mint_account.data)?;
        let scaled_amount = amount * 10_u64.pow(mint_data.decimals as u32);

        instructions.push(spl_token::instruction::mint_to(
            &spl_token::id(),
            &mint,
            &destination_ata,
            &signer_pubkey,
            &[],
            scaled_amount,
        )?);

        let recent_blockhash = self.client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &instructions,
            Some(&signer_pubkey),
            &[&self.signer],
            recent_blockhash,
        );
        self.client.send_and_confirm_transaction(&transaction)?;
        Ok(())
    }

    pub fn revoke_mint_authority(&self, mint: Pubkey) -> Result<()> {
        let signer_pubkey = self.signer.pubkey();
        let revoke_ix = spl_token::instruction::set_authority(
            &spl_token::id(),
            &mint,
            None,
            spl_token::instruction::AuthorityType::MintTokens,
            &signer_pubkey,
            &[&signer_pubkey],
        )?;

        let recent_blockhash = self.client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &[revoke_ix],
            Some(&signer_pubkey),
            &[&self.signer],
            recent_blockhash,
        );
        self.client.send_and_confirm_transaction(&transaction)?;
        Ok(())
    }
}
