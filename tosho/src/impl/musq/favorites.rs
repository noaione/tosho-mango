use color_print::cformat;
use tosho_musq::MUClient;

use crate::cli::ExitCode;

use super::common::do_print_search_information;

pub(crate) async fn musq_my_favorites(
    client: &MUClient,
    account: &super::config::Config,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(&cformat!(
        "Getting favorites list for user <m,s>{}</>",
        account.id
    ));
    let results = client.get_my_manga().await;

    match results {
        Err(e) => {
            console.error(&cformat!("Unable to connect to MU!: {}", e));
            1
        }
        Ok(results) => {
            if results.favorites.is_empty() {
                console.error("You don't have any favorites.");
                return 0;
            }

            console.info(&cformat!(
                "Your favorites list (<m,s>{}</> results):",
                results.favorites.len()
            ));

            do_print_search_information(results.favorites, false, None);

            0
        }
    }
}

pub(crate) async fn musq_my_history(
    client: &MUClient,
    account: &super::config::Config,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(&cformat!(
        "Getting favorites list for user <m,s>{}</>",
        account.id
    ));

    let results = client.get_my_manga().await;

    match results {
        Err(e) => {
            console.error(&cformat!("Unable to connect to MU!: {}", e));
            1
        }
        Ok(results) => {
            if results.history.is_empty() {
                console.error("You don't have any reading history.");
                return 0;
            }

            console.info(&cformat!(
                "Your read history (<m,s>{}</> results):",
                results.history.len()
            ));

            do_print_search_information(results.history, false, None);

            0
        }
    }
}
