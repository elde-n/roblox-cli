use roblox_api::{
    Paging,
    api::{
        badges::{self, v1::BadgesResponse},
        games, users,
    },
    client::Client,
};

use crate::{object, object::Value, objects::badge::Badge};

fn object_printer(result: BadgesResponse) {
    let badges: Vec<Value> = result
        .badges
        .into_iter()
        .map(|badge| Value::from(Badge::from_badge(badge)))
        .collect();

    let object = object!(
        (
            "Next cursor",
            result.next_cursor.unwrap_or_default().to_owned()
        ),
        (
            "Previous cursor",
            result.previous_cursor.unwrap_or_default().to_owned()
        ),
        ("Badges", badges)
    );

    print!("{}", object);
}

pub(crate) async fn place(client: &mut Client, place_id: u64) {
    let place_details = games::v1::batch_place_details(client, &[place_id])
        .await
        .unwrap();
    let place_details = place_details.first().unwrap();

    let result = badges::v1::universe_badges(
        client,
        place_details.universe_id,
        None,
        Paging::new(None, Some(100), None),
    )
    .await
    .expect("error: failed to get place badges");

    object_printer(result);
}

pub(crate) async fn user(client: &mut Client, id: Option<u64>) {
    let id = id.unwrap_or(users::v1::authenticated_details(client).await.unwrap().id);
    let result = badges::v1::user_badges(client, id, Paging::default())
        .await
        .expect("error: failed to get user badges");

    object_printer(result);
}
