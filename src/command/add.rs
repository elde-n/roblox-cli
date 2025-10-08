use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub(crate) struct AddCommand {
    #[command(subcommand)]
    pub(crate) command: AddCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum AddCommands {
    /// Add an account from a cookie
    Account { name: String, cookie: String },
}
