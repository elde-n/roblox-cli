use console::style;
use roblox_api::{
    Paging,
    api::{
        games::{self, v2::GamesResponse},
        users,
    },
    client::Client,
};

use crate::object::{Field, FieldStyle, ObjectBuilder, Value};

fn print_experience_creations(result: GamesResponse) {
    if result.games.is_empty() {
        return println!("{} entity has no experiences", style("info:").bold());
    }

    let mut games = Vec::new();
    for creation in &result.games {
        games.push(Value::Object(
            ObjectBuilder::default()
                .with_field(Field::new(
                    "Creation",
                    Value::Object(
                        ObjectBuilder::default()
                            .with_field(Field::new("Id", Value::from(creation.id)))
                            .with_field(Field::new("Name", Value::from(creation.name.to_owned())))
                            .with_field(Field::new(
                                "Root place",
                                Value::Object(
                                    ObjectBuilder::default()
                                        .with_field(Field::new(
                                            "Id",
                                            Value::from(creation.root_place.id),
                                        ))
                                        .build(),
                                ),
                            ))
                            .with_field(
                                Field::new(
                                    "Price",
                                    Value::from(creation.price.unwrap_or_default()),
                                )
                                .with_style(FieldStyle::Price),
                            )
                            .with_field(Field::new("Visits", Value::from(creation.place_visits)))
                            .with_field(Field::new(
                                "Creation date",
                                Value::from(creation.created.to_string()),
                            ))
                            .with_field(Field::new(
                                "Last updated",
                                Value::from(creation.updated.to_string()),
                            ))
                            .with_field(
                                Field::new(
                                    "About",
                                    Value::from(
                                        creation.description.to_owned().unwrap_or(String::new()),
                                    ),
                                )
                                .with_style(FieldStyle::Description),
                            )
                            .build(),
                    ),
                ))
                .build(),
        ));
    }

    let object = ObjectBuilder::default()
        .with_field(Field::new(
            "Next cursor",
            Value::from(result.next_cursor.unwrap_or_default().to_owned()),
        ))
        .with_field(Field::new(
            "Previous cursor",
            Value::from(result.previous_cursor.unwrap_or_default().to_owned()),
        ))
        .with_field(Field::new("Games", Value::Vector(games)))
        .build();

    print!("{}", object);
}

pub(crate) async fn group(client: &mut Client, id: u64) {
    print_experience_creations(
        games::v2::group_games_v2(client, id, 1, Paging::default())
            .await
            .expect("error: failed to get group experience creations"),
    );
}

pub(crate) async fn user(client: &mut Client, id: Option<u64>) {
    let id = id.unwrap_or(users::v1::authenticated_details(client).await.unwrap().id);

    print_experience_creations(
        games::v2::user_games(client, id, 2, Paging::default())
            .await
            .expect("error: failed to get user experience creations"),
    );
}
