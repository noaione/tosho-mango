use tokio::time::{Duration, sleep};

use color_print::cformat;
use num_format::{Locale, ToFormattedString};
use tosho_amap::{AMClient, helper::ComicPurchase, models::ComicEpisodeInfoNode};

use crate::cli::ExitCode;

use super::{common::common_purchase_select, config::Config};

pub(crate) async fn amap_purchase(
    title_id: u64,
    client: &AMClient,
    account: &Config,
    console: &mut crate::term::Terminal,
) -> ExitCode {
    let (results, comic, user_bal) =
        common_purchase_select(title_id, client, account, false, false, false, console).await;

    match (results, comic, user_bal) {
        (Ok(results), Some(comic), Some(user_bal)) => {
            if results.is_empty() {
                return 1;
            }

            let mut claimed_total: u64 = 0;
            let mut failed_claimed: Vec<(ComicEpisodeInfoNode, String)> = vec![];

            let mut ticket_purse = user_bal.clone();
            for (idx, chapter) in results.iter().enumerate() {
                console.status(format!(
                    "Purchasing chapter(s): ({}/{})",
                    idx + 1,
                    results.len()
                ));

                let consume = ComicPurchase::from_episode_and_comic(
                    &comic,
                    chapter.info(),
                    &mut ticket_purse,
                );

                if consume.is_none() {
                    console.warn(cformat!(
                        "Unable to purchase chapter <magenta,bold>{}</> ({}), insufficient point balance!",
                        chapter.info().title(), chapter.info().id()
                    ));
                    failed_claimed.push((
                        chapter.info().clone(),
                        "Insufficient point balance".to_string(),
                    ));
                    continue;
                }

                let consume = consume.unwrap();
                let ch_view = client.get_comic_viewer(title_id, &consume).await;

                match ch_view {
                    Ok(ch_view) => {
                        if ch_view.info().pages().is_empty() {
                            console.warn(cformat!(
                                "Unable to purchase chapter <magenta,bold>{}</> ({}), no images found!",
                                chapter.info().title(),
                                chapter.info().id()
                            ));
                            failed_claimed
                                .push((chapter.info().clone(), "Failed when claiming".to_string()));
                            continue;
                        }

                        super::common::save_session_config(client, account);

                        // Sleep for 500ms to avoid being too fast
                        // and made the claiming failed
                        sleep(Duration::from_millis(500)).await;
                        claimed_total += 1;
                    }
                    Err(err) => {
                        console.warn(cformat!(
                            "Unable to purchase chapter <magenta,bold>{}</> ({}), error: {}",
                            chapter.info().title(),
                            chapter.info().id(),
                            err
                        ));
                        failed_claimed.push((chapter.info().clone(), format!("Error: {err}")));
                        continue;
                    }
                }
            }

            console.status(format!(
                "Purchased <magenta,bold>{}</> chapters",
                claimed_total.to_formatted_string(&Locale::en)
            ));
            if !failed_claimed.is_empty() {
                console.warn(format!(
                    "We failed to purchase {} chapters, you might want to retry",
                    failed_claimed.len()
                ));
                for (chapter, reason) in failed_claimed {
                    console.warn(cformat!(
                        "  - <bold>{}</> (ID: {}): <red,bold>{}</>",
                        chapter.title(),
                        chapter.id(),
                        reason
                    ));
                }
            }

            0
        }
        _ => 1,
    }
}

pub(crate) async fn amap_purchase_precalculate(
    title_id: u64,
    client: &AMClient,
    account: &Config,
    console: &crate::term::Terminal,
) -> ExitCode {
    let (results, _, _) =
        common_purchase_select(title_id, client, account, false, true, false, console).await;

    match results {
        Ok(results) => {
            if results.is_empty() {
                return 1;
            }

            console.info("Calculating chapters cost...");
            let price_ticket: u64 = results.iter().map(|c| c.info().price()).sum();

            let price_ticket_fmt = price_ticket.to_formatted_string(&Locale::en);
            let ch_count = results.len().to_formatted_string(&Locale::en);

            console.info("Precalculated purchase cost:");
            console.info(cformat!("  - <s>Total</>: {}", ch_count));
            console.info(cformat!("  - <s>Cost</>: {}T", price_ticket_fmt));

            0
        }
        _ => 1,
    }
}
