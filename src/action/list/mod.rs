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

use crate::object::{Field, FieldStyle, ObjectBuilder, Value};

pub(crate) mod badges;
pub(crate) mod experiences;
pub(crate) mod gamepasses;
pub(crate) mod name_history;

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
        ObjectBuilder::default()
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
                Value::Object(
                    ObjectBuilder::default()
                        .with_field(Field::new("Id", Value::from(asset.id)))
                        .with_field(Field::new("Name", Value::from(asset.name.to_owned())))
                        .with_field(Field::new("Instance Id", Value::from(asset.instance_id)))
                        .build(),
                ),
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
                        Value::Object(
                            ObjectBuilder::default()
                                .with_field(Field::new(
                                    "Name",
                                    Value::from(asset.owner.name.to_owned()),
                                ))
                                .build(),
                        ),
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

            assets.push(Value::Object(builder.build()));
        }

        ObjectBuilder::default()
            .with_field(Field::new(
                "Next cursor",
                Value::from(result.next_cursor.unwrap_or_default()),
            ))
            .with_field(Field::new(
                "Previous cursor",
                Value::from(result.previous_cursor.unwrap_or_default()),
            ))
            .with_field(Field::new("Assets", Value::Vector(assets)))
    }
    .build();

    print!("{}", object);
}

