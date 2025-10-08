use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub(crate) struct InfoCommand {
    #[command(subcommand)]
    pub(crate) command: InfoCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum InfoCommands {
    /// Display info of an asset
    Asset { id: u64 },

    /// Display info of a user
    User { id: u64 },

    /// Display info of a group
    Group { id: u64 },

    /// Display info of game
    Game { id: u64 },

    /// Display info of a badge
    Badge { id: u64 },

    /// Display info of a gamepass
    Gamepass { id: u64 },
    //DeveloperProduct
    //Subscription
}
