use console::style;

use roblox_api::{
    AssetTypeId, Paging, SortOrder,
    api::{
        avatar, friends, groups, inventory, notifications, platform_chat,
        private_messages::{self, v1::MessageTab},
        users,
    },
    client::Client,
};

use crate::object;
use crate::object::{Field, FieldStyle, ObjectBuilder, Value};

pub(crate) mod badges;
pub(crate) mod experiences;
pub(crate) mod gamepasses;
pub(crate) mod name_history;

pub(crate) async fn favorites(client: &mut Client, id: Option<u64>, asset_kind: AssetTypeId) {
    let id = id.unwrap_or(users::v1::authenticated_details(client).await.unwrap().id);
    todo!()
}

pub(crate) async fn inventory(
    client: &mut Client,
    id: Option<u64>,
    asset_kind: AssetTypeId,
    verbose: bool,
    _in_json: bool,
) {
    let id = id.unwrap_or(users::v1::authenticated_details(client).await.unwrap().id);

    if !inventory::v1::can_view_inventory(client, id).await.unwrap() {
        return eprintln!(
            "{} {}",
            style("error:").red().bold(),
            style("user has private inventory").bold()
        );
    };

    let object = if matches!(asset_kind, AssetTypeId::Gamepass) {
        object!()
    } else {
        let result = inventory::v2::user_owned_assets(
            client,
            id,
            asset_kind,
            Paging::new(None, Some(100), Some(SortOrder::default())),
        )
        .await
        .unwrap();

        let mut assets = Vec::new();
        for asset in &result.assets {
            let mut builder = ObjectBuilder::default().with_field(Field::new(
                "Asset",
                Value::from(object!(
                    ("Id", asset.id),
                    ("Name", asset.name.to_owned()),
                    ("Instance Id", asset.instance_id)
                )),
            ));

            if verbose {
                if let Some(id) = &asset.collectible_id {
                    builder = builder
                        .with_field(Field::new("Collectible Id", Value::from(id.to_owned())));
                }

                if let Some(id) = &asset.collectible_instance_id {
                    builder = builder.with_field(Field::new(
                        "Collectible instance Id",
                        Value::from(id.to_owned()),
                    ));
                }

                if let Some(serial) = asset.serial {
                    builder = builder.with_field(Field::new("Serial number", Value::from(serial)));
                }

                builder = builder
                    .with_field(Field::new(
                        "Owner",
                        Value::from(object!(("Name", asset.owner.name.to_owned()))),
                    ))
                    .with_field(Field::new(
                        "Creation date",
                        Value::from(asset.created.to_string()),
                    ))
                    .with_field(Field::new(
                        "Last updated",
                        Value::from(asset.updated.to_string()),
                    ))
            }

            assets.push(Value::from(builder.build()));
        }

        object!(
            ("Next cursor", result.next_cursor.unwrap_or_default()),
            (
                "Previous cursor",
                result.previous_cursor.unwrap_or_default()
            ),
            ("Assets", assets)
        )
    };

    print!("{}", object);
}

pub(crate) async fn groups(client: &mut Client, id: Option<u64>) {
    let id = id.unwrap_or(users::v1::authenticated_details(client).await.unwrap().id);

    let result = groups::v1::user_roles(client, id)
        .await
        .expect("error: failed to get user groups");

    for (info, role) in &result {
        let owner_field = match &info.owner {
            Some(owner) => Value::from(object!(
                ("Id", owner.id),
                ("Name", owner.name.to_owned()),
                ("Display name", owner.display_name.to_owned()),
            )),

            None => Value::from("None"),
        };

        let shout_field = match &info.shout {
            Some(shout) => Value::from(object!(
                ("Content", shout.body.to_owned(), FieldStyle::Description),
                ("Poster", {
                    ("Id", shout.poster.id),
                    ("Name", shout.poster.name.to_owned()),
                    ("Display name", shout.poster.display_name.to_owned()),
                    ("Posted at", shout.created.to_string()),
                    ("Updated at", shout.updated.to_string()),
                })
            )),

            None => Value::from("None"),
        };

        let group = object!(
            ("Group", info.name.to_owned()),
            ("Members", info.member_count.unwrap_or(0).to_string()),
            ("Public", info.is_public),
            ("Premium only", info.premium_only),
            ("Owner", owner_field),
            ("Shout", shout_field),
            ("About", info.description.to_owned(), FieldStyle::Description),
            ("Role", {
                ("Id", role.id),
                ("Name", role.name.to_owned()),
                ("Rank", role.rank),
            }),
        );

        print!("{}", group);
    }
}

