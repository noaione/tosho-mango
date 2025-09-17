//! Serde related helper for datetime format in NI

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Deserializer, Serializer};

/// Serialize [`DateTime`] into a ISO 8601 string that the API use.
pub fn serialize<S>(date: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = date.to_rfc3339();
    serializer.serialize_str(&s)
}

/// Deserialize a ISO 8601 string into [`DateTime`].
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

pub fn serialize_opt_yyyymmdd<S>(
    date: &Option<DateTime<FixedOffset>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(date) => {
            let s = date.format("%Y-%m-%d").to_string();
            serializer.serialize_str(&s)
        }
        None => serializer.serialize_none(),
    }
}

pub fn deserialize_opt_yyyymmdd<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<FixedOffset>>, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer) {
        Ok(s) => {
            let datetime =
                DateTime::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom)?;
            Ok(Some(datetime))
        }
        Err(_) => Ok(None),
    }
}
