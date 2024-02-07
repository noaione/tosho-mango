use color_print::cformat;
use tosho_amap::AMClient;

use super::{common::do_print_search_information, config::Config};
use crate::cli::ExitCode;

pub(crate) async fn amap_discovery(
    client: &AMClient,
    acc_info: &Config,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(&cformat!(
        "Getting discovery for <magenta,bold>{}</>...",
        acc_info.id
    ));
    let results = client.get_discovery().await;

    match results {
        Ok(results) => {
            super::common::save_session_config(client, acc_info);

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
