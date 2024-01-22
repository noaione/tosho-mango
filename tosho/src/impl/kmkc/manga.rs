use color_print::cformat;
use tosho_kmkc::{
    constants::BASE_HOST,
    models::{GenreNode, MagazineCategory},
};

use super::super::parser::WeeklyCodeCli;
use crate::{cli::ExitCode, linkify};

use super::common::{do_print_search_information, select_single_account};

pub(crate) async fn kmkc_search(
    query: &str,
    account_id: Option<&str>,
    console: &crate::term::Terminal,
) -> ExitCode {
    let account = select_single_account(account_id);

    if account.is_none() {
        console.warn("Aborted");
        return 1;
    }

    let account = account.unwrap();
    console.info(&cformat!("Searching for <magenta,bold>{}</>...", query));
    let client = super::common::make_client(&account.into());

    let results = client.search(query, Some(50)).await;
    match results {
        Ok(results) => {
            if results.is_empty() {
                console.warn("No results found");
                return 1;
            }

            console.info(&cformat!(
                "Search results (<magenta,bold>{}</> results):",
                results.len()
            ));

            do_print_search_information(results, false, None);

            0
        }
        Err(e) => {
            console.error(&cformat!("Unable to connect to KM: {}", e));
            1
        }
    }
}

pub(crate) async fn kmkc_search_weekly(
    weekday: WeeklyCodeCli,
    account_id: Option<&str>,
    console: &crate::term::Terminal,
) -> ExitCode {
    let account = select_single_account(account_id);

    if account.is_none() {
        console.warn("Aborted");
        return 1;
    }

    let account = account.unwrap();
    console.info(&cformat!(
        "Getting weekly manga for week <magenta,bold>{}</>...",
        weekday.to_name()
    ));
    let client = super::common::make_client(&account.into());

    let results = client.get_weekly().await;
    match results {
        Ok(results) => {
            let mut title_ids_list = vec![];
            for weekly_info in results.contents {
                if weekly_info.weekday == weekday.indexed() {
                    title_ids_list = weekly_info.titles;
                    break;
                }
            }

            let mut titles = vec![];
            for title_id in title_ids_list {
                let find_title = results.titles.iter().find(|t| t.id == title_id);

                if let Some(title) = find_title {
                    titles.push(title.clone());
                }
            }

            if titles.is_empty() {
                console.warn("No results found");
                return 1;
            }

            console.info(&cformat!(
                "Weekday <bold>{}</> results (<magenta,bold>{}</> results):",
                weekday.to_name(),
                titles.len()
            ));

            do_print_search_information(titles, false, None);

            0
        }
        Err(e) => {
            console.error(&cformat!("Unable to connect to KM: {}", e));
            1
        }
    }
}

fn format_tag_name(tag_name: String) -> String {
    let tag_split = tag_name.split('･').collect::<Vec<&str>>();

    let mut tag_name = tag_split.join(" & ");
    if tag_split.len() > 2 {
        // merge everything with comma except the last one
        tag_name = tag_split[..tag_split.len() - 1].join(", ");
        tag_name = format!("{}, and {}", tag_name, tag_split[tag_split.len() - 1]);
    }
    tag_name
}

fn format_tags(tags: Vec<GenreNode>) -> String {
    let parsed_tags = tags
        .iter()
        .map(|tag| {
            cformat!(
                "<p(244),reverse,bold>{}</>",
                format_tag_name(tag.clone().name)
            )
        })
        .collect::<Vec<String>>()
        .join(", ");
    parsed_tags
}

