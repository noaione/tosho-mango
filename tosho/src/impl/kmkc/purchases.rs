use color_print::cformat;
use num_format::{Locale, ToFormattedString};
use tosho_kmkc::{
    constants::BASE_HOST,
    models::{EpisodeNode, TicketInfoType},
};

use crate::{cli::ExitCode, linkify};

use super::common::{common_purchase_select, select_single_account};

pub(crate) async fn kmkc_purchase(
    title_id: i32,
    account_id: Option<&str>,
    console: &mut crate::term::Terminal,
) -> ExitCode {
    let account = select_single_account(account_id);

    if account.is_none() {
        console.warn("Aborted");
        return 1;
    }

    let account = account.unwrap();
    let (results, _, _, client, user_point) =
        common_purchase_select(title_id, &account, false, false, false, console).await;

    match (results, user_point) {
        (Ok(results), Some(user_point)) => {
            if results.is_empty() {
                return 1;
            }

            let mut wallet_copy = user_point.point.point.clone();
            let mut ticket_entry = user_point.ticket.clone();

            let mut chapter_point_claim: Vec<EpisodeNode> = vec![];
            let mut ticketing_claim: Vec<(EpisodeNode, TicketInfoType)> = vec![];
            // let mut chapter_point_back: Vec<EpisodeNode> = vec![];
            for chapter in results {
                if chapter.is_available() {
                    console.warn(&cformat!(
                        "Chapter <m,s>{}</> is already purchased, skipping",
                        chapter.title
                    ));
                    continue;
                }

                if chapter.is_ticketable() && ticket_entry.is_title_available() {
                    // if chapter.bonus_point > 0 {
                    //     chapter_point_back.push(chapter.clone());
                    // }
                    ticketing_claim.push((
                        chapter,
                        TicketInfoType::Title(ticket_entry.info.title.clone()),
                    ));
                    ticket_entry.subtract_title();
                } else if chapter.is_ticketable() && ticket_entry.is_premium_available() {
                    // if chapter.bonus_point > 0 {
                    //     chapter_point_back.push(chapter.clone());
                    // }
                    ticketing_claim.push((
                        chapter,
                        TicketInfoType::Premium(ticket_entry.info.premium.clone()),
                    ));
                    ticket_entry.subtract_premium();
                } else if wallet_copy.can_purchase(chapter.point.try_into().unwrap_or(0)) {
                    wallet_copy.subtract(chapter.point.try_into().unwrap_or(0));
                    // if chapter.bonus_point > 0 {
                    //     chapter_point_back.push(chapter.clone());
                    // }
                    chapter_point_claim.push(chapter);
                }
            }

            let total_claim = chapter_point_claim.len() + ticketing_claim.len();

            if total_claim == 0 {
                console.warn("No chapter selected, aborting...");

                return 1;
            }

            console.info("Precalculate purchase information...");
            console.info(&cformat!(
                "  - <bold>With point:</> {} chapters",
                chapter_point_claim.len()
            ));
            console.info(&cformat!(
                "  - <bold>With ticket:</> {} chapters",
                ticketing_claim.len()
            ));

            console.status(format!("Purchasing chapter(s)... (1/{})", total_claim));
            let mut purchase_count = 0;
            let mut failure_count = 0_u64;

            for (chapter, ticket_info) in ticketing_claim {
                purchase_count += 1;
                console.status(format!(
                    "Purchasing chapter(s)... ({}/{})",
                    purchase_count, total_claim
                ));

                let result = client
                    .claim_episode_with_ticket(chapter.id, &ticket_info)
                    .await;

                if let Err(error) = result {
                    console.error(&format!("Failed to purchase chapter: {}", error));
                    failure_count += 1;
                }
            }

            if !chapter_point_claim.is_empty() {
                console.status(format!(
                    "Purchasing chapter(s)... ({}/{}) [point]",
                    purchase_count, total_claim
                ));

                // convert Vec<EpisodeNode> to Vec<&EpisodeNode>
                let temp_chapter_claim: Vec<&EpisodeNode> =
                    chapter_point_claim.iter().collect::<Vec<&EpisodeNode>>();

                let mut mutable_point = user_point.point.point.clone();

                let result = client
                    .claim_episodes(temp_chapter_claim, &mut mutable_point)
                    .await;

                match result {
                    Ok(_) => {
                        purchase_count += chapter_point_claim.len();
                    }
                    Err(error) => {
                        console.error(&format!("Failed to purchase chapter: {}", error));
                        failure_count += chapter_point_claim.len() as u64;
                    }
                }
            }

            console.stop_status_msg(cformat!("Purchased <m,s>{}</> chapters", purchase_count));

            if failure_count > 0 {
                console.warn(&cformat!(
                    "  There is <m,s>{}</> chapters that we failed to purchase",
                    failure_count
                ));
            }

            // disable point claim for now
            // if !chapter_point_back.is_empty() {
            //     console.info(&cformat!(
            //         "Claiming back point from <s>{}</> chapters...",
            //         chapter_point_back.len(),
            //     ));

            //     for chapter in chapter_point_back {
            //         let result = client.get_episode_viewer(&chapter).await;
            //         match result {
            //             Ok(_) => {
            //                 console.info(&cformat!(
            //                     "  Claiming back point from <m,s>{}</> ({})",
            //                     chapter.title,
            //                     chapter.id
            //                 ));
            //                 let claim_back_res = client.finish_episode_viewer(&chapter).await;
            //                 match claim_back_res {
            //                     Ok(finish_res) => {
            //                         console.info(&cformat!(
            //                             "   Claimed back <yellow,s>{}</> point from <m,s>{}</> ({})",
            //                             finish_res.bonus_point,
            //                             chapter.title,
            //                             chapter.id
            //                         ));
            //                     }
            //                     Err(error) => {
            //                         console.error(&cformat!(
            //                             "   Failed to claim back point from <m,s>{}</> ({}): <red,s>{}</>",
            //                             chapter.title,
            //                             chapter.id,
            //                             error
            //                         ));
            //                     }
            //                 }
            //             }
            //             Err(err) => {
            //                 console.warn(&cformat!(
            //                     "   Chapter <m,s>{}</> ({}) is not available: <red,s>{}</>",
            //                     chapter.title,
            //                     chapter.id,
            //                     err
            //                 ));
            //             }
            //         }
            //     }
            // }

            0
        }
        _ => 1,
    }
}

