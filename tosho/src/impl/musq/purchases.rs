use tokio::time::{sleep, Duration};

use color_print::cformat;
use num_format::{Locale, ToFormattedString};
use tosho_musq::{proto::ChapterV2, MUClient};

use crate::cli::ExitCode;

use super::common::common_purchase_select;

pub(crate) async fn musq_purchase(
    title_id: u64,
    client: &MUClient,
    console: &mut crate::term::Terminal,
) -> ExitCode {
    let (results, _, user_bal) =
        common_purchase_select(title_id, client, false, true, false, console).await;

    match (results, user_bal) {
        (Ok(results), Some(user_bal)) => {
            if results.is_empty() {
                return 1;
            }

            let mut claimed_total: u64 = 0;
            let mut failed_claimed: Vec<(ChapterV2, String)> = vec![];

            let mut user_point = user_bal;
            for (idx, chapter) in results.iter().enumerate() {
                console.status(format!(
                    "Purchasing chapter(s): ({}/{})",
                    idx + 1,
                    results.len()
                ));

                let consume = match client.calculate_coin(&user_point, chapter) {
                    Ok(consume) => consume,
                    Err(e) => {
                        console.warn(cformat!(
                            "Unable to purchase chapter <magenta,bold>{}</> (ID: {}), error: {}",
                            chapter.title(),
                            title_id,
                            e
                        ));
                        failed_claimed.push((chapter.clone(), e.to_string()));
                        continue;
                    }
                };

                if !consume.is_possible() {
                    console.warn(cformat!(
                        "Unable to purchase chapter <magenta,bold>{}</> (ID: {}), insufficient point balance!",
                        chapter.title(), title_id
                    ));
                    failed_claimed
                        .push((chapter.clone(), "Insufficient point balance".to_string()));
                    continue;
                }

                user_point.subtract_free(consume.get_free());
                user_point.subtract_event(consume.get_event());
                user_point.subtract_paid(consume.get_paid());
                let img_chapter = client
                    .get_chapter_images(chapter.id(), tosho_musq::ImageQuality::High, Some(consume))
                    .await
                    .unwrap();
                if img_chapter.blocks().is_empty() {
                    console.warn(cformat!(
                        "Unable to purchase chapter <magenta,bold>{}</> (ID: {}), no images found!",
                        chapter.title(),
                        title_id
                    ));
                    failed_claimed.push((chapter.clone(), "Failed when claiming".to_string()));
                    continue;
                }

                // Sleep for 500ms to avoid being too fast
                // and made the claiming failed
                sleep(Duration::from_millis(500)).await;
                claimed_total += 1;
            }

            console.stop_status_msg(format!("Purchased {} chapters!", claimed_total));
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

pub(crate) async fn musq_purchase_precalculate(
    title_id: u64,
    client: &MUClient,
    console: &crate::term::Terminal,
) -> ExitCode {
    let (results, _, _) =
        common_purchase_select(title_id, client, false, true, false, console).await;

    match results {
        Ok(results) => {
            if results.is_empty() {
                return 1;
            }

            console.info("Calculating chapters cost...");
            let total_coin: u64 = results.iter().map(|c| c.price()).sum();

            let total_coin_fmt = total_coin.to_formatted_string(&Locale::en);
            let ch_count = results.len().to_formatted_string(&Locale::en);

            console.info("Precalculated purchase cost:");
            console.info(cformat!("  - <s>Total</>: {}", ch_count));
            console.info(cformat!("  - <s>Cost</>: {}c", total_coin_fmt));

            0
        }
        _ => 1,
    }
}
