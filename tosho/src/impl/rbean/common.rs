use color_print::cformat;
use tosho_rbean::{constants::BASE_HOST, models::MangaNode, RBClient};

use crate::{config::save_config, linkify, term::get_console};

use super::config::Config;

pub(super) fn do_print_search_information(
    results: &[MangaNode],
    with_number: bool,
    spacing: Option<usize>,
) {
    let term = get_console(0);
    let spacing = spacing.unwrap_or(2);

    for (idx, result) in results.iter().enumerate() {
        let manga_url = format!("https://{}/series/{}", *BASE_HOST, result.slug);
        let linked = linkify!(&manga_url, &result.title);
        let text_data = cformat!("<s>{}</s> ({})", linked, result.uuid);

        let pre_space = " ".repeat(spacing);
        let pre_space_lupd = " ".repeat(spacing + 1);
        let pre_space_url = " ".repeat(spacing + 2);

        match with_number {
            true => term.info(&format!("{}[{:02}] {}", pre_space, idx + 1, text_data)),
            false => term.info(&format!("{}{}", pre_space, text_data)),
        }
        let updated_at = result.last_updated.format("%Y-%m-%d").to_string();
        term.info(&cformat!(
            "{}<s>Last update</s>: {}",
            pre_space_lupd,
            updated_at
        ));
        term.info(&format!("{}{}", pre_space_url, manga_url));
    }
}

pub(super) fn save_session_config(client: &RBClient, config: &Config) {
    let mut config = config.clone();
    config.access_token = client.get_token().to_string();
    if let Some(expiry_at) = client.get_expiry_at() {
        config.expiry = expiry_at;
    }

    save_config(config.into(), None);
}
