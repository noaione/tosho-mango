//! Serde related helper for datetime format in RB

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Deserializer, Serializer};

/// Serialize [`DateTime`] into a RFC3339 string that the API use.
pub fn serialize<S>(date: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = date.to_rfc3339();
    serializer.serialize_str(&s)
}

/// Deserialize a RFC3339 string into [`DateTime`].
pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let datetime = DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)?;
    Ok(datetime)
}

/// Serialize an optional [`DateTime`] into a RFC3339 string that the API use.
pub fn serialize_opt<S>(
    date: &Option<DateTime<FixedOffset>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(date) => {
            let s = date.to_rfc3339();
            serializer.serialize_str(&s)
        }
        None => serializer.serialize_none(),
    }
}

/// Deserialize an optional RFC3339 string into [`DateTime`].
pub fn deserialize_opt<'de, D>(deserializer: D) -> Result<Option<DateTime<FixedOffset>>, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer) {
        Ok(s) => {
            let datetime = DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)?;
            Ok(Some(datetime))
        }
        Err(_) => Ok(None),
    }
}
