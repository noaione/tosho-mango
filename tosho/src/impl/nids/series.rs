use std::collections::HashMap;

use crate::{
    cli::ExitCode,
    r#impl::nids::common::{
        PaginateAction, fmt_date, format_series_run, format_series_run_date, pagination_helper,
    },
    linkify,
};
use color_print::cformat;
use num_format::{Locale, ToFormattedString};
use tosho_nids::{constants::BASE_HOST, models::SaleStatus};

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
        1 => series_smols.push(cformat!("<s>1 issue</s>")),
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