pub(crate) async fn groups(client: &mut Client, id: Option<u64>) {
    let id = id.unwrap_or(users::v1::authenticated_details(client).await.unwrap().id);

    let result = groups::v1::user_roles(client, id)
        .await
        .expect("error: failed to get user groups");

    for (info, role) in &result {
        let owner_field = match &info.owner {
            Some(owner) => Value::Object(
                ObjectBuilder::default()
                    .with_field(Field::new("Id", Value::from(owner.id)))
                    .with_field(Field::new("Name", Value::from(owner.name.to_owned())))
                    .with_field(Field::new(
                        "Display name",
                        Value::from(owner.display_name.to_owned()),
                    ))
                    .build(),
            ),

            None => Value::from("None"),
        };

        let shout_field = match &info.shout {
            Some(shout) => Value::Object(
                ObjectBuilder::default()
                    .with_field(
                        Field::new("Content", Value::from(shout.body.to_owned()))
                            .with_style(FieldStyle::Description),
                    )
                    .with_field(Field::new(
                        "Poster",
                        Value::Object(
                            ObjectBuilder::default()
                                .with_field(Field::new("Id", Value::from(shout.poster.id)))
                                .with_field(Field::new(
                                    "Name",
                                    Value::from(shout.poster.name.to_owned()),
                                ))
                                .with_field(Field::new(
                                    "Display name",
                                    Value::from(shout.poster.display_name.to_owned()),
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

        let group = ObjectBuilder::default()
            .with_field(Field::new("Group", Value::from(info.name.to_owned())))
            .with_field(Field::new(
                "Members",
                Value::from(info.member_count.unwrap_or(0).to_string()),
            ))
            .with_field(Field::new("Public", Value::from(info.is_public)))
            .with_field(Field::new("Premium only", Value::from(info.premium_only)))
            .with_field(Field::new("Owner", owner_field))
            .with_field(Field::new("Shout", shout_field))
            .with_field(
                Field::new("About", Value::from(info.description.to_owned()))
                    .with_style(FieldStyle::Description),
            )
            .with_field(Field::new(
                "Role",
                Value::Object(
                    ObjectBuilder::default()
                        .with_field(Field::new("Id", Value::from(role.id)))
                        .with_field(Field::new("Name", Value::from(role.name.to_owned())))
                        .with_field(Field::new("Rank", Value::from(role.rank)))
                        .build(),
                ),
            ))
            .build();

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

        let object = ObjectBuilder::default()
            .with_field(Field::new(
                "Asset",
                Value::Object(
                    ObjectBuilder::default()
                        .with_field(Field::new("Id", Value::from(asset.id)))
                        .with_field(Field::new("Name", Value::from(asset.name.to_owned())))
                        .with_field(Field::new(
                            "Kind name",
                            Value::from(asset.kind.name.to_owned()),
                        ))
                        .with_field(Field::new("Kind Id", Value::from(asset.kind.id)))
                        .with_field(Field::new(
                            "Current version Id",
                            Value::from(asset.current_version_id),
                        ))
                        .build(),
                ),
            ))
            .build();

        assets.push(Value::Object(object));
    }

    let mut emotes = Vec::new();
    for emote in avatar.emotes {
        let object = ObjectBuilder::default()
            .with_field(Field::new(
                "Emote",
                Value::Object(
                    ObjectBuilder::default()
                        .with_field(Field::new("Id", Value::from(emote.id)))
                        .with_field(Field::new("Name", Value::from(emote.name.to_owned())))
                        .with_field(Field::new("Position", Value::from(emote.position)))
                        .build(),
                ),
            ))
            .build();

        emotes.push(Value::Object(object));
    }

    let object = ObjectBuilder::default()
        .with_field(Field::new(
            "Avatar type",
            Value::from(avatar.kind.to_string()),
        ))
        .with_field(Field::new(
            "Default shirt",
            Value::from(avatar.default_shirt_applied),
        ))
        .with_field(Field::new(
            "Default pants",
            Value::from(avatar.default_pants_applied),
        ))
        .with_field(Field::new(
            "Scales",
            Value::Object(
                ObjectBuilder::default()
                    .with_field(Field::new(
                        "Height",
                        Value::from(avatar.scales.height.to_string()),
                    ))
                    .with_field(Field::new(
                        "Width",
                        Value::from(avatar.scales.width.to_string()),
                    ))
                    .with_field(Field::new(
                        "Head",
                        Value::from(avatar.scales.head.to_string()),
                    ))
                    .with_field(Field::new(
                        "Depth",
                        Value::from(avatar.scales.depth.to_string()),
                    ))
                    .with_field(Field::new(
                        "Proportion",
                        Value::from(avatar.scales.proportion.to_string()),
                    ))
                    .with_field(Field::new(
                        "Body type",
                        Value::from(avatar.scales.body_type.to_string()),
                    ))
                    .build(),
            ),
        ))
        .with_field(Field::new(
            "Body colors",
            Value::Object(
                ObjectBuilder::default()
                    .with_field(Field::new(
                        "Head",
                        Value::from(avatar.body_colors.head.to_string()),
                    ))
                    .with_field(Field::new(
                        "Torso",
                        Value::from(avatar.body_colors.torso.to_string()),
                    ))
                    .with_field(Field::new(
                        "Right arm",
                        Value::from(avatar.body_colors.right_arm.to_string()),
                    ))
                    .with_field(Field::new(
                        "Left arm",
                        Value::from(avatar.body_colors.left_arm.to_string()),
                    ))
                    .with_field(Field::new(
                        "Right leg",
                        Value::from(avatar.body_colors.right_leg.to_string()),
                    ))
                    .with_field(Field::new(
                        "Left leg",
                        Value::from(avatar.body_colors.left_leg.to_string()),
                    ))
                    .build(),
            ),
        ))
        .with_field(Field::new("Assets", Value::Vector(assets)))
        .with_field(Field::new("Emotes", Value::Vector(emotes)))
        .build();

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
        outfits.push(Value::Object(
            ObjectBuilder::default()
                .with_field(Field::new(
                    "Outfit",
                    Value::Object(
                        ObjectBuilder::default()
                            .with_field(Field::new("Id", Value::from(outfit.id)))
                            .with_field(Field::new("Name", Value::from(outfit.name.to_owned())))
                            .with_field(Field::new(
                                "Is editable",
                                Value::from(outfit.is_editable.to_string()),
                            ))
                            .build(),
                    ),
                ))
                .build(),
        ));
    }

    let object = ObjectBuilder::default()
        .with_field(Field::new("Total", Value::from(result.total)))
        .with_field(Field::new(
            "Filtered count",
            Value::from(result.filtered_count),
        ))
        .with_field(Field::new("Outfits", Value::Vector(outfits)))
        .build();

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
                Value::Object(
                    ObjectBuilder::default()
                        .with_field(Field::new(
                            "Sender Id",
                            Value::from(
                                payload
                                    .sender_user_id
                                    .to_owned()
                                    .unwrap_or("None".to_string()),
                            ),
                        ))
                        .with_field(Field::new(
                            "Universe Id",
                            Value::from(
                                payload.universe_id.to_owned().unwrap_or("None".to_string()),
                            ),
                        ))
                        .with_field(Field::new(
                            "Place Id",
                            Value::from(payload.place_id.to_owned().unwrap_or("None".to_string())),
                        ))
                        .with_field(Field::new(
                            "Root place Id",
                            Value::from(
                                payload
                                    .root_place_id
                                    .to_owned()
                                    .unwrap_or("None".to_string()),
                            ),
                        ))
                        .with_field(Field::new(
                            "Trigger",
                            Value::from(payload.trigger.to_owned().unwrap_or("None".to_string())),
                        ))
                        .build(),
                )
            } else {
                Value::from("None")
            }
        };

        notifications.push(Value::Object(
            ObjectBuilder::default()
                .with_field(Field::new(
                    "Notification",
                    Value::Object(
                        ObjectBuilder::default()
                            .with_field(Field::new("Id", Value::from(notification.id.to_owned())))
                            .with_field(Field::new(
                                "Event date",
                                Value::from(notification.event_date.to_string()),
                            ))
                            .with_field(Field::new(
                                "Since",
                                Value::from(notification.timestamp.to_owned()),
                            ))
                            .with_field(Field::new(
                                "Interacted with",
                                Value::from(notification.is_interacted.to_string()),
                            ))
                            .with_field(Field::new(
                                "Event count",
                                Value::from(notification.event_count),
                            ))
                            .with_field(Field::new(
                                "Content",
                                Value::Object(
                                    ObjectBuilder::default()
                                        .with_field(Field::new(
                                            "Notification type",
                                            Value::from(
                                                notification.content.notification_type.to_string(),
                                            ),
                                        ))
                                        .with_field(Field::new(
                                            "Current state",
                                            Value::from(
                                                notification.content.current_state.to_string(),
                                            ),
                                        ))
                                        .with_field(Field::new("Content", client_events_payload))
                                        .build(),
                                ),
                            ))
                            .build(),
                    ),
                ))
                .build(),
        ));
    }

    let object = ObjectBuilder::default()
        .with_field(Field::new("Notifications", Value::Vector(notifications)))
        .build();

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
            messages.push(Value::Object(
                ObjectBuilder::default()
                    .with_field(Field::new(
                        "Message",
                        Value::Object(
                            ObjectBuilder::default()
                                .with_field(Field::new("Id", Value::from(message.id)))
                                .with_field(
                                    Field::new("Content", Value::from(message.content))
                                        .with_style(FieldStyle::Description),
                                )
                                .with_field(Field::new("Kind", Value::from(message.kind)))
                                .with_field(Field::new("Sent by", Value::from(message.sender_id)))
                                .with_field(Field::new(
                                    "Is deleted",
                                    Value::from(message.is_deleted.to_string()),
                                ))
                                .with_field(Field::new(
                                    "Creation date",
                                    Value::from(message.created.to_string()),
                                ))
                                .build(),
                        ),
                    ))
                    .build(),
            ));
        }

        conversations.push(Value::Object(
            ObjectBuilder::default()
                .with_field(Field::new(
                    "Conversation",
                    Value::Object(
                        ObjectBuilder::default()
                            .with_field(Field::new(
                                "Id",
                                Value::from(conversation.id.unwrap_or("None".to_string())),
                            ))
                            .with_field(Field::new("Name", Value::from(conversation.name)))
                            .with_field(Field::new("Source", Value::from(conversation.source)))
                            .with_field(Field::new(
                                "Creator Id",
                                Value::from(conversation.creator_id.unwrap_or(0)),
                            ))
                            .with_field(Field::new(
                                "Creation date",
                                Value::from(conversation.created.to_string()),
                            ))
                            .with_field(Field::new(
                                "Last updated",
                                Value::from(conversation.updated.to_string()),
                            ))
                            .with_field(Field::new(
                                "Participants",
                                Value::Vector(
                                    conversation
                                        .participants
                                        .into_iter()
                                        .map(|x| Value::from(x.to_string()))
                                        .collect(),
                                ),
                            ))
                            .with_field(Field::new(
                                "Unread message count",
                                Value::from(conversation.unread_message_count),
                            ))
                            .with_field(Field::new("Messages", Value::Vector(messages)))
                            .build(),
                    ),
                ))
                .build(),
        ));
    }

    let object = ObjectBuilder::default()
        .with_field(Field::new(
            "Next cursor",
            Value::from(result.next_cursor.unwrap_or_default()),
        ))
        .with_field(Field::new(
            "Previous cursor",
            Value::from(result.previous_cursor.unwrap_or_default()),
        ))
        .with_field(Field::new("Conversations", Value::Vector(conversations)))
        .build();

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
        let sender = ObjectBuilder::default()
            .with_field(Field::new("Id", Value::from(message.sender.id)))
            .with_field(Field::new("Name", Value::from(message.sender.name)))
            .with_field(Field::new(
                "Display name",
                Value::from(message.sender.display_name),
            ))
            .with_field(Field::new(
                "Is verified",
                Value::from(message.sender.is_verified.to_string()),
            ))
            .build();

        let recipient = ObjectBuilder::default()
            .with_field(Field::new("Id", Value::from(message.recipient.id)))
            .with_field(Field::new("Name", Value::from(message.recipient.name)))
            .with_field(Field::new(
                "Display name",
                Value::from(message.recipient.display_name),
            ))
            .with_field(Field::new(
                "Is verified",
                Value::from(message.recipient.is_verified.to_string()),
            ))
            .build();

        messages.push(Value::Object(
            ObjectBuilder::default()
                .with_field(Field::new(
                    "Message",
                    Value::Object(
                        ObjectBuilder::default()
                            .with_field(Field::new("Id", Value::from(message.id)))
                            .with_field(Field::new(
                                "Subject",
                                Value::from(message.subject.to_owned()),
                            ))
                            .with_field(Field::new("Sender", Value::Object(sender)))
                            .with_field(Field::new("Recipient", Value::Object(recipient)))
                            .with_field(Field::new(
                                "Is read",
                                Value::from(message.is_read.to_string()),
                            ))
                            .with_field(Field::new(
                                "Is system message",
                                Value::from(message.is_system_message.to_string()),
                            ))
                            .with_field(
                                Field::new("Content", Value::from(message.body.to_owned()))
                                    .with_style(FieldStyle::Description),
                            )
                            .with_field(Field::new(
                                "Creation date",
                                Value::from(message.created.to_string()),
                            ))
                            .with_field(Field::new(
                                "Last updated",
                                Value::from(message.updated.to_string()),
                            ))
                            .build(),
                    ),
                ))
                .build(),
        ));
    }

    let object = ObjectBuilder::default()
        .with_field(Field::new("Current page", Value::from(result.current_page)))
        .with_field(Field::new("Total pages", Value::from(result.total_pages)))
        .with_field(Field::new(
            "Total collection size",
            Value::from(result.total_collection_size),
        ))
        .with_field(Field::new("Messages", Value::Vector(messages)))
        .build();

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
            Value::Object(
                ObjectBuilder::default()
                    .with_field(Field::new("Id", Value::from(user.id)))
                    .with_field(Field::new(
                        "Is verified",
                        Value::from(user.is_verified.unwrap_or(false).to_string()),
                    ))
                    .build(),
            )
        })
        .collect();

    let object = ObjectBuilder::default()
        .with_field(Field::new(
            "Next cursor",
            Value::from(result.next_cursor.unwrap_or_default()),
        ))
        .with_field(Field::new(
            "Previous cursor",
            Value::from(result.previous_cursor.unwrap_or_default()),
        ))
        .with_field(Field::new("Followers", Value::Vector(followers)))
        .build();

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
            Value::Object(
                ObjectBuilder::default()
                    .with_field(Field::new("Id", Value::from(user.id)))
                    .with_field(Field::new(
                        "Is verified",
                        Value::from(user.is_verified.unwrap_or(false).to_string()),
                    ))
                    .build(),
            )
        })
        .collect();

    let object = ObjectBuilder::default()
        .with_field(Field::new(
            "Next cursor",
            Value::from(result.next_cursor.unwrap_or_default()),
        ))
        .with_field(Field::new(
            "Previous cursor",
            Value::from(result.previous_cursor.unwrap_or_default()),
        ))
        .with_field(Field::new("Followings", Value::Vector(followings)))
        .build();

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
            Value::Object(
                ObjectBuilder::default()
                    .with_field(Field::new("Id", Value::from(user.id)))
                    .with_field(Field::new(
                        "Is verified",
                        Value::from(user.is_verified.unwrap_or(false).to_string()),
                    ))
                    .build(),
            )
        })
        .collect();

    let object = ObjectBuilder::default()
        .with_field(Field::new(
            "Next cursor",
            Value::from(result.next_cursor.unwrap_or_default()),
        ))
        .with_field(Field::new(
            "Previous cursor",
            Value::from(result.previous_cursor.unwrap_or_default()),
        ))
        .with_field(Field::new("Friends", Value::Vector(friends)))
        .build();

    print!("{}", object);
}

