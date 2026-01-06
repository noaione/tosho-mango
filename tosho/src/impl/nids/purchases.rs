use std::collections::HashMap;

use crate::{
    r#impl::nids::common::{PaginateAction, pagination_helper, print_series_summary},
    linkify,
};
use color_eyre::eyre::Context;
use color_print::cformat;
use num_format::{Locale, ToFormattedString};
use tosho_nids::constants::BASE_HOST;

pub async fn nids_get_purchased_series(
    base_filters: &mut tosho_nids::Filter,
    client: &tosho_nids::NIClient,
    account: &super::config::Config,
    console: &crate::term::Terminal,
) -> color_eyre::Result<()> {
    let username = if account.username().is_empty() {
        account.email()
    } else {
        account.username()
    };

    console.info(cformat!(
        "Fetching purchased series for user <m,s>{}</m,s>...",
        username
    ));

    let purchased_series = client
        .get_series_run_collections(Some(base_filters))
        .await
        .context("Failed to fetch purchased series")?;

    if purchased_series.data().is_empty() {
        console.warn("No purchased series found.");
        return Ok(());
    }

    if purchased_series.pages() > 1 {
        // Do paginated response
        let mut current_page = 1u32;
        let mut maximum_pages = purchased_series.pages();
        let mut collected_series: HashMap<u32, Vec<tosho_nids::models::SeriesRunDetailed>> =
            HashMap::from([(1u32, purchased_series.data().to_vec())]);
        let mut current_data = collected_series.get(&1).expect("Should be exist");

        loop {
            console.info(cformat!(
                "Showing page <m,s>{}</m,s> of <m,s>{}</m,s>:",
                current_page,
                maximum_pages
            ));

            for series in current_data.iter() {
                print_series_summary(series, console, true);
            }

            if current_data.is_empty() {
                console.info("No series found on this page.");
            }

            match pagination_helper(current_page, maximum_pages, console).await {
                PaginateAction::Next => {
                    current_page += 1;
                }
                PaginateAction::Previous => {
                    if current_page > 1 {
                        current_page -= 1;
                    }
                }
                PaginateAction::Exit(_) => {
                    break;
                }
            }

            // Fetch new stuff
            base_filters.set_page(current_page);
            if let Some(series) = collected_series.get(&current_page) {
                current_data = series;
                console.clear_screen();
            } else {
                console.info(cformat!("Loading page <m,s>{}</>...", current_page));
                let new_series = client
                    .get_series_run_collections(Some(base_filters))
                    .await
                    .context("Failed to fetch purchased series")?;

                console.clear_screen();

                maximum_pages = new_series.pages();
                // add correct data to collected_series
                collected_series.insert(current_page, new_series.data().to_vec());
                current_data = collected_series
                    .get(&current_page)
                    .expect("Somehow missing page after insert");
            }
        }
    } else {
        // print all results
        for series in purchased_series.data() {
            print_series_summary(series, console, true);
        }
    }

    Ok(())
}

fn print_purchased_issue_summary(
    item: &tosho_nids::models::PurchasedIssue,
    console: &crate::term::Terminal,
) {
    let item_url = format!(
        "https://{}/mycollection/{}",
        BASE_HOST,
        item.series_run().uuid()
    );
    let linked_title = linkify!(&item_url, item.full_title());

    console.info(cformat!(
        "  <s>{}</s> (<m,s>{}</m,s> / {})",
        linked_title,
        item.id(),
        item.uuid()
    ));

    // Print owned editions
    let another_text = match item.total_editions() {
        0 => {
            // No issues? Weird.
            cformat!("<s>No</s> issues")
        }
        1 => cformat!("<s>1</s> edition"),
        n => cformat!("<s>{}</s> editions", n.to_formatted_string(&Locale::en)),
    };

    console.info(format!("    {}", another_text));
}

pub async fn nids_get_purchased_issues(
    series_run_uuid: &str,
    base_filters: &mut tosho_nids::Filter,
    client: &tosho_nids::NIClient,
    account: &super::config::Config,
    console: &crate::term::Terminal,
) -> color_eyre::Result<()> {
    let username = if account.username().is_empty() {
        account.email()
    } else {
        account.username()
    };

    console.info(cformat!(
        "Fetching purchased issues <m,s>{}</m,s> for user <m,s>{}</m,s>...",
        series_run_uuid,
        username
    ));

    let purchased_issues = client
        .get_issue_collections(base_filters)
        .await
        .context("Failed to fetch purchased issues")?;

    if purchased_issues.data().is_empty() {
        console.warn("No purchased issues found for this series.");
        return Ok(());
    }

    if purchased_issues.pages() > 1 {
        // Do paginated response
        let mut current_page = 1u32;
        let mut maximum_pages = purchased_issues.pages();
        let mut collected_issues: HashMap<u32, Vec<tosho_nids::models::PurchasedIssue>> =
            HashMap::from([(1u32, purchased_issues.data().to_vec())]);
        let mut current_data = collected_issues.get(&1).expect("Should be exist");

        loop {
            console.info(cformat!(
                "Showing page <m,s>{}</m,s> of <m,s>{}</m,s>:",
                current_page,
                maximum_pages
            ));

            for series in current_data.iter() {
                print_purchased_issue_summary(series, console);
            }

            if current_data.is_empty() {
                console.info("No series found on this page.");
            }

            match pagination_helper(current_page, maximum_pages, console).await {
                PaginateAction::Next => {
                    current_page += 1;
                }
                PaginateAction::Previous => {
                    if current_page > 1 {
                        current_page -= 1;
                    }
                }
                PaginateAction::Exit(_) => {
                    break;
                }
            }

            // Fetch new stuff
            base_filters.set_page(current_page);
            if let Some(series) = collected_issues.get(&current_page) {
                current_data = series;
                console.clear_screen();
            } else {
                console.info(cformat!("Loading page <m,s>{}</>...", current_page));
                let new_issues = client
                    .get_issue_collections(base_filters)
                    .await
                    .context("Failed to fetch purchased issues")?;

                console.clear_screen();

                maximum_pages = new_issues.pages();
                // add correct data to collected_series
                collected_issues.insert(current_page, new_issues.data().to_vec());
                current_data = collected_issues
                    .get(&current_page)
                    .expect("Somehow missing page after insert");
            }
        }
    } else {
        // print all results
        for issue in purchased_issues.data() {
            print_purchased_issue_summary(issue, console);
        }
    }

    Ok(())
}
