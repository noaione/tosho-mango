use color_eyre::eyre::Context;
use color_print::cformat;
use tosho_mplus::MPClient;

use super::common::do_print_search_information;

pub(crate) async fn mplus_my_favorites(
    client: &MPClient,
    account: &super::config::Config,
    console: &crate::term::Terminal,
) -> color_eyre::Result<()> {
    console.info(cformat!(
        "Getting favorites list for user <m,s>{}</>",
        account.id
    ));
    let results = client
        .get_bookmarked_titles()
        .await
        .context("Unable to connect to M+")?;

    match results {
        tosho_mplus::APIResponse::Success(results) => {
            if results.titles().is_empty() {
                console.warn("You don't have any favorites.");
                return Ok(());
            }

            console.info(cformat!(
                "Your favorites list (<m,s>{}</> results):",
                results.titles().len()
            ));

            do_print_search_information(results.titles(), false, None);

            Ok(())
        }
        tosho_mplus::APIResponse::Error(e) => {
            console.error(format!(
                "Failed to get your favorites list: {}",
                e.as_string()
            ));
            Err(color_eyre::eyre::eyre!(
                "Failed to get your favorites list: {}",
                e.as_string()
            ))
        }
    }
}
