use std::path::{Path, PathBuf};
use std::sync::Arc;

use color_print::cformat;
use tosho_kmkc::models::ImagePageNode;
use tosho_kmkc::{
    KMClient,
    models::{EpisodeNode, EpisodeViewerResponse, TicketInfoType, TitleNode},
};

use crate::r#impl::common::check_downloaded_image_count;
use crate::term::Terminal;
use crate::{
    cli::ExitCode,
    r#impl::models::{ChapterDetailDump, MangaDetailDump},
};

use super::{common::common_purchase_select, config::Config};

#[derive(Clone, Debug, Default)]
pub(crate) struct KMDownloadCliConfig {
    /// Disable all input prompt (a.k.a `autodownload`)
    pub(crate) no_input: bool,
    pub(crate) auto_purchase: bool,
    pub(crate) show_all: bool,

    /// The ID of the title to download.
    pub(crate) chapter_ids: Vec<usize>,

    /// Parallel download
    pub(crate) parallel: bool,

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

fn create_chapters_info(title: &TitleNode, chapters: &[EpisodeNode]) -> MangaDetailDump {
    let dumped_chapters: Vec<ChapterDetailDump> =
        chapters.iter().map(ChapterDetailDump::from).collect();

    MangaDetailDump::new(
        title.title().to_string(),
        title.author().to_string(),
        dumped_chapters,
    )
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

struct KMKCDownloadNode {
    client: KMClient,
    image: ImagePageNode,
    idx: usize,
    extension: String,
    seed: Option<u32>,
}

async fn kmkc_actual_downloader(
    node: KMKCDownloadNode,
    image_dir: PathBuf,
    console: Terminal,
    progress: Arc<indicatif::ProgressBar>,
) -> anyhow::Result<()> {
    let image_fn = format!("p{:03}.{}", node.idx, node.extension);
    let img_dl_path = image_dir.join(&image_fn);

    let writer = tokio::fs::File::create(&img_dl_path).await?;

    if console.is_debug() {
        console.log(cformat!(
            "   Downloading image <s>{}</> to <s>{}</>...",
            node.image.file_name(),
            image_fn
        ));
    }

    match node
        .client
        .stream_download(node.image.url(), node.seed, writer)
        .await
    {
        Ok(_) => {}
        Err(err) => {
            console.error(format!("    Failed to download image: {err}"));
            // silent delete the file
            tokio::fs::remove_file(&img_dl_path).await?;
        }
    }

    progress.inc(1);

    Ok(())
}

pub(crate) async fn kmkc_download(
    title_id: i32,
    dl_config: KMDownloadCliConfig,
    output_dir: PathBuf,
    client: &KMClient,
    account: &Config,
    console: &mut crate::term::Terminal,
) -> ExitCode {
    if let (Some(start), Some(end)) = (dl_config.start_from, dl_config.end_at)
        && start > end
    {
        console.error("Start chapter is greater than end chapter!");
        return 1;
    }

    let (results, title_detail, all_chapters, user_point) = common_purchase_select(
        title_id,
        client,
        account,
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
                                ch.id() >= start && ch.id() <= end
                            }
                            (Some(start), None) => {
                                ch.id() >= start // start to end
                            }
                            (None, Some(end)) => {
                                ch.id() <= end // 0 to end
                            }
                            _ => true,
                        }
                    } else {
                        dl_config.chapter_ids.is_empty()
                            || dl_config.chapter_ids.contains(&(ch.id() as usize))
                    }
                })
                .collect();

            if results.is_empty() {
                console.warn("No chapters after filtered by selected chapter ids");
                return 1;
            }

            let mut wallet_copy = user_point.point.point().clone();
            let mut ticket_entry = user_point.ticket.clone();
            console.info(format!("Downloading {} chapters...", results.len()));
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
                        chapter.title(),
                        chapter.id(),
                        chapter.point()
                    );
                    should_purchase = console.confirm(Some(&prompt));
                }

                if should_purchase {
                    if chapter.is_ticketable() && !dl_config.no_ticket {
                        let mut ticket_info = None;
                        if ticket_entry.is_title_available() {
                            console.info(cformat!(
                                "  Using title ticket to purchase chapter <m,s>{}</> (<s>{}</>)...",
                                chapter.title(),
                                chapter.id()
                            ));
                            ticket_info =
                                Some(TicketInfoType::Title(ticket_entry.info().title().unwrap()));
                            ticket_entry.subtract_title();
                        } else if ticket_entry.is_premium_available() {
                            console.info(cformat!(
                                "  Using premium ticket to purchase chapter <m,s>{}</> (<s>{}</>)...",
                                chapter.title(),
                                chapter.id()
                            ));
                            ticket_info = Some(TicketInfoType::Premium(
                                ticket_entry.info().premium().unwrap(),
                            ));
                            ticket_entry.subtract_premium();
                        }

                        if let Some(ticket) = ticket_info {
                            match client
                                .claim_episode_with_ticket(chapter.id(), &ticket)
                                .await
                            {
                                Ok(_) => {
                                    download_chapters.push(chapter);
                                    // if chapter.bonus_point > 0 {
                                    //     chapters_with_bonus.push(chapter.id);
                                    // }
                                    continue;
                                }
                                Err(e) => {
                                    console.error(format!(
                                        "   Failed to purchase chapter, ignoring: {e}"
                                    ));
                                }
                            }
                        }
                    }

                    if dl_config.no_point {
                        continue;
                    }

                    if !wallet_copy.can_purchase(chapter.point().try_into().unwrap_or(0)) {
                        console.warn(cformat!(
                            "   Chapter <m,s>{}</> (<s>{}</>), is not available for purchase, skipping",
                            chapter.title(),
                            chapter.id()
                        ));
                        let mut warn_info = format!("    Need {} point", chapter.point());
                        if chapter.is_ticketable() {
                            warn_info += " or ticket";
                        }
                        console.warn(&warn_info);
                        continue;
                    }

                    console.info(cformat!(
                        "  Purchasing chapter <m,s>{}</> (<s>{}</>) for {}P...",
                        chapter.title(),
                        chapter.id(),
                        chapter.point()
                    ));
                    match client.claim_episode(chapter, &mut wallet_copy).await {
                        Ok(_) => {
                            download_chapters.push(chapter);
                            // if chapter.bonus_point > 0 {
                            //     chapters_with_bonus.push(chapter.id);
                            // }
                        }
                        Err(e) => {
                            console.error(format!("   Failed to purchase chapter, ignoring: {e}"));
                        }
                    }
                }
            }

            if download_chapters.is_empty() {
                console.warn("No chapters to download after filtering, aborting");
                return 1;
            }

            download_chapters.sort_by(|&a, &b| a.id().cmp(&b.id()));

            let title_dir = get_output_directory(&output_dir, title_id, None, true);
            let dump_info = create_chapters_info(&title_detail, &all_chapters);

            let title_dump_path = title_dir.join("_info.json");
            dump_info
                .dump(&title_dump_path)
                .expect("Failed to dump title info");

            for chapter in download_chapters {
                console.info(cformat!(
                    "  Downloading chapter <m,s>{}</> ({})...",
                    chapter.title(),
                    chapter.id()
                ));

                let viewer_info = client.get_episode_viewer(chapter).await;

                if let Err(e) = viewer_info {
                    console.error(format!("Failed to get viewer info, ignoring: {e}"));
                    continue;
                }

                let viewer_info = viewer_info.unwrap();
                let image_dir =
                    get_output_directory(&output_dir, title_id, Some(chapter.id()), false);

                // precheck
                match &viewer_info {
                    EpisodeViewerResponse::Web(web) => {
                        if web.pages().is_empty() {
                            console.warn(cformat!(
                                "   Chapter <m,s>{}</> (<s>{}</>) has no pages, skipping",
                                chapter.title(),
                                chapter.id()
                            ));
                            continue;
                        }

                        if let Some(count) = check_downloaded_image_count(&image_dir, "png")
                            && count <= web.pages().len()
                        {
                            console.warn(cformat!(
                                "   Chapter <m,s>{}</> (<s>{}</>) already downloaded, skipping",
                                chapter.title(),
                                chapter.id()
                            ));
                            continue;
                        }

                        if console.is_debug() {
                            console.log(format!("    Seed: {}", web.scramble_seed()));
                        }
                    }
                    EpisodeViewerResponse::Mobile(mobile) => {
                        if mobile.pages().is_empty() {
                            console.warn(cformat!(
                                "   Chapter <m,s>{}</> (<s>{}</>) has no pages, skipping",
                                chapter.title(),
                                chapter.id()
                            ));
                            continue;
                        }

                        if let Some(count) = check_downloaded_image_count(&image_dir, "jpg")
                            && count >= mobile.pages().len()
                        {
                            console.warn(cformat!(
                                "   Chapter <m,s>{}</> (<s>{}</>) already downloaded, skipping",
                                chapter.title(),
                                chapter.id()
                            ));
                            continue;
                        }
                    }
                };

                // create dir
                std::fs::create_dir_all(&image_dir).unwrap();
                let image_blocks: Vec<tosho_kmkc::models::ImagePageNode> = match &viewer_info {
                    EpisodeViewerResponse::Mobile(mobile) => mobile.pages().to_vec(),
                    EpisodeViewerResponse::Web(web) => web
                        .pages()
                        .iter()
                        .map(|p| p.into())
                        .collect::<Vec<tosho_kmkc::models::ImagePageNode>>(),
                };
                let scramble_seed = match &viewer_info {
                    EpisodeViewerResponse::Mobile(_) => None,
                    EpisodeViewerResponse::Web(web) => Some(web.scramble_seed()),
                };
                let force_extensions = match &viewer_info {
                    EpisodeViewerResponse::Mobile(_) => "jpg",
                    EpisodeViewerResponse::Web(_) => "png",
                };

                let total_image_count = image_blocks.len() as u64;

                let progress = console.make_progress_arc(total_image_count, Some("Downloading"));

                if dl_config.parallel {
                    let tasks: Vec<_> = image_blocks
                        .iter()
                        .enumerate()
                        .map(|(idx, image)| {
                            // wrap function in async block
                            let wrap_client = client.clone();
                            let image_dir = image_dir.clone();
                            let cnsl = console.clone();
                            let image = image.clone();
                            let progress = Arc::clone(&progress);

                            tokio::spawn(async move {
                                match kmkc_actual_downloader(
                                    KMKCDownloadNode {
                                        client: wrap_client,
                                        image,
                                        idx,
                                        extension: force_extensions.to_string(),
                                        seed: scramble_seed,
                                    },
                                    image_dir,
                                    cnsl.clone(),
                                    progress,
                                )
                                .await
                                {
                                    Ok(_) => {}
                                    Err(e) => {
                                        cnsl.error(format!("    Failed to download image: {e}"));
                                    }
                                }
                            })
                        })
                        .collect();

                    futures_util::future::join_all(tasks).await;
                } else {
                    for (idx, image) in image_blocks.iter().enumerate() {
                        match kmkc_actual_downloader(
                            KMKCDownloadNode {
                                client: client.clone(),
                                image: image.clone(),
                                idx,
                                extension: force_extensions.to_string(),
                                seed: scramble_seed,
                            },
                            image_dir.clone(),
                            console.clone(),
                            Arc::clone(&progress),
                        )
                        .await
                        {
                            Ok(_) => {}
                            Err(e) => {
                                console.error(format!("    Failed to download image: {e}"));
                            }
                        }
                    }
                }

                progress.finish_with_message("Downloaded");
            }

            0
        }
        _ => 1,
    }
}
