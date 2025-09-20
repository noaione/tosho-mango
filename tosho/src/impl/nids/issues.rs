use crate::{
    cli::ExitCode,
    r#impl::nids::common::{PaginateAction, fmt_date, pagination_helper},
    linkify,
};
use color_print::cformat;
use tosho_nids::constants::BASE_HOST;

fn print_issue_summary(issue: &tosho_nids::models::IssueSummary, console: &crate::term::Terminal) {
    let item_url = format!("https://{}/item/{}/{}", BASE_HOST, issue.id(), issue.slug());
    let linked_title = linkify!(&item_url, issue.full_title());

    let title_text = cformat!(
        "<s>{}</s> (<m,s>{}</m,s> / {})",
        linked_title,
        issue.id(),
        issue.uuid()
    );
    let title_smol_info = cformat!(
        "<b,s>{}</b,s> | <s>{}</s> | <g,s>$</g,s>{:.2}",
        fmt_date(issue.release_date()),
        issue.publisher().name(),
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
        let mut correct_data = issues.data().to_vec();
        loop {
            console.info(cformat!(
                "Showing page <magenta,bold>{}</> of <magenta,bold>{}</>:",
                current_page,
                maximum_pages
            ));
            for issue in correct_data.iter() {
                print_issue_summary(issue, console);
            }
            if correct_data.is_empty() {
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
            console.info(cformat!("Loading page <m,s>{}</m,s>...", current_page));
            base_filter.set_page(current_page);
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
            correct_data = new_issues.data().to_vec();
        }
    } else {
        // Print all issues
        for issue in issues.data() {
            print_issue_summary(issue, console);
        }
    }

    stop_code
}