pub(crate) async fn friend_requests(client: &mut Client) {
    let result = friends::v1::friend_requests(client, Paging::new(None, Some(100), None))
        .await
        .expect("error: failed to get friend requests");

    let mut friend_requests = Vec::new();
    for request in result.requests {
        friend_requests.push(Value::Object(
            ObjectBuilder::default()
                .with_field(Field::new(
                    "Friend request",
                    Value::Object(
                        ObjectBuilder::default()
                            .with_field(Field::new(
                                "Requestor",
                                Value::Object(
                                    ObjectBuilder::default()
                                        .with_field(Field::new(
                                            "Id",
                                            Value::from(request.requester.id),
                                        ))
                                        .with_field(Field::new(
                                            "Display name",
                                            Value::from(request.requester.display_name.to_owned()),
                                        ))
                                        .with_field(Field::new(
                                            "Contact name",
                                            Value::from(
                                                request
                                                    .requester
                                                    .contact_name
                                                    .unwrap_or("None".to_string())
                                                    .to_owned(),
                                            ),
                                        ))
                                        .with_field(Field::new(
                                            "Universe Id",
                                            Value::from(request.requester.source_universe_id),
                                        ))
                                        //.with_field(Field::new("Origin source", Value::from(   request.requester.origin_source_type.to_string())))
                                        .with_field(Field::new(
                                            "Sent at",
                                            Value::from(request.requester.sent_at.to_string()),
                                        ))
                                        .build(),
                                ),
                            ))
                            .with_field(Field::new(
                                "Mutual friends",
                                Value::Vector(
                                    request
                                        .mutual_friends_list
                                        .into_iter()
                                        .map(Value::from)
                                        .collect(),
                                ),
                            ))
                            .build(),
                    ),
                ))
                .build(),
        ));
    }

    let object = ObjectBuilder::default()
        .with_field(Field::new(
            "Next cursor",
            Value::from(result.next_cursor.unwrap_or_default()),
        ))
        .with_field(Field::new(
            "Previous cursor",
            Value::from(result.previous_cursor.unwrap_or_default()),
        ))
        .with_field(Field::new(
            "Friend requests",
            Value::Vector(friend_requests),
        ))
        .build();

    print!("{}", object);
}
