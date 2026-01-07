use std::path::PathBuf;

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

pub(super) fn check_downloaded_image_count(
    image_dir: &PathBuf,
    extension: &str,
) -> color_eyre::Result<Option<usize>> {
    // check if dir exist
    if !image_dir.exists() {
        return Ok(None);
    }

    // check if dir is dir
    if !image_dir.is_dir() {
        return Ok(None);
    }

    // check how many .[extension] files in the dir
    let mut count = 0;
    for entry in std::fs::read_dir(image_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == extension {
            count += 1;
        }
    }

    Ok(Some(count))
}
