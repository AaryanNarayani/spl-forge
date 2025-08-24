use colored::Colorize;

pub struct Help;

impl Help {
    pub async fn help_log() -> anyhow::Result<()> {
        let commands = [
            ("config", "Securely connect your wallet and set defaults."),
            ("create", "Forge new SPL tokens, markets, and liquidity pools."),
            ("analyze", "Run a security audit on any SPL token."),
            ("transfer", "Send SPL tokens to another wallet."),
            ("manage", "Manage your project, including airdrops."),
            ("watch", "Watch a wallet or token for real-time activity."),
            ("help", "Show this help message."),
        ];

        println!();
        println!("{}", "SPL-FORGE".bold().yellow());
        println!("   {}", "Forge on-chain assets. The essential CLI for creating, analyzing, and deploying SPL tokens.");
        println!();

        println!("{}", "Commands:".bold().yellow());
        for (command, description) in commands {
            let formatted_line = format!(
                "  {:<12} {}",
                command.bright_red(),
                description.white()
            );
            println!("{}", formatted_line);
        }
        println!();

        Ok(())
    }
}