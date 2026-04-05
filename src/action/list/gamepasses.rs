use roblox_api::{
    Paging,
    api::{gamepasses, games, users},
    client::Client,
};

use crate::object;
use crate::object::{FieldStyle, Value};

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
        let object = object!(("Gamepass", {
            ("Id", gamepass.id),
            ("Name", gamepass.name.to_owned()),
            ("Display name", gamepass.display_name.to_owned()),
            ("Price", gamepass.price.unwrap_or(0).to_string(), FieldStyle::Price),
            ("Owned", gamepass.owned),
        }));

        gamepasses.push(Value::from(object));
    }

    let object = object!(
        (
            "Next cursor",
            result.next_cursor.unwrap_or_default().to_owned()
        ),
        (
            "Previous cursor",
            result.previous_cursor.unwrap_or_default().to_owned()
        ),
        ("Gamepasses", gamepasses),
    );

    print!("{}", object);
}

pub(crate) async fn user(client: &mut Client, id: Option<u64>) {
    let id = id.unwrap_or(users::v1::authenticated_details(client).await.unwrap().id);

    let result = gamepasses::v1::user_gamepasses(client, id, Paging::default())
        .await
        .expect("error: failed to get user gamepasses");

    let mut gamepasses = Vec::new();
    for gamepass in &result {
        let creator = object!(
            ("Id", gamepass.creator.id),
            ("Name", gamepass.creator.name.to_owned()),
            // TODO
            // ("Kind", gamepass.creator.kind.to_string()),
        );

        let object = object!(("Gamepass", {
            ("Id", gamepass.id),
            ("Name", gamepass.name.to_owned()),
            ("On sale", gamepass.on_sale),
            ("Price", gamepass.price.unwrap_or(0).to_string(), FieldStyle::Price),
            ("Creator", creator),
            ("About", gamepass.description.to_owned(), FieldStyle::Description),
        }));

        gamepasses.push(Value::from(object));
    }

    let object = object!(("Gamepasses", gamepasses));
    print!("{}", object);
}
