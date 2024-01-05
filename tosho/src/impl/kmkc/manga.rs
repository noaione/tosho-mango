use color_print::cformat;

use crate::cli::{ExitCode, WeeklyCodeCli};

use super::common::{do_print_search_information, select_single_account};

pub(crate) async fn kmkc_search(
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
    let client = super::common::make_client(&account.into());

    let results = client.search(query, Some(50)).await;
    match results {
        Ok(results) => {
            if results.is_empty() {
                console.warn("No results found");
                return 1;
            }

            console.info(&cformat!(
                "Search results (<magenta,bold>{}</> results):",
                results.len()
            ));

            do_print_search_information(results, false, None);

            0
        }
        Err(e) => {
            console.error(&cformat!("Unable to connect to KM: {}", e));
            1
        }
    }
}

pub(crate) async fn kmkc_search_weekly(
    weekday: WeeklyCodeCli,
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
    let client = super::common::make_client(&account.into());

    let results = client.get_weekly().await;
    match results {
        Ok(results) => {
            let mut title_ids_list = vec![];
            for weekly_info in results.contents {
                if weekly_info.weekday == weekday.indexed() {
                    title_ids_list = weekly_info.titles;
                    break;
                }
            }

            let mut titles = vec![];
            for title_id in title_ids_list {
                let find_title = results.titles.iter().find(|t| t.id == title_id);

                if let Some(title) = find_title {
                    titles.push(title.clone());
                }
            }

            if titles.is_empty() {
                console.warn("No results found");
                return 1;
            }

            console.info(&cformat!(
                "Weekday <bold>{}</> results (<magenta,bold>{}</> results):",
                weekday.to_name(),
                titles.len()
            ));

            do_print_search_information(titles, false, None);

            0
        }
        Err(e) => {
            console.error(&cformat!("Unable to connect to KM: {}", e));
            1
        }
    }
}
