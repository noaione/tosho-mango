use color_print::cformat;
use tosho_mplus::{
    MPClient,
    constants::BASE_HOST,
    helper::SubscriptionPlan,
    proto::{Tag, TitleLanguages, TitleReleaseSchedule},
};

use super::common::{do_print_search_information, get_cached_titles_data, search_manga_by_text};
use crate::{cli::ExitCode, r#impl::common::unix_timestamp_to_string, linkify};

pub(crate) async fn mplus_search(
    query: &str,
    client: &MPClient,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(cformat!("Searching for <magenta,bold>{}</>...", query));

    let results = get_cached_titles_data(client).await;

    match results {
        Ok(results) => {
            if results.is_empty() {
                console.warn("No titles exist");
                return 1;
            }

            let merged_titles: Vec<tosho_mplus::proto::Title> =
                results.iter().flat_map(|x| x.titles().to_vec()).collect();
            let filtered = search_manga_by_text(&merged_titles, query);

            if filtered.is_empty() {
                console.warn("No results found");
                return 1;
            }

            do_print_search_information(&filtered, false, None);

            0
        }
        _ => 1,
    }
}

fn format_tags(tags: &[Tag]) -> String {
    let parsed_tags = tags
        .iter()
        .map(|tag| cformat!("<p(244),reverse,bold>{}</>", tag.slug()))
        .collect::<Vec<String>>()
        .join(", ");
    parsed_tags
}

fn format_other_languages(title_lang: &[TitleLanguages]) -> String {
    let parsed_lang = title_lang
        .iter()
        .map(|lang| {
            let lang_url = format!("https://{}/titles/{}", &*BASE_HOST, lang.id());
            let linked = linkify!(&lang_url, &lang.language().pretty_name());
            cformat!("<p(244),reverse,bold>{}</>", linked)
        })
        .collect::<Vec<String>>()
        .join(", ");
    parsed_lang
}

pub(crate) async fn mplus_title_info(
    title_id: u64,
    show_chapters: bool,
    show_related: bool,
    client: &MPClient,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(cformat!(
        "Fetching info for ID <magenta,bold>{}</>...",
        title_id
    ));

    let result = client.get_title_details(title_id).await;

    match result {
        Ok(tosho_mplus::APIResponse::Success(title_info)) => {
            let title = title_info.title().unwrap();
            let manga_url = format!("https://{}/titles/{}", &*BASE_HOST, title.id());
            let linked = linkify!(manga_url, title.title());

            console.info(cformat!(
                "Title information for <magenta,bold>{}</>",
                linked,
            ));

            let mut merged_labels = Vec::new();
            let mut title_plan_type = SubscriptionPlan::Basic;
            if let Some(title_labels) = title_info.title_labels() {
                if title_labels.release_schedule() != TitleReleaseSchedule::None {
                    merged_labels.push(cformat!(
                        "<y!,bold>[<rev>{}</rev>]</y!,bold>",
                        title_labels.release_schedule().pretty_name()
                    ));
                }
                if title_labels.simulpublish() {
                    merged_labels.push(cformat!("<r!,bold>[<rev>Simulpub</rev>]</r!,bold>"));
                }
                match title_labels.plan_type() {
                    SubscriptionPlan::Basic => merged_labels.push(cformat!("[<bold>Basic</>]")),
                    SubscriptionPlan::Standard => merged_labels.push(cformat!(
                        "<c!,bold>[<rev>Standard / Deluxe</rev>]</c!,bold>"
                    )),
                    SubscriptionPlan::Deluxe => {
                        merged_labels.push(cformat!("<m!,bold>[<rev>Deluxe</rev>]</m!,bold>"))
                    }
                }
                title_plan_type = title_labels.plan_type();
            }

            if !merged_labels.is_empty() {
                console.info(cformat!("  {}", merged_labels.join(" ")));
            }

            console.info(cformat!("  <s>Author</>: {}", title.author()));
            if !title_info.tags().is_empty() {
                console.info(cformat!(
                    "  <s>Genre/Tags</>: {}",
                    format_tags(title_info.tags())
                ));
            }
            console.info(cformat!(
                "  <s>Language</>: {}",
                title.language().pretty_name()
            ));
            let filtered_alt_langs = title_info
                .other_languages()
                .iter()
                .filter_map(|x| if x.id() != title.id() { Some(*x) } else { None })
                .collect::<Vec<TitleLanguages>>();
            if !filtered_alt_langs.is_empty() {
                console.info(cformat!(
                    "   <s>Alt Languages</>: {}",
                    format_other_languages(&filtered_alt_langs)
                ));
            }
            console.info(cformat!("  <s>Summary</>"));
            let split_desc = title_info.overview().split('\n');
            for desc in split_desc {
                console.info(format!("    {}", desc));
            }

            println!();
            let all_chapters = title_info.flat_chapters_group();
            console.info(cformat!(
                "  <s>Chapters</>: {} chapters",
                all_chapters.len()
            ));

            let title_ticket: Vec<u64> = title_info
                .ticket_chapters()
                .iter()
                .map(|x| x.chapter_id())
                .collect();

            let user_plan = title_info
                .user_subscription()
                .map(|d| d.plan())
                .unwrap_or(SubscriptionPlan::Basic);

            if show_chapters && !all_chapters.is_empty() {
                for chapter in all_chapters {
                    let ch_url = format!("https://{}/viewer/{}", &*BASE_HOST, chapter.chapter_id());
                    let linked_ch = linkify!(ch_url, chapter.title());
                    let mut base_txt =
                        cformat!("    <s>{}</> ({})", linked_ch, chapter.chapter_id());
                    base_txt =
                        if chapter.is_ticketed() || title_ticket.contains(&chapter.chapter_id()) {
                            cformat!("{} <y,strong>[Ticket]</y,strong>", base_txt)
                        } else if chapter.is_free() {
                            cformat!("{} <g,strong>[FREE]</g,strong>", base_txt)
                        } else if user_plan >= title_plan_type {
                            cformat!("{} [<c!,strong>Subscription</>]", base_txt)
                        } else {
                            cformat!("{} [<r!,strong>Locked</>]", base_txt)
                        };

                    console.info(&base_txt);
                    if !chapter.subtitle().is_empty() {
                        console.info(cformat!("     <s>{}</>", chapter.subtitle()));
                    }

                    if let Some(pub_at_fmt) = unix_timestamp_to_string(chapter.published_at()) {
                        console.info(cformat!("      <s>Published</>: {}", pub_at_fmt));
                    }
                }

                println!();
            }

            if title_info.next_update() > 0 {
                if let Some(next_upd_fmt) = unix_timestamp_to_string(title_info.next_update()) {
                    console.info(cformat!("  <s>Next Update</>: {}", next_upd_fmt));
                }
            }

            console.info(cformat!("  <s>Your Plan</>: {}", user_plan.to_name()));

            if show_related && !title_info.recommended_titles().is_empty() {
                println!();
                console.info(cformat!(
                    "  <s>Related titles</>: {} titles",
                    title_info.recommended_titles().len()
                ));

                do_print_search_information(title_info.recommended_titles(), false, Some(3));
            }

            0
        }
        Ok(tosho_mplus::APIResponse::Error(e)) => {
            console.error(format!("Failed to get title info: {}", e.as_string()));
            1
        }
        Err(e) => {
            console.error(format!("Unable to connect to M+: {}", e));
            1
        }
    }
}
