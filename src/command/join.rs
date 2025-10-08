use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub(crate) struct JoinCommand {
    #[command(subcommand)]
    pub(crate) command: JoinCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum JoinCommands {
    /// Joins a roblox game instance
    Game { id: u64, job_id: Option<String> },

    /// Joins or requests to join a group
    Group { id: u64 },
}
