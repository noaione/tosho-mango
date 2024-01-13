use std::path::{Path, PathBuf};

use color_print::cformat;
use tosho_kmkc::models::{EpisodeNode, EpisodeViewerResponse, TicketInfoType, TitleNode};

use crate::{
    cli::ExitCode,
    r#impl::models::{ChapterDetailDump, MangaDetailDump},
};

use super::common::{common_purchase_select, select_single_account};

#[derive(Clone, Debug, Default)]
pub(crate) struct KMDownloadCliConfig {
    /// Disable all input prompt (a.k.a `autodownload`)
    pub(crate) no_input: bool,
    pub(crate) auto_purchase: bool,
    pub(crate) show_all: bool,

    pub(crate) chapter_ids: Vec<usize>,
    /// The start chapter range.
    ///
    /// Used only when `no_input` is `true`.
    pub(crate) start_from: Option<i32>,
    /// The end chapter range.
    ///
    /// Used only when `no_input` is `true`.
    pub(crate) end_at: Option<i32>,

    pub(crate) no_ticket: bool,
    pub(crate) no_point: bool,
}

fn check_downloaded_image_count(image_dir: &PathBuf, extension: &str) -> Option<usize> {
    // check if dir exist
    if !image_dir.exists() {
        return None;
    }

    // check if dir is dir
    if !image_dir.is_dir() {
        return None;
    }

    // check how many .avif files in the dir
    let mut count = 0;
    for entry in std::fs::read_dir(image_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().unwrap() == extension {
            count += 1;
        }
    }

    Some(count)
}

fn create_chapters_info(title: &TitleNode, chapters: Vec<EpisodeNode>) -> MangaDetailDump {
    let mut dumped_chapters: Vec<ChapterDetailDump> = vec![];
    for chapter in chapters {
        dumped_chapters.push(ChapterDetailDump::from(chapter));
    }

    MangaDetailDump::new(title.title.clone(), title.author.clone(), dumped_chapters)
}

fn get_output_directory(
    output_dir: &Path,
    title_id: i32,
    chapter_id: Option<i32>,
    create_folder: bool,
) -> PathBuf {
    let mut pathing = output_dir.to_path_buf();
    pathing.push(title_id.to_string());

    if let Some(chapter_id) = chapter_id {
        pathing.push(chapter_id.to_string());
    }

    if create_folder {
        std::fs::create_dir_all(&pathing).unwrap();
    }

    pathing
}

