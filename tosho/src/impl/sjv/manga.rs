use color_print::cformat;
use tosho_sjv::{
    constants::{EXPAND_SJ_NAME, EXPAND_VM_NAME},
    models::MangaImprint,
    SJClient,
};

use crate::{
    cli::ExitCode,
    linkify,
    r#impl::{
        parser::NumberOrString,
        sjv::common::{sort_chapters, unix_timestamp_to_string},
    },
};

use super::common::{do_print_search_information, get_cached_store_data, search_manga_by_text};

pub(crate) async fn sjv_search(
    query: &str,
    client: &SJClient,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(&cformat!("Searching for <magenta,bold>{}</>...", query));

    let results = get_cached_store_data(client).await;

    match results {
        Ok(results) => {
            if results.series.is_empty() {
                console.warn("No series exist");
                return 1;
            }

            let filtered = search_manga_by_text(&results.series, query);

            let filtered: Vec<tosho_sjv::models::MangaDetail> =
                filtered.iter().map(|&x| x.clone()).collect();

            if filtered.is_empty() {
                console.warn("No match found");
                return 1;
            }

            console.info(&cformat!(
                "Search results (<magenta,bold>{}</> results):",
                filtered.len()
            ));

            do_print_search_information(&filtered, false, None);

            0
        }
        Err(e) => {
            console.error(&format!("Failed to fetch cached store: {}", e));

            1
        }
    }
}

pub(crate) async fn sjv_title_info(
    title_or_slug: NumberOrString,
    show_chapters: bool,
    client: &SJClient,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(&cformat!(
        "Fetching info for <magenta,bold>{}</>...",
        title_or_slug
    ));

    let results = get_cached_store_data(client).await;

    match results {
        Ok(results) => {
            if results.series.is_empty() {
                console.warn("No series exist");
                return 1;
            }

            let title = results.series.iter().find(|x| {
                if let NumberOrString::Number(n) = title_or_slug {
                    x.id == n as u32
                } else {
                    x.slug == title_or_slug.to_string()
                }
            });

            if title.is_none() {
                console.warn("No series found");
                return 1;
            }

            let result = title.unwrap();

            let manga_url = format!(
                "https://{}/{}",
                tosho_sjv::constants::BASE_HOST.as_str(),
                result.slug
            );
            let linked = linkify!(&manga_url, &result.title);
            console.info(&cformat!(
                "Title information for <magenta,bold>{}</>",
                linked,
            ));

            if let Some(author) = &result.author {
                console.info(&cformat!("  <s>Author</>: {}", author));
            }
            if result.imprint != MangaImprint::Undefined {
                console.info(&cformat!(
                    "  <s>Imprint</>: {}",
                    result.imprint.pretty_name()
                ));
            }

            console.info(&cformat!("  <s>Rating</>: {}", result.rating.to_name()));
            console.info(&cformat!("  <s>Copyright</>: {}", result.copyright));
            let synopsis = result.synopsis.replace("\r\n", "\n");
            let synopsis = synopsis.trim();
            let split_desc = synopsis.split('\n');
            console.info(&cformat!("  <s>Summary</>"));
            if let Some(tagline) = &result.tagline {
                console.info(&cformat!("   <blue>{}</>", tagline));
            }
            for desc in split_desc {
                console.info(&format!("    {}", desc));
            }

            println!();
            console.info(&cformat!(
                "  <s>Chapters</>: {} chapters",
                result.total_chapters
            ));
            if result.total_volumes > 0 {
                console.info(&cformat!(
                    "   <s>Volumes</>: {} volumes",
                    result.total_volumes
                ));
            }

            if show_chapters && result.total_chapters > 0 {
                let chapters_info = client.get_chapters(result.id).await;
                match chapters_info {
                    Err(_) => {
                        console.warn(&cformat!(
                            "   <red,s>Error</>: Unable to get chapters information"
                        ));
                        println!();
                    }
                    Ok(mut chapters_info) => {
                        sort_chapters(&mut chapters_info, true);
                        for chapter in chapters_info {
                            let episode_url = match chapter.subscription_type {
                                Some(tosho_sjv::models::SubscriptionType::SJ) => {
                                    format!(
                                        "https://{}/{}/{}/chapter/{}?action=read",
                                        *tosho_sjv::constants::BASE_HOST,
                                        *EXPAND_SJ_NAME,
                                        result.slug,
                                        chapter.id
                                    )
                                }
                                Some(tosho_sjv::models::SubscriptionType::VM) => {
                                    format!(
                                        "https://{}/{}/{}/chapter/{}?action=read",
                                        *tosho_sjv::constants::BASE_HOST,
                                        *EXPAND_VM_NAME,
                                        result.slug,
                                        chapter.id
                                    )
                                }
                                None => linked.clone(),
                            };

                            // Skip for now
                            if chapter.chapter.is_none() {
                                continue;
                            }

                            let ep_linked = linkify!(&episode_url, &chapter.pretty_title());

                            console.info(&cformat!("    <s>{}</> ({})", ep_linked, chapter.id));
                            console.info(&cformat!("     {}", episode_url));

                            let created_at = chapter.created_at.format("%b %d, %Y").to_string();
                            console.info(&cformat!("     <s>Published</>: {}", created_at));
                            if let Some(expiry_at) = chapter.expiry_at {
                                let expiry_at = unix_timestamp_to_string(expiry_at)
                                    .unwrap_or("N/A".to_string());
                                console.info(&cformat!("      <s>Expires</>: {}", expiry_at));
                            }
                        }
                    }
                }
            }

            0
        }
        Err(e) => {
            console.error(&format!("Failed to fetch cached store: {}", e));

            1
        }
    }
}
