use std::{path::PathBuf, sync::Arc};

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

pub(super) fn check_downloaded_image_count(image_dir: &PathBuf, extension: &str) -> Option<usize> {
    // check if dir exist
    if !image_dir.exists() {
        return None;
    }

    // check if dir is dir
    if !image_dir.is_dir() {
        return None;
    }

    // check how many .[extension] files in the dir
    let mut count = 0;
    for entry in std::fs::read_dir(image_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == extension {
            count += 1;
        }
    }

    Some(count)
}

/// Create an Atomic Reference Counted progress bar
///
/// This wrap [`indicatif::ProgressBar`] with [`std::sync::Arc`] to allow it to be shared between threads
///
/// # Arguments
/// * `total` - The total number of items to be processed
pub(super) fn create_progress_bar(total: u64) -> Arc<indicatif::ProgressBar> {
    let progress = Arc::new(indicatif::ProgressBar::new(total));
    progress.enable_steady_tick(std::time::Duration::from_millis(120));
    progress.set_style(
        indicatif::ProgressStyle::with_template(
            "{spinner:.blue} {msg} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len}",
        )
        .unwrap()
        .progress_chars("#>-")
        .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏", " "]),
    );
    progress.set_message("Downloading");

    progress
}
