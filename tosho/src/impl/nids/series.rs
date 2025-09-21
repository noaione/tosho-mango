use std::collections::HashMap;

use crate::{
    cli::ExitCode,
    r#impl::nids::common::{PaginateAction, fmt_date, format_series_run_date, pagination_helper},
    linkify,
};
use color_print::cformat;
use num_format::{Locale, ToFormattedString};
use tosho_nids::constants::BASE_HOST;

fn print_series_summary(
    series: &tosho_nids::models::SeriesRunDetailed,
    console: &crate::term::Terminal,
) {
    let item_url = format!(
        "https://{}/series/{}/{}",
        BASE_HOST,
        series.id(),
        series.slug()
    );
    let linked_title = linkify!(&item_url, series.title());

    console.info(&cformat!(
        "  <s>{}</s> (<m,s>{}</m,s> / {})",
        linked_title,
        series.id(),
        series.uuid()
    ));
    let mut series_smols = vec![cformat!("<b,s>{}</b,s>", series.publisher().name())];
    if let Some(run) = format_series_run_date(series.start_date(), series.end_date()) {
        series_smols.push(cformat!("<s,dim>{}</s,dim>", run));
    }
    match series.issues_count() {
        0 => {}
        1 => series_smols.push(cformat!("<s>1</s> issue")),
        n => series_smols.push(cformat!(
            "<s>{}</s> issues",
            n.to_formatted_string(&Locale::en)
        )),
    };

    console.info(&format!("   {}", item_url));
    console.info(&format!("   {}", series_smols.join(" | ")));
}

pub async fn nids_get_series(
    base_filter: &mut tosho_nids::Filter,
    client: &tosho_nids::NIClient,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info("Fetching initial series with provided filters...");
    let series = match client.get_series_runs(&base_filter).await {
        Ok(series) => series,
        Err(err) => {
            console.error(&format!("Failed to fetch series: {err}"));
            return 1;
        }
    };

    if series.data().is_empty() {
        console.warn("No series found with the provided filters.");
        return 0;
    }

    let mut stop_code = 0;
    if series.pages() > 1 {
        // Do paginated response
        let mut current_page: u32 = 1;
        let mut maximum_pages: u32 = series.pages();
        let mut collected_issues: HashMap<u32, Vec<tosho_nids::models::SeriesRunDetailed>> =
            HashMap::from([(1, series.data().to_vec())]);
        let mut correct_data = collected_issues.get(&1).expect("We just inserted this");

        loop {
            console.info(cformat!(
                "Showing page <m,s>{}</> of <m,s>{}</>:",
                current_page.to_formatted_string(&Locale::en),
                maximum_pages.to_formatted_string(&Locale::en)
            ));

            for series in correct_data.iter() {
                print_series_summary(series, console);
            }

            if correct_data.is_empty() {
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
                PaginateAction::Exit(code) => {
                    stop_code = code;
                    break;
                }
            }

            // Fetch new stuff
            base_filter.set_page(current_page);
            if let Some(series) = collected_issues.get(&current_page) {
                correct_data = series;
                console.clear_screen();
            } else {
                console.info(cformat!("Loading page <m,s>{}</>...", current_page));
                let new_series = match client.get_series_runs(&base_filter).await {
                    Ok(series) => series,
                    Err(err) => {
                        console.error(&format!("Failed to fetch series: {err}"));
                        stop_code = 1;
                        break;
                    }
                };

                console.clear_screen();

                maximum_pages = new_series.pages();
                // add correct data to collected_issues
                collected_issues.insert(current_page, new_series.data().to_vec());
                correct_data = collected_issues
                    .get(&current_page)
                    .expect("Somehow missing page after insert");
            }
        }
    } else {
        // print all results
        for series in series.data() {
            print_series_summary(series, console);
        }
    }

    stop_code
}

