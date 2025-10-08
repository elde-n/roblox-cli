use clap::{Parser, Subcommand};

use add::AddCommand;
use download::DownloadCommand;
use info::InfoCommand;
use join::JoinCommand;
use list::ListCommand;
use login::LoginCommand;

pub(crate) mod add;
pub(crate) mod download;
pub(crate) mod info;
pub(crate) mod join;
pub(crate) mod list;
pub(crate) mod login;

#[derive(Debug, Parser)]
#[command(version, about)]
pub(crate) struct Command {
    #[command(subcommand)]
    pub(crate) command: Commands,

    /// The account username to operate on, defaults to the first entry in the config
    #[arg(long)]
    pub(crate) account: Option<String>,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Print the status of your accounts
    Status,

    /// Add an object to an instance
    Add(AddCommand),
    /// Print the info of an object
    Info(InfoCommand),
    /// Join a specific instance (game, group, etc.)
    Join(JoinCommand),
    /// Download an asset or thumbnail (decal, audio, model, etc.)
    Download(DownloadCommand),
    /// List instances of a object
    List(ListCommand),

    /// Authorize or login into accounts
    Login(LoginCommand),
}