pub(crate) async fn kmkc_purchased(
    account_id: Option<&str>,
    console: &crate::term::Terminal,
) -> ExitCode {
    let account = select_single_account(account_id);

    if account.is_none() {
        console.warn("Aborted");
        return 1;
    }

    let account = account.unwrap();
    console.info(&cformat!(
        "Getting user purchased title for <m,s>{}</>...",
        account.get_username()
    ));

    let client = super::common::make_client(&account.into());

    let results = client.get_purchased().await;

    match results {
        Ok(results) => {
            if results.is_empty() {
                console.warn("No purchased title found");
                return 1;
            }

            console.info(&cformat!(
                "Purchased title (<m,s>{}</> results):",
                results.len()
            ));

            for result in results {
                let manga_url = format!("https://{}/title/{}", BASE_HOST.as_str(), result.id);
                let linked = linkify!(&manga_url, &result.title);

                console.info(&cformat!("  {} ({})", linked, result.id));
                console.info(&format!("   {}", manga_url));
            }

            0
        }
        Err(error) => {
            console.error(&format!("Failed to get purchased title: {}", error));
            1
        }
    }
}

pub(crate) async fn kmkc_purchase_precalculate(
    title_id: i32,
    account_id: Option<&str>,
    console: &mut crate::term::Terminal,
) -> ExitCode {
    let account = select_single_account(account_id);

    if account.is_none() {
        console.warn("Aborted");
        return 1;
    }

    let account = account.unwrap();
    let (results, _, _, _, user_point) =
        common_purchase_select(title_id, &account, false, false, false, console).await;

    match (results, user_point) {
        (Ok(results), Some(user_point)) => {
            if results.is_empty() {
                return 1;
            }

            let mut wallet_copy = user_point.point.point.clone();
            let mut ticket_entry = user_point.ticket.clone();

            let mut chapter_point_claim: Vec<EpisodeNode> = vec![];
            let mut ticketing_claim: Vec<(EpisodeNode, TicketInfoType)> = vec![];
            for chapter in results {
                if chapter.is_available() {
                    continue;
                }

                if chapter.is_ticketable() && ticket_entry.is_title_available() {
                    ticketing_claim.push((
                        chapter,
                        TicketInfoType::Title(ticket_entry.info.title.clone()),
                    ));
                    ticket_entry.subtract_title();
                } else if chapter.is_ticketable() && ticket_entry.is_premium_available() {
                    ticketing_claim.push((
                        chapter,
                        TicketInfoType::Premium(ticket_entry.info.premium.clone()),
                    ));
                    ticket_entry.subtract_premium();
                } else if wallet_copy.can_purchase(chapter.point.try_into().unwrap_or(0)) {
                    wallet_copy.subtract(chapter.point.try_into().unwrap_or(0));
                    wallet_copy.add(chapter.bonus_point.try_into().unwrap_or(0));
                    chapter_point_claim.push(chapter);
                }
            }

            let total_claim = chapter_point_claim.len() + ticketing_claim.len();

            if total_claim == 0 {
                console.warn("No chapter selected, aborting...");

                return 1;
            }

            console.info("Your current point balance:");
            let total_bal = user_point
                .point
                .point
                .total_point()
                .to_formatted_string(&Locale::en);
            let paid_point = user_point
                .point
                .point
                .paid_point
                .to_formatted_string(&Locale::en);
            let free_point = user_point
                .point
                .point
                .free_point
                .to_formatted_string(&Locale::en);
            let premium_ticket = user_point
                .point
                .ticket
                .total_num
                .to_formatted_string(&Locale::en);
            console.info(&cformat!(
                "  - <bold>Total:</> <cyan!,bold><reverse>{}</>c</cyan!,bold>",
                total_bal
            ));
            console.info(&cformat!(
                "  - <bold>Paid point:</> <g,bold><reverse>{}</>c</g,bold>",
                paid_point
            ));
            console.info(&cformat!(
                "  - <bold>Free point:</> <cyan,bold><reverse>{}</>c</cyan,bold>",
                free_point
            ));
            console.info(&cformat!(
                "  - <bold>Premium ticket:</> <yellow,bold><reverse>{}</> ticket</yellow,bold>",
                premium_ticket
            ));
            console.info(&cformat!(
                "  - <bold>Title ticket?</bold>: {}",
                ticket_entry.is_title_available()
            ));

            let coin_total = chapter_point_claim
                .iter()
                .map(|ch| ch.point)
                .sum::<i32>()
                .to_formatted_string(&Locale::en);
            let ticket_total = ticketing_claim.len().to_formatted_string(&Locale::en);
            let total_claim = total_claim.to_formatted_string(&Locale::en);
            let use_title_ticket = ticketing_claim
                .iter()
                .filter(|(_, ticket)| matches!(ticket, TicketInfoType::Title(_)))
                .count()
                > 0;

            console.info("Precalculated purchase cost:");
            console.info(&cformat!("  - <bold>Total</>: {}", total_claim));
            console.info(&cformat!("  - <bold>Coins</>: {}c", coin_total));
            console.info(&cformat!("  - <bold>Ticket</>: {}c", ticket_total));

            if use_title_ticket {
                console.info("     Will also use title ticket!")
            }

            0
        }
        _ => 1,
    }
}
