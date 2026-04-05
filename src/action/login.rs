use std::io::Write;

use roblox_api::{api::auth_token_service, client::Client};

use crate::{config::Account, object};

pub(crate) async fn quick_login(client: &mut Client, account: &Account) {
    // TODO: print authenticated account and prompt first

    if !prompt(&format!("Create login for {}?", account.name)) {
        println!("warn: login creation cancelled");
        return;
    };

    let token = auth_token_service::v1::login_create(client)
        .await
        .expect("error: failed to create authentication login ticket");

    let object = object!(
        ("Token", {
            ("Code", token.code.clone()),
            ("Status", token.status),
            ("Private key", token.private_key.clone()),
            ("Expiration time", token.expiration_time.to_string()),
            ("QR code image url", format!(
                "{}/login/qr-code-image?key={}&code={}",
                auth_token_service::v1::URL,
                token.private_key,
                token.code)
            ),
        }),
    );

    print!("{}", object);
}

pub(crate) async fn authorize_login(client: &mut Client, code: &str) {
    let _ = auth_token_service::v1::inspect_code(client, code).await;
    let info = auth_token_service::v1::inspect_code(client, code)
        .await
        .expect("error: failed to inspect authentication code");

    let object = object!(
        ("Info", {
            ("Location", info.location),
            ("Device info", info.device_info)
        })
    );

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