pub async fn nids_get_series_info(
    series_run_id: u32,
    with_marketplace: bool,
    client: &tosho_nids::NIClient,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(cformat!(
        "Fetching series ID <m,s>{}</m,s>...",
        series_run_id
    ));
    let series_detail = match client.get_series_run(series_run_id).await {
        Ok(series) => series,
        Err(e) => {
            console.error(format!("Failed to get issues: {}", e));
            return 1;
        }
    };

    console.info(cformat!(
        "Fetching issues for series <m,s>{}</m,s>...",
        series_detail.title()
    ));

    // For now, we force only the first 25 issues to be fetched
    let max_page_issues = series_detail.issues_count().max(25);
    let issue_filters = tosho_nids::Filter::new()
        .add_filter(tosho_nids::FilterType::SeriesRunId, series_run_id)
        .add_filter(tosho_nids::FilterType::Format, "issue,ashcan")
        .with_order(tosho_nids::SortBy::IssueNumber, tosho_nids::SortOrder::ASC)
        .with_per_page(max_page_issues);

    let issues_resp = match client.get_issues(&issue_filters).await {
        Ok(issues) => issues,
        Err(e) => {
            console.error(format!("Failed to get issues: {}", e));
            return 1;
        }
    };

    // If we ask for marketplace, fetch that too
    let marketplace_editions = if with_marketplace {
        console.info("Fetching marketplace editions...");
        let filters_books = tosho_nids::Filter::new()
            .add_filter(tosho_nids::FilterType::SeriesRunId, series_run_id)
            .with_order(tosho_nids::SortBy::FullTitle, tosho_nids::SortOrder::ASC)
            .with_per_page(25);

        match client.get_marketplace_books(Some(&filters_books)).await {
            Ok(editions) => Some(editions),
            Err(e) => {
                console.warn(format!("Failed to get marketplace data: {}", e));
                None
            }
        }
    } else {
        None
    };

    let series_url = format!(
        "https://{}/series/{}/{}",
        BASE_HOST,
        series_detail.id(),
        series_detail.slug()
    );
    let linked_title = linkify!(&series_url, series_detail.title());

    console.info(cformat!(
        "Showing information for <m,s>{}</m,s>:",
        linked_title
    ));
    console.info(cformat!(
        "  <s>ID</s>: {} / {}",
        series_detail.id(),
        series_detail.uuid()
    ));
    let series_run = format_series_run_date(series_detail.start_date(), series_detail.end_date())
        .unwrap_or("TBD".to_string());
    let status_text = match series_detail.end_date() {
        Some(_) => "Completed",
        None => "Ongoing",
    };
    console.info(cformat!(
        "  <s>Status</s>: {} ({})",
        status_text,
        series_run,
    ));

    // Print total issues and collected editions
    let mut issues_count_text = vec![];
    match series_detail.issues_count() {
        0 => issues_count_text.push(cformat!("<dim,s>No</dim,s> issues")),
        1 => issues_count_text.push(cformat!("<s>1</s> issue")),
        n => issues_count_text.push(cformat!(
            "<s>{}</s> issues",
            n.to_formatted_string(&Locale::en)
        )),
    };
    match series_detail.editions_count() {
        0 => issues_count_text.push(cformat!("<dim,s>No</dim,s> collected editions")),
        1 => issues_count_text.push(cformat!("<s>1</s> collected edition")),
        n => issues_count_text.push(cformat!(
            "<s>{}</s> collected editions",
            n.to_formatted_string(&Locale::en)
        )),
    };
    console.info(cformat!(
        "  <s>Issues</s>: {}",
        issues_count_text.join(" | ")
    ));

    // Do Publisher + Imprint
    let pubs_text = if let Some(imprint) = series_detail.imprint()
        && let Some(imprint_type) = imprint.imprint_type()
        && imprint_type != "primary"
    {
        format!("{} / {}", series_detail.publisher().name(), imprint.name())
    } else {
        series_detail.publisher().name().to_string()
    };
    console.info(cformat!("  <s>Publisher</s>: {}", pubs_text));

    // Print desc
    if let Some(desc) = series_detail.description() {
        console.info(cformat!("  <s>Description</s>: {}", desc));
    }

    // Print collected editions
    if !series_detail.editions().is_empty() {
        console.info(cformat!(
            "  <s>Collected Editions</s> (<m,s>{}</m,s> total):",
            series_detail
                .editions()
                .len()
                .to_formatted_string(&Locale::en)
        ));
        for edition in series_detail.editions() {
            let edition_url = format!("https://{}/item/{}", BASE_HOST, edition.id());
            let vol_text = match (
                edition.volume(),
                (edition.issue_range().min(), edition.issue_range().max()),
            ) {
                (Some(vol), (Some(min), Some(max))) if min == max => Some(format!("Vol. {}", vol)),
                (Some(vol), (Some(min), Some(max))) => {
                    Some(format!("Vol. {} (Issues #{}-#{})", vol, min, max))
                }
                (Some(vol), (Some(min), None)) => Some(format!("Vol. {} (Issues #{}+)", vol, min)),
                (Some(vol), (None, Some(max))) => Some(format!("Vol. {} (Up to {})", vol, max)),
                (Some(vol), (None, None)) => Some(format!("Vol. {}", vol)),
                (None, (Some(min), Some(max))) if min == max => Some(format!("Issue {}", min)),
                (None, (Some(min), Some(max))) => Some(format!("Issues {}-{}", min, max)),
                (None, (Some(min), None)) => Some(format!("Issue #{}+", min)),
                (None, (None, Some(max))) => Some(format!("Up to Issue {}", max)),
                (None, (None, None)) => None,
            };

            let linked_edition = linkify!(&edition_url, edition.title());

            let print_result = match vol_text {
                Some(vol_text) => {
                    cformat!("   - <m,s>{}</m,s> <s>{}</s>", linked_edition, vol_text)
                }
                None => cformat!("   - <m,s>{}</m,s>", linked_edition),
            };
            console.info(print_result);
            console.info(cformat!("     {}", edition_url));
            console.info(cformat!(
                "     <s>Price</s>: <g,s>$</g,s>{:.2}",
                tosho_nids::format_price(edition.price_usd())
            ));
        }
    } else {
        console.info("  <s>Collected Editions</s>: <dim,s>None</dim,s>");
    }

    if !issues_resp.data().is_empty() {
        console.info(cformat!(
            "  <s>Issues</s> (<m,s>{}</m,s> total):",
            issues_resp.data().len().to_formatted_string(&Locale::en)
        ));
        if issues_resp.count() > issues_resp.data().len() as u64 {
            console.info(cformat!(
                "    <dim,s>Note: There are more issues ({}) not shown here...</dim,s>",
                issues_resp.count().to_formatted_string(&Locale::en)
            ));
        }
        for issue in issues_resp.data() {
            let item_url = format!("https://{}/item/{}/{}", BASE_HOST, issue.id(), issue.slug());
            let linked_title = linkify!(&item_url, issue.full_title());

            let title_text = cformat!(
                "<s>{}</s> (<m,s>{}</m,s> / {})",
                linked_title,
                issue.id(),
                issue.uuid()
            );

            let title_smol_info = cformat!(
                "<b,s>{}</b,s> | <g,s>$</g,s>{:.2}",
                fmt_date(issue.release_date()),
                tosho_nids::format_price(issue.price_usd())
            );

            console.info(&format!("   - {}", title_text));
            console.info(&format!("     {}", item_url));
            console.info(&format!("     {}", title_smol_info));
        }
    }

    if let Some(marketplace_editions) = marketplace_editions
        && !marketplace_editions.data().is_empty()
    {
        console.info(cformat!(
            "  <s>Marketplaces</s> (<m,s>{}</m,s> total):",
            marketplace_editions
                .data()
                .len()
                .to_formatted_string(&Locale::en)
        ));
        if marketplace_editions.count() > marketplace_editions.data().len() as u64 {
            console.info(cformat!(
                "    <dim,s>Note: There are more issues ({}) not shown here...</dim,s>",
                marketplace_editions
                    .count()
                    .to_formatted_string(&Locale::en)
            ));
        }
        for book in marketplace_editions.data() {
            let book_url = format!("https://{}/item/{}", BASE_HOST, book.id());
            let linked_title = linkify!(&book_url, book.full_title());

            let title_text = cformat!(
                "<s>{}</s> (<m,s>{}</m,s> / {})",
                linked_title,
                book.id(),
                book.uuid()
            );
            let mut smol_infos = vec![
                cformat!(
                    "<s>{}</s> Editions",
                    book.marketplace_count().to_formatted_string(&Locale::en)
                ),
                cformat!(
                    "<s>{}</s> Remarques",
                    book.marketplace_remarque_count()
                        .to_formatted_string(&Locale::en)
                ),
            ];
            let (min_price, max_price) = (book.minimum_price(), book.maximum_price());

            if min_price == max_price {
                smol_infos.push(cformat!(
                    "<g,s>$</g,s>{:.2}",
                    tosho_nids::format_price(min_price)
                ))
            } else {
                smol_infos.push(cformat!(
                    "<g,s>$</g,s>{:.2} - <g,s>$</g,s>{:.2}",
                    tosho_nids::format_price(min_price),
                    tosho_nids::format_price(max_price)
                ));
            }

            console.info(&format!("   - {}", title_text));
            console.info(&format!("     {}", book_url));
            console.info(&format!("     {}", smol_infos.join(" | ")));
        }
    } else if with_marketplace {
        console.info("  <s>Marketplace Editions</s>: <dim,s>None</dim,s>");
    }

    0
}
