use color_print::cformat;

use super::common::do_print_search_information;
use super::common::make_client;
use super::common::select_single_account;
use crate::cli::ExitCode;

pub(crate) async fn kmkc_my_favorites(
    account_id: Option<&str>,
    console: &crate::term::Terminal,
) -> ExitCode {
    let acc_info = select_single_account(account_id);

    if acc_info.is_none() {
        console.warn("Aborted!");

        return 1;
    }

    let acc_info = acc_info.unwrap();

    let client = make_client(&acc_info.clone().into());

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

                    if let Some(title) = title {
                        Some(title.clone())
                    } else {
                        None
                    }
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
