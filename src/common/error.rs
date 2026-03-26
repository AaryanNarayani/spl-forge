use crate::common::theme;
pub struct ErrorLog;

impl ErrorLog {
    pub fn error_log(error: &anyhow::Error) {
        eprintln!("{} {}", theme::error("Error:"), error);

        if let Some(root_cause) = error.root_cause().to_string().lines().last() {
             if root_cause != error.to_string() {
                 eprintln!("{} {}", theme::muted("Root Cause:"), theme::muted(root_cause));
             }
        }
    }
}