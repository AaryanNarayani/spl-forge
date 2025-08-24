use spl_forge::cli::Args;
use spl_forge::run;
use clap::Parser;
use spl_forge::common::error::ErrorLog;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if let Err(error) = run(args).await {
        ErrorLog::error_log(&error);
        std::process::exit(1);
    }
    Ok(())
}