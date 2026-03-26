use crate::common::theme;

pub struct Help;

impl Help {
    pub async fn help_log() -> anyhow::Result<()> {
        let commands = [
            ("config", "Securely connect your wallet and set defaults."),
            ("create", "Forge new SPL tokens, markets, and liquidity pools."),
            ("wallet", "Inspect wallet details and request local/devnet airdrops."),
            ("help", "Show this help message."),
        ];

        println!();
        println!("{}", theme::app_name("SPL-FORGE"));
        println!(
            "   {}",
            theme::muted("Forge on-chain assets. The essential CLI for creating, analyzing, and deploying SPL tokens.")
        );
        println!();

        println!("{}", theme::heading("Commands:"));
        for (command, description) in commands {
            let formatted_line = format!(
                "  {:<12} {}",
                theme::command(command),
                theme::muted(description)
            );
            println!("{}", formatted_line);
        }
        println!();

        Ok(())
    }
}