use roblox_api::{
    Paging,
    api::{
        groups,
        users::{self, v1::user_username_history},
    },
    client::Client,
};

use crate::object;
use crate::object::Value;

pub(crate) async fn user(client: &mut Client, id: Option<u64>) {
    let id = id.unwrap_or(users::v1::authenticated_details(client).await.unwrap().id);

    let result = user_username_history(client, id, Paging::new(None, Some(100), None))
        .await
        .expect("error: failed to get user's name history");

    let object = object!(
        (
            "Next cursor",
            result.next_cursor.unwrap_or_default().to_owned()
        ),
        (
            "Previous cursor",
            result.previous_cursor.unwrap_or_default().to_owned()
        ),
        ("Names", result.names),
    );

    print!("{}", object);
}

pub(crate) async fn group(client: &mut Client, id: u64) {
    let result = groups::v1::name_history(client, id)
        .await
        .expect("error: failed to get group name history");

    let (names, dates): (Vec<Value>, Vec<Value>) = result
        .names
        .into_iter()
        .map(|(x, y)| (Value::from(x), Value::from(y.to_string())))
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
        ("Names", names),
        ("Dates", dates)
    );

    print!("{}", object);
}