pub(crate) async fn kmkc_download(
    title_id: i32,
    dl_config: KMDownloadCliConfig,
    account_id: Option<&str>,
    output_dir: PathBuf,
    console: &mut crate::term::Terminal,
) -> ExitCode {
    let account = select_single_account(account_id);

    if account.is_none() {
        console.warn("Aborted");
        return 1;
    }

    if let (Some(start), Some(end)) = (dl_config.start_from, dl_config.end_at) {
        if start > end {
            console.error("Start chapter is greater than end chapter!");
            return 1;
        }
    }

    let account = account.unwrap();
    let (results, title_detail, all_chapters, client, user_point) = common_purchase_select(
        title_id,
        &account,
        true,
        dl_config.show_all,
        dl_config.no_input,
        console,
    )
    .await;

    match (results, title_detail, user_point) {
        (Ok(results), Some(title_detail), Some(user_point)) => {
            let results: Vec<&EpisodeNode> = results
                .iter()
                .filter(|&ch| {
                    if dl_config.no_input {
                        // check if chapter id is in range
                        match (dl_config.start_from, dl_config.end_at) {
                            (Some(start), Some(end)) => {
                                // between start and end
                                ch.id >= start && ch.id <= end
                            }
                            (Some(start), None) => {
                                ch.id >= start // start to end
                            }
                            (None, Some(end)) => {
                                ch.id <= end // 0 to end
                            }
                            _ => true,
                        }
                    } else {
                        dl_config.chapter_ids.is_empty()
                            || dl_config.chapter_ids.contains(&(ch.id as usize))
                    }
                })
                .collect();

            if results.is_empty() {
                console.warn("No chapters after filtered by selected chapter ids");
                return 1;
            }

            let mut wallet_copy = user_point.point.point.clone();
            let mut ticket_entry = user_point.ticket.clone();
            console.info(&format!("Downloading {} chapters...", results.len()));
            let mut download_chapters = vec![];
            // let mut chapters_with_bonus = vec![];
            for chapter in results {
                if chapter.is_available() {
                    download_chapters.push(chapter);
                    continue;
                }

                let mut should_purchase = dl_config.auto_purchase;
                if !dl_config.auto_purchase && !dl_config.no_input {
                    let prompt = cformat!(
                        "Chapter <m,s>{}</> (<s>{}</>) need to be purchased for {}P, continue?",
                        chapter.title,
                        chapter.id,
                        chapter.point
                    );
                    should_purchase = console.confirm(Some(&prompt));
                }

                if should_purchase {
                    if chapter.is_ticketable() && !dl_config.no_ticket {
                        let mut ticket_info = None;
                        if ticket_entry.is_title_available() {
                            console.info(&cformat!(
                                "  Using title ticket to purchase chapter <m,s>{}</> (<s>{}</>)...",
                                chapter.title,
                                chapter.id
                            ));
                            ticket_info = Some(TicketInfoType::Title(
                                ticket_entry.info.title.clone().unwrap(),
                            ));
                            ticket_entry.subtract_title();
                        } else if ticket_entry.is_premium_available() {
                            console.info(&cformat!(
                                "  Using premium ticket to purchase chapter <m,s>{}</> (<s>{}</>)...",
                                chapter.title,
                                chapter.id
                            ));
                            ticket_info = Some(TicketInfoType::Premium(
                                ticket_entry.info.premium.clone().unwrap(),
                            ));
                            ticket_entry.subtract_premium();
                        }

                        if let Some(ticket) = ticket_info {
                            match client.claim_episode_with_ticket(chapter.id, &ticket).await {
                                Ok(_) => {
                                    download_chapters.push(chapter);
                                    // if chapter.bonus_point > 0 {
                                    //     chapters_with_bonus.push(chapter.id);
                                    // }
                                    continue;
                                }
                                Err(e) => {
                                    console.error(&format!(
                                        "   Failed to purchase chapter, ignoring: {}",
                                        e
                                    ));
                                }
                            }
                        }
                    }

                    if dl_config.no_point {
                        continue;
                    }

                    if !wallet_copy.can_purchase(chapter.point.try_into().unwrap_or(0)) {
                        console.warn(&cformat!(
                            "   Chapter <m,s>{}</> (<s>{}</>), is not available for purchase, skipping",
                            chapter.title,
                            chapter.id
                        ));
                        let mut warn_info = format!("    Need {} point", chapter.point);
                        if chapter.is_ticketable() {
                            warn_info += " or ticket";
                        }
                        console.warn(&warn_info);
                        continue;
                    }

                    console.info(&cformat!(
                        "  Purchasing chapter <m,s>{}</> (<s>{}</>) for {}P...",
                        chapter.title,
                        chapter.id,
                        chapter.point
                    ));
                    match client.claim_episode(chapter, &mut wallet_copy).await {
                        Ok(_) => {
                            download_chapters.push(chapter);
                            // if chapter.bonus_point > 0 {
                            //     chapters_with_bonus.push(chapter.id);
                            // }
                        }
                        Err(e) => {
                            console
                                .error(&format!("   Failed to purchase chapter, ignoring: {}", e));
                        }
                    }
                }
            }

            if download_chapters.is_empty() {
                console.warn("No chapters to download after filtering, aborting");
                return 1;
            }

            download_chapters.sort_by(|&a, &b| a.id.cmp(&b.id));

            let title_dir = get_output_directory(&output_dir, title_id, None, true);
            let dump_info = create_chapters_info(&title_detail, all_chapters);

            let title_dump_path = title_dir.join("_info.json");
            dump_info
                .dump(&title_dump_path)
                .expect("Failed to dump title info");

            for chapter in download_chapters {
                console.info(&cformat!(
                    "  Downloading chapter <m,s>{}</> ({})...",
                    chapter.title,
                    chapter.id
                ));

                let viewer_info = client.get_episode_viewer(chapter).await;

                if let Err(e) = viewer_info {
                    console.error(&format!("Failed to get viewer info, ignoring: {}", e));
                    continue;
                }

                let viewer_info = viewer_info.unwrap();
                let image_dir =
                    get_output_directory(&output_dir, title_id, Some(chapter.id), false);

                // precheck
                match &viewer_info {
                    EpisodeViewerResponse::Web(web) => {
                        if web.pages.is_empty() {
                            console.warn(&cformat!(
                                "   Chapter <m,s>{}</> (<s>{}</>) has no pages, skipping",
                                chapter.title,
                                chapter.id
                            ));
                            continue;
                        }

                        if let Some(count) = check_downloaded_image_count(&image_dir, "png") {
                            if count >= web.pages.len() {
                                console.warn(&cformat!(
                                    "   Chapter <m,s>{}</> (<s>{}</>) already downloaded, skipping",
                                    chapter.title,
                                    chapter.id
                                ));
                                continue;
                            }
                        }

                        if console.is_debug() {
                            console.log(&format!("    Seed: {}", web.scramble_seed));
                        }
                    }
                    EpisodeViewerResponse::Mobile(mobile) => {
                        if mobile.pages.is_empty() {
                            console.warn(&cformat!(
                                "   Chapter <m,s>{}</> (<s>{}</>) has no pages, skipping",
                                chapter.title,
                                chapter.id
                            ));
                            continue;
                        }

                        if let Some(count) = check_downloaded_image_count(&image_dir, "jpg") {
                            if count >= mobile.pages.len() {
                                console.warn(&cformat!(
                                    "   Chapter <m,s>{}</> (<s>{}</>) already downloaded, skipping",
                                    chapter.title,
                                    chapter.id
                                ));
                                continue;
                            }
                        }
                    }
                };

                // create dir
                std::fs::create_dir_all(&image_dir).unwrap();
                let image_blocks: Vec<tosho_kmkc::models::ImagePageNode> = match &viewer_info {
                    EpisodeViewerResponse::Mobile(mobile) => mobile.pages.clone(),
                    EpisodeViewerResponse::Web(web) => {
                        web.pages
                            .iter()
                            .map(|p| p.clone().into())
                            .collect::<Vec<tosho_kmkc::models::ImagePageNode>>()
                    }
                };
                let scramble_seed = match &viewer_info {
                    EpisodeViewerResponse::Mobile(_) => None,
                    EpisodeViewerResponse::Web(web) => Some(web.scramble_seed),
                };
                let force_extensions = match &viewer_info {
                    EpisodeViewerResponse::Mobile(_) => "jpg",
                    EpisodeViewerResponse::Web(_) => "png",
                };
                let total_image_count = image_blocks.len() as u64;
                for (idx, image) in image_blocks.iter().enumerate() {
                    let image_fn = format!("p{:03}.{}", idx, force_extensions);
                    let img_dl_path = image_dir.join(&image_fn);

                    let writer = tokio::fs::File::create(&img_dl_path)
                        .await
                        .expect("Failed to create image file!");

                    if console.is_debug() {
                        console.log(&cformat!(
                            "   Downloading image <s>{}</> to <s>{}</>...",
                            image.file_name(),
                            image_fn
                        ));
                    } else {
                        console.progress(total_image_count, 1, Some("Downloading".to_string()));
                    }

                    match client
                        .stream_download(&image.url, scramble_seed, writer)
                        .await
                    {
                        Ok(_) => {}
                        Err(err) => {
                            console.error(&format!("    Failed to download image: {}", err));
                            // silent delete the file
                            tokio::fs::remove_file(&img_dl_path)
                                .await
                                .unwrap_or_default();
                        }
                    }

                    // claim bonus point (disable for now :D)
                    // if chapters_with_bonus.contains(&chapter.id) {
                    //     console.info(&cformat!(
                    //         "   Claiming bonus point for chapter <m,s>{}</> (<s>{}</>)...",
                    //         chapter.title,
                    //         chapter.id
                    //     ));

                    //     match client.finish_episode_viewer(chapter).await {
                    //         Ok(finish_res) => {
                    //             console.info(&cformat!(
                    //                 "    Claimed <s,yellow>{}</> bonus point for chapter <m,s>{}</> (<s>{}</>)",
                    //                 finish_res.bonus_point,
                    //                 chapter.title,
                    //                 chapter.id
                    //             ));
                    //         }
                    //         Err(err) => {
                    //             console.error(&format!("    Failed to claim bonus point: {}", err));
                    //         }
                    //     }
                    // }
                }
                console.stop_progress(Some("Downloaded".to_string()));
            }

            0
        }
        _ => 1,
    }
}
