use roblox_api::{api::groups, client::Client};

pub(crate) mod game;

pub(crate) async fn group(client: &mut Client, id: u64) {
    client.ensure_token().await.unwrap();
    groups::v1::join(client, id).await.unwrap();
}
