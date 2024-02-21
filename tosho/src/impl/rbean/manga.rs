use std::collections::HashMap;

use clap::ValueEnum;
use color_print::cformat;
use tosho_rbean::{
    constants::BASE_HOST,
    models::{Separator, SortOption},
    RBClient,
};

use crate::{cli::ExitCode, linkify};

use super::{
    common::{do_print_search_information, save_session_config},
    config::Config,
};

#[derive(Debug, Clone, Copy)]
pub(crate) enum CLISortOption {
    Alphabetical,
    Popular,
    Recent,
}

impl ValueEnum for CLISortOption {
    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        let input = if ignore_case {
            input.to_lowercase()
        } else {
            input.to_string()
        };
        match input.as_str() {
            "alphabetical" => Ok(CLISortOption::Alphabetical),
            "popular" => Ok(CLISortOption::Popular),
            "recent" => Ok(CLISortOption::Recent),
            _ => Err(format!("Invalid sort option: {}", input)),
        }
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            CLISortOption::Alphabetical => Some(clap::builder::PossibleValue::new("alphabetical")),
            CLISortOption::Popular => Some(clap::builder::PossibleValue::new("popular")),
            CLISortOption::Recent => Some(clap::builder::PossibleValue::new("recent")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[
            CLISortOption::Alphabetical,
            CLISortOption::Popular,
            CLISortOption::Recent,
        ]
    }
}

impl From<CLISortOption> for SortOption {
    fn from(value: CLISortOption) -> Self {
        match value {
            CLISortOption::Alphabetical => SortOption::Alphabetical,
            CLISortOption::Popular => SortOption::Popular,
            CLISortOption::Recent => SortOption::Recent,
        }
    }
}

pub(crate) async fn rbean_search(
    query: &str,
    limit: Option<u32>,
    sort_options: Option<CLISortOption>,
    client: &mut RBClient,
    account: &Config,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(&cformat!("Searching for <magenta,bold>{}</>...", query));

    let results = client
        .search(query, Some(0), limit, sort_options.map(SortOption::from))
        .await;

    match results {
        Ok(results) => {
            super::common::save_session_config(client, account);

            if results.results.is_empty() {
                console.warn("No results found!");
                return 0;
            }

            console.info(&cformat!(
                "Search results (<magenta,bold>{}</> results):",
                results.results.len()
            ));

            do_print_search_information(&results.results, false, None);

            0
        }
        Err(e) => {
            console.error(&format!("Failed to search: {}", e));

            1
        }
    }
}

fn format_named_vec(items: Vec<String>) -> String {
    if items.is_empty() {
        return "Unknown".to_string();
    }

    if items.len() == 1 {
        return items[0].clone();
    }

    if items.len() > 2 {
        // Author 1, Author 2, and Author 3
        let mut items = items.clone();
        let last = items.pop().unwrap();
        let first = items.join(", ");
        format!("{}, and {}", first, last)
    } else {
        // Author 1 and Author 2
        items.join(" and ")
    }
}

