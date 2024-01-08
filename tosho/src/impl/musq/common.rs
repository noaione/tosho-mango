use chrono::{DateTime, FixedOffset};
use color_print::cformat;
use tosho_musq::{
    constants::{get_constants, BASE_HOST},
    proto::{BadgeManga, LabelBadgeManga, MangaResultNode},
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

// TODO: REMOVE THIS
#[allow(dead_code)]
pub(super) fn parse_published(
    published: Option<&str>,
) -> Option<chrono::LocalResult<DateTime<FixedOffset>>> {
    // %b %d, %Y
    match published {
        Some(published) => {
            // assume JST
            let published = chrono::NaiveDateTime::parse_from_str(published, "%b %d, %Y")
                .expect("Failed to parse published date");
            Some(published.and_local_timezone(chrono::FixedOffset::east_opt(9 * 3600).unwrap()))
        }
        None => None,
    }
}
