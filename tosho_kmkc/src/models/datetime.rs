//! Serde related helper for datetime format in KM

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer, Serializer};

/// The datetime used by KM
const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

/// Serialize a [`DateTime`] into the specified format supported by the API.
pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date.format(FORMAT));
    serializer.serialize_str(&s)
}

/// Deserialize a specific format from API into a [`DateTime`].
pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
    Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
}

/// Serialize an optional [`DateTime`] into the specified format supported by the API.
pub fn serialize_opt<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(date) => {
            let s = format!("{}", date.format(FORMAT));
            serializer.serialize_str(&s)
        }
        None => serializer.serialize_none(),
    }
}

/// Deserialize an optional specific format from API into a [`DateTime`].
pub fn deserialize_opt<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer) {
        Ok(s) => {
            let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
            Ok(Some(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)))
        }
        Err(_) => Ok(None),
    }
}
