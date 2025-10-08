use roblox_api::{
    Paging,
    api::{
        groups,
        users::{self, v1::user_username_history},
    },
    client::Client,
};

use crate::object::{Field, ObjectBuilder, Value};

pub(crate) async fn user(client: &mut Client, id: Option<u64>) {
    let id = id.unwrap_or(users::v1::authenticated_details(client).await.unwrap().id);

    let result = user_username_history(client, id, Paging::new(None, Some(100), None))
        .await
        .expect("error: failed to get user's name history");

    let object = ObjectBuilder::default()
        .with_field(Field::new(
            "Next cursor",
            Value::from(result.next_cursor.unwrap_or_default().to_owned()),
        ))
        .with_field(Field::new(
            "Previous cursor",
            Value::from(result.previous_cursor.unwrap_or_default().to_owned()),
        ))
        .with_field(Field::new(
            "Names",
            Value::Vector(result.names.into_iter().map(Value::from).collect()),
        ))
        .build();

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

    let object = ObjectBuilder::default()
        .with_field(Field::new(
            "Next cursor",
            Value::from(result.next_cursor.unwrap_or_default().to_owned()),
        ))
        .with_field(Field::new(
            "Previous cursor",
            Value::from(result.previous_cursor.unwrap_or_default().to_owned()),
        ))
        .with_field(Field::new("Names", Value::Vector(names)))
        .with_field(Field::new("Dates", Value::Vector(dates)))
        .build();

    print!("{}", object);
}
