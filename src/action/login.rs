use std::io::Write;

use roblox_api::{api::auth_token_service, client::Client};

use crate::{
    config::Account,
    object::{Field, ObjectBuilder, Value},
};

pub(crate) async fn quick_login(client: &mut Client, account: &Account) {
    // TODO: print authenticated account and prompt first

    if !prompt(&format!("Create login for {}?", account.name)) {
        println!("warn: login creation cancelled");
        return;
    };

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
    let info = auth_token_service::v1::inspect_code(client, code).await;
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

    if !prompt("Verify login?") {
        println!("warn: login verification cancelled");
        return;
    }

    auth_token_service::v1::validate_code(client, code)
        .await
        .expect("error: failed to validate authentication code");
    println!("info: validating code");
}

fn prompt(title: &str) -> bool {
    loop {
        print!("{title} [y/N] ");
        std::io::stdout()
            .flush()
            .expect("error: failed to flush to stdout");

        let mut prompt = String::new();
        std::io::stdin()
            .read_line(&mut prompt)
            .expect("error: failed to read stdin");

        match prompt
            .to_lowercase()
            .split_whitespace()
            .collect::<String>()
            .as_str()
        {
            "y" => {
                return true;
            }

            "n" | "" => {
                return false;
            }

            _ => {
                println!("error: unable to read prompt, try again")
            }
        };
    }
}
