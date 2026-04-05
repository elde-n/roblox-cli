use console::style;
use roblox_api::{
    Paging,
    api::{
        games::{self, v2::GamesResponse},
        users,
    },
    client::Client,
};

use crate::object;
use crate::object::{FieldStyle, Value};

fn print_experience_creations(result: GamesResponse) {
    if result.games.is_empty() {
        return println!("{} entity has no experiences", style("info:").bold());
    }

    let mut games = Vec::new();
    for creation in &result.games {
        games.push(Value::from(object!(
            ("Creation", {
                ("Id", creation.id),
                ("Name", creation.name.to_owned()),
                ("Root place", {
                    ("Id", creation.root_place.id),
                    ("Price", creation.price.unwrap_or_default(), FieldStyle::Price),
                    ("Visits", creation.place_visits),
                    ("Creation date", creation.created.to_string()),
                    ("Last updated", creation.updated.to_string()),
                    ("About", creation.description.to_owned().unwrap_or(String::new()), FieldStyle::Description),
                }), 
            })
        )));
    }

    let object = object!( 
        ("Next cursor", result.next_cursor.unwrap_or_default().to_owned()),
        ("Previous cursor", result.previous_cursor.unwrap_or_default().to_owned()),
        ("Games", games)
    );

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
