use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub(crate) struct LoginCommand {
    #[command(subcommand)]
    pub(crate) command: LoginCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum LoginCommands {
    /// Create a login token to add to the config accounts list
    NewQuick,
    /// Authorize a login request from quick-login
    Authorize { code: String },
}
