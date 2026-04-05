use roblox_api::api::badges::v1::Badge as ApiBadge;

use crate::object;
use crate::object::{FieldStyle, Object, Value};

pub(crate) struct Badge {}
impl Badge {
    pub(crate) fn from_badge(badge: ApiBadge) -> Object {
        let creator = match badge.creator {
            Some(creator) => Value::from(object!(
                ("Id", creator.id),
                ("Name", creator.name),
                ("Kind", creator.kind.to_string())
            )),

            None => Value::from("None"),
        };

        let awarder = match badge.awarder {
            Some(awarder) => Value::from(object!(
                ("Id", awarder.id),
                ("Kind", awarder.kind.to_string())
            )),

            None => Value::from("None"),
        };

        let statistics = Value::from(object!(
            ("Rewarded today", badge.statistics.awarded_today.to_string()),
            (
                "Rewarded in total",
                badge.statistics.awarded_total.to_string()
            ),
            ("Rarity", badge.statistics.win_rate_percentage.to_string()),
        ));

        let universe = match badge.universe {
            Some(universe) => Value::from(object!(
                ("Id", universe.id),
                ("Name", universe.name.to_owned()),
                ("Root place Id", universe.root_place_id)
            )),

            None => Value::from("None"),
        };

        object!(
            ("Badge", {
                ("Id", badge.id),
                ("Name", badge.name.to_owned()),
                ("Display name", badge.display_name.to_owned()),

                ("Achievable", badge.enabled.to_string()),

                ("Icon image Id", badge.icon_image_id),

                ("Creation date", badge.created.to_string()),
                ("Last updated", badge.updated.to_string()),

                ("Description", badge.description.to_owned(), FieldStyle::Description),

                ("Statistics", statistics),
                ("Creator", creator),
                ("Awarder", awarder),
                ("Universe", universe)
            }),
        )
    }
}
