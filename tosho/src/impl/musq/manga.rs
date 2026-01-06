use color_eyre::eyre::Context;
use color_print::cformat;
use tosho_musq::{
    MUClient, WeeklyCode,
    constants::BASE_HOST,
    proto::{ConsumptionType, Tag},
};

use crate::linkify;

use super::common::do_print_search_information;

pub(crate) async fn musq_search(
    query: &str,
    client: &MUClient,
    console: &crate::term::Terminal,
) -> color_eyre::Result<()> {
    console.info(cformat!("Searching for <magenta,bold>{}</>...", query));

    let results = client
        .search(query)
        .await
        .context("Unable to connect to MU!")?;

    if results.titles().is_empty() {
        console.warn("No results found");
        return Err(color_eyre::eyre::eyre!("No results found"));
    }

    // Cut to first 25 results
    let cutoff_results = if results.titles().len() > 25 {
        &results.titles()[..25]
    } else {
        results.titles()
    };

    console.info(cformat!(
        "Search results (<magenta,bold>{}</> results):",
        cutoff_results.len()
    ));

    do_print_search_information(cutoff_results, false, None);

    Ok(())
}

pub(crate) async fn musq_search_weekly(
    weekday: WeeklyCode,
    client: &MUClient,
    console: &crate::term::Terminal,
) -> color_eyre::Result<()> {
    console.info(cformat!(
        "Getting weekly manga for week <magenta,bold>{}</>...",
        weekday.to_name()
    ));

    let results = client
        .get_weekly_titles(weekday)
        .await
        .context("Unable to connect to MU!")?;

    if results.titles().is_empty() {
        console.warn("No results found");
        return Err(color_eyre::eyre::eyre!("No results found"));
    }

    console.info(cformat!(
        "Weekday <bold>{}</> results (<magenta,bold>{}</> results):",
        weekday.to_name(),
        results.titles().len()
    ));

    do_print_search_information(results.titles(), false, None);

    Ok(())
}

fn format_tags(tags: &[Tag]) -> String {
    tags.iter()
        .map(|tag| {
            let tag_url = format!("https://{}/genre/{}", BASE_HOST, tag.id());
            let linked = linkify!(&tag_url, &tag.name());

            cformat!("<p(244),reverse,bold>{}</>", linked)
        })
        .collect::<Vec<String>>()
        .join(", ")
}

pub(crate) async fn musq_title_info(
    title_id: u64,
    show_chapters: bool,
    show_related: bool,
    client: &MUClient,
    console: &crate::term::Terminal,
) -> color_eyre::Result<()> {
    console.info(cformat!(
        "Fetching info for ID <magenta,bold>{}</>...",
        title_id
    ));

    let result = client
        .get_manga(title_id)
        .await
        .context("Failed to fetch title info")?;

    let manga_url = format!("https://{}/manga/{}", BASE_HOST, title_id);
    let linked = linkify!(&manga_url, &result.title());

    console.info(cformat!(
        "Title information for <magenta,bold>{}</>",
        linked,
    ));

    console.info(cformat!("  <s>Author</>: {}", result.authors()));
    console.info(cformat!(
        "  <s>Genre/Tags</>: {}",
        format_tags(result.tags())
    ));
    console.info(cformat!("  <s>Summary</>"));
    let split_desc = result.description().split('\n');
    for desc in split_desc {
        console.info(format!("    {desc}"));
    }

    if !result.warning().is_empty() {
        console.warn(cformat!("  <s>Warning</>: {}", result.warning()));
    }
    println!();
    console.info(cformat!(
        "  <s>Chapters</>: {} chapters",
        result.chapters().len()
    ));

    if show_chapters {
        for chapter in result.chapters() {
            let mut base_txt = cformat!("    <s>{}</> ({})", chapter.title(), chapter.id());
            if chapter.is_free() {
                match chapter.consumption() {
                    ConsumptionType::Subscription => {
                        base_txt = cformat!("{} <y,strong>[<rev>SUBS</rev>]</y,strong>", base_txt);
                    }
                    ConsumptionType::Free => {
                        base_txt = cformat!("{} <b,strong>[FREE]</b,strong>", base_txt);
                    }
                    _ => {}
                }
            } else {
                base_txt = cformat!("{} [<y,strong>{}</>c]", base_txt, chapter.price());
            }
            console.info(&base_txt);

            if !chapter.subtitle().is_empty() {
                console.info(cformat!("     <s>{}</>", chapter.subtitle()));
            }
            if !chapter.published_at().is_empty() {
                console.info(cformat!(
                    "      <s>Published</>: {}",
                    chapter.published_at()
                ));
            }
        }
        println!();
    }

    if !result.next_update().is_empty() {
        console.info(cformat!("  <s>Next update</>: {}", result.next_update()));
    }

    let trim_copyright = result.copyright().trim();

    if !trim_copyright.is_empty() {
        let copyrights: Vec<&str> = trim_copyright.split('\n').collect();
        console.info(cformat!("  <s>Copyright</>: {}", copyrights[0]));

        for copyr in copyrights.iter().skip(1) {
            console.info(format!("             {copyr}"));
        }
    }

    if show_related && !result.related_manga().is_empty() {
        println!();
        console.info(cformat!(
            "  <s>Related manga</>: {} titles",
            result.related_manga().len()
        ));

        do_print_search_information(result.related_manga(), false, Some(3));
    }

    Ok(())
}
