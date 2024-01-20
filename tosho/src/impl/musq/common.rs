use color_print::cformat;
use num_format::{Locale, ToFormattedString};
use tosho_musq::{
    constants::{get_constants, BASE_HOST},
    proto::{BadgeManga, ChapterV2, LabelBadgeManga, MangaDetailV2, MangaResultNode, UserPoint},
    MUClient,
};

use crate::{
    config::{get_all_config, get_config},
    linkify,
    term::{get_console, ConsoleChoice},
};

use super::config::Config;

pub(super) fn select_single_account(account_id: Option<&str>) -> Option<Config> {
    let term = get_console(0);

    if let Some(account_id) = account_id {
        let config = get_config(account_id, crate::r#impl::Implementations::Musq, None);

        if let Some(config) = config {
            return match config {
                crate::config::ConfigImpl::Kmkc(_) => unreachable!(),
                crate::config::ConfigImpl::Musq(c) => Some(c),
            };
        }

        term.warn(&format!("Account ID {} not found!", account_id));
    }

    let all_configs = get_all_config(crate::r#impl::Implementations::Musq, None);
    let all_choices: Vec<ConsoleChoice> = all_configs
        .iter()
        .filter_map(|c| match c {
            crate::config::ConfigImpl::Kmkc(_) => None,
            crate::config::ConfigImpl::Musq(c) => Some(ConsoleChoice {
                name: c.id.clone(),
                value: format!("{} [{}]", c.id, c.r#type().to_name()),
            }),
        })
        .collect();

    if all_configs.is_empty() {
        term.warn("No accounts found!");
        return None;
    }

    // if only 1, return
    if all_configs.len() == 1 {
        return match &all_configs[0] {
            crate::config::ConfigImpl::Musq(c) => Some(c.clone()),
            _ => unreachable!(),
        };
    }

    let selected = term.choice("Select an account:", all_choices);
    match selected {
        Some(selected) => {
            let config = all_configs
                .iter()
                .find(|&c| match c {
                    crate::config::ConfigImpl::Kmkc(_) => false,
                    crate::config::ConfigImpl::Musq(c) => c.id == selected.name,
                })
                .unwrap();

            match config {
                crate::config::ConfigImpl::Kmkc(_) => unreachable!(),
                crate::config::ConfigImpl::Musq(c) => Some(c.clone()),
            }
        }
        None => None,
    }
}

pub(super) fn make_client(config: &Config) -> tosho_musq::MUClient {
    let constants = get_constants(config.r#type() as u8);

    tosho_musq::MUClient::new(&config.session, constants.clone())
}

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
    account: &Config,
    download_mode: bool,
    show_all: bool,
    no_input: bool,
    console: &crate::term::Terminal,
) -> (
    anyhow::Result<Vec<ChapterV2>>,
    Option<MangaDetailV2>,
    MUClient,
    Option<UserPoint>,
) {
    console.info(&cformat!(
        "Fetching for ID <magenta,bold>{}</>...",
        title_id
    ));
    let client = super::common::make_client(account);

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
                    client,
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

                return (Ok(vec![]), None, client, Some(user_bal));
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

                        return (Ok(vec![]), None, client, Some(user_bal));
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

                    (Ok(selected_chapters), Some(result), client, Some(user_bal))
                }
                None => {
                    console.warn("Aborted");
                    (
                        Err(anyhow::anyhow!("Aborted")),
                        Some(result.clone()),
                        client,
                        Some(user_bal),
                    )
                }
            }
        }
        Err(e) => {
            console.error(&cformat!("Unable to connect to MU!: {}", e));

            (Err(e), None, client, None)
        }
    }
}
