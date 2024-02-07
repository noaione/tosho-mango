use color_print::cformat;
use num_format::{Locale, ToFormattedString};
use tosho_musq::{
    constants::BASE_HOST,
    proto::{BadgeManga, ChapterV2, LabelBadgeManga, MangaDetailV2, MangaResultNode, UserPoint},
    MUClient,
};

use crate::{
    linkify,
    term::{get_console, ConsoleChoice},
};

pub(super) fn do_print_search_information(
    results: Vec<MangaResultNode>,
    with_number: bool,
    spacing: Option<usize>,
) {
    let term = get_console(0);
    let spacing = spacing.unwrap_or(2);

    for (idx, result) in results.iter().enumerate() {
        let id = result.id;
        let manga_url = format!("https://{}/manga/{}", BASE_HOST.as_str(), result.id);
        let linked = linkify!(&manga_url, &result.title);
        let mut text_data = color_print::cformat!("<s>{}</s> ({})", linked, id);

        text_data = match result.badge() {
            BadgeManga::New => cformat!("{} <c!,rev,strong>[NEW]</c!,rev,strong>", text_data),
            BadgeManga::Unread => cformat!("{} <b,rev,strong>●</b,rev,strong>", text_data),
            BadgeManga::Update => cformat!("{} <g,rev,strong>UP</g,rev,strong>", text_data),
            BadgeManga::UpdateWeek => {
                cformat!("{} <y,rev,strong>UP (Week)</y,rev,strong>", text_data)
            }
            _ => text_data,
        };

        text_data = match result.label_badge() {
            LabelBadgeManga::Original => {
                cformat!(
                    "{} [<b!,strong><reverse>MU!</reverse> Original</>]",
                    text_data
                )
            }
            _ => text_data,
        };
        let pre_space = " ".repeat(spacing);
        let pre_space_url = " ".repeat(spacing + 1);

        if with_number {
            term.info(&format!("{}[{:02}] {}", pre_space, idx + 1, text_data));
        } else {
            term.info(&format!("{}{}", pre_space, text_data));
        }
        term.info(&format!("{}{}", pre_space_url, manga_url));
    }
}

pub(super) async fn common_purchase_select(
    title_id: u64,
    client: &MUClient,
    download_mode: bool,
    show_all: bool,
    no_input: bool,
    console: &crate::term::Terminal,
) -> (
    anyhow::Result<Vec<ChapterV2>>,
    Option<MangaDetailV2>,
    Option<UserPoint>,
) {
    console.info(&cformat!(
        "Fetching for ID <magenta,bold>{}</>...",
        title_id
    ));

    let results = client.get_manga(title_id).await;
    match results {
        Ok(result) => {
            let user_bal = result.user_point.clone().unwrap();
            let total_bal = user_bal.sum().to_formatted_string(&Locale::en);
            let paid_point = user_bal.paid.to_formatted_string(&Locale::en);
            let xp_point = user_bal.event.to_formatted_string(&Locale::en);
            let free_point = user_bal.free.to_formatted_string(&Locale::en);

            console.info("Your current point balance:");
            console.info(&cformat!("  - <s>Total</>: {}", total_bal));
            console.info(&cformat!("  - <s>Paid point</>: {}c", paid_point));
            console.info(&cformat!("  - <s>Event/XP point</>: {}c", xp_point));
            console.info(&cformat!("  - <s>Free point</>: {}c", free_point));

            console.info("Title information:");
            console.info(&cformat!("  - <s>ID</>: {}", title_id));
            console.info(&cformat!("  - <s>Title</>: {}", result.title));
            console.info(&cformat!("  - <s>Chapters</>: {}", result.chapters.len()));

            if no_input {
                return (
                    Ok(result.chapters.clone()),
                    Some(result.clone()),
                    Some(user_bal),
                );
            }

            let select_choices: Vec<ConsoleChoice> = result
                .chapters
                .iter()
                .filter_map(|ch| {
                    if download_mode && !show_all && !ch.is_free() {
                        None
                    } else {
                        let value = if ch.is_free() {
                            ch.title.clone()
                        } else {
                            format!("{} ({}c)", ch.title, ch.price)
                        };
                        Some(ConsoleChoice {
                            name: ch.id.to_string(),
                            value,
                        })
                    }
                })
                .collect();

            if select_choices.is_empty() {
                console.warn("No chapters selected, aborting...");

                return (Ok(vec![]), None, Some(user_bal));
            }

            let sel_prompt = if download_mode {
                "Select chapter to download"
            } else {
                "Select chapter to purchase"
            };
            let selected = console.select(sel_prompt, select_choices);

            match selected {
                Some(selected) => {
                    if selected.is_empty() {
                        console.warn("No chapter selected, aborting...");

                        return (Ok(vec![]), None, Some(user_bal));
                    }

                    let mut selected_chapters: Vec<ChapterV2> = vec![];

                    for chapter in selected {
                        let ch_id = chapter.name.parse::<u64>().unwrap();
                        let ch = result
                            .chapters
                            .iter()
                            .find(|ch| ch.id == ch_id)
                            .unwrap()
                            .clone();

                        selected_chapters.push(ch);
                    }

                    (Ok(selected_chapters), Some(result), Some(user_bal))
                }
                None => {
                    console.warn("Aborted");
                    (
                        Err(anyhow::anyhow!("Aborted")),
                        Some(result.clone()),
                        Some(user_bal),
                    )
                }
            }
        }
        Err(e) => {
            console.error(&cformat!("Unable to connect to MU!: {}", e));

            (Err(e), None, None)
        }
    }
}
