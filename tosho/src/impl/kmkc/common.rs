use std::path::PathBuf;

use color_print::cformat;
use num_format::{Locale, ToFormattedString};
use tosho_kmkc::{
    constants::BASE_HOST,
    models::{TitleNode, TitleTicketListNode, UserPointResponse},
    KMClient, KMConfigWeb,
};

use crate::{
    linkify,
    term::{get_console, ConsoleChoice},
};

use super::config::Config;

pub(super) fn do_print_search_information(
    results: Vec<TitleNode>,
    with_number: bool,
    spacing: Option<usize>,
) {
    let term = get_console(0);
    let spacing = spacing.unwrap_or(2);

    for (idx, result) in results.iter().enumerate() {
        let id = result.id;
        let manga_url = format!("https://{}/title/{}", BASE_HOST.as_str(), result.id);
        let linked = linkify!(&manga_url, &result.title);
        let mut text_data = cformat!("<s>{}</s> ({})", linked, id);
        if result.next_update.is_some() {
            text_data = cformat!(
                "{} [<y,s>{}</>]",
                text_data,
                result.next_update.clone().unwrap()
            );
        }
        if !result.update_cycle.is_empty() {
            text_data = cformat!("{} [<b!,s>{}</>]", text_data, result.update_cycle);
        }

        let pre_space = " ".repeat(spacing);
        let pre_space_url = " ".repeat(spacing + 1);

        match with_number {
            true => term.info(&format!("{}[{:02}] {}", pre_space, idx + 1, text_data)),
            false => term.info(&format!("{}{}", pre_space, text_data)),
        }
        term.info(&format!("{}{}", pre_space_url, manga_url))
    }
}

pub(super) fn parse_netscape_cookies(cookie_path: PathBuf) -> KMConfigWeb {
    let term = get_console(0);

    let read_cookie = match std::fs::read_to_string(cookie_path) {
        Ok(read_cookie) => read_cookie,
        Err(e) => {
            term.error(&format!("Failed to read cookie file: {}", e));
            std::process::exit(1);
        }
    };

    let config: KMConfigWeb = match read_cookie.try_into() {
        Ok(config) => config,
        Err(e) => {
            term.error(&format!("Failed to parse cookie file: {}", e));
            std::process::exit(1);
        }
    };
    config
}

#[derive(Clone)]
pub(super) struct PurchasePoint {
    pub(super) point: UserPointResponse,
    pub(super) ticket: TitleTicketListNode,
}

