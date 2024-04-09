use color_print::cformat;
use num_format::{Locale, ToFormattedString};
use tosho_amap::{
    constants::BASE_HOST,
    models::{ComicEpisodeInfo, ComicInfo, ComicSimpleInfo, IAPInfo},
    AMClient, SESSION_COOKIE_NAME,
};

use super::config::Config;
use crate::r#impl::common::unix_timestamp_to_string;
use crate::{
    config::save_config,
    linkify,
    term::{get_console, ConsoleChoice},
};

impl From<super::config::Config> for tosho_amap::AMConfig {
    fn from(config: super::config::Config) -> Self {
        tosho_amap::AMConfig {
            token: config.token,
            identifier: config.identifier,
            session_v2: config.session,
        }
    }
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
            if let Some(last_update) = unix_timestamp_to_string(last_update as i64) {
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

pub(super) async fn common_purchase_select(
    title_id: u64,
    client: &AMClient,
    account: &Config,
    download_mode: bool,
    show_all: bool,
    no_input: bool,
    console: &crate::term::Terminal,
) -> (
    anyhow::Result<Vec<ComicEpisodeInfo>>,
    Option<ComicInfo>,
    Option<IAPInfo>,
) {
    console.info(&cformat!(
        "Fetching for ID <magenta,bold>{}</>...",
        title_id
    ));

    let results = client.get_comic(title_id).await;
    match results {
        Ok(result) => {
            save_session_config(client, account);

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

                return (Ok(vec![]), None, Some(balance.clone()));
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

                        return (Ok(vec![]), None, Some(balance.clone()));
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
                        Some(balance.clone()),
                    )
                }
                None => {
                    console.warn("Aborted");
                    (
                        Err(anyhow::anyhow!("Aborted")),
                        Some(result.info.clone()),
                        Some(result.account.clone()),
                    )
                }
            }
        }
        Err(e) => {
            console.error(&cformat!("Unable to connect to AM: {}", e));

            (Err(e), None, None)
        }
    }
}

pub(super) fn save_session_config(client: &AMClient, config: &Config) {
    let mut config = config.clone();
    let masked_cookie = SESSION_COOKIE_NAME.as_str();
    let store = client.get_cookie_store();
    for cookie in store.iter_any() {
        if cookie.name() == masked_cookie {
            config.session = cookie.value().to_string();
        }
    }

    save_config(crate::config::ConfigImpl::Amap(config), None);
}
