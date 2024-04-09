use chrono::TimeZone;

pub(super) fn unix_timestamp_to_string(timestamp: i64) -> Option<String> {
    let dt = chrono::Utc.timestamp_opt(timestamp, 0).single();

    match dt {
        Some(dt) => {
            let local = dt.with_timezone(&chrono::Local);

            // Format YYYY-MM-DD
            Some(local.format("%Y-%m-%d").to_string())
        }
        None => None,
    }
}
