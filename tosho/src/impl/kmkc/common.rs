use std::path::PathBuf;

use color_eyre::eyre::{Context, OptionExt};
use color_print::cformat;
use num_format::{Locale, ToFormattedString};
use tosho_kmkc::{
    KMClient, KMConfigWeb,
    constants::BASE_HOST,
    models::{TitleNode, TitleTicketListNode, UserPointResponse},
};
use tosho_macros::AutoGetter;

use crate::{
    linkify,
    term::{ConsoleChoice, get_console},
};

use super::config::Config;

pub(super) fn do_print_search_information(
    results: &[TitleNode],
    with_number: bool,
    spacing: Option<usize>,
) {
    let term = get_console(0);
    let spacing = spacing.unwrap_or(2);

    for (idx, result) in results.iter().enumerate() {
        let id = result.id();
        let manga_url = format!("https://{}/title/{}", BASE_HOST, id);
        let linked = linkify!(&manga_url, result.title());
        let mut text_data = cformat!("<s>{}</s> ({})", linked, id);
        if let Some(next_update) = result.next_update() {
            text_data = cformat!("{} [<y,s>{}</>]", text_data, next_update);
        }
        if !result.update_cycle().is_empty() {
            text_data = cformat!("{} [<b!,s>{}</>]", text_data, result.update_cycle());
        }

        let pre_space = " ".repeat(spacing);
        let pre_space_url = " ".repeat(spacing + 1);

        match with_number {
            true => term.info(format!("{}[{:02}] {}", pre_space, idx + 1, text_data)),
            false => term.info(format!("{pre_space}{text_data}")),
        }
        term.info(format!("{pre_space_url}{manga_url}"))
    }
}

pub(super) fn parse_netscape_cookies(cookie_path: PathBuf) -> color_eyre::Result<KMConfigWeb> {
    let read_cookie =
        std::fs::read_to_string(&cookie_path).context("Failed to read cookie file")?;
    let config: KMConfigWeb = read_cookie
        .try_into()
        .context("Failed to parse cookie file")?;

    Ok(config)
}

#[derive(Clone)]
pub(super) struct PurchasePoint {
    pub(super) point: UserPointResponse,
    pub(super) ticket: TitleTicketListNode,
}

#[derive(Clone, AutoGetter)]
pub(super) struct KMPurchaseResult {
    episodes: Vec<tosho_kmkc::models::EpisodeNode>,
    all_episodes: Vec<tosho_kmkc::models::EpisodeNode>,
    title: tosho_kmkc::models::TitleNode,
    balance: PurchasePoint,
}

impl KMPurchaseResult {
    fn from_result_custom(
        episodes: Vec<tosho_kmkc::models::EpisodeNode>,
        all_episodes: Vec<tosho_kmkc::models::EpisodeNode>,
        title: &tosho_kmkc::models::TitleNode,
        balance: &PurchasePoint,
    ) -> Self {
        Self {
            episodes,
            all_episodes,
            title: title.clone(),
            balance: balance.clone(),
        }
    }

    fn from_result_no_input(
        all_episodes: Vec<tosho_kmkc::models::EpisodeNode>,
        title: &tosho_kmkc::models::TitleNode,
        balance: &PurchasePoint,
    ) -> Self {
        Self {
            episodes: all_episodes.clone(),
            all_episodes,
            title: title.clone(),
            balance: balance.clone(),
        }
    }
}

