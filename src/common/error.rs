use colored::Colorize;
pub struct ErrorLog;

impl ErrorLog {
    pub fn error_log(error: &anyhow::Error) {
        eprintln!("{} {}", "Error:".red().bold(), error);

        if let Some(root_cause) = error.root_cause().to_string().lines().last() {
             if root_cause != error.to_string() {
                 eprintln!("{} {}", "Root Cause:".dimmed(), root_cause.dimmed());
             }
        }
    }
}