use console::style;
use roblox_api::{
    api::{
        assets::{self, v1::Creator},
        badges,
        gamepasses::{self, v1::PriceInformation},
        games, groups, premium_features, presence, users,
    },
    client::Client,
};

use crate::{
    object,
    object::{FieldStyle, Value},
    objects::badge::Badge,
};

pub(crate) async fn user(client: &mut Client, id: u64) {
    let info = users::v1::user_details(client, id).await.unwrap();
    let is_premium = premium_features::v1::is_premium(client, id).await.unwrap();

    let presences = presence::v1::presence(client, &[id]).await.unwrap();
    let presence = presences.first().unwrap();

    let object = object!(
        ("User", info.name),
        ("Display name", info.display_name),
        ("Creation date", info.created.to_string()),
        ("Premium", is_premium),
        ("Presence", presence.status.to_owned()),
        ("About", info.description, FieldStyle::Description),
    );

    print!("{}", object);
}

pub(crate) async fn group(client: &mut Client, id: u64) {
    let info = groups::v1::information(client, id).await.unwrap();

    let owner_field = match info.owner {
        Some(owner) => Value::from(object!(
            ("Id", owner.id),
            ("Name", owner.name),
            ("Display name", owner.display_name),
        )),

        None => Value::from("None"),
    };

    let shout_field = match info.shout {
        Some(shout) => Value::from(object!(
            ("Content", shout.body, FieldStyle::Description),
            ("Poster", {
                ("Id", shout.poster.id),
                ("Name", shout.poster.name),
                ("Display name", shout.poster.display_name),
            }),
            ("Posted at", shout.created.to_string()),
            ("Updated at", shout.updated.to_string()),
        )),

        None => Value::from("None"),
    };

    let object = object!(
        ("Group", info.name),
        ("Members", info.member_count.unwrap_or(0)),
        ("Public", info.is_public),
        ("Premium only", info.premium_only),
        ("Owner", owner_field),
        ("Shout", shout_field),
        ("About", info.description, FieldStyle::Description),
    );

    print!("{}", object);
}

pub(crate) async fn asset(client: &mut Client, id: u64) {
    let info = assets::v1::asset(client, id).await.unwrap();

    let owner_id = match info.creation_context.creator {
        Creator::UserId(id) => id,
        Creator::GroupId(id) => id,
    };

    let object = object!(
        ("Asset", info.name),
        ("Path", info.path),
        ("State", info.state),
        ("Kind", info.asset_type.to_string()),
        ("Owner", { ("Id", owner_id) }),
        ("About", info.description, FieldStyle::Description),
    );

    print!("{}", object);
}

pub(crate) async fn place(client: &mut Client, id: u64) {
    let info = games::v1::batch_place_details(client, &[id]).await.unwrap();
    let info = match info.first() {
        Some(info) => info,
        None => {
            return eprintln!(
                "{} {}",
                style("error:").red().bold(),
                style("game not found").bold()
            );
        }
    };

    let votes = games::v1::universe_votes(client, &[info.universe_id])
        .await
        .unwrap();
    let votes = votes.first().unwrap();

    let favorites_count = games::v1::universe_favorite_count(client, info.universe_id)
        .await
        .unwrap();

    let object = object!(
        ("Game", info.name.to_owned()),
        ("Universe Id", info.universe_id),
        ("Price", info.price, FieldStyle::Price),
        ("Playable", info.is_playable),
        ("Rating", {
            ("Favorites", favorites_count),
            ("Likes", votes.likes.to_string()),
            ("Disikes", votes.dislikes.to_string()),
        }),
        ("Owner", {
            ("Id", info.builder_id),
            ("Name", info.builder.to_owned()),
        }),
        ("About", info.description.to_owned(), FieldStyle::Description),
    );

    print!("{}", object);
}

pub(crate) async fn badge(client: &mut Client, id: u64) {
    let badge = badges::v1::information(client, id)
        .await
        .expect("error: failed to get information");

    print!("{}", Badge::from_badge(badge));
}

pub(crate) async fn gamepass(client: &mut Client, id: u64) {
    let info = gamepasses::v1::details(client, id)
        .await
        .expect("error: failed to get gamepass details");

    let object = object!(("Badge", {
        ("Id", info.id),
        ("Name", info.name.to_owned()),
        ("On sale", info.on_sale.to_string()),
        ("Price", info.price_information.unwrap_or(PriceInformation {
            enabled_features: Vec::new(),
            price_in_robux: 0,
        }).price_in_robux, FieldStyle::Price),
        ("Place Id", info.place_id),
        ("Icon image Id", info.icon_image_id),
        ("Creation date", info.created.to_string()),
        ("Last updated", info.updated.to_string()),
        ("Description", info.description.to_owned(), FieldStyle::Description),
    }));

    print!("{}", object);
}
