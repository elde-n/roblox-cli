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
    object::{Field, FieldStyle, ObjectBuilder, Value},
    objects::badge::Badge,
};

pub(crate) async fn user(client: &mut Client, id: u64) {
    let info = users::v1::user_details(client, id).await.unwrap();
    let is_premium = premium_features::v1::is_premium(client, id).await.unwrap();

    let presences = presence::v1::presence(client, &[id]).await.unwrap();
    let presence = presences.first().unwrap();

    let object = ObjectBuilder::default()
        .with_field(Field::new("User", Value::from(info.name)))
        .with_field(Field::new("Display name", Value::from(info.display_name)))
        .with_field(Field::new(
            "Creation date",
            Value::from(info.created.to_string()),
        ))
        .with_field(Field::new("Premium", Value::from(is_premium)))
        .with_field(Field::new(
            "Presence",
            Value::from(presence.status.to_owned()),
        ))
        .with_field(
            Field::new("About", Value::from(info.description)).with_style(FieldStyle::Description),
        )
        .build();

    print!("{}", object);
}

pub(crate) async fn group(client: &mut Client, id: u64) {
    let info = groups::v1::information(client, id).await.unwrap();

    let owner_field = match info.owner {
        Some(owner) => Value::Object(
            ObjectBuilder::default()
                .with_field(Field::new("Id", Value::from(owner.id)))
                .with_field(Field::new("Name", Value::from(owner.name)))
                .with_field(Field::new("Display name", Value::from(owner.display_name)))
                .build(),
        ),

        None => Value::from("None"),
    };

    let shout_field = match info.shout {
        Some(shout) => Value::Object(
            ObjectBuilder::default()
                .with_field(
                    Field::new("Content", Value::from(shout.body))
                        .with_style(FieldStyle::Description),
                )
                .with_field(Field::new(
                    "Poster",
                    Value::Object(
                        ObjectBuilder::default()
                            .with_field(Field::new("Id", Value::from(shout.poster.id)))
                            .with_field(Field::new("Name", Value::from(shout.poster.name)))
                            .with_field(Field::new(
                                "Display name",
                                Value::from(shout.poster.display_name),
                            ))
                            .build(),
                    ),
                ))
                .with_field(Field::new(
                    "Posted at",
                    Value::from(shout.created.to_string()),
                ))
                .with_field(Field::new(
                    "Updated at",
                    Value::from(shout.updated.to_string()),
                ))
                .build(),
        ),

        None => Value::from("None"),
    };

    let object = ObjectBuilder::default()
        .with_field(Field::new("Group", Value::from(info.name)))
        .with_field(Field::new(
            "Members",
            Value::from(info.member_count.unwrap_or(0)),
        ))
        .with_field(Field::new("Public", Value::from(info.is_public)))
        .with_field(Field::new("Premium only", Value::from(info.premium_only)))
        .with_field(Field::new("Owner", owner_field))
        .with_field(Field::new("Shout", shout_field))
        .with_field(
            Field::new("About", Value::from(info.description)).with_style(FieldStyle::Description),
        )
        .build();

    print!("{}", object);
}

pub(crate) async fn asset(client: &mut Client, id: u64) {
    let info = assets::v1::asset(client, id).await.unwrap();

    let owner_id = match info.creation_context.creator {
        Creator::UserId(id) => id,
        Creator::GroupId(id) => id,
    };

    let object = ObjectBuilder::default()
        .with_field(Field::new("Asset", Value::from(info.name)))
        .with_field(Field::new("Path", Value::from(info.path)))
        .with_field(Field::new("State", Value::from(info.state)))
        .with_field(Field::new("Kind", Value::from(info.asset_type.to_string())))
        .with_field(Field::new(
            "Owner",
            Value::Object(
                ObjectBuilder::default()
                    .with_field(Field::new("Id", Value::from(owner_id)))
                    .build(),
            ),
        ))
        .with_field(
            Field::new("About", Value::from(info.description)).with_style(FieldStyle::Description),
        )
        .build();

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

    let object = ObjectBuilder::default()
        .with_field(Field::new("Game", Value::from(info.name.to_owned())))
        .with_field(Field::new("Universe Id", Value::from(info.universe_id)))
        .with_field(Field::new("Price", Value::from(info.price)).with_style(FieldStyle::Price))
        .with_field(Field::new("Playable", Value::from(info.is_playable)))
        .with_field(Field::new(
            "Rating",
            Value::Object(
                ObjectBuilder::default()
                    .with_field(Field::new("Favorites", Value::from(favorites_count)))
                    .with_field(Field::new("Likes", Value::from(votes.likes.to_string())))
                    .with_field(Field::new(
                        "Disikes",
                        Value::from(votes.dislikes.to_string()),
                    ))
                    .build(),
            ),
        ))
        .with_field(Field::new(
            "Owner",
            Value::Object(
                ObjectBuilder::default()
                    .with_field(Field::new("Id", Value::from(info.builder_id)))
                    .with_field(Field::new("Name", Value::from(info.builder.to_owned())))
                    .build(),
            ),
        ))
        .with_field(
            Field::new("About", Value::from(info.description.to_owned()))
                .with_style(FieldStyle::Description),
        )
        .build();

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

    let object = ObjectBuilder::default()
        .with_field(Field::new(
            "Badge",
            Value::Object(
                ObjectBuilder::default()
                    .with_field(Field::new("Id", Value::from(info.id)))
                    .with_field(Field::new("Name", Value::from(info.name.to_owned())))
                    .with_field(Field::new("On sale", Value::from(info.on_sale.to_string())))
                    .with_field(
                        Field::new(
                            "Price",
                            Value::from(
                                info.price_information
                                    .unwrap_or(PriceInformation {
                                        enabled_features: Vec::new(),
                                        price_in_robux: 0,
                                    })
                                    .price_in_robux,
                            ),
                        )
                        .with_style(FieldStyle::Price),
                    )
                    .with_field(Field::new("Place Id", Value::from(info.place_id)))
                    .with_field(Field::new("Icon image Id", Value::from(info.icon_image_id)))
                    .with_field(Field::new(
                        "Creation date",
                        Value::from(info.created.to_string()),
                    ))
                    .with_field(Field::new(
                        "Last updated",
                        Value::from(info.updated.to_string()),
                    ))
                    .with_field(
                        Field::new("Description", Value::from(info.description.to_owned()))
                            .with_style(FieldStyle::Description),
                    )
                    .build(),
            ),
        ))
        .build();

    print!("{}", object);
}
