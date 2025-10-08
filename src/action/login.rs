use roblox_api::{api::auth_token_service, client::Client};

use crate::object::{Field, ObjectBuilder, Value};

pub(crate) async fn quick_login(client: &mut Client) {
    // TODO: print authenticated account and prompt first

    let token = auth_token_service::v1::login_create(client)
        .await
        .expect("error: failed to create authentication login ticket");

    let object = ObjectBuilder::default()
        .with_field(Field::new(
            "Token",
            Value::Object(
                ObjectBuilder::default()
                    .with_field(Field::new("Code", Value::from(token.code.clone())))
                    .with_field(Field::new("Status", Value::from(token.status)))
                    .with_field(Field::new(
                        "Private key",
                        Value::from(token.private_key.clone()),
                    ))
                    .with_field(Field::new(
                        "Expiration time",
                        Value::from(token.expiration_time.to_string()),
                    ))
                    .with_field(Field::new(
                        "QR code image url",
                        Value::from(format!(
                            "{}/login/qr-code-image?key={}&code={}",
                            auth_token_service::v1::URL,
                            token.private_key,
                            token.code
                        )),
                    ))
                    .build(),
            ),
        ))
        .build();

    print!("{}", object);
}

pub(crate) async fn authorize_login(client: &mut Client, code: &str) {
    let info = auth_token_service::v1::inspect_code(client, code)
        .await
        .expect("error: failed to inspect authentication code");

    let object = ObjectBuilder::default()
        .with_field(Field::new(
            "Info",
            Value::Object(
                ObjectBuilder::default()
                    .with_field(Field::new("Location", Value::from(info.location)))
                    .with_field(Field::new("Device info", Value::from(info.device_info)))
                    .build(),
            ),
        ))
        .build();

    print!("{}", object);

    print!("Verify login? [y/N]\nn");

    // auth_token_service::v1::validate_code(client, code)
    //     .await
    //     .expect("error: failed to validate authentication code");
}
