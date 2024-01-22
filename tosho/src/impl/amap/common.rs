use chrono::TimeZone;
use color_print::cformat;
use num_format::{Locale, ToFormattedString};
use tosho_amap::{
    constants::BASE_HOST,
    models::{ComicEpisodeInfo, ComicInfo, ComicSimpleInfo, IAPInfo},
    AMClient, AMConfig,
};

use crate::{
    config::{get_all_config, get_config},
    linkify,
    term::{get_console, ConsoleChoice},
};

use super::config::Config;

pub(super) fn select_single_account(account_id: Option<&str>) -> Option<Config> {
    let term: crate::term::Terminal = get_console(0);

    if let Some(account_id) = account_id {
        let config = get_config(account_id, crate::r#impl::Implementations::Amap, None);

        if let Some(config) = config {
            return match config {
                crate::config::ConfigImpl::Amap(c) => Some(c),
                _ => unreachable!(),
            };
        }

        term.warn(&format!("Account ID {} not found!", account_id));
    }

    let all_configs = get_all_config(crate::r#impl::Implementations::Amap, None);
    let all_choices: Vec<ConsoleChoice> = all_configs
        .iter()
        .filter_map(|c| match c {
            crate::config::ConfigImpl::Amap(c) => Some(ConsoleChoice {
                name: c.id.clone(),
                value: format!("{} - {} [{}]", c.id, c.email, c.r#type().to_name()),
            }),
            _ => None,
        })
        .collect();

    if all_configs.is_empty() {
        term.warn("No accounts found!");
        return None;
    }

    // only 1? return
    if all_configs.len() == 1 {
        return match &all_configs[0] {
            crate::config::ConfigImpl::Amap(c) => Some(c.clone()),
            _ => unreachable!(),
        };
    }

    let selected = term.choice("Select an account:", all_choices);
    match selected {
        Some(selected) => {
            let config = all_configs
                .iter()
                .find(|&c| match c {
                    crate::config::ConfigImpl::Amap(c) => c.id == selected.name,
                    _ => false,
                })
                .unwrap();

            match config {
                crate::config::ConfigImpl::Amap(c) => Some(c.clone()),
                _ => unreachable!(),
            }
        }
        None => None,
    }
}

impl From<Config> for tosho_amap::AMConfig {
    fn from(value: Config) -> Self {
        Self {
            token: value.token,
            identifier: value.identifier,
            session_v2: value.session,
        }
    }
}

pub(super) fn make_client(config: &AMConfig) -> tosho_amap::AMClient {
    tosho_amap::AMClient::new(config.clone())
}

pub(super) fn do_print_search_information(
    results: &[ComicSimpleInfo],
    with_number: bool,
    spacing: Option<usize>,
) {
    let term = get_console(0);
    let spacing = spacing.unwrap_or(2);

    for (idx, result) in results.iter().enumerate() {
        let result = &result.info;
        let id = result.id;
        let manga_url = format!("https://{}/manga/{}", BASE_HOST.as_str(), result.id);
        let linked = linkify!(&manga_url, &result.title);
        let mut text_data = cformat!("<s>{}</s> ({})", linked, id);

        if result.new_update {
            text_data = cformat!("{} [<b,s>NEW</b,s>]", text_data);
        }

        let mut add_url_pre = 1;
        let mut last_upd: Option<String> = None;
        if let Some(last_update) = result.update_date {
            if let Some(last_update) = unix_timestamp_to_string(last_update) {
                last_upd = Some(cformat!("Last update: <s>{}</>", last_update));
                add_url_pre += 1;
            }
        }

        let pre_space = " ".repeat(spacing);
        let pre_space_lupd = " ".repeat(spacing + 1);
        let pre_space_url = " ".repeat(spacing + add_url_pre);

        match with_number {
            true => term.info(&format!("{}[{:02}] {}", pre_space, idx + 1, text_data)),
            false => term.info(&format!("{}{}", pre_space, text_data)),
        }
        if let Some(last_upd) = last_upd {
            term.info(&format!("{}{}", pre_space_lupd, last_upd));
        }
        term.info(&format!("{}{}", pre_space_url, manga_url));
    }
}

pub(super) fn unix_timestamp_to_string(timestamp: u64) -> Option<String> {
    let dt = chrono::Utc
        .timestamp_opt(timestamp.try_into().unwrap(), 0)
        .single();

    match dt {
        Some(dt) => {
            let local = dt.with_timezone(&chrono::Local);

            // Format YYYY-MM-DD
            Some(local.format("%Y-%m-%d").to_string())
        }
        None => None,
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
    anyhow::Result<Vec<ComicEpisodeInfo>>,
    Option<ComicInfo>,
    AMClient,
    Option<IAPInfo>,
) {
    console.info(&cformat!(
        "Fetching for ID <magenta,bold>{}</>...",
        title_id
    ));
    let config: AMConfig = account.clone().into();
    let client = make_client(&config);

    let results = client.get_comic(title_id).await;
    match results {
        Ok(result) => {
            let balance = &result.account;
            let total_ticket = balance.sum().to_formatted_string(&Locale::en);
            let purchased = balance.purchased.to_formatted_string(&Locale::en);
            let premium = balance.premium.to_formatted_string(&Locale::en);
            let total_point = balance.sum_point().to_formatted_string(&Locale::en);

            console.info("Your current point balance:");
            console.info(&cformat!(
                "  - <s>Total</>: <magenta,bold><reverse>{}</>T</magenta,bold>",
                total_ticket
            ));
            console.info(&cformat!(
                "  - <s>Purchased</>: <yellow,bold><reverse>{}</>T</yellow,bold>",
                purchased
            ));
            console.info(&cformat!(
                "  - <s>Premium</>: <green,bold><reverse>{}</>T</green,bold>",
                premium
            ));
            console.info(&cformat!(
                "  - <s>Total point</>: <cyan!,bold><reverse>{}</>p</cyan!,bold>",
                total_point
            ));

            if no_input {
                return (
                    Ok(result.info.episodes.clone()),
                    Some(result.info.clone()),
                    client,
                    Some(balance.clone()),
                );
            }

            let select_choices: Vec<ConsoleChoice> = result
                .info
                .episodes
                .iter()
                .filter_map(|ch| {
                    if download_mode && !show_all && !ch.info.is_available() {
                        None
                    } else {
                        let value = if ch.info.is_available() {
                            ch.info.title.clone()
                        } else {
                            format!("{} ({}T)", ch.info.title, ch.info.price)
                        };
                        Some(ConsoleChoice {
                            name: ch.info.id.to_string(),
                            value,
                        })
                    }
                })
                .collect();

            if select_choices.is_empty() {
                console.warn("No chapters selected, aborting...");

                return (Ok(vec![]), None, client, Some(balance.clone()));
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

                        return (Ok(vec![]), None, client, Some(balance.clone()));
                    }

                    let mut selected_chapters: Vec<ComicEpisodeInfo> = vec![];

                    for chapter in selected {
                        let ch_id = chapter.name.parse::<u64>().unwrap();
                        let ch = result
                            .info
                            .episodes
                            .iter()
                            .find(|&ch| ch.info.id == ch_id)
                            .unwrap()
                            .clone();

                        selected_chapters.push(ch);
                    }

                    (
                        Ok(selected_chapters),
                        Some(result.info),
                        client,
                        Some(balance.clone()),
                    )
                }
                None => {
                    console.warn("Aborted");
                    (
                        Err(anyhow::anyhow!("Aborted")),
                        Some(result.info.clone()),
                        client,
                        Some(result.account.clone()),
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
