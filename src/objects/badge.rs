use roblox_api::api::badges::v1::Badge as ApiBadge;

use crate::object::{Field, FieldStyle, Object, ObjectBuilder, Value};

pub(crate) struct Badge {}
impl Badge {
    pub(crate) fn from_badge(badge: ApiBadge) -> Object {
        let creator = match badge.creator {
            Some(creator) => Value::Object(
                ObjectBuilder::default()
                    .with_field(Field::new("Id", Value::from(creator.id)))
                    .with_field(Field::new("Name", Value::from(creator.name)))
                    .with_field(Field::new("Kind", Value::from(creator.kind.to_string())))
                    .build(),
            ),

            None => Value::from("None"),
        };

        let awarder = match badge.awarder {
            Some(awarder) => Value::Object(
                ObjectBuilder::default()
                    .with_field(Field::new("Id", Value::from(awarder.id)))
                    .with_field(Field::new("Kind", Value::from(awarder.kind.to_string())))
                    .build(),
            ),

            None => Value::from("None"),
        };

        let statistics = Value::Object(
            ObjectBuilder::default()
                .with_field(Field::new(
                    "Rewarded today",
                    Value::from(badge.statistics.awarded_today.to_string()),
                ))
                .with_field(Field::new(
                    "Rewarded in total",
                    Value::from(badge.statistics.awarded_total.to_string()),
                ))
                .with_field(Field::new(
                    "Rarity",
                    Value::from(badge.statistics.win_rate_percentage.to_string()),
                ))
                .build(),
        );

        let universe = match badge.universe {
            Some(universe) => Value::Object(
                ObjectBuilder::default()
                    .with_field(Field::new("Id", Value::from(universe.id)))
                    .with_field(Field::new("Name", Value::from(universe.name.to_owned())))
                    .with_field(Field::new(
                        "Root place Id",
                        Value::from(universe.root_place_id),
                    ))
                    .build(),
            ),

            None => Value::from("None"),
        };

        ObjectBuilder::default()
            .with_field(Field::new(
                "Badge",
                Value::Object(
                    ObjectBuilder::default()
                        .with_field(Field::new("Id", Value::from(badge.id)))
                        .with_field(Field::new("Name", Value::from(badge.name.to_owned())))
                        .with_field(Field::new(
                            "Display name",
                            Value::from(badge.display_name.to_owned()),
                        ))
                        .with_field(Field::new(
                            "Achievable",
                            Value::from(badge.enabled.to_string()),
                        ))
                        .with_field(Field::new(
                            "Icon image Id",
                            Value::from(badge.icon_image_id),
                        ))
                        .with_field(Field::new(
                            "Creation date",
                            Value::from(badge.created.to_string()),
                        ))
                        .with_field(Field::new(
                            "Last updated",
                            Value::from(badge.updated.to_string()),
                        ))
                        .with_field(
                            Field::new("Description", Value::from(badge.description.to_owned()))
                                .with_style(FieldStyle::Description),
                        )
                        .with_field(Field::new("Statistics", statistics))
                        .with_field(Field::new("Creator", creator))
                        .with_field(Field::new("Awarder", awarder))
                        .with_field(Field::new("Universe", universe))
                        .build(),
                ),
            ))
            .build()
    }
}
