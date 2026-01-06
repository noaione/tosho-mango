use color_eyre::eyre::Context;
use color_print::cformat;
use tosho_kmkc::KMClient;

use super::{common::do_print_search_information, config::Config};

pub(crate) async fn kmkc_my_favorites(
    client: &KMClient,
    acc_info: &Config,
    console: &crate::term::Terminal,
) -> color_eyre::Result<()> {
    console.info(cformat!(
        "Getting favorites list for <magenta,bold>{}</>...",
        acc_info.get_username()
    ));

    let results = client
        .get_favorites()
        .await
        .context("Failed to fetch favorites")?;

    if results.favorites().is_empty() {
        console.error("You don't have any favorites.");
        return Ok(());
    }

    let mapped_favorites: Vec<tosho_kmkc::models::TitleNode> = results
        .favorites()
        .iter()
        .filter_map(|favorite| {
            let title = results
                .titles()
                .iter()
                .find(|title| title.id() == favorite.id());

            title.cloned()
        })
        .collect();

    console.info(cformat!(
        "Your favorites list (<m,s>{}</> results):",
        mapped_favorites.len()
    ));

    do_print_search_information(&mapped_favorites, false, None);

    Ok(())
}