pub(crate) async fn rbean_title_info(
    uuid: &str,
    show_chapters: bool,
    client: &mut RBClient,
    account: &Config,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(&cformat!(
        "Fetching info for ID <magenta,bold>{}</>...",
        uuid
    ));

    let result = client.get_manga(uuid).await;

    if let Err(e) = result {
        console.error(&format!("Failed to fetch manga: {}", e));
        return 1;
    }

    let result = result.unwrap();

    save_session_config(client, account);
    let mut chapter_meta: Option<tosho_rbean::models::ChapterListResponse> = None;

    if show_chapters {
        console.info(&cformat!(
            "Fetching chapters for <magenta,bold>{}</>...",
            result.title
        ));

        let fetch_chapters = client.get_chapter_list(uuid).await;

        if let Err(e) = fetch_chapters {
            console.error(&format!("Failed to fetch chapters: {}", e));
            return 1;
        }

        chapter_meta = Some(fetch_chapters.unwrap());
        save_session_config(client, account);
    }

    let manga_url = format!("https://{}/series/{}", *BASE_HOST, result.slug);
    let linked = linkify!(&manga_url, &result.title);

    console.info(&cformat!(
        "Title information for <magenta,bold>{}</>:",
        linked
    ));

    let creators: Vec<String> = result.creators.iter().map(|c| c.name.clone()).collect();

    console.info(&cformat!("  <s>Authors</>: {}", format_named_vec(creators)));
    let genres: Vec<String> = result
        .genres
        .iter()
        .map(|g| {
            let genre_url = format!("https://{}/series?tags={}", *BASE_HOST, g.slug);
            linkify!(&genre_url, &g.name)
        })
        .collect();
    if !genres.is_empty() {
        console.info(&cformat!("  <s>Genres</>: {}", format_named_vec(genres)));
    }

    let publish_url = format!(
        "https://{}/publishers/{}",
        *BASE_HOST, result.publisher.slug
    );
    let linked_pub = linkify!(&publish_url, &result.publisher.name);
    console.info(&cformat!("  <s>Publisher</>: {}", linked_pub));

    if let Some(release_schedule) = &result.release_schedule {
        console.info(&cformat!("  <s>Release schedule</>: {}", release_schedule));
    }

    console.info(&cformat!("  <s>Summary</>"));
    console.info(&format!("   {}", result.description));

    println!();

    console.info(&cformat!("  <s>Chapters</>: {} chapters", result.chapters));

    if let Some(chapter_meta) = chapter_meta {
        let mut index_to_separator = HashMap::new();
        for separator in chapter_meta.separators.iter() {
            match separator {
                Separator::AlaCarteNotice(purchase) => {
                    index_to_separator.insert(purchase.index, separator.clone());
                }
                Separator::ChapterGap(volume) => {
                    index_to_separator.insert(volume.index, separator.clone());
                }
                Separator::PremiumNotice(volume) => {
                    index_to_separator.insert(volume.index, separator.clone());
                }
                _ => {}
            }
        }

        let mut ignored_uuid = vec![];
        for (idx, chapter) in chapter_meta.chapters.iter().enumerate() {
            let separator = index_to_separator.remove(&(idx as i32));
            if let Some(ref separator) = separator {
                match &separator {
                    Separator::PremiumNotice(sep) => {
                        console.info(&cformat!(
                            "    <m>Unlock the first {} chapters by <s>upgrading to Premium</></>",
                            sep.data.count
                        ));
                    }
                    Separator::ChapterGap(sep) => {
                        console.info(&cformat!(
                            "    <b>Chapters <s>{}-{}</> is currently on backlog!</>",
                            sep.data.range.start,
                            sep.data.range.end
                        ));
                    }
                    _ => {}
                }
            }

            if ignored_uuid.contains(&chapter.uuid) {
                continue;
            }

            let ch_title = chapter.formatted_title();
            let ch_url = format!("{}/read/{}", manga_url, chapter.uuid);

            let linked_ch = linkify!(&ch_url, &ch_title);
            let mut banner_title = cformat!("    <s>{}</> ({})", linked_ch, chapter.uuid);

            if !chapter.premium {
                banner_title.push_str(&cformat!(" <green,bold>[<rev>FREE</>]</>"));
            }

            if chapter.new {
                banner_title.push_str(&cformat!(" <yellow,bold>[<rev>NEW</>]</>"));
            }

            if let Some(Separator::AlaCarteNotice(volume)) = separator {
                let total_chapters_next = volume.data.count;
                let mut volume_chapters: HashMap<String, Vec<String>> = HashMap::new();

                for ch in chapter_meta
                    .chapters
                    .iter()
                    .skip(idx + 1)
                    .take(total_chapters_next as usize)
                {
                    // insert ch.volume_uuid -> [ch.chapter]
                    let volume_uuid = ch.volume_uuid.clone().unwrap_or_else(|| "".to_string());
                    let ch_num = ch.chapter.clone();
                    ignored_uuid.push(ch.uuid.clone());
                    volume_chapters.entry(volume_uuid).or_default().push(ch_num);
                }

                console.info(&cformat!(
                    "    <b!>Unlock more chapters by <s>buying volumes</></>",
                ));

                // we have an ordering of volumes from chapter_meta.volume_order which is a Vec<String>
                let mut sorted_volume_chapters = vec![];
                for volume_uuid in chapter_meta.volume_order.iter() {
                    if let Some(chapters) = volume_chapters.get(volume_uuid) {
                        sorted_volume_chapters.push((volume_uuid, chapters.clone()));
                    }
                }

                for (volume_uuid, chapters) in sorted_volume_chapters {
                    let volume = chapter_meta.volumes.get(volume_uuid);

                    if let Some(volume) = volume {
                        let first_last = format!(
                            "Chapters {}-{}",
                            chapters.first().unwrap(),
                            chapters.last().unwrap()
                        );

                        let vol_url_link = format!("{}/volumes", manga_url);
                        let vol_url_linked = linkify!(&vol_url_link, &volume.short_title);

                        console.info(&cformat!("     <s>{}</>: {}", vol_url_linked, first_last));
                    }
                }
            } else if chapter.upcoming {
                console.info(&cformat!("    <s,m>Upcoming</>: {}", ch_title));
            } else {
                console.info(&banner_title);
                console.info(&format!("     {}", ch_url));
                if let Some(publish_at) = chapter.published {
                    let publish_at = publish_at.format("%b %d, %Y").to_string();
                    console.info(&cformat!("      <s>Published</>: {}", publish_at));
                }
            }
        }

        println!();
    }

    if let Some(credits) = &result.credits {
        let split_credits = credits.split('\n').collect::<Vec<&str>>();
        if split_credits.len() > 1 {
            console.info(&cformat!("  <s>Credits</>:"));
            for credit in split_credits {
                console.info(&cformat!("    {}", credit));
            }
        } else {
            console.info(&cformat!("  <s>Credits</>: {}", credits));
        }
    }

    0
}