pub(crate) async fn avatar(client: &mut Client, id: Option<u64>) {
    let id = id.unwrap_or(users::v1::authenticated_details(client).await.unwrap().id);
    let avatar = avatar::v1::user_avatar(client, id)
        .await
        .expect("error: failed to get user avatar");

    let mut assets = Vec::new();
    for asset in avatar.assets {
        //pub meta: Option<AssetMeta>,
        assets.push(Value::from(object!(("Asset", {
            ("Id", asset.id),
            ("Name", asset.name.to_owned()),
            ("Kind name", asset.kind.name.to_owned()),
            ("Kind Id", asset.kind.id),
            ("Current version Id", asset.current_version_id),
        }))));
    }

    let mut emotes = Vec::new();
    for emote in avatar.emotes {
        emotes.push(Value::from(object!(("Emote", {
            ("Id", emote.id),
            ("Name", emote.name.to_owned()),
            ("Position", emote.position),
        }))));
    }

    let object = object!(
        ("Avatar type", avatar.kind.to_string()),
        ("Default shirt", avatar.default_shirt_applied),
        ("Default pants", avatar.default_pants_applied),
        ("Scales", {
            ("Height", avatar.scales.height.to_string()),
            ("Width", avatar.scales.width.to_string()),
            ("Head", avatar.scales.head.to_string()),
            ("Depth", avatar.scales.depth.to_string()),
            ("Proportion", avatar.scales.proportion.to_string()),
            ("Body type", avatar.scales.body_type.to_string()),
        }),
        ("Body colors", {
            ("Head", avatar.body_colors.head.to_string()),
            ("Torso", avatar.body_colors.torso.to_string()),
            ("Right arm", avatar.body_colors.right_arm.to_string()),
            ("Left arm", avatar.body_colors.left_arm.to_string()),
            ("Right leg", avatar.body_colors.right_leg.to_string()),
            ("Left leg", avatar.body_colors.left_leg.to_string()),
        }),
        ("Assets", assets),
        ("Emotes", emotes));

    print!("{}", object);
}

pub(crate) async fn outfits(client: &mut Client, id: Option<u64>) {
    let id = id.unwrap_or(users::v1::authenticated_details(client).await.unwrap().id);
    let result = avatar::v1::user_outfits(client, id, Paging::new(None, Some(100), None), None)
        .await
        .expect("error: failed to get user outfits");

    let mut outfits = Vec::new();
    for outfit in &result.outfits {
        // TODO: also display the assets of the outfits

        // outfit.outfit_type,
        outfits.push(Value::from(object!(("Outfit", {
            ("Id", outfit.id),
            ("Name", outfit.name.to_owned()),
            ("Is editable", outfit.is_editable.to_string()),
        }))));
    }

    let object = object!(
        ("Total", result.total),
        ("Filtered count", result.filtered_count),
        ("Outfits", outfits)
    );

    print!("{}", object);
}

pub(crate) async fn notificatons(client: &mut Client) {
    let result = notifications::v2::recent(client, Paging::new(None, Some(20), None))
        .await
        .expect("error: failed to get notifications");

    let mut notifications = Vec::new();
    for notification in &result {
        let client_events_payload = {
            if &notification.content.notification_type == "ExperienceInvitation" {
                let payload = &notification.content.client_events_payload;
                Value::from(object!(
                    (
                        "Sender Id",
                        payload
                            .sender_user_id
                            .to_owned()
                            .unwrap_or("None".to_string())
                    ),
                    (
                        "Universe Id",
                        payload.universe_id.to_owned().unwrap_or("None".to_string())
                    ),
                    (
                        "Place Id",
                        payload.place_id.to_owned().unwrap_or("None".to_string())
                    ),
                    (
                        "Root place Id",
                        payload
                            .root_place_id
                            .to_owned()
                            .unwrap_or("None".to_string())
                    ),
                    (
                        "Trigger",
                        payload.trigger.to_owned().unwrap_or("None".to_string())
                    )
                ))
            } else {
                Value::from("None")
            }
        };

        notifications.push(Value::from(object!(("Notification", {
            ("Id", notification.id.to_owned()),
            ("Event date", notification.event_date.to_string()),
            ("Since", notification.timestamp.to_owned()),
            ("Interacted with", notification.is_interacted.to_string()),
            ("Event count", notification.event_count),
            ("Content", {
                ("Notification type", notification.content.notification_type.to_string()),
                ("Current state", notification.content.current_state.to_string()),
                ("Content", client_events_payload),
            })
        }))));
    }

    let object = object!(("Notifications", notifications));
    print!("{}", object);
}

pub(crate) async fn conversations(client: &mut Client) {
    let result = platform_chat::v1::user_conversations(client, Paging::new(None, Some(100), None))
        .await
        .expect("error: failed to get user conversations");

    let mut conversations = Vec::new();
    for conversation in result.conversations {
        let mut messages = Vec::new();
        for message in conversation.messages {
            messages.push(Value::from(object!(("Message", {
                ("Id", message.id),
                ("Content", message.content, FieldStyle::Description),
                ("Kind", message.kind),
                ("Sent by", message.sender_id),
                ("Is deleted", message.is_deleted.to_string()),
                ("Creation date", message.created.to_string()),
            }))));
        }

        conversations.push(Value::from(object!(("Conversation", {
            ("Id", conversation.id.unwrap_or("None".to_string())),
            ("Name", conversation.name),
            ("Source", conversation.source),
            ("Creator Id", conversation.creator_id.unwrap_or(0)),
            ("Creation date", conversation.created.to_string()),
            ("Last updated", conversation.updated.to_string()),
            ("Participants",conversation.participants),
            ("Unread message count", conversation.unread_message_count),
            ("Messages", messages)
        }))));
    }

    let object = object!(
        ("Next cursor", result.next_cursor.unwrap_or_default()),
        (
            "Previous cursor",
            result.previous_cursor.unwrap_or_default()
        ),
        ("Conversations", conversations)
    );

    print!("{}", object);
}

