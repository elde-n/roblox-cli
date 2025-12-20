mod action;
mod command;
mod conclusion;
mod config;
mod object;
mod objects;

use std::ops::Not;

use clap::Parser;
use roblox_api::{
    AssetTypeId,
    api::thumbnails::v1::{ThumbnailRequestType, ThumbnailSize},
    client::{Client, Cookie},
};

use command::{
    Command, Commands, add::AddCommands, download::DownloadCommands, info::InfoCommands,
    join::JoinCommands, list::ListCommands, login::LoginCommands,
};
use config::{Account, Config};

#[tokio::main]
async fn main() {
    let cli = Command::parse();
    let mut cfg: Config = confy::load(env!("CARGO_BIN_NAME"), Some("config")).unwrap();

    let account = match cli.account {
        Some(name) => {
            let mut filter = cfg
                .accounts
                .iter()
                .filter(|x| x.name.to_lowercase() == name.to_lowercase());
            filter
                .next()
                .expect(&format!("error: account with username: {name} not found"))
        }

        _ => cfg.accounts.first().expect("error: no account entry found"),
    };

    let mut client = Client::from_cookie(Cookie::from(account.cookie.as_str()));

    match &cli.command {
        Commands::Status => {
            action::status::print(&cfg).await;
        }

        Commands::Add(add) => match &add.command {
            AddCommands::Account { name, cookie } => {
                let unique_name = cfg
                    .accounts
                    .iter()
                    .any(|x| x.name.to_lowercase() == *name.to_lowercase())
                    .not();

                if !unique_name {
                    return eprintln!("error: account with username: {name} already exists");
                }

                cfg.accounts.push(Account {
                    name: name.to_owned().to_lowercase(),
                    cookie: cookie.to_owned(),
                });

                confy::store(env!("CARGO_BIN_NAME"), Some("config"), cfg).unwrap();

                println!("info: added account: {name} to the list");
            }
        },

        Commands::Info(info) => match &info.command {
            InfoCommands::Asset { id } => {
                action::info::asset(&mut client, *id).await;
            }

            InfoCommands::User { id } => {
                action::info::user(&mut client, *id).await;
            }

            InfoCommands::Group { id } => {
                action::info::group(&mut client, *id).await;
            }

            InfoCommands::Game { id } => action::info::place(&mut client, *id).await,
            InfoCommands::Gamepass { id } => action::info::gamepass(&mut client, *id).await,

            InfoCommands::Badge { id } => action::info::badge(&mut client, *id).await,
        },

        Commands::Join(join) => match &join.command {
            JoinCommands::Game { id, job_id } => {
                action::join::game::run(account, *id, job_id.as_deref())
            }

            JoinCommands::Group { id } => action::join::group(&mut client, *id).await,
        },

        Commands::Download(download) => match &download.command {
            DownloadCommands::Asset { id } => action::download::asset(&mut client, &cfg, *id).await,

            DownloadCommands::Thumbnail { kind, id, size } => {
                let kind = ThumbnailRequestType::try_from(kind.as_str())
                    .expect("error: unknown thumnbail kind");
                let size = match size {
                    Some(size) => ThumbnailSize::try_from(size.as_str())
                        .expect("error: unknown thumbnail size"),
                    _ => ThumbnailSize::S420x420,
                };

                action::download::thumbnail(&mut client, &cfg, *id, kind, size).await;
            }
        },

        Commands::List(list) => match &list.command {
            ListCommands::Avatar { user_id } => {
                action::list::avatar(&mut client, *user_id).await;
            }

            ListCommands::Badges { user_id, place_id } => {
                if let Some(place_id) = place_id {
                    action::list::badges::place(&mut client, *place_id).await;
                } else {
                    action::list::badges::user(&mut client, *user_id).await;
                }
            }

            ListCommands::Experiences { user_id, group_id } => {
                if let Some(group_id) = group_id {
                    action::list::experiences::group(&mut client, *group_id).await;
                } else {
                    action::list::experiences::user(&mut client, *user_id).await;
                }
            }

            ListCommands::Followers { user_id } => {
                action::list::followers(&mut client, *user_id).await
            }

            ListCommands::Followings { user_id } => {
                action::list::followings(&mut client, *user_id).await
            }

            ListCommands::Friends { user_id } => action::list::friends(&mut client, *user_id).await,

            ListCommands::Groups { user_id } => {
                action::list::groups(&mut client, *user_id).await;
            }

            ListCommands::Gamepasses { user_id, place_id } => {
                if let Some(place_id) = place_id {
                    action::list::gamepasses::place(&mut client, *place_id).await;
                } else {
                    action::list::gamepasses::user(&mut client, *user_id).await;
                }
            }

            ListCommands::Inventory {
                user_id,
                kind,
                verbose,
                json,
            } => {
                let kind = AssetTypeId::try_from(kind.as_str()).expect("error: unknown asset kind");
                action::list::inventory(&mut client, *user_id, kind, *verbose, *json).await;
            }

            ListCommands::NameHistory { user_id, group_id } => {
                if let Some(group_id) = group_id {
                    action::list::name_history::group(&mut client, *group_id).await;
                } else {
                    action::list::name_history::user(&mut client, *user_id).await;
                }
            }

            ListCommands::Outfits { user_id } => {
                action::list::outfits(&mut client, *user_id).await;
            }

            ListCommands::Messages => {
                action::list::messages(&mut client).await;
            }

            ListCommands::Conversations => {
                action::list::conversations(&mut client).await;
            }

            ListCommands::Notifications => {
                action::list::notificatons(&mut client).await;
            }

            ListCommands::FriendRequests => action::list::friend_requests(&mut client).await,

            ListCommands::Trades(trades) => todo!(),
        },

        Commands::Login(login) => match &login.command {
            LoginCommands::NewQuick => action::login::quick_login(&mut client, &account).await,
            LoginCommands::Authorize { code } => {
                action::login::authorize_login(&mut client, &code).await
            }
        },
    }
}
