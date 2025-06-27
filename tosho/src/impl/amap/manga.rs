use color_print::cformat;
use tosho_amap::{AMClient, constants::BASE_HOST, models::ComicTagInfo};

use crate::{cli::ExitCode, linkify};

use super::{common::do_print_search_information, config::Config};
use crate::r#impl::common::unix_timestamp_to_string;

pub(crate) async fn amap_search(
    query: &str,
    client: &AMClient,
    acc_info: &Config,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(cformat!("Searching for <magenta,bold>{}</>...", query));
    let results = client.search(query, None, None, None, None).await;

    match results {
        Ok(results) => {
            super::common::save_session_config(client, acc_info);

            if results.comics().is_empty() {
                console.warn("No results found");
                return 1;
            }

            console.info(cformat!(
                "Search results (<magenta,bold>{}</> results):",
                results.comics().len()
            ));

            do_print_search_information(results.comics(), false, None);

            0
        }
        Err(e) => {
            console.error(format!("Failed to fetch balance: {e}"));

            1
        }
    }
}

fn format_tags(tags: &[ComicTagInfo]) -> String {
    tags.iter()
        .map(|tag| cformat!("<p(244),reverse,bold>{}</>", tag.info().name()))
        .collect::<Vec<String>>()
        .join(", ")
}

pub(crate) async fn amap_title_info(
    title_id: u64,
    show_chapters: bool,
    client: &AMClient,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(cformat!(
        "Fetching info for ID <magenta,bold>{}</>...",
        title_id
    ));
    let results = client.get_comic(title_id).await;

    match results {
        Ok(results) => {
            let info = results.info();
            let manga_url = format!("https://{}/manga/{}", &*BASE_HOST, title_id);
            let linked = linkify!(&manga_url, info.title());

            console.info(cformat!(
                "Title information for <magenta,bold>{}</>:",
                linked
            ));

            let mapped_authors = info
                .authors()
                .iter()
                .map(|a| (a.info().kind().to_string(), a.info().name().to_string()))
                .collect::<Vec<(String, String)>>();

            console.info(cformat!("  <s>Authors</>:"));
            for (kind, name) in mapped_authors.iter() {
                console.info(cformat!("    - <s>{}</>: {}", kind, name));
            }

            console.info(cformat!("  <s>Tags</>: {}", format_tags(info.tags())));
            console.info(cformat!("  <s>Status</>: {}", info.status().to_name()));

            console.info(cformat!("  <s>Summary</>"));
            console.info(format!("   {}", info.description()));

            println!();

            console.info(cformat!(
                "  <s>Chapters</>: {} chapters",
                info.episodes().len()
            ));

            if show_chapters {
                for episode in info.episodes().iter() {
                    let mut base_info = cformat!(
                        "    <s>{}</> ({})",
                        episode.info().title(),
                        episode.info().id()
                    );
                    // add ticket price
                    if episode.info().price() > 0 {
                        base_info =
                            cformat!("{} [<r!,strong>{}</>T]", base_info, episode.info().price());

                        if episode.info().is_free_daily() {
                            base_info = cformat!(
                                "{} <g!,strong>[<rev>Free Daily</rev>]</g!,strong>",
                                base_info
                            );
                        }
                    } else {
                        base_info =
                            cformat!("{} <b!,strong>[<rev>Free</rev>]</b!,strong>", base_info);
                    }
                    console.info(&base_info);
                    if let Some(update_at) =
                        unix_timestamp_to_string(episode.info().update_date() as i64)
                    {
                        console.info(cformat!("     Published at: <s>{}</>", update_at));
                    }
                    if let Some(expiry_time) = episode.info().expiry_time() {
                        if let Some(expiry_time) = unix_timestamp_to_string(expiry_time as i64) {
                            console.info(cformat!("      Expires at: <s>{}</>", expiry_time));
                        }
                    }
                }
            }

            println!();

            let prod_binding = info.productions().replace("\r\n", "\n");
            let prod_participants = prod_binding.split('\n');
            console.info(cformat!("  <s>Production Participants</>"));
            for prod in prod_participants {
                console.info(format!("    - {prod}"));
            }

            if let Some(free_daily) = info.free_daily() {
                let is_free_daily = if info.has_free_daily() { "Yes" } else { "No" };
                console.info(cformat!("  <s>Free Daily</>: {}", is_free_daily));
                console.info(cformat!("    - <s>Term:</> {}", free_daily.term()));
                if let Some(next_reload) = unix_timestamp_to_string(free_daily.next() as i64) {
                    console.info(cformat!("    - <s>Next Reload:</> {}", next_reload));
                }
            }
            if let Some(rental_term) = info.rental_term() {
                console.info(cformat!("  <s>Rental Term</>: {}", rental_term));
            }

            0
        }
        Err(e) => {
            console.error(format!("Failed to fetch title info: {e}"));

            1
        }
    }
}
