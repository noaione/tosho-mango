use color_print::cformat;
use tosho_musq::WeeklyCode;

use crate::cli::ExitCode;

use super::common::{do_print_search_information, select_single_account};

pub(crate) async fn musq_search(
    query: &str,
    account_id: Option<&str>,
    console: &crate::term::Terminal,
) -> ExitCode {
    let account = select_single_account(account_id);

    if account.is_none() {
        console.warn("Aborted");
        return 1;
    }

    let account = account.unwrap();
    console.info(&cformat!("Searching for <magenta,bold>{}</>...", query));
    let client = super::common::make_client(&account);

    let results = client.search(query).await;
    match results {
        Ok(results) => {
            if results.titles.is_empty() {
                console.warn("No results found");
                return 1;
            }

            // Cut to first 25 results
            let cutoff_results = results.titles[..25].to_vec();

            console.info(&cformat!(
                "Search results (<magenta,bold>{}</> results):",
                cutoff_results.len()
            ));

            do_print_search_information(cutoff_results, false);

            0
        }
        Err(e) => {
            console.error(&cformat!("Unable to connect to MU!: {}", e));
            1
        }
    }
}

pub(crate) async fn musq_search_weekly(
    weekday: WeeklyCode,
    account_id: Option<&str>,
    console: &crate::term::Terminal,
) -> ExitCode {
    let account = select_single_account(account_id);

    if account.is_none() {
        console.warn("Aborted");
        return 1;
    }

    let account = account.unwrap();
    console.info(&cformat!(
        "Getting weekly manga for week <magenta,bold>{}</>...",
        weekday.to_name()
    ));
    let client = super::common::make_client(&account);

    let results = client.get_weekly_titles(weekday).await;
    match results {
        Ok(results) => {
            if results.titles.is_empty() {
                console.warn("No results found");
                return 1;
            }

            console.info(&cformat!(
                "Weekday <bold>{}</> results (<magenta,bold>{}</> results):",
                weekday.to_name(),
                results.titles.len()
            ));

            do_print_search_information(results.titles, false);

            0
        }
        Err(e) => {
            console.error(&cformat!("Unable to connect to MU!: {}", e));
            1
        }
    }
}
