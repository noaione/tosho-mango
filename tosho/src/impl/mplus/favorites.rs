use color_print::cformat;
use tosho_mplus::MPClient;

use crate::cli::ExitCode;

use super::common::do_print_search_information;

pub(crate) async fn mplus_my_favorites(
    client: &MPClient,
    account: &super::config::Config,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(cformat!(
        "Getting favorites list for user <m,s>{}</>",
        account.id
    ));
    let results = client.get_bookmarked_titles().await;

    match results {
        Ok(tosho_mplus::APIResponse::Success(results)) => {
            if results.titles().is_empty() {
                console.warn("You don't have any favorites.");
                return 0;
            }

            console.info(cformat!(
                "Your favorites list (<m,s>{}</> results):",
                results.titles().len()
            ));

            do_print_search_information(results.titles(), false, None);

            0
        }
        Ok(tosho_mplus::APIResponse::Error(e)) => {
            console.error(format!(
                "Failed to get your favorites list: {}",
                e.as_string()
            ));
            1
        }
        Err(e) => {
            console.error(format!("Unable to connect to M+: {}", e));
            1
        }
    }
}
