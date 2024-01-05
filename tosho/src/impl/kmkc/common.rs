use std::path::PathBuf;

use tosho_kmkc::{constants::BASE_HOST, models::TitleNode, KMConfig, KMConfigWeb};

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
                crate::config::ConfigImpl::Kmkc(c) => Some(c),
                crate::config::ConfigImpl::Musq(_) => None,
            };
        }

        term.warn(&format!("Account ID {} not found!", account_id));
    }

    let all_configs = get_all_config(crate::r#impl::Implementations::Kmkc, None);
    let all_choices: Vec<ConsoleChoice> = all_configs
        .iter()
        .filter_map(|c| match c {
            crate::config::ConfigImpl::Kmkc(c) => Some(match c {
                super::config::Config::Mobile(cc) => ConsoleChoice {
                    name: cc.id.clone(),
                    value: format!(
                        "{} [{} - {}]",
                        cc.id,
                        cc.r#type().to_name(),
                        cc.platform().to_name()
                    ),
                },
                super::config::Config::Web(cc) => ConsoleChoice {
                    name: cc.id.clone(),
                    value: format!("{} [{}]", cc.id, cc.r#type().to_name()),
                },
            }),
            _ => None,
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
                    crate::config::ConfigImpl::Kmkc(c) => match c {
                        super::config::Config::Mobile(cc) => cc.id == selected.name,
                        super::config::Config::Web(cc) => cc.id == selected.name,
                    },
                    crate::config::ConfigImpl::Musq(_) => false,
                })
                .unwrap();

            match config {
                crate::config::ConfigImpl::Kmkc(c) => Some(c.clone()),
                crate::config::ConfigImpl::Musq(_) => None,
            }
        }
        None => None,
    }
}

pub(super) fn make_client(config: &KMConfig) -> tosho_kmkc::KMClient {
    tosho_kmkc::KMClient::new(config.clone())
}

/// TODO: Remove this
#[allow(dead_code)]
pub(super) fn do_print_search_information(results: Vec<TitleNode>, with_number: bool) {
    let term = get_console(0);

    for (idx, result) in results.iter().enumerate() {
        let id = result.id;
        let manga_url = format!("https://{}/title/{}", BASE_HOST.as_str(), result.id);
        let linked = linkify!(&manga_url, &result.title);
        let mut text_data = color_print::cformat!("<s>{}</s> ({})", linked, id);
        if result.next_update.is_some() {
            text_data = format!(
                "{} [<y,s>{}</>]",
                text_data,
                result.next_update.clone().unwrap()
            );
        }
        if !result.update_cycle.is_empty() {
            text_data = format!("{} [<y!,s>{}</>]", text_data, result.update_cycle);
        }
        match with_number {
            true => term.info(&format!("  [{:02}] {}", idx + 1, text_data)),
            false => term.info(&format!("  {}", text_data)),
        }
        term.info(&format!("   {}", manga_url))
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
