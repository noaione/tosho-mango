use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S>(date: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = date.to_rfc3339();
    serializer.serialize_str(&s)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let datetime = DateTime::parse_from_rfc3339(&s).unwrap();
    Ok(datetime)
}

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

pub fn deserialize_opt<'de, D>(deserializer: D) -> Result<Option<DateTime<FixedOffset>>, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer) {
        Ok(s) => {
            let datetime = DateTime::parse_from_rfc3339(&s).unwrap();
            Ok(Some(datetime))
        }
        Err(_) => Ok(None),
    }
}
