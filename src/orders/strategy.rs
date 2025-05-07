use std::fmt;
use std::fmt::Display;
use std::str::FromStr;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "urgency", rename_all = "snake_case")]
pub enum Urgency {
    #[serde(rename = "super-passive")]
    SuperPassive,
    #[serde(rename = "passive")]
    Passive,
    #[serde(rename = "moderate")]
    Moderate,
    #[serde(rename = "aggressive")]
    Aggressive,
    #[serde(rename = "super-aggressive")]
    SuperAggressive,
}

impl FromStr for Urgency {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "super-passive" => Ok(Urgency::SuperPassive),
            "passive" => Ok(Urgency::Passive),
            "moderate" => Ok(Urgency::Moderate),
            "aggressive" => Ok(Urgency::Aggressive),
            "super-aggressive" => Ok(Urgency::SuperAggressive),
            other => Err(crate::Error::new(
                crate::error::ErrorType::ParseError,
                format!("Invalid Urgency: {}", other),
            )),
        }
    }
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "destination", rename_all = "snake_case")]
pub enum Destination {
    Arcx, // NYSE ARCA
    Bats, // BATS Exchange
    Baty, // BATS Y Exchange
    Edga, // EDGA Exchange
    Edgx, // EDGX Exchange
    Eprl, // MIAX Pearl Equities
    Iexg, // Investors' Exchange
    Memx, // Members' Exchange
    Xase, // NYSE American
    Xbos, // NASDAQ BX Exchange
    Xcis, // NYSE National
    Xnms, // NASDAQ/NMS (Global Market)
    Xnys, // New York Stock Exchange
}

impl Display for Destination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Destination::Arcx => "arcx",
            Destination::Bats => "bats",
            Destination::Baty => "baty",
            Destination::Edga => "edga",
            Destination::Edgx => "edgx",
            Destination::Eprl => "eprl",
            Destination::Iexg => "iexg",
            Destination::Memx => "memx",
            Destination::Xase => "xase",
            Destination::Xbos => "xbos",
            Destination::Xcis => "xcis",
            Destination::Xnms => "xnms",
            Destination::Xnys => "xnys",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub enum Strategy {
    SmartOrderRoute {
        start_at: Option<i64>,
        end_at: Option<i64>,
        urgency: Option<Urgency>,
    },
    DirectMarketAccess {
        destination: Destination
    },
}

impl Serialize for Strategy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        match self {
            Strategy::SmartOrderRoute {
                start_at,
                end_at,
                urgency, ..
            } => {
                map.serialize_entry("type", "sor")?;

                if let Some(start_at) = start_at {
                    map.serialize_entry("start_at", start_at)?;
                }
                if let Some(end_at) = end_at {
                    map.serialize_entry("end_at", end_at)?;
                }
                if let Some(urgency) = urgency {
                    map.serialize_entry("urgency", urgency)?;
                }
            },
            &Strategy::DirectMarketAccess {
                destination,
            } => {
                map.serialize_entry("type", "dma")?;
                map.serialize_entry("destination", &destination.to_string())?;
            }
        }

        map.end()
    }
}


impl<'de> Deserialize<'de> for Strategy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Type,
            StartAt,
            EndAt,
            Urgency,
            Unknown,
            Destination,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct FieldVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = Field;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a valid strategy field")
            }

            fn visit_str<E>(self, v: &str) -> Result<Field, E>
            where
                E: de::Error,
            {
                Ok(match v {
                    "type" => Field::Type,
                    "start_at" => Field::StartAt,
                    "end_at" => Field::EndAt,
                    "urgency" => Field::Urgency,
                    "destination" => Field::Destination,
                    _ => Field::Unknown,
                })
            }
        }

        struct StrategyVisitor;

        impl<'de> Visitor<'de> for StrategyVisitor {
            type Value = Strategy;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a valid strategy map")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Strategy, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut strategy_type: Option<String> = None;
                let mut start_at: Option<i64> = None;
                let mut end_at: Option<i64> = None;
                let mut urgency: Option<Urgency> = None;
                let mut destination: Option<Destination> = None;

                while let Some(key) = map.next_key::<Field>()? {
                    match key {
                        Field::Type => {
                            strategy_type = Some(map.next_value()?);
                        }
                        Field::StartAt => {
                            start_at = Some(map.next_value()?);
                        }
                        Field::EndAt => {
                            end_at = Some(map.next_value()?);
                        }
                        Field::Urgency => {
                            urgency = Some(map.next_value()?);
                        }
                        Field::Destination => {
                            destination = Some(map.next_value()?);
                        }
                        Field::Unknown => {
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                match strategy_type.as_deref() {
                    Some("sor") => Ok(Strategy::SmartOrderRoute {
                        start_at,
                        end_at,
                        urgency,
                    }),
                    Some("dma") => {
                        if let Some(dest) = destination {
                            Ok(Strategy::DirectMarketAccess { destination: dest })
                        } else {
                            Err(de::Error::missing_field("destination"))
                        }
                    }
                    Some(other) => Err(de::Error::custom(format!("unsupported strategy type: {}", other))),
                    None => Err(de::Error::missing_field("type")),
                }
            }
        }

        deserializer.deserialize_map(StrategyVisitor)
    }
}