use aho_corasick::AhoCorasick;
use chrono::TimeZone;
use tosho_sjv::models::MangaDetail;

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

/// Search the big cache JSON for specific title
#[allow(dead_code)]
pub(crate) fn search_manga_by_text(contents: Vec<MangaDetail>, target: &str) -> Vec<MangaDetail> {
    // split by spaces
    let target: Vec<&str> = target.split_ascii_whitespace().collect();

    let ac = AhoCorasick::builder()
        .ascii_case_insensitive(true)
        .build(target)
        .unwrap();

    let mut matches = vec![];
    for content in contents {
        if ac.find(&content.title).is_some() {
            matches.push(content);
        }
    }

    matches
}
