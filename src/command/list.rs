use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub(crate) struct ListCommand {
    #[command(subcommand)]
    pub(crate) command: ListCommands,
}

#[derive(Debug, Args)]
pub(crate) struct ListTradeCommand {
    #[command(subcommand)]
    pub(crate) command: ListTradeCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum ListCommands {
    /// List avatar assets of `user`
    Avatar {
        #[arg(short, long)]
        user_id: Option<u64>,
    },

    /// List badges of `user` or `place`
    #[group(multiple = false)]
    Badges {
        #[arg(short, long)]
        user_id: Option<u64>,
        #[arg(short, long)]
        place_id: Option<u64>,
    },

    /// List created experiences of the `user` or `group`
    Experiences {
        #[arg(short, long)]
        user_id: Option<u64>,
        #[arg(short, long)]
        group_id: Option<u64>,
    },

    // /// List favorited assets of `user`
    // Favorites {
    //     #[arg(short, long)]
    //     kind: String,
    //     #[arg(short, long)]
    //     user_id: Option<u64>,
    // },
    /// List the users the `user` is being followed by
    Followers {
        #[arg(short, long)]
        user_id: Option<u64>,
    },

    /// List the users the `user` is following
    Followings {
        #[arg(short, long)]
        user_id: Option<u64>,
    },

    /// List the friends of the `user`
    Friends {
        #[arg(short, long)]
        user_id: Option<u64>,
    },

    /// List gamepasses of `user` or `place`
    Gamepasses {
        #[arg(short, long)]
        user_id: Option<u64>,
        #[arg(short, long)]
        place_id: Option<u64>,
    },

    /// List groups the `user` is in
    Groups {
        #[arg(short, long)]
        user_id: Option<u64>,
    },

    /// List inventory assets of `user`
    Inventory {
        #[arg(short, long)]
        kind: String,
        #[arg(short, long)]
        user_id: Option<u64>,

        #[arg(short, long)]
        verbose: bool,
        #[arg(short, long)]
        json: bool,
    },

    /// List username history of `user` or `group`
    #[group(multiple = false)]
    NameHistory {
        #[arg(short, long)]
        user_id: Option<u64>,
        #[arg(short, long)]
        group_id: Option<u64>,
    },

    /// List avatar outfits of `user`
    Outfits {
        #[arg(short, long)]
        user_id: Option<u64>,
    },

    // Authenticated account only
    /// List messages of account
    Messages,
    /// List chats of account
    Conversations,
    /// List notifications of account
    Notifications,
    /// List friend-requests of account
    FriendRequests,
    /// List trades of account
    Trades(ListTradeCommand),
}

#[derive(Debug, Subcommand)]
pub(crate) enum ListTradeCommands {
    /// List inbound trades
    Inbound,
    /// List outbound trades
    Outbound,
    /// List completed trades
    Completed,
    /// List inactive trades
    Inactive,
}
