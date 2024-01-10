use tokio::time::{sleep, Duration};

use color_print::cformat;
use num_format::{Locale, ToFormattedString};
use tosho_musq::proto::ChapterV2;

use crate::cli::ExitCode;

use super::common::{common_purchase_select, select_single_account};

pub(crate) async fn musq_purchase(
    title_id: u64,
    account_id: Option<&str>,
    console: &mut crate::term::Terminal,
) -> ExitCode {
    let account = select_single_account(account_id);

    if account.is_none() {
        console.warn("Aborted");
        return 1;
    }

    let account = account.unwrap();
    let (results, _, client, user_bal) =
        common_purchase_select(title_id, &account, false, true, false, console).await;

    match (results, user_bal) {
        (Ok(results), Some(user_bal)) => {
            if results.is_empty() {
                return 1;
            }

            let mut claimed_total: u64 = 0;
            let mut failed_claimed: Vec<(ChapterV2, String)> = vec![];

            let mut user_point = user_bal.clone();
            for (idx, chapter) in results.iter().enumerate() {
                console.status(format!(
                    "Purchasing chapter(s): ({}/{})",
                    idx + 1,
                    results.len()
                ));

                let consume = client.calculate_coin(&user_point, chapter);

                if !consume.is_possible() {
                    console.warn(&cformat!(
                        "Unable to purchase chapter <magenta,bold>{}</> (ID: {}), insufficient point balance!",
                        chapter.title, title_id
                    ));
                    failed_claimed
                        .push((chapter.clone(), "Insufficient point balance".to_string()));
                    continue;
                }

                user_point.free -= consume.get_free();
                user_point.paid -= consume.get_paid();
                user_point.event -= consume.get_event();
                let img_chapter = client
                    .get_chapter_images(chapter.id, tosho_musq::ImageQuality::High, Some(consume))
                    .await
                    .unwrap();
                if img_chapter.blocks.is_empty() {
                    console.warn(&cformat!(
                        "Unable to purchase chapter <magenta,bold>{}</> (ID: {}), no images found!",
                        chapter.title,
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
                console.warn(&format!(
                    "We failed to purchase {} chapters, you might want to retry",
                    failed_claimed.len()
                ));
                for (chapter, reason) in failed_claimed {
                    console.warn(&cformat!(
                        "  - <bold>{}</> (ID: {}): <red,bold>{}</>",
                        chapter.title,
                        chapter.id,
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
    account_id: Option<&str>,
    console: &crate::term::Terminal,
) -> ExitCode {
    let account = select_single_account(account_id);

    if account.is_none() {
        console.warn("Aborted");
        return 1;
    }

    let account = account.unwrap();
    let (results, _, _, user_bal) =
        common_purchase_select(title_id, &account, false, true, false, console).await;

    match (results, user_bal) {
        (Ok(results), Some(user_bal)) => {
            if results.is_empty() {
                return 1;
            }

            console.info("Calculating chapters cost...");
            let total_coin: u64 = results.iter().map(|c| c.price).sum();

            let total_bal = user_bal.sum().to_formatted_string(&Locale::en);
            let paid_point = user_bal.paid.to_formatted_string(&Locale::en);
            let xp_point = user_bal.event.to_formatted_string(&Locale::en);
            let free_point = user_bal.free.to_formatted_string(&Locale::en);

            let total_coin_fmt = total_coin.to_formatted_string(&Locale::en);
            let ch_count = results.len().to_formatted_string(&Locale::en);

            console.info("Your current point balance:");
            console.info(&cformat!("  - <s>Total</>: {}", total_bal));
            console.info(&cformat!("  - <s>Paid point</>: {}c", paid_point));
            console.info(&cformat!("  - <s>Event/XP point</>: {}c", xp_point));
            console.info(&cformat!("  - <s>Free point</>: {}c", free_point));
            console.info("Precalculated purchase cost:");
            console.info(&cformat!("  - <s>Total</>: {}", ch_count));
            console.info(&cformat!("  - <s>Cost</>: {}c", total_coin_fmt));

            0
        }
        _ => 1,
    }
}
