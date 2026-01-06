use color_eyre::eyre::{Context, OptionExt};
use color_print::cformat;
use num_format::{Locale, ToFormattedString};
use tosho_amap::{
    AMClient, SESSION_COOKIE_NAME,
    constants::BASE_HOST,
    models::{ComicEpisodeInfo, ComicInfo, ComicInfoResponse, ComicSimpleInfo, IAPInfo},
};
use tosho_macros::AutoGetter;

use super::config::Config;
use crate::r#impl::common::unix_timestamp_to_string;
use crate::{
    config::save_config,
    linkify,
    term::{ConsoleChoice, get_console},
};

impl From<super::config::Config> for tosho_amap::AMConfig {
    fn from(config: super::config::Config) -> Self {
        tosho_amap::AMConfig::new(&config.token, &config.identifier, &config.session)
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
        let result = result.info();
        let id = result.id();
        let manga_url = format!("https://{}/manga/{}", BASE_HOST, id);
        let linked = linkify!(&manga_url, result.title());
        let mut text_data = cformat!("<s>{}</s> ({})", linked, id);

        if result.new_update() {
            text_data = cformat!("{} [<b,s>NEW</b,s>]", text_data);
        }

        let mut add_url_pre = 1;
        let mut last_upd: Option<String> = None;
        if let Some(last_update_prev) = result.update_date()
            && let Some(last_update) = unix_timestamp_to_string(last_update_prev as i64)
        {
            last_upd = Some(cformat!("Last update: <s>{}</>", last_update));
            add_url_pre += 1;
        }

        let pre_space = " ".repeat(spacing);
        let pre_space_lupd = " ".repeat(spacing + 1);
        let pre_space_url = " ".repeat(spacing + add_url_pre);

        if with_number {
            term.info(format!("{}[{:02}] {}", pre_space, idx + 1, text_data))
        } else {
            term.info(format!("{pre_space}{text_data}"))
        }
        if let Some(last_upd) = last_upd {
            term.info(format!("{pre_space_lupd}{last_upd}"));
        }
        term.info(format!("{pre_space_url}{manga_url}"));
    }
}

#[derive(Debug, AutoGetter)]
pub(super) struct AMPurchaseResult {
    episodes: Vec<ComicEpisodeInfo>,
    comic: ComicInfo,
    balance: IAPInfo,
}

impl AMPurchaseResult {
    fn from_result(result: &ComicInfoResponse, balance: &IAPInfo) -> Self {
        Self {
            episodes: result.info().episodes().to_vec(),
            comic: result.info().clone(),
            balance: balance.clone(),
        }
    }

    fn from_result_custom(
        episodes: Vec<ComicEpisodeInfo>,
        comic: &ComicInfo,
        balance: &IAPInfo,
    ) -> Self {
        Self {
            episodes,
            comic: comic.clone(),
            balance: balance.clone(),
        }
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
) -> color_eyre::Result<AMPurchaseResult> {
    console.info(cformat!("Fetching for ID <magenta,bold>{}</>...", title_id));

    let result = client
        .get_comic(title_id)
        .await
        .context("Unable to connect to AM")?;

    save_session_config(client, account);

    let balance = result.account();
    let total_ticket = balance.sum().to_formatted_string(&Locale::en);
    let purchased = balance.purchased().to_formatted_string(&Locale::en);
    let premium = balance.premium().to_formatted_string(&Locale::en);
    let total_point = balance.sum_point().to_formatted_string(&Locale::en);

    console.info("Your current point balance:");
    console.info(cformat!(
        "  - <s>Total</>: <magenta,bold><reverse>{}</>T</magenta,bold>",
        total_ticket
    ));
    console.info(cformat!(
        "  - <s>Purchased</>: <yellow,bold><reverse>{}</>T</yellow,bold>",
        purchased
    ));
    console.info(cformat!(
        "  - <s>Premium</>: <green,bold><reverse>{}</>T</green,bold>",
        premium
    ));
    console.info(cformat!(
        "  - <s>Total point</>: <cyan!,bold><reverse>{}</>p</cyan!,bold>",
        total_point
    ));

    console.info("Title information:");
    console.info(cformat!("  - <s>ID</>: {}", title_id));
    console.info(cformat!("  - <s>Title</>: {}", result.info().title()));
    console.info(cformat!(
        "  - <s>Chapters</>: {}",
        result.info().episodes().len()
    ));

    if no_input {
        return Ok(AMPurchaseResult::from_result(&result, &balance));
    }

    let select_choices: Vec<ConsoleChoice> = result
        .info()
        .episodes()
        .iter()
        .filter_map(|ch| {
            if download_mode && !show_all && !ch.info().is_available() {
                None
            } else {
                let value = if ch.info().is_available() {
                    ch.info().title().to_string()
                } else {
                    format!("{} ({}T)", ch.info().title(), ch.info().price())
                };
                Some(ConsoleChoice {
                    name: ch.info().id().to_string(),
                    value,
                })
            }
        })
        .collect();

    if select_choices.is_empty() {
        console.warn("No chapters selected, aborting...");

        return Ok(AMPurchaseResult::from_result_custom(
            vec![],
            result.info(),
            &balance,
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

                return Ok(AMPurchaseResult::from_result_custom(
                    vec![],
                    result.info(),
                    &balance,
                ));
            }

            let mut selected_chapters: Vec<ComicEpisodeInfo> = vec![];

            for chapter in selected {
                let ch_id = chapter.name.parse::<u64>()?;
                let ch = result
                    .info()
                    .episodes()
                    .iter()
                    .find(|&ch| ch.info().id() == ch_id)
                    .ok_or_eyre(format!("Failed to find chapter ID: {}", ch_id))?
                    .clone();

                selected_chapters.push(ch);
            }

            Ok(AMPurchaseResult::from_result_custom(
                selected_chapters,
                result.info(),
                &balance,
            ))
        }
        None => {
            console.warn("Aborted");

            Err(color_eyre::eyre::eyre!("Aborted by user"))
        }
    }
}

pub(super) fn save_session_config(client: &AMClient, config: &Config) {
    let mut config = config.clone();
    let store = client.get_cookie_store();

    let session = store
        .iter_any()
        .find(|&cookie| cookie.name() == SESSION_COOKIE_NAME);

    if let Some(session) = session {
        config.session = session.value().to_string();
    }

    save_config(crate::config::ConfigImpl::Amap(config), None);
}
