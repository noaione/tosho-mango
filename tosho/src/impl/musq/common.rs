use color_eyre::eyre::{Context, OptionExt};
use color_print::cformat;
use num_format::{Locale, ToFormattedString};
use tosho_macros::AutoGetter;
use tosho_musq::{
    MUClient,
    constants::BASE_HOST,
    proto::{BadgeManga, ChapterV2, LabelBadgeManga, MangaDetailV2, MangaResultNode, UserPoint},
};

use crate::{
    linkify,
    term::{ConsoleChoice, get_console},
};

pub(super) fn do_print_search_information(
    results: &[MangaResultNode],
    with_number: bool,
    spacing: Option<usize>,
) {
    let term = get_console(0);
    let spacing = spacing.unwrap_or(2);

    for (idx, result) in results.iter().enumerate() {
        let manga_url = format!("https://{}/manga/{}", BASE_HOST, result.id());
        let linked = linkify!(&manga_url, result.title());
        let mut text_data = color_print::cformat!("<s>{}</s> ({})", linked, result.id());

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
            term.info(format!("{}[{:02}] {}", pre_space, idx + 1, text_data));
        } else {
            term.info(format!("{pre_space}{text_data}"));
        }
        term.info(format!("{pre_space_url}{manga_url}"));
    }
}

#[derive(Clone, AutoGetter)]
pub(super) struct MUPurchaseResult {
    chapters: Vec<ChapterV2>,
    title: MangaDetailV2,
    balance: UserPoint,
}

impl MUPurchaseResult {
    fn from_result(result: &MangaDetailV2, balance: &UserPoint) -> Self {
        Self {
            chapters: result.chapters().to_vec(),
            title: result.clone(),
            balance: *balance,
        }
    }

    fn from_result_custom(
        episodes: Vec<ChapterV2>,
        title: &MangaDetailV2,
        balance: &UserPoint,
    ) -> Self {
        Self {
            chapters: episodes,
            title: title.clone(),
            balance: *balance,
        }
    }
}

pub(super) async fn common_purchase_select(
    title_id: u64,
    client: &MUClient,
    download_mode: bool,
    show_all: bool,
    no_input: bool,
    console: &crate::term::Terminal,
) -> color_eyre::Result<MUPurchaseResult> {
    console.info(cformat!("Fetching for ID <magenta,bold>{}</>...", title_id));

    let result = client
        .get_manga(title_id)
        .await
        .context("Unable to connect to MU!")?;
    let user_bal = result.user_point().unwrap_or_default();
    let total_bal = user_bal.sum().to_formatted_string(&Locale::en);
    let paid_point = user_bal.paid().to_formatted_string(&Locale::en);
    let xp_point = user_bal.event().to_formatted_string(&Locale::en);
    let free_point = user_bal.free().to_formatted_string(&Locale::en);

    console.info("Your current point balance:");
    console.info(cformat!("  - <s>Total</>: {}", total_bal));
    console.info(cformat!("  - <s>Paid point</>: {}c", paid_point));
    console.info(cformat!("  - <s>Event/XP point</>: {}c", xp_point));
    console.info(cformat!("  - <s>Free point</>: {}c", free_point));

    console.info("Title information:");
    console.info(cformat!("  - <s>ID</>: {}", title_id));
    console.info(cformat!("  - <s>Title</>: {}", result.title()));
    console.info(cformat!("  - <s>Chapters</>: {}", result.chapters().len()));

    if no_input {
        return Ok(MUPurchaseResult::from_result(&result, &user_bal));
    }

    let select_choices: Vec<ConsoleChoice> = result
        .chapters()
        .iter()
        .filter_map(|ch| {
            if download_mode && !show_all && !ch.is_free() {
                None
            } else {
                let value = if ch.is_free() {
                    ch.title().to_string()
                } else {
                    format!("{} ({}c)", ch.title(), ch.price())
                };
                Some(ConsoleChoice {
                    name: ch.id().to_string(),
                    value,
                })
            }
        })
        .collect();

    if select_choices.is_empty() {
        console.warn("No chapters selected, aborting...");
        return Err(color_eyre::eyre::eyre!(
            "No chapters to be selected, aborting"
        ));
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

                return Err(color_eyre::eyre::eyre!(
                    "No chapters to be selected, aborting"
                ));
            }

            let mut selected_chapters: Vec<ChapterV2> = vec![];

            for chapter in selected {
                let ch_id = chapter.name.parse::<u64>()?;
                let ch = result
                    .chapters()
                    .iter()
                    .find(|ch| ch.id() == ch_id)
                    .ok_or_eyre(format!("Failed to find chapter ID: {ch_id}"))?
                    .clone();

                selected_chapters.push(ch);
            }

            Ok(MUPurchaseResult::from_result_custom(
                selected_chapters,
                &result,
                &user_bal,
            ))
        }
        None => {
            console.warn("Aborted");
            Err(color_eyre::eyre::eyre!("Aborted by user"))
        }
    }
}