pub(crate) async fn kmkc_title_info(
    title_id: i32,
    account_id: Option<&str>,
    show_chapters: bool,
    console: &crate::term::Terminal,
) -> ExitCode {
    let account = select_single_account(account_id);

    if account.is_none() {
        console.warn("Aborted");
        return 1;
    }

    let account = account.unwrap();
    console.info(&cformat!(
        "Fetching info for ID <magenta,bold>{}</>...",
        title_id
    ));
    let client = super::common::make_client(&account.into());

    let results = client.get_titles(vec![title_id]).await;

    match results {
        Err(e) => {
            console.error(&cformat!("Unable to connect to KM: {}", e));
            1
        }
        Ok(results) => {
            if results.is_empty() {
                console.warn("Unable to find title information.");
                return 1;
            }

            let result = results.first().unwrap();

            let mut genre_results = vec![];
            if !result.genre_ids.is_empty() {
                let genre_resp = client.get_genres().await;
                match genre_resp {
                    Err(e) => {
                        console.error(&cformat!("Unable to connect to KM: {}", e));
                        return 1;
                    }
                    Ok(genre_resp) => {
                        genre_results = genre_resp.genres.clone();
                    }
                }
            }

            let mut chapters_info = vec![];
            if show_chapters {
                console.info(&cformat!(
                    "Fetching <magenta,bold>{}</> chapters information...",
                    result.episode_ids.len()
                ));

                for chunk_eps in result.episode_ids.chunks(50) {
                    let chap_req = client.get_episodes(chunk_eps.to_vec()).await;

                    match chap_req {
                        Err(e) => {
                            console.error(&cformat!("Failed to get chapter information: {}", e));
                            return 1;
                        }
                        Ok(chap_req) => {
                            chapters_info.extend(chap_req);
                        }
                    }
                }
            }

            let manga_url = format!("https://{}/title/{}", BASE_HOST.as_str(), title_id);
            let linked = linkify!(&manga_url, &result.title);

            console.info(&cformat!(
                "Title information for <magenta,bold>{}</>",
                linked,
            ));
            console.info(&cformat!("  <s>Author</>: {}", result.author));

            if !genre_results.is_empty() {
                console.info(&cformat!(
                    "  <s>Genre/Tags</>: {}",
                    format_tags(genre_results)
                ));
            }
            if result.magazine != MagazineCategory::Undefined {
                console.info(&cformat!(
                    "  <s>Magazine</>: {}",
                    result.magazine.pretty_name()
                ));
            }

            console.info(&cformat!("  <s>Summary</>"));
            if !result.summary.is_empty() {
                console.info(&cformat!("   <blue>{}</>", result.summary));
            }
            let split_desc: Vec<&str> = result.description.split('\n').collect();
            for desc in split_desc {
                console.info(&format!("    {}", desc));
            }

            if !result.notice.is_empty() {
                console.warn(&cformat!("  <s>Notice</>: {}", result.notice));
            }

            println!();
            console.info(&cformat!(
                "  <s>Chapters</>: {} chapters",
                result.episode_ids.len()
            ));
            if show_chapters && chapters_info.is_empty() {
                console.warn("   <red,s>Error</>: Unable to get chapters information");
                println!();
            } else if show_chapters && !chapters_info.is_empty() {
                for chapter in chapters_info {
                    let episode_url = format!("{}/episode/{}", manga_url, chapter.id);
                    let ep_linked = linkify!(&episode_url, &chapter.title);

                    let mut text_info = cformat!("    <s>{}</> ({})", ep_linked, chapter.id);
                    match chapter.badge {
                        tosho_kmkc::models::EpisodeBadge::Purchaseable => {
                            if chapter.ticket_rental.into() {
                                let ticket_emote = if console.is_modern() {
                                    "🎫"
                                } else {
                                    "TICKET"
                                };

                                text_info = cformat!(
                                    "{} [<yellow,bold>{} <rev>FREE</rev></>]",
                                    text_info,
                                    ticket_emote
                                );
                            } else {
                                text_info = cformat!(
                                    "{} [<green,bold,reverse>P{}</>]",
                                    text_info,
                                    chapter.point
                                );
                            }
                        }
                        tosho_kmkc::models::EpisodeBadge::Free => {
                            text_info = cformat!("{} [<p(18),bold,reverse>FREE</>]", text_info);
                        }
                        tosho_kmkc::models::EpisodeBadge::Purchased => {
                            text_info = cformat!("{} [<green,bold>Purchased</>]", text_info);
                        }
                        tosho_kmkc::models::EpisodeBadge::Rental => {
                            if let Some(rental_time) = chapter.rental_rest_time {
                                text_info = cformat!(
                                    "{} [<yellow,bold>Renting: {}</>]",
                                    text_info,
                                    rental_time
                                );
                            }
                        }
                    }

                    console.info(&text_info);
                    let st_fmt = chapter.start_time.format("%b %d, %Y");
                    console.info(&cformat!("     <s>Published</>: {}", st_fmt));
                }
                println!();
            }

            if let Some(next_update) = &result.next_update {
                console.info(&cformat!("  <s>Next update</>: {}", next_update));
            }

            0
        }
    }
}

pub(crate) async fn kmkc_magazines_list(
    account_id: Option<&str>,
    console: &crate::term::Terminal,
) -> ExitCode {
    let account = select_single_account(account_id);

    if account.is_none() {
        console.warn("Aborted");
        return 1;
    }

    let account = account.unwrap();
    console.info("Fetching magazines list...");
    let client = super::common::make_client(&account.into());

    let results = client.get_magazines().await;

    match results {
        Err(e) => {
            console.error(&cformat!("Unable to connect to KM: {}", e));
            1
        }
        Ok(results) => {
            if results.categories.is_empty() {
                console.warn("No magazine results found.");
                return 1;
            }

            let mut unknown_ids = vec![];
            for magazine in results.categories {
                if magazine.id == 0 {
                    continue;
                }

                let mag_text = cformat!("<s>{}</> ({})", magazine.name, magazine.id);
                let mag = MagazineCategory::from(magazine.id);

                if mag != MagazineCategory::Undefined {
                    console.info(&cformat!("{} <s>{}</>", mag_text, mag.pretty_name()));

                    let doc_text = mag.get_doc();
                    if let Ok(doc_text) = doc_text {
                        let first_line = doc_text.split('\n').next().unwrap();
                        console.info(&cformat!("  <s>{}</s>", first_line));
                    }
                } else {
                    console.warn(&cformat!("{} <s>Unknown</>", mag_text));
                    unknown_ids.push(magazine.id);
                }
            }

            if !unknown_ids.is_empty() {
                console.warn(&cformat!(
                    "Found <red,bold>{}</> unknown magazine IDs",
                    unknown_ids.len()
                ));
                let unknown_join = unknown_ids
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                console.warn(&cformat!("  {}", unknown_join));
            }

            0
        }
    }
}
