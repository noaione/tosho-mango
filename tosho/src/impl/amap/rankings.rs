use color_print::cformat;

use super::common::do_print_search_information;
use super::common::make_client;
use super::common::select_single_account;
use crate::cli::ExitCode;

pub(crate) async fn amap_discovery(
    account_id: Option<&str>,
    console: &crate::term::Terminal,
) -> ExitCode {
    let acc_info = select_single_account(account_id);

    if acc_info.is_none() {
        console.warn("Aborted!");

        return 1;
    }

    let acc_info = acc_info.unwrap();

    let client = make_client(&acc_info.clone().into());

    console.info(&cformat!(
        "Getting discovery for <magenta,bold>{}</>...",
        acc_info.id
    ));
    let results = client.get_discovery().await;

    match results {
        Ok(results) => {
            // updated
            for updated in results.updated.iter() {
                console.info(&format!("{}:", updated.header.title));
                do_print_search_information(&updated.comics, false, None);
                println!();
            }

            // free campaigns
            if !results.free_campaigns.is_empty() {
                console.info("Free Campaigns:");
            }
            for campaign in results.free_campaigns.iter() {
                console.info(&format!("  {}:", campaign.header.title));
                do_print_search_information(&campaign.comics, false, Some(4));
                println!();
            }

            // tags1
            for tags1 in results.tags1.iter() {
                console.info(&format!("Popular {}:", tags1.header.title));
                do_print_search_information(&tags1.comics, false, None);
                println!();
            }
            // tags2
            for tags2 in results.tags2.iter() {
                console.info(&format!("Popular {}:", tags2.header.title));
                do_print_search_information(&tags2.comics, false, None);
                println!();
            }

            // completed
            for completed in results.completed.iter() {
                console.info(&format!("{}:", completed.header.title));
                do_print_search_information(&completed.comics, false, None);
                println!();
            }

            0
        }
        Err(e) => {
            console.error(&format!("Failed to fetch home discovery: {}", e));

            1
        }
    }
}
