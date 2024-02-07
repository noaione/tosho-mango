use color_print::cformat;
use tosho_amap::AMClient;

use super::{common::do_print_search_information, config::Config};
use crate::cli::ExitCode;

pub(crate) async fn amap_my_favorites(
    client: &AMClient,
    acc_info: &Config,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(&cformat!(
        "Getting favorites list for <magenta,bold>{}</>...",
        acc_info.email
    ));

    let results = client.get_favorites().await;

    match results {
        Ok(results) => {
            if results.comics.is_empty() {
                console.error("You don't have any favorites.");
                return 0;
            }

            console.info(&cformat!(
                "Your favorites list (<m,s>{}</> results):",
                results.comics.len()
            ));

            do_print_search_information(&results.comics, false, None);

            0
        }
        Err(err) => {
            console.error(&format!("Failed to fetch favorites: {}", err));

            1
        }
    }
}
