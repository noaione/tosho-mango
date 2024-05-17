use aho_corasick::AhoCorasick;
use color_print::cformat;
use prost::Message;
use tosho_mplus::{
    constants::BASE_HOST,
    proto::{Language, Title, TitleListV2, TitleUpdateStatus},
    APIResponse, MPClient,
};

use crate::{config::get_user_path, linkify, term::get_console};

pub(super) fn do_print_search_information(
    results: &[Title],
    with_number: bool,
    spacing: Option<usize>,
) {
    let term = get_console(0);
    let spacing = spacing.unwrap_or(2);

    for (idx, result) in results.iter().enumerate() {
        let manga_url = format!("https://{}/titles/{}", *BASE_HOST, result.id);
        let linked = linkify!(&manga_url, &result.title);
        let mut text_data = cformat!("<s>{}</s> ({})", linked, result.id);

        if result.language() != Language::English {
            text_data = cformat!("{} [<s>{}</>]", text_data, result.language().pretty_name());
        }

        text_data = match result.status() {
            TitleUpdateStatus::None | TitleUpdateStatus::Unrecognized => text_data,
            TitleUpdateStatus::New => {
                cformat!("{} <m!,strong>[<rev>NEW</rev>]</m!,strong>", text_data)
            }
            TitleUpdateStatus::Updated => {
                cformat!("{} <r,strong>[<rev>UP</rev>]</r,strong>", text_data)
            }
            TitleUpdateStatus::ReEdition => {
                cformat!("{} <y,strong>[Re-edition]</y,strong>", text_data)
            }
            TitleUpdateStatus::Creator => {
                cformat!("{} <b,strong>[M+ Creator]</b,strong>", text_data)
            }
        };

        let pre_space = " ".repeat(spacing);
        let pre_space_url = " ".repeat(spacing + 1);

        if with_number {
            term.info(&format!("{}[{:02}] {}", pre_space, idx + 1, text_data))
        } else {
            term.info(&format!("{}{}", pre_space, text_data))
        }

        term.info(&format!("{}{}", pre_space_url, manga_url))
    }
}

/// Search the big cache proto for specific title
pub(super) fn search_manga_by_text(contents: &[Title], target: &str) -> Vec<Title> {
    // remove diacritics and lower case
    // we're not normalizing the string, surely it would work fine :clueless:
    let clean_target = secular::lower_lay_string(target);
    // split by spaces
    let mut target: Vec<&str> = clean_target.split_ascii_whitespace().collect();
    // include back the original target
    target.insert(0, &clean_target);

    let ac = AhoCorasick::builder()
        .ascii_case_insensitive(true)
        .match_kind(aho_corasick::MatchKind::LeftmostLongest)
        .build(target)
        .unwrap();

    let mut matches: Vec<(aho_corasick::Match, Title)> = contents
        .iter()
        .filter_map(|content| {
            let cleaned_title = secular::lower_lay_string(&content.title);
            ac.find(&cleaned_title)
                .map(|matchstick| (matchstick, content.clone()))
        })
        .collect();

    // aho-corasick return the Match Span, sort by the span that has the lowest start
    // also the longest match
    matches.sort_by(|a, b| {
        if a.0.start() == b.0.start() {
            // compare range
            a.0.len().cmp(&b.0.len())
        } else {
            a.0.start().cmp(&b.0.start())
        }
    });

    let actual_match: Vec<Title> = matches.iter().map(|x| x.1.clone()).collect();

    actual_match
}

// 12 hours
const CACHE_EXPIRY: i64 = 12 * 60 * 60;

#[derive(Clone, PartialEq, ::prost::Message)]
pub(super) struct TitleListCache {
    #[prost(message, repeated, tag = "1")]
    pub titles: ::prost::alloc::vec::Vec<TitleListV2>,
    #[prost(int64, tag = "2")]
    pub last_updated: i64,
}

pub(super) async fn get_cached_titles_data(client: &MPClient) -> anyhow::Result<Vec<TitleListV2>> {
    let term = get_console(0);

    let base_path = get_user_path();
    let cache_path = base_path.join("mplus_titles.tmdata");

    if cache_path.exists() {
        let read_data = tokio::fs::read(&cache_path).await;
        if let Ok(data) = read_data {
            term.info("Cache file found, reading...");
            let parsed = TitleListCache::decode(&data[..]);
            if let Ok(parsed) = parsed {
                let now = chrono::Utc::now().timestamp();
                if now - parsed.last_updated < CACHE_EXPIRY {
                    return Ok(parsed.titles);
                }
                term.warn("Cache expired, fetching fresh data from server...");
            }
        } else {
            term.warn("Failed to read cache file, fetching fresh data from server...");
        }
    } else {
        term.info("Fetching fresh data from server...");
    }

    let titles = client.get_all_titles().await;
    if let Err(e) = titles {
        term.error(&format!("Failed to fetch data from server: {}", e));
        anyhow::bail!("Failed to fetch data from server: {}", e);
    }

    let titles = titles.unwrap();

    match titles {
        APIResponse::Success(titles) => {
            let cache = TitleListCache {
                titles: titles.titles.clone(),
                last_updated: chrono::Utc::now().timestamp(),
            };

            let mut buf = Vec::new();
            cache.encode(&mut buf).unwrap();
            tokio::fs::write(cache_path, buf).await?;
            Ok(titles.titles.clone())
        }
        APIResponse::Error(e) => {
            term.error(&format!(
                "Failed to fetch data from server: {}",
                e.as_string()
            ));
            anyhow::bail!("Failed to fetch data from server: {:?}", e);
        }
    }
}