pub(crate) async fn messages(client: &mut Client) {
    let result = private_messages::v1::messages(
        client,
        MessageTab::Inbox,
        Paging::new(Some("0"), Some(100), None),
    )
    .await
    .expect("error: failed to get user messages");

    let mut messages = Vec::new();
    for message in result.collection {
        messages.push(Value::from(object!(("Message", {
            ("Id", message.id),
            ("Subject", message.subject.to_owned()),
            ("Sender", {
                ("Id", message.sender.id),
                ("Name", message.sender.name),
                ("Display name", message.sender.display_name),
                ("Is verified", message.sender.is_verified.to_string()),
            }),
            ("Recipient", {
                ("Id", message.recipient.id),
                ("Name", message.recipient.name),
                ("Display name", message.recipient.display_name),
                ("Is verified", message.recipient.is_verified.to_string()),
            }),
            ("Is read", message.is_read.to_string()),
            ("Is system message", message.is_system_message.to_string()),
            ("Content", message.body.to_owned(), FieldStyle::Description),
            ("Creation date", message.created.to_string()),
            ("Last updated", message.updated.to_string()),
        }))));
    }

    let object = object!(
        ("Current page", result.current_page),
        ("Total pages", result.total_pages),
        ("Total collection size", result.total_collection_size),
        ("Messages", messages),
    );

    print!("{}", object);
}

pub(crate) async fn followers(client: &mut Client, id: Option<u64>) {
    let id = id.unwrap_or(users::v1::authenticated_details(client).await.unwrap().id);
    let result = friends::v1::user_followers(client, id)
        .await
        .expect("error: failed to get user followers");

    let followers: Vec<Value> = result
        .users
        .into_iter()
        .map(|user| {
            Value::from(object!(
                ("Id", user.id),
                ("Is verified", user.is_verified.unwrap_or(false).to_string()),
            ))
        })
        .collect();

    let object = object!(
        ("Next cursor", result.next_cursor.unwrap_or_default()),
        (
            "Previous cursor",
            result.previous_cursor.unwrap_or_default()
        ),
        ("Followers", followers)
    );

    print!("{}", object);
}

pub(crate) async fn followings(client: &mut Client, id: Option<u64>) {
    let id = id.unwrap_or(users::v1::authenticated_details(client).await.unwrap().id);
    let result = friends::v1::user_followings(client, id)
        .await
        .expect("error: failed to get user followings");

    let followings: Vec<Value> = result
        .users
        .into_iter()
        .map(|user| {
            Value::from(object!(
                ("Id", user.id),
                ("Is verified", user.is_verified.unwrap_or(false).to_string()),
            ))
        })
        .collect();

    let object = object!(
        ("Next cursor", result.next_cursor.unwrap_or_default()),
        (
            "Previous cursor",
            result.previous_cursor.unwrap_or_default()
        ),
        ("Followings", followings)
    );

    print!("{}", object);
}

pub(crate) async fn friends(client: &mut Client, id: Option<u64>) {
    let id = id.unwrap_or(users::v1::authenticated_details(client).await.unwrap().id);
    let result = friends::v1::user_friends_find(client, id, Paging::new(None, Some(100), None))
        .await
        .expect("error: failed to get user friends");

    let friends: Vec<Value> = result
        .users
        .into_iter()
        .map(|user| {
            Value::from(object!(
                ("Id", user.id),
                ("Is verified", user.is_verified.unwrap_or(false).to_string())
            ))
        })
        .collect();

    let object = object!(
        ("Next cursor", result.next_cursor.unwrap_or_default()),
        (
            "Previous cursor",
            result.previous_cursor.unwrap_or_default()
        ),
        ("Friends", friends)
    );

    print!("{}", object);
}

pub(crate) async fn friend_requests(client: &mut Client) {
    let result = friends::v1::friend_requests(client, Paging::new(None, Some(100), None))
        .await
        .expect("error: failed to get friend requests");

    let mut friend_requests = Vec::new();
    for request in result.requests {
        friend_requests.push(Value::from(object!(("Friend request", {
            ("Requestor", {
                ("Id", request.requester.id),
                ("Display name", request.requester.display_name.to_owned()),
                ("Contact name", request.requester.contact_name.unwrap_or("None".to_string()).to_owned()),
                ("Universe Id", request.requester.source_universe_id.unwrap_or(0)),
                //("Origin source", request.requester.origin_source_type.to_string()),
                ("Sent at", request.requester.sent_at.to_string()),
            }),
            ("Mutual friends", request.mutual_friends_list),
        }))));
    }

    let object = object!(
        ("Next cursor", result.next_cursor.unwrap_or_default()),
        (
            "Previous cursor",
            result.previous_cursor.unwrap_or_default()
        ),
        ("Friend requests", friend_requests)
    );

    print!("{}", object);
}
