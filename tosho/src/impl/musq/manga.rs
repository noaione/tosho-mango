use color_print::cformat;
use tosho_musq::{
    constants::BASE_HOST,
    proto::{ConsumptionType, Tag},
    MUClient, WeeklyCode,
};

use crate::{cli::ExitCode, linkify};

use super::common::do_print_search_information;

pub(crate) async fn musq_search(
    query: &str,
    client: &MUClient,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(&cformat!("Searching for <magenta,bold>{}</>...", query));

    let results = client.search(query).await;
    match results {
        Ok(results) => {
            if results.titles.is_empty() {
                console.warn("No results found");
                return 1;
            }

            // Cut to first 25 results
            let cutoff_results = if results.titles.len() > 25 {
                results.titles[..25].to_vec()
            } else {
                results.titles
            };

            console.info(&cformat!(
                "Search results (<magenta,bold>{}</> results):",
                cutoff_results.len()
            ));

            do_print_search_information(cutoff_results, false, None);

            0
        }
        Err(e) => {
            console.error(&cformat!("Unable to connect to MU!: {}", e));
            1
        }
    }
}

pub(crate) async fn musq_search_weekly(
    weekday: WeeklyCode,
    client: &MUClient,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(&cformat!(
        "Getting weekly manga for week <magenta,bold>{}</>...",
        weekday.to_name()
    ));

    let results = client.get_weekly_titles(weekday).await;
    match results {
        Ok(results) => {
            if results.titles.is_empty() {
                console.warn("No results found");
                return 1;
            }

            console.info(&cformat!(
                "Weekday <bold>{}</> results (<magenta,bold>{}</> results):",
                weekday.to_name(),
                results.titles.len()
            ));

            do_print_search_information(results.titles, false, None);

            0
        }
        Err(e) => {
            console.error(&cformat!("Unable to connect to MU!: {}", e));
            1
        }
    }
}

fn format_tags(tags: Vec<Tag>) -> String {
    let parsed_tags = tags
        .iter()
        .map(|tag| {
            let tag_url = format!("https://{}/genre/{}", BASE_HOST.as_str(), tag.id);
            let linked = linkify!(&tag_url, &tag.name);

            cformat!("<p(244),reverse,bold>{}</>", linked)
        })
        .collect::<Vec<String>>()
        .join(", ");
    parsed_tags
}

pub(crate) async fn musq_title_info(
    title_id: u64,
    show_chapters: bool,
    show_related: bool,
    client: &MUClient,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(&cformat!(
        "Fetching info for ID <magenta,bold>{}</>...",
        title_id
    ));

    let result = client.get_manga(title_id).await;

    match result {
        Err(e) => {
            console.error(&cformat!("Unable to connect to MU!: {}", e));
            1
        }
        Ok(result) => {
            let manga_url = format!("https://{}/manga/{}", BASE_HOST.as_str(), title_id);
            let linked = linkify!(&manga_url, &result.title);

            console.info(&cformat!(
                "Title information for <magenta,bold>{}</>",
                linked,
            ));

            console.info(&cformat!("  <s>Author</>: {}", result.authors));
            console.info(&cformat!(
                "  <s>Genre/Tags</>: {}",
                format_tags(result.tags.clone())
            ));
            console.info(&cformat!("  <s>Summary</>"));
            let split_desc = result.description.split('\n');
            for desc in split_desc {
                console.info(&format!("    {}", desc));
            }

            if !result.warning().is_empty() {
                console.warn(&cformat!("  <s>Warning</>: {}", result.warning()));
            }
            println!();
            console.info(&cformat!(
                "  <s>Chapters</>: {} chapters",
                result.chapters.len()
            ));

            if show_chapters {
                for chapter in result.chapters.clone() {
                    let mut base_txt = cformat!("    <s>{}</> ({})", chapter.title, chapter.id);
                    if chapter.is_free() {
                        match chapter.consumption() {
                            ConsumptionType::Subscription => {
                                base_txt =
                                    cformat!("{} <y,strong,rev>[SUBS]</y,strong,rev>", base_txt);
                            }
                            ConsumptionType::Free => {
                                base_txt = cformat!("{} <b,strong>[FREE]</b,strong>", base_txt);
                            }
                            _ => {}
                        }
                        base_txt = cformat!("{} <g,strong>[FREE]</g,strong>", base_txt);
                    } else {
                        base_txt = cformat!("{} [<y,strong>{}</>c]", base_txt, chapter.price);
                    }
                    console.info(&base_txt);

                    if chapter.subtitle.is_some() {
                        console.info(&cformat!("     <s>{}</>", chapter.subtitle.unwrap()));
                    }
                    if chapter.published_at.is_some() {
                        console.info(&cformat!(
                            "      <s>Published</>: {}",
                            chapter.published_at.unwrap()
                        ));
                    }
                    console.info(&cformat!("      <s>Price</>: {}c", chapter.price));
                }
                println!();
            }

            if !result.next_update().is_empty() {
                console.info(&cformat!("  <s>Next update</>: {}", result.next_update()));
            }

            let trim_copyright = result.copyright.trim();

            if !trim_copyright.is_empty() {
                let copyrights: Vec<&str> = trim_copyright.split('\n').collect();
                console.info(&cformat!("  <s>Copyright</>: {}", copyrights[0]));

                for copyr in copyrights.iter().skip(1) {
                    console.info(&format!("             {}", copyr));
                }
            }

            if show_related && !result.related_manga.is_empty() {
                println!();
                console.info(&cformat!(
                    "  <s>Related manga</>: {} titles",
                    result.related_manga.len()
                ));

                do_print_search_information(result.related_manga, false, Some(3));
            }

            0
        }
    }
}
