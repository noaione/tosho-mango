use std::collections::HashMap;

use crate::{
    cli::ExitCode,
    r#impl::nids::common::{PaginateAction, fmt_date, format_series_run, pagination_helper},
    linkify,
};
use color_print::cformat;
use num_format::{Locale, ToFormattedString};
use tosho_nids::{constants::BASE_HOST, models::SaleStatus};

fn print_issue_summary(issue: &tosho_nids::models::IssueSummary, console: &crate::term::Terminal) {
    let item_url = format!("https://{}/item/{}/{}", BASE_HOST, issue.id(), issue.slug());
    let linked_title = linkify!(&item_url, issue.full_title());

    let title_text = cformat!(
        "<s>{}</s> (<m,s>{}</m,s> / {})",
        linked_title,
        issue.id(),
        issue.uuid()
    );

    let pubs_text = if let Some(imprint) = issue.imprint()
        && let Some(imprint_type) = imprint.imprint_type()
        && imprint_type != "primary"
    {
        cformat!(
            "<s>{}</s> / <s>{}</s>",
            issue.publisher().name(),
            imprint.name()
        )
    } else {
        cformat!("<s>{}</s>", issue.publisher().name())
    };

    let title_smol_info = cformat!(
        "<b,s>{}</b,s> | {} | <g,s>$</g,s>{:.2}",
        fmt_date(issue.release_date()),
        pubs_text,
        tosho_nids::format_price(issue.price_usd())
    );

    console.info(format!("  {}", title_text));
    console.info(format!("   {}", item_url));
    console.info(format!("   {}", title_smol_info));
}

