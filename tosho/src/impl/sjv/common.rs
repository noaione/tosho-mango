use aho_corasick::AhoCorasick;
use color_print::cformat;
use tosho_sjv::constants::BASE_HOST;
use tosho_sjv::{
    SJClient,
    models::{MangaChapterDetail, MangaDetail, MangaStoreInfo, MangaStoreResponse},
};

use crate::linkify;
use crate::{config::get_user_path, term::get_console};

pub(super) fn do_print_search_information(
    results: &[MangaDetail],
    with_number: bool,
    spacing: Option<usize>,
) {
    let term = get_console(0);
    let spacing = spacing.unwrap_or(2);

    for (idx, result) in results.iter().enumerate() {
        let id = result.id();
        let manga_url = format!("https://{}/{}", &*BASE_HOST, result.slug());
        let linked = linkify!(&manga_url, result.title());
        let text_data = cformat!("<s>{}</s> ({})", linked, id);

        let pre_space = " ".repeat(spacing);
        let pre_space_lupd = " ".repeat(spacing + 1);
        let pre_space_url = " ".repeat(spacing + 2);

        match with_number {
            true => term.info(format!("{}[{:02}] {}", pre_space, idx + 1, text_data)),
            false => term.info(format!("{pre_space}{text_data}")),
        }
        let updated_at = result.updated_at().format("%Y-%m-%d").to_string();
        term.info(cformat!(
            "{}<s>Last update</s>: {}",
            pre_space_lupd,
            updated_at
        ));
        term.info(format!("{pre_space_url}{manga_url}"));
    }
}

/// Search the big cache JSON for specific title
pub(super) fn search_manga_by_text(contents: &[MangaDetail], target: &str) -> Vec<MangaDetail> {
    // Remove diacritics and lower case the target string
    let clean_target = secular::lower_lay_string(target);
    // Split target by spaces and collect patterns
    let target: Vec<&str> = clean_target.split_ascii_whitespace().collect();

    // Create aho-corasick automaton
    let ac = AhoCorasick::builder()
        .ascii_case_insensitive(true)
        .match_kind(aho_corasick::MatchKind::LeftmostLongest)
        .build(target)
        .unwrap();

    // try matching
    let mut matches: Vec<(Vec<aho_corasick::Match>, &MangaDetail)> = contents
        .iter()
        .filter_map(|content| {
            // Remove diacritics and lower case the title
            let cleaned_title = secular::lower_lay_string(content.title());
            let matches: Vec<aho_corasick::Match> = ac.find_iter(&cleaned_title).collect();

            if matches.is_empty() {
                None
            } else {
                Some((matches, content))
            }
        })
        .collect();

    // sort by the most match
    matches.sort_by(|a, b| {
        // check ny the longest span match
        let a_len = a.0.iter().map(|x| x.len()).sum::<usize>();
        let b_len = b.0.iter().map(|x| x.len()).sum::<usize>();
        let ab_comp = a_len.cmp(&b_len);
        if ab_comp == std::cmp::Ordering::Equal {
            // check by the most match
            a.0.len().cmp(&b.0.len())
        } else {
            ab_comp
        }
    });

    // get the actual match, reverse then take 20
    let actual_match: Vec<MangaDetail> =
        matches.iter().rev().map(|x| x.1.clone()).take(20).collect();

    actual_match
}

// 12 hours
const CACHE_EXPIRY: i64 = 12 * 60 * 60;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(super) struct WrappedStoreCache {
    pub(super) series: Vec<MangaDetail>,
    pub(super) chapters: Vec<MangaChapterDetail>,
    #[serde(rename = "_last_updated")]
    pub(super) last_updated: i64,
}

impl From<MangaStoreResponse> for WrappedStoreCache {
    fn from(value: MangaStoreResponse) -> Self {
        let series = value
            .contents()
            .iter()
            .filter_map(|x| match x {
                MangaStoreInfo::Manga(m) => Some(m.clone()),
                _ => None,
            })
            .collect();

        let chapters = value
            .contents()
            .iter()
            .filter_map(|x| match x {
                MangaStoreInfo::Chapter(c) => Some(c.clone()),
                _ => None,
            })
            .collect();

        Self {
            series,
            chapters,
            last_updated: chrono::Utc::now().timestamp(),
        }
    }
}

pub(super) async fn get_cached_store_data(client: &SJClient) -> anyhow::Result<WrappedStoreCache> {
    let term = get_console(0);

    let base_path = get_user_path();
    let mode_name = match client.get_mode() {
        tosho_sjv::SJMode::SJ => "sj",
        tosho_sjv::SJMode::VM => "vm",
    };
    let plat_name = match client.get_platform() {
        tosho_sjv::SJPlatform::Android => "android",
        tosho_sjv::SJPlatform::Apple => "apple",
        tosho_sjv::SJPlatform::Web => "web",
    };

    let filename = format!("sjv_store_cache_{mode_name}_{plat_name}.json");
    let cache_path = base_path.join(filename);
    if cache_path.exists() {
        let read_data = tokio::fs::read(&cache_path).await;
        if let Ok(data) = read_data {
            term.info("Cache file found, reading...");
            let parsed: Result<WrappedStoreCache, _> = serde_json::from_slice(&data);
            if let Ok(parsed) = parsed {
                let now = chrono::Utc::now().timestamp();
                if now - parsed.last_updated < CACHE_EXPIRY {
                    return Ok(parsed);
                }
                term.warn("Cache expired, fetching fresh data from server...");
            }
        } else {
            term.warn("Failed to read cache file, fetching fresh data from server...");
        }
    } else {
        term.info("Fetching fresh data from server...");
    }

    let cache_store = client.get_store_cache().await.map_err(|e| {
        term.error(format!("Failed to get store cache: {e}"));
        anyhow::anyhow!("Failed to get store cache: {}", e)
    })?;
    let wrapped = WrappedStoreCache::from(cache_store);

    let write_data = serde_json::to_vec(&wrapped)?;
    tokio::fs::write(cache_path, write_data).await?;

    Ok(wrapped)
}

pub(super) fn sort_chapters(chapters: &mut [MangaChapterDetail], reverse: bool) {
    // sort by "chapter" (which is a string of float)
    // then if "chapter" is None, sort by id
    // default to ascending; if reverse is true, reverse the order
    chapters.sort_by(|a, b| {
        let an = a
            .chapter()
            .as_ref()
            .map(|x| x.parse::<f64>().unwrap_or(0.0));
        let bn = b
            .chapter()
            .as_ref()
            .map(|x| x.parse::<f64>().unwrap_or(0.0));

        match (an, bn) {
            (Some(an), Some(bn)) => an.partial_cmp(&bn).unwrap(),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.id().cmp(&b.id()),
        }
    });
    if reverse {
        chapters.reverse();
    }
}
