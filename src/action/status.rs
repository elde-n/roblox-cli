use roblox_api::{
    api::{economy, premium_features, presence, users},
    client::Client,
};

use crate::{
    Config,
    object::{Field, FieldStyle, ObjectBuilder, Value},
};

// TODO: move tokio::spawn nest in here
//async fn fetch_account_status() ->  {}

pub(crate) async fn print(cfg: &Config) {
    println!("fetching account info..");

    let mut handles = Vec::new();
    for account in &cfg.accounts {
        let account = account.clone();

        // TODO: could save a few milliseconds by doing the api calls separately
        handles.push(tokio::spawn(async move {
            let mut client = Client::from_cookie(account.cookie.as_str().into());

            let details = users::v1::authenticated_details(&mut client).await.unwrap();
            let currency = economy::v1::currency(&mut client).await.unwrap();
            let is_premium = premium_features::v1::is_premium(&mut client, details.id)
                .await
                .unwrap();

            // TODO: move outside of loop, presence api supports 50 users at once
            let presences = presence::v1::presence(&mut client, &[details.id])
                .await
                .unwrap();

            let gender = users::v1::gender(&mut client).await.unwrap();
            let country_code = users::v1::authenticated_country_code(&mut client)
                .await
                .unwrap();
            let info = users::v1::user_details(&mut client, details.id)
                .await
                .unwrap();

            (
                account,
                info,
                details,
                currency,
                is_premium,
                presences,
                gender,
                country_code,
            )
        }));
    }

    let results = futures::future::join_all(handles).await;
    for result in &results {
        match result {
            Ok((account, info, details, currency, is_premium, presences, gender, country_code)) => {
                let presence = presences.first().unwrap();

                let object = ObjectBuilder::default()
                    .with_field(Field::new(
                        "Account",
                        Value::Object(
                            ObjectBuilder::default()
                                .with_field(Field::new("Id", Value::from(details.id)))
                                .with_field(Field::new(
                                    "Aliased name",
                                    Value::from(account.name.to_owned()),
                                ))
                                .with_field(Field::new(
                                    "Display name",
                                    Value::from(details.display_name.to_owned()),
                                ))
                                .with_field(
                                    Field::new("Gender", Value::from(gender.to_string()))
                                        .with_style(FieldStyle::Enum),
                                )
                                .with_field(Field::new(
                                    "Creation date",
                                    Value::from(info.created.to_string()),
                                ))
                                .with_field(Field::new("Premium", Value::from(*is_premium)))
                                .with_field(
                                    Field::new("Robux", Value::from(currency.to_owned()))
                                        .with_style(FieldStyle::Price),
                                )
                                .with_field(
                                    Field::new("Country", Value::from(country_code.to_owned()))
                                        .with_style(FieldStyle::Enum),
                                )
                                .with_field(Field::new(
                                    "Presence",
                                    Value::from(presence.status.to_owned()),
                                ))
                                .build(),
                        ),
                    ))
                    .build();

                print!("{}", object);
            }

            Err(error) => {
                eprintln!("{}", error)
            }
        }
    }
}