pub(super) async fn common_purchase_select(
    title_id: i32,
    client: &KMClient,
    account: &Config,
    download_mode: bool,
    show_all: bool,
    no_input: bool,
    console: &crate::term::Terminal,
) -> color_eyre::Result<KMPurchaseResult> {
    console.info(cformat!(
        "Getting user point for <m,s>{}</>...",
        account.get_username()
    ));
    let user_point = client
        .get_user_point()
        .await
        .context("Unable to get user point")?;

    console.info(cformat!(
        "Getting title information for ID <m,s>{}</>...",
        title_id
    ));
    let results = client
        .get_titles(vec![title_id])
        .await
        .context("Failed to get title information")?;

    let result = results
        .first()
        .ok_or_eyre("Unable to find title information.")?;

    console.info(cformat!(
        "Fetching <m,s>{}</> title ticket...",
        result.title()
    ));
    let ticket_entry = client
        .get_title_ticket(result.id())
        .await
        .context("Failed to get title ticket")?;

    let mut chapters_entry = vec![];
    console.info(cformat!(
        "Fetching <m,s>{}</> <s>{}</> chapters...",
        result.title(),
        result.episode_ids().len()
    ));
    for (chunk_idx, episodes) in result.episode_ids().chunks(50).enumerate() {
        let chapters = client
            .get_episodes(episodes.to_vec())
            .await
            .context(format!("Failed to get chapters chunk #{}", chunk_idx + 1))?;

        chapters_entry.extend(chapters);
    }

    console.info("Your current point balance:");
    let total_bal = user_point
        .point()
        .total_point()
        .to_formatted_string(&Locale::en);
    let paid_point = user_point
        .point()
        .paid_point()
        .to_formatted_string(&Locale::en);
    let free_point = user_point
        .point()
        .free_point()
        .to_formatted_string(&Locale::en);
    let premium_ticket = user_point
        .ticket()
        .total_num()
        .to_formatted_string(&Locale::en);
    console.info(cformat!(
        "  - <bold>Total:</> <cyan!,bold><reverse>{}</>c</cyan!,bold>",
        total_bal
    ));
    console.info(cformat!(
        "  - <bold>Paid point:</> <g,bold><reverse>{}</>c</g,bold>",
        paid_point
    ));
    console.info(cformat!(
        "  - <bold>Free point:</> <cyan,bold><reverse>{}</>c</cyan,bold>",
        free_point
    ));
    console.info(cformat!(
        "  - <bold>Premium ticket:</> <yellow,bold><reverse>{}</> ticket</yellow,bold>",
        premium_ticket
    ));
    console.info(cformat!(
        "  - <bold>Title ticket?</bold>: {}",
        ticket_entry.is_title_available()
    ));

    console.info("Title information:");
    console.info(cformat!("  - <bold>ID:</> {}", result.id()));
    console.info(cformat!("  - <bold>Title:</> {}", result.title()));
    console.info(cformat!(
        "  - <bold>Chapters:</> {} chapters",
        chapters_entry.len()
    ));

    let purchase_point = PurchasePoint {
        point: user_point,
        ticket: ticket_entry,
    };

    if no_input {
        return Ok(KMPurchaseResult::from_result_no_input(
            chapters_entry,
            result,
            &purchase_point,
        ));
    }

    let select_choices: Vec<ConsoleChoice> = chapters_entry
        .iter()
        .filter_map(|ch| {
            if download_mode && !show_all && !ch.is_available() {
                None
            } else {
                let value = if ch.is_available() {
                    ch.title().to_string()
                } else if ch.is_ticketable() {
                    format!("{} ({}P/Ticket)", ch.title(), ch.point())
                } else {
                    format!("{} ({}P)", ch.title(), ch.point())
                };
                Some(ConsoleChoice {
                    name: ch.id().to_string(),
                    value,
                })
            }
        })
        .collect();

    let sel_prompt = if download_mode {
        "Select chapter to download"
    } else {
        "Select chapter to purchase"
    };

    let selected_chapters = console.select(sel_prompt, select_choices);
    match selected_chapters {
        Some(selected) => {
            let mapped_chapters: Vec<tosho_kmkc::models::EpisodeNode> = selected
                .iter()
                .map(|ch| {
                    let ch_id = ch.name.parse::<i32>().expect("Must be integer");

                    chapters_entry
                        .iter()
                        .find(|ch| ch.id() == ch_id)
                        .expect(&format!("Failed to find chapter ID: {}", ch_id))
                        .clone()
                })
                .collect();

            if mapped_chapters.is_empty() {
                console.warn("No chapters selected, aborting...");

                return Ok(KMPurchaseResult::from_result_custom(
                    vec![],
                    chapters_entry,
                    result,
                    &purchase_point,
                ));
            }

            Ok(KMPurchaseResult::from_result_custom(
                mapped_chapters,
                chapters_entry,
                result,
                &purchase_point,
            ))
        }
        None => {
            console.warn("Aborted!");

            Err(color_eyre::eyre::eyre!("Aborted by user"))
        }
    }
}