pub(super) async fn common_purchase_select(
    title_id: i32,
    client: &KMClient,
    account: &Config,
    download_mode: bool,
    show_all: bool,
    no_input: bool,
    console: &crate::term::Terminal,
) -> (
    anyhow::Result<Vec<tosho_kmkc::models::EpisodeNode>>,
    Option<TitleNode>,
    Vec<tosho_kmkc::models::EpisodeNode>,
    Option<PurchasePoint>,
) {
    console.info(&cformat!(
        "Getting user point for <m,s>{}</>...",
        account.get_username()
    ));
    let user_point = client.get_user_point().await;
    if let Err(error) = user_point {
        console.error(&format!("Unable to get user point: {}", error));
        return (Err(error), None, vec![], None);
    }
    let user_point = user_point.unwrap();

    console.info(&cformat!(
        "Getting title information for ID <m,s>{}</>...",
        title_id
    ));
    let results = client.get_titles(vec![title_id]).await;
    if let Err(error) = results {
        console.error(&format!("Failed to get title information: {}", error));
        return (Err(error), None, vec![], None);
    }

    let results = results.unwrap();
    if results.is_empty() {
        console.error("Unable to find title information");
        return (
            Err(anyhow::anyhow!("Unable to find title information")),
            None,
            vec![],
            None,
        );
    }

    let result = results.first().unwrap();

    console.info(&cformat!(
        "Fetching <m,s>{}</> title ticket...",
        result.title
    ));
    let ticket_entry = client.get_title_ticket(result.id).await;
    if let Err(error) = ticket_entry {
        console.error(&format!("Failed to get title ticket: {}", error));
        return (Err(error), Some(result.clone()), vec![], None);
    }

    let ticket_entry = ticket_entry.unwrap();

    let mut chapters_entry = vec![];
    console.info(&cformat!("Fetching <m,s>{}</> chapters...", result.title));
    for episodes in result.episode_ids.clone().chunks(50) {
        let chapters = client.get_episodes(episodes.to_vec()).await;

        if let Err(error) = chapters {
            console.error(&format!("Failed to get chapters: {}", error));
            return (
                Err(error),
                Some(result.clone()),
                chapters_entry,
                Some(PurchasePoint {
                    point: user_point,
                    ticket: ticket_entry,
                }),
            );
        }

        chapters_entry.extend(chapters.unwrap());
    }

    console.info("Your current point balance:");
    let total_bal = user_point
        .point
        .total_point()
        .to_formatted_string(&Locale::en);
    let paid_point = user_point.point.paid_point.to_formatted_string(&Locale::en);
    let free_point = user_point.point.free_point.to_formatted_string(&Locale::en);
    let premium_ticket = user_point.ticket.total_num.to_formatted_string(&Locale::en);
    console.info(&cformat!(
        "  - <bold>Total:</> <cyan!,bold><reverse>{}</>c</cyan!,bold>",
        total_bal
    ));
    console.info(&cformat!(
        "  - <bold>Paid point:</> <g,bold><reverse>{}</>c</g,bold>",
        paid_point
    ));
    console.info(&cformat!(
        "  - <bold>Free point:</> <cyan,bold><reverse>{}</>c</cyan,bold>",
        free_point
    ));
    console.info(&cformat!(
        "  - <bold>Premium ticket:</> <yellow,bold><reverse>{}</> ticket</yellow,bold>",
        premium_ticket
    ));
    console.info(&cformat!(
        "  - <bold>Title ticket?</bold>: {}",
        ticket_entry.is_title_available()
    ));

    console.info("Title information:");
    console.info(&cformat!("  - <bold>ID:</> {}", result.id));
    console.info(&cformat!("  - <bold>Title:</> {}", result.title));
    console.info(&cformat!(
        "  - <bold>Chapters:</> {} chapters",
        chapters_entry.len()
    ));

    if no_input {
        return (
            Ok(chapters_entry.clone()),
            Some(result.clone()),
            chapters_entry,
            Some(PurchasePoint {
                point: user_point,
                ticket: ticket_entry,
            }),
        );
    }

    let select_choices: Vec<ConsoleChoice> = chapters_entry
        .iter()
        .filter_map(|ch| {
            if download_mode && !show_all && !ch.is_available() {
                None
            } else {
                let value = if ch.is_available() {
                    ch.title.clone()
                } else if ch.is_ticketable() {
                    format!("{} ({}P/Ticket)", ch.title, ch.point)
                } else {
                    format!("{} ({}P)", ch.title, ch.point)
                };
                Some(ConsoleChoice {
                    name: ch.id.to_string(),
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
                    let ch_id = ch.name.parse::<i32>().unwrap();
                    let ch = chapters_entry
                        .iter()
                        .find(|ch| ch.id == ch_id)
                        .unwrap()
                        .clone();

                    ch
                })
                .collect();

            if mapped_chapters.is_empty() {
                console.warn("No chapters selected, aborting...");

                return (
                    Ok(vec![]),
                    Some(result.clone()),
                    chapters_entry,
                    Some(PurchasePoint {
                        point: user_point,
                        ticket: ticket_entry,
                    }),
                );
            }

            (
                Ok(mapped_chapters),
                Some(result.clone()),
                chapters_entry,
                Some(PurchasePoint {
                    point: user_point,
                    ticket: ticket_entry,
                }),
            )
        }
        None => {
            console.warn("Aborted!");
            (
                Err(anyhow::anyhow!("Aborted!")),
                Some(result.clone()),
                chapters_entry,
                Some(PurchasePoint {
                    point: user_point,
                    ticket: ticket_entry,
                }),
            )
        }
    }
}