pub async fn nids_get_issues(
    base_filter: &mut tosho_nids::Filter,
    client: &tosho_nids::NIClient,
    console: &crate::term::Terminal,
) -> ExitCode {
    // Do initial request
    console.info("Fetching initial issues with the filter...");
    let issues = match client.get_issues(base_filter).await {
        Ok(issues) => issues,
        Err(e) => {
            console.error(format!("Failed to get issues: {}", e));
            return 1;
        }
    };

    if issues.data().is_empty() {
        console.info("No issues found with the given filters.");
        return 0;
    }

    let mut stop_code = 0;
    if issues.pages() > 1 {
        // Do paginated response
        let mut current_page: u32 = 1;
        let mut maximum_pages: u32 = issues.pages();
        let mut collected_issues: HashMap<u32, Vec<tosho_nids::models::IssueSummary>> =
            HashMap::from([(1, issues.data().to_vec())]);
        let mut current_data = collected_issues.get(&1).expect("Somehow missing page 1");

        loop {
            console.info(cformat!(
                "Showing page <magenta,bold>{}</> of <magenta,bold>{}</>:",
                current_page,
                maximum_pages
            ));
            for issue in current_data.iter() {
                print_issue_summary(issue, console);
            }
            if current_data.is_empty() {
                console.info("No issues found on this page.");
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
            if let Some(issues) = collected_issues.get(&current_page) {
                current_data = issues;
                console.clear_screen();
            } else {
                console.info(cformat!("Loading page <m,s>{}</m,s>...", current_page));
                let new_issues = match client.get_issues(base_filter).await {
                    Ok(issues) => issues,
                    Err(e) => {
                        console.error(format!("Failed to get issues: {}", e));
                        stop_code = 1;
                        break;
                    }
                };

                console.clear_screen();

                maximum_pages = new_issues.pages();
                // add correct data to collected_issues
                collected_issues.insert(current_page, new_issues.data().to_vec());
                current_data = collected_issues
                    .get(&current_page)
                    .expect("Somehow missing page after insert");
            }
        }
    } else {
        // Print all issues
        for issue in issues.data() {
            print_issue_summary(issue, console);
        }
    }

    stop_code
}

fn format_tags(genres: &[tosho_nids::models::Genre]) -> String {
    genres
        .iter()
        .map(|genre| {
            // let tag_url = format!("https://{}/genre/{}", BASE_HOST, tag.id());
            // let linked = linkify!(&tag_url, &genre.name());

            cformat!("<p(244),reverse,bold>{}</>", genre.name())
        })
        .collect::<Vec<String>>()
        .join(", ")
}

fn format_sale_status(status: SaleStatus) -> String {
    match status {
        SaleStatus::ForSale => cformat!("<g!,s>Available</g!,s>"),
        SaleStatus::PostSale => cformat!("<y!,s>Marketplace only</y!,s>"),
    }
}

pub async fn nids_get_issue(
    issue_id: u32,
    with_marketplace: bool,
    client: &tosho_nids::NIClient,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(cformat!("Fetching issue ID <m,s>{}</m,s>...", issue_id));
    let issue_detail = match client.get_issue(issue_id).await {
        Ok(issues) => issues,
        Err(e) => {
            console.error(format!("Failed to get issues: {}", e));
            return 1;
        }
    };

    // If we ask for marketplace, fetch that too
    let marketplace_editions = if with_marketplace {
        console.info("Fetching marketplace editions...");
        match client
            .get_marketplace_book_editions(issue_detail.uuid(), None)
            .await
        {
            Ok(editions) => Some(editions.data().to_vec()),
            Err(e) => {
                console.warn(format!("Failed to get marketplace data: {}", e));
                None
            }
        }
    } else {
        None
    };

    let issue_url = format!(
        "https://{}/item/{}/{}",
        BASE_HOST,
        issue_detail.id(),
        issue_detail.slug()
    );
    let linked_title = linkify!(&issue_url, issue_detail.full_title());

    console.info(cformat!(
        "Showing information for <m,s>{}</m,s>:",
        linked_title
    ));
    console.info(cformat!(
        "  <s>ID</s>: {} / {}",
        issue_detail.id(),
        issue_detail.uuid()
    ));
    if let Some(identifier) = issue_detail.variant_identifier()
        && !identifier.is_empty()
    {
        console.info(cformat!("  <s>Variant</s>: {}", identifier));
    }

    let series_text = format_series_run(
        issue_detail.series_run().title(),
        issue_detail.series_run().start_date(),
        issue_detail.series_run().end_date(),
    );
    console.info(cformat!(
        "  <s>Series</s>: {} (<dim>{}</dim> / <dim>{}</dim>)",
        series_text,
        issue_detail.series_run().id(),
        issue_detail.series_run().uuid()
    ));

    // Do release date
    console.info(cformat!(
        "  <s>Release Date</s>: {}",
        fmt_date(issue_detail.release_date())
    ));
    // Do Publisher + Imprint
    let pubs_text = if let Some(imprint) = issue_detail.imprint()
        && let Some(imprint_type) = imprint.imprint_type()
        && imprint_type != "primary"
    {
        format!("{} / {}", issue_detail.publisher().name(), imprint.name())
    } else {
        issue_detail.publisher().name().to_string()
    };
    console.info(cformat!("  <s>Publisher</s>: {}", pubs_text));

    // Print desc
    if let Some(desc) = issue_detail.description() {
        console.info(cformat!("  <s>Description</s>: {}", desc));
    }

    // Print price
    let post_sale_text = format_sale_status(issue_detail.status());
    console.info(cformat!(
        "  <s>Price</s>: <g,s>$</g,s>{:.2} ({})",
        tosho_nids::format_price(issue_detail.price_usd()),
        post_sale_text,
    ));

    // Do tags/genres
    if !issue_detail.genres().is_empty() {
        console.info(cformat!(
            "  <s>Genres/Tags</s>: {}",
            format_tags(issue_detail.genres())
        ));
    }

    // Show the downloadable/remarquable/resellable status
    let mut status_flags: Vec<String> = Vec::new();
    if issue_detail.is_resellable() {
        status_flags.push(cformat!("<g!,s>Resellable</g!,s>"));
    } else {
        status_flags.push(cformat!("<r!,s>Not Resellable</r!,s>"));
    }
    if issue_detail.is_remarquable() {
        status_flags.push(cformat!("<g!,s>Remarquable</g!,s>"));
    } else {
        status_flags.push(cformat!("<r!,s>Not Remarquable</r!,s>"));
    }
    if issue_detail.is_downloadable() {
        let mut download_txt = cformat!("<g!,s>Downloadable</g!,s>");
        match issue_detail.download_type() {
            Some(dt) => match dt {
                tosho_nids::models::DownloadType::Unavailable => {} // Do nothing
                tosho_nids::models::DownloadType::DRMFree => {
                    download_txt.push_str(&cformat!(" (<c>DRM-free</c>)"));
                }
                tosho_nids::models::DownloadType::Watermarked => {
                    download_txt.push_str(&cformat!(" (<r!>Visible Watermark)</r!>"));
                }
                tosho_nids::models::DownloadType::InvisibleWatermarked => {
                    download_txt.push_str(&cformat!(" (<r,s>Invisible Watermark</r,s>)"));
                }
            },
            None => {} // Do nothing
        }
        status_flags.push(download_txt);
    } else {
        status_flags.push(cformat!("<r!,s>Not Downloadable</r!,s>"));
    }
    console.info(format!("  {}", status_flags.join(" | ")));

    // Print creators
    if !issue_detail.creators().is_empty() {
        console.info(cformat!("  <s>Creators</s>:"));
    }
    for creator in issue_detail.creators() {
        let creator_url = format!("https://{}/creator/{}", BASE_HOST, creator.creator().id());
        let link_creator = linkify!(&creator_url, creator.creator().name());
        console.info(cformat!(
            "   - <s>{}</s>: {} [ID: {}]",
            creator.role(),
            link_creator,
            creator.creator().id()
        ));
    }

    // Print age, pages, language
    if !issue_detail.age_rating().is_empty() {
        console.info(cformat!("  <s>Age</s>: {}", issue_detail.age_rating()));
    }
    let fmt_num_pages = issue_detail.total_pages().to_formatted_string(&Locale::en);
    console.info(cformat!("  <s>Pages</s>: {}", fmt_num_pages));
    if !issue_detail.language().is_empty() {
        console.info(cformat!("  <s>Language</s>: {}", issue_detail.language()));
    }

    // If there is a variants, show 'em
    if !issue_detail.variants().is_empty() {
        console.info(cformat!(
            "  <s>Variants</s>: {} variant(s) available",
            issue_detail.variants().len()
        ));
        for variant in issue_detail.variants() {
            let variant_url = format!(
                "https://{}/item/{}/{}",
                BASE_HOST,
                variant.id(),
                variant.slug()
            );
            let var_text = format!("Variant {}", variant.variant_identifier().unwrap_or("N/A"));
            let link_variant = linkify!(&variant_url, var_text);
            console.info(cformat!(
                "   - <s>{}</s> (<m,s>{}</m,s> / {})",
                link_variant,
                variant.id(),
                variant.uuid()
            ));
            let post_sale_text = format_sale_status(variant.status());

            console.info(cformat!(
                "     <s>Price</s>: <g,s>$</g,s>{:.2} ({})",
                tosho_nids::format_price(variant.price_usd()),
                post_sale_text,
            ));
        }
    }

    if let Some(marketplace_editions) = marketplace_editions {
        println!(); // New line

        if marketplace_editions.is_empty() {
            console.info("  No marketplace editions found for this issue.");
        } else {
            console.info(cformat!(
                "  <s>Marketplaces</s>: {} edition(s) found",
                marketplace_editions.len()
            ));

            for edition in marketplace_editions {
                let mut post_info_text = String::new();
                if edition.signature() {
                    post_info_text.push_str(" | <b,s>Signed</b,s>");
                }
                console.info(cformat!(
                    "   - <s>Edition {}</s>{} - <g,s>$</g,s>{:.2}",
                    edition.index(),
                    post_info_text,
                    tosho_nids::format_price(edition.price_usd())
                ));
                console.info(cformat!(
                    "     <s>Owner</s>: {}",
                    edition.seller().username(),
                ));
                if let Some(notes) = edition.notes()
                    && !notes.is_empty()
                {
                    console.info(cformat!("     <s>Notes</s>: {}", notes));
                }
                console.info(cformat!(
                    "     <s>Listed At</s>: {}",
                    fmt_date(edition.created_at())
                ));
            }
        }
    }

    0
}
