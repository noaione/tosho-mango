use std::collections::HashMap;

use crate::{
    cli::ExitCode,
    r#impl::nids::common::{PaginateAction, pagination_helper},
    linkify,
};
use color_print::cformat;
use num_format::{Locale, ToFormattedString};
use tosho_nids::constants::BASE_HOST;

fn print_publisher_list(
    publisher: &tosho_nids::models::Publisher,
    console: &crate::term::Terminal,
) {
    let publisher_url = format!("https://{}/publisher/{}", BASE_HOST, publisher.slug());
    let linked_title = linkify!(&publisher_url, publisher.name());

    let title_text = cformat!(
        "<s>{}</s> (<m,s>{}</m,s> / {})",
        linked_title,
        publisher.id(),
        publisher.slug()
    );

    console.info(format!("  {}", title_text));
    console.info(format!("   {}", publisher_url));
    let fmt_series = publisher
        .series_count()
        .unwrap_or(0)
        .to_formatted_string(&Locale::en);
    let fmt_issues = publisher
        .issues_count()
        .unwrap_or(0)
        .to_formatted_string(&Locale::en);
    let fmt_editions = publisher
        .collected_editions_count()
        .unwrap_or(0)
        .to_formatted_string(&Locale::en);
    console.info(cformat!(
        "   <m,s>{}</m,s> Series | <m,s>{}</m,s> Issues | <m,s>{}</m,s> Collected Editions",
        fmt_series,
        fmt_issues,
        fmt_editions
    ));
}

pub async fn nids_get_publishers(
    client: &tosho_nids::NIClient,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info("Fetching initial publishers...");

    let mut filters = tosho_nids::Filter::new()
        .with_order(tosho_nids::SortBy::Name, tosho_nids::SortOrder::ASC)
        .with_per_page(25)
        .with_page(1);

    let publishers = match client.get_publishers(Some(&filters)).await {
        Ok(publishers) => publishers,
        Err(e) => {
            console.error(format!("Failed to get publishers: {}", e));
            return 1;
        }
    };

    if publishers.data().is_empty() {
        console.info("No publishers found.");
        return 0;
    }

    let mut stop_code = 0;
    if publishers.pages() > 1 {
        // Do paginated response
        let mut current_page: u32 = 1;
        let mut maximum_pages: u32 = publishers.pages();
        let mut collected_issues: HashMap<u32, Vec<tosho_nids::models::Publisher>> =
            HashMap::from([(1, publishers.data().to_vec())]);
        let mut current_data = collected_issues.get(&1).expect("Somehow missing page 1");

        loop {
            console.info(cformat!(
                "Showing page <m,s>{}</m,s> of <m,s>{}</m,s>:",
                current_page,
                maximum_pages
            ));

            for publisher in current_data.iter() {
                print_publisher_list(publisher, console);
            }

            if current_data.is_empty() {
                console.info("No publishers found on this page.");
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
            filters.set_page(current_page);
            if let Some(pubs) = collected_issues.get(&current_page) {
                current_data = pubs;
                console.clear_screen();
            } else {
                console.info(cformat!("Loading page <m,s>{}</m,s>...", current_page));
                let new_pubs = match client.get_publishers(Some(&filters)).await {
                    Ok(issues) => issues,
                    Err(e) => {
                        console.error(format!("Failed to get issues: {}", e));
                        stop_code = 1;
                        break;
                    }
                };

                console.clear_screen();

                maximum_pages = new_pubs.pages();
                collected_issues.insert(current_page, new_pubs.data().to_vec());
                current_data = collected_issues
                    .get(&current_page)
                    .expect("Somehow missing page after insert");
            }
        }
    } else {
        // Print all publishers
        for publisher in publishers.data().iter() {
            print_publisher_list(publisher, console);
        }
    }

    stop_code
}

pub async fn nids_get_publisher(
    publisher_slug: &str,
    with_imprints: bool,
    client: &tosho_nids::NIClient,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(cformat!(
        "Fetching publisher <m,s>{}</m,s>...",
        publisher_slug
    ));

    let publisher = match client.get_publisher(publisher_slug).await {
        Ok(publisher) => publisher,
        Err(e) => {
            console.error(format!("Failed to get publisher: {}", e));
            return 1;
        }
    };

    let publisher_imprints = if with_imprints {
        match client.get_publisher_imprints(publisher_slug).await {
            Ok(imprints) => Some(imprints),
            Err(e) => {
                console.error(format!("Failed to get publisher imprints: {}", e));
                return 1;
            }
        }
    } else {
        None
    };

    let pub_url = format!("https://{}/publisher/{}", BASE_HOST, publisher.slug());
    let linked = linkify!(&pub_url, publisher.name());

    console.info(cformat!("Showing information for <m,s>{}</m,s>:", linked));
    console.info(cformat!(
        "  <s>ID</s>: {} / {}",
        publisher.id(),
        publisher.uuid()
    ));
    console.info(cformat!("  <s>URL</s>: {}", pub_url));
    console.info(cformat!(
        "  <s>Series</s>: {}",
        publisher
            .series_count()
            .unwrap_or(0)
            .to_formatted_string(&Locale::en)
    ));
    console.info(cformat!(
        "  <s>Issues</s>: {}",
        publisher
            .issues_count()
            .unwrap_or(0)
            .to_formatted_string(&Locale::en)
    ));
    console.info(cformat!(
        "  <s>Collected Editions</s>: {}",
        publisher
            .collected_editions_count()
            .unwrap_or(0)
            .to_formatted_string(&Locale::en)
    ));

    if let Some(imprints) = publisher_imprints
        && !imprints.data().is_empty()
    {
        console.info(cformat!(
            "  <s>Imprints</s> (<s>{}</s>):",
            imprints.data().len()
        ));

        for imprint in imprints.data() {
            console.info(cformat!(
                "   - <s>{}</s> (ID: {})",
                imprint.name(),
                imprint.id()
            ));
        }
    }

    0
}
