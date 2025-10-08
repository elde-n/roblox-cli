use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub(crate) struct DownloadCommand {
    #[command(subcommand)]
    pub(crate) command: DownloadCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum DownloadCommands {
    /// Asset types include plugins, decals, models, mesh-parts, lua scripts, audios and videos
    Asset { id: u64 },

    Thumbnail {
        #[arg(short, long)]
        kind: String,

        id: u64,

        #[arg(short, long)]
        size: Option<String>,
    },
}
