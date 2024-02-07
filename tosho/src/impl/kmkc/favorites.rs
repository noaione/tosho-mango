use color_print::cformat;
use tosho_kmkc::KMClient;

use super::{common::do_print_search_information, config::Config};
use crate::cli::ExitCode;

pub(crate) async fn kmkc_my_favorites(
    client: &KMClient,
    acc_info: &Config,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(&cformat!(
        "Getting favorites list for <magenta,bold>{}</>...",
        acc_info.get_username()
    ));

    let results = client.get_favorites().await;

    match results {
        Ok(results) => {
            if results.favorites.is_empty() {
                console.error("You don't have any favorites.");
                return 0;
            }

            let mapped_favorites: Vec<tosho_kmkc::models::TitleNode> = results
                .favorites
                .iter()
                .filter_map(|favorite| {
                    let title = results.titles.iter().find(|title| title.id == favorite.id);

                    title.cloned()
                })
                .collect();

            console.info(&cformat!(
                "Your favorites list (<m,s>{}</> results):",
                mapped_favorites.len()
            ));

            do_print_search_information(mapped_favorites, false, None);

            0
        }
        Err(err) => {
            console.error(&format!("Failed to fetch favorites: {}", err));

            1
        }
    }
}
