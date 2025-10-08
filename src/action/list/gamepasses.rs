use roblox_api::{
    Paging,
    api::{gamepasses, games, users},
    client::Client,
};

use crate::object::{Field, FieldStyle, ObjectBuilder, Value};

pub(crate) async fn place(client: &mut Client, place_id: u64) {
    let place_details = games::v1::batch_place_details(client, &[place_id])
        .await
        .unwrap();
    let place_details = place_details.first().unwrap();

    let result = games::v1::universe_gamepasses(
        client,
        place_details.universe_id,
        Paging::new(None, Some(100), None),
    )
    .await
    .unwrap();

    let mut gamepasses = Vec::new();
    for gamepass in &result.gamepasses {
        let object = ObjectBuilder::default()
            .with_field(Field::new(
                "Gamepass",
                Value::Object(
                    ObjectBuilder::default()
                        .with_field(Field::new("Id", Value::from(gamepass.id)))
                        .with_field(Field::new("Name", Value::from(gamepass.name.to_owned())))
                        .with_field(Field::new(
                            "Display name",
                            Value::from(gamepass.display_name.to_owned()),
                        ))
                        .with_field(
                            Field::new(
                                "Price",
                                Value::from(gamepass.price.unwrap_or(0).to_string()),
                            )
                            .with_style(FieldStyle::Price),
                        )
                        .with_field(Field::new("Owned", Value::from(gamepass.owned)))
                        .build(),
                ),
            ))
            .build();

        gamepasses.push(Value::Object(object));
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
        .with_field(Field::new("Gamepasses", Value::Vector(gamepasses)))
        .build();

    print!("{}", object);
}

pub(crate) async fn user(client: &mut Client, id: Option<u64>) {
    let id = id.unwrap_or(users::v1::authenticated_details(client).await.unwrap().id);

    let result = gamepasses::v1::user_gamepasses(client, id, Paging::default())
        .await
        .expect("error: failed to get user gamepasses");

    let mut gamepasses = Vec::new();
    for gamepass in &result {
        let creator = ObjectBuilder::default()
            .with_field(Field::new("Id", Value::from(gamepass.creator.id)))
            .with_field(Field::new(
                "Name",
                Value::from(gamepass.creator.name.to_owned()),
            ))
            // TODO
            // .with_field(Field::new(
            //     "Kind",
            //     Value::from(gamepass.creator.kind.to_string()),
            // ))
            .build();

        let object = ObjectBuilder::default()
            .with_field(Field::new(
                "Gamepass",
                Value::Object(
                    ObjectBuilder::default()
                        .with_field(Field::new("Id", Value::from(gamepass.id)))
                        .with_field(Field::new("Name", Value::from(gamepass.name.to_owned())))
                        .with_field(Field::new("On sale", Value::from(gamepass.on_sale)))
                        .with_field(
                            Field::new(
                                "Price",
                                Value::from(gamepass.price.unwrap_or(0).to_string()),
                            )
                            .with_style(FieldStyle::Price),
                        )
                        .with_field(Field::new("Creator", Value::Object(creator)))
                        .with_field(
                            Field::new("About", Value::from(gamepass.description.to_owned()))
                                .with_style(FieldStyle::Description),
                        )
                        .build(),
                ),
            ))
            .build();

        gamepasses.push(Value::Object(object));
    }

    let object = ObjectBuilder::default()
        .with_field(Field::new("Gamepasses", Value::Vector(gamepasses)))
        .build();

    print!("{}", object);
}
