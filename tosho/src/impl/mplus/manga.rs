use color_print::cformat;
use tosho_mplus::MPClient;

use super::common::{do_print_search_information, get_cached_titles_data, search_manga_by_text};
use crate::cli::ExitCode;

pub(crate) async fn mplus_search(
    query: &str,
    client: &MPClient,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(&cformat!("Searching for <magenta,bold>{}</>...", query));

    let results = get_cached_titles_data(client).await;

    match results {
        Ok(results) => {
            if results.is_empty() {
                console.warn("No titles exist");
                return 1;
            }

            let merged_titles: Vec<tosho_mplus::proto::Title> =
                results.iter().flat_map(|x| x.titles.clone()).collect();
            let filtered = search_manga_by_text(&merged_titles, query);

            if filtered.is_empty() {
                console.warn("No results found");
                return 1;
            }

            do_print_search_information(&filtered, false, None);

            0
        }
        _ => 1,
    }
}
