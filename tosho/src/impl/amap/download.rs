use std::path::{Path, PathBuf};

use color_print::cformat;
use tosho_amap::{
    AMClient,
    helper::ComicPurchase,
    models::{ComicEpisodeInfo, ComicInfo},
};

use crate::{
    cli::ExitCode,
    r#impl::{
        common::check_downloaded_image_count,
        models::{ChapterDetailDump, MangaDetailDump},
    },
};

use super::{common::common_purchase_select, config::Config};

#[derive(Clone, Debug, Default)]
pub(crate) struct AMDownloadCliConfig {
    /// Disable all input prompt (a.k.a `autodownload`)
    pub(crate) no_input: bool,
    pub(crate) auto_purchase: bool,
    pub(crate) show_all: bool,

    pub(crate) chapter_ids: Vec<usize>,
    /// The start chapter range.
    ///
    /// Used only when `no_input` is `true`.
    pub(crate) start_from: Option<u64>,
    /// The end chapter range.
    ///
    /// Used only when `no_input` is `true`.
    pub(crate) end_at: Option<u64>,

    // Ticket related
    pub(crate) no_premium: bool,
    pub(crate) no_purchased: bool,
}

fn create_chapters_info(manga_detail: &ComicInfo) -> MangaDetailDump {
    let chapters: Vec<ChapterDetailDump> = manga_detail
        .episodes()
        .iter()
        .map(ChapterDetailDump::from)
        .collect();

    let merged_authors = manga_detail
        .authors()
        .iter()
        .map(|a| a.info().name().to_string())
        .collect::<Vec<String>>()
        .join(", ");

    MangaDetailDump::new(manga_detail.title().to_string(), merged_authors, chapters)
}

fn get_output_directory(
    output_dir: &Path,
    title_id: u64,
    chapter_id: Option<u64>,
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

pub(crate) async fn amap_download(
    title_id: u64,
    dl_config: AMDownloadCliConfig,
    output_dir: PathBuf,
    client: &AMClient,
    account: &Config,
    console: &mut crate::term::Terminal,
) -> ExitCode {
    let (results, manga_detail, user_bal) = common_purchase_select(
        title_id,
        client,
        account,
        true,
        dl_config.show_all,
        dl_config.no_input,
        console,
    )
    .await;

    match (results, manga_detail, user_bal) {
        (Ok(results), Some(manga_detail), Some(coin_purse)) => {
            let results: Vec<&ComicEpisodeInfo> = results
                .iter()
                .filter(|&ch| {
                    if dl_config.no_input {
                        // check if chapter id is in range
                        match (dl_config.start_from, dl_config.end_at) {
                            (Some(start), Some(end)) => {
                                // between start and end
                                ch.info().id() >= start && ch.info().id() <= end
                            }
                            (Some(start), None) => {
                                ch.info().id() >= start // start to end
                            }
                            (None, Some(end)) => {
                                ch.info().id() <= end // 0 to end
                            }
                            _ => true,
                        }
                    } else {
                        // allow if chapter_ids is empty or chapter id is in chapter_ids
                        dl_config.chapter_ids.is_empty()
                            || dl_config.chapter_ids.contains(&(ch.info().id() as usize))
                    }
                })
                .collect();

            if results.is_empty() {
                return 1;
            }

            let mut ticket_purse = coin_purse.clone();

            if dl_config.no_premium {
                ticket_purse.set_premium(0);
            }

            if dl_config.no_purchased {
                ticket_purse.set_purchased(0);
            }

            console.info(format!("Downloading {} chapters...", results.len()));
            let mut download_chapters = vec![];
            for chapter in results {
                if chapter.info().is_available() {
                    download_chapters.push(chapter);
                    continue;
                }

                let consume = ComicPurchase::from_episode_and_comic(
                    &manga_detail,
                    chapter.info(),
                    &mut ticket_purse,
                );

                if consume.is_none() {
                    if !dl_config.no_input {
                        console.warn(cformat!(
                            "  Chapter <m,s>{}</> (<s>{}</>) is not available for purchase, skipping",
                            chapter.info().title(),
                            chapter.info().id()
                        ));
                    }

                    continue;
                }

                let mut should_purchase = dl_config.auto_purchase;
                if !dl_config.auto_purchase && !dl_config.no_input {
                    let prompt = cformat!(
                        "Chapter <m,s>{}</> (<s>{}</>) need to be purchased for {:?}, continue?",
                        chapter.info().title(),
                        chapter.info().id(),
                        consume
                    );
                    should_purchase = console.confirm(Some(&prompt));
                }

                if should_purchase {
                    console.info(cformat!(
                        "  Purchasing chapter <m,s>{}</> (<s>{}</>) with consumption <s>{:?}</>...",
                        chapter.info().title(),
                        chapter.info().id(),
                        consume
                    ));

                    let consume = consume.unwrap();

                    let purchase_result = client.get_comic_viewer(title_id, &consume).await;

                    match purchase_result {
                        Err(err) => {
                            console.error(format!("   Failed to purchase chapter: {err}"));
                            console.error(format!(
                                "    Skipping chapter <m,s>{}</> (<s>{}</>)",
                                chapter.info().title(),
                                chapter.info().id()
                            ));
                        }
                        Ok(ch_view) => {
                            if ch_view.info().pages().is_empty() {
                                console.warn(cformat!(
                                    "   Unable to purchase chapter <m,s>{}</> (<s>{}</>) since image block is empty, skipping",
                                    chapter.info().title(),
                                    chapter.info().id()
                                ));
                            } else {
                                download_chapters.push(chapter);
                                ticket_purse.subtract_bonus(consume.bonus);
                                ticket_purse.subtract_premium(consume.purchased);
                                ticket_purse.subtract_premium(consume.premium);
                                super::common::save_session_config(client, account);
                            }
                        }
                    }
                }
            }

            if download_chapters.is_empty() {
                console.warn("No chapters to be download after filtering, aborting");
                return 1;
            }

            download_chapters.sort_by(|&a, &b| a.info().id().cmp(&b.info().id()));

            let title_dir = get_output_directory(&output_dir, title_id, None, true);
            let dump_info = create_chapters_info(&manga_detail);

            let title_dump_path = title_dir.join("_info.json");
            dump_info
                .dump(&title_dump_path)
                .expect("Failed to dump title info");

            for chapter in download_chapters {
                let info = chapter.info();
                console.info(cformat!(
                    "  Downloading chapter <m,s>{}</> ({})...",
                    info.title(),
                    info.id()
                ));

                let rent_term = manga_detail.rental_term();

                let consume = ComicPurchase {
                    id: info.id(),
                    rental_term: rent_term.map(|e| e.to_string()),
                    is_free_daily: info.is_free_daily(),
                    ..Default::default()
                };

                let ch_view = client.get_comic_viewer(title_id, &consume).await;
                if let Err(err) = ch_view {
                    console.error(format!("Failed to download chapter: {err}"));
                    console.error(cformat!(
                        "   Skipping chapter <m,s>{}</> (<s>{}</>)",
                        info.title(),
                        info.id(),
                    ));
                    continue;
                }

                let ch_view = ch_view.unwrap();
                if ch_view.info().pages().is_empty() {
                    console.warn(cformat!(
                        "   Unable to download chapter <m,s>{}</> (<s>{}</>) since image block is empty, skipping",
                        info.title(),
                        info.id(),
                    ));
                    continue;
                }

                // save session_v2
                super::common::save_session_config(client, account);

                let ch_pages = ch_view.info().pages();
                let ch_dir = get_output_directory(&output_dir, title_id, Some(info.id()), false);
                if let Some(count) = check_downloaded_image_count(&ch_dir, "jpg")
                    && count >= ch_pages.len()
                {
                    console.warn(cformat!(
                        "   Chapter <m,s>{}</> (<s>{}</>) has been downloaded, skipping",
                        info.title(),
                        info.id(),
                    ));
                    continue;
                }

                // create folder
                std::fs::create_dir_all(&ch_dir).unwrap();

                // download images
                let total_image_count = ch_pages.len() as u64;
                for (idx, image) in ch_pages.iter().enumerate() {
                    let img_fn = format!("p{idx:03}.jpg");
                    let img_dl_path = ch_dir.join(&img_fn);
                    // async download
                    let writer = tokio::fs::File::create(&img_dl_path)
                        .await
                        .expect("Failed to create image file");

                    if console.is_debug() {
                        console.log(cformat!(
                            "   Downloading image <s>{}</> to <s>{}</>...",
                            idx + 1,
                            img_fn
                        ));
                    } else {
                        console.progress(total_image_count, 1, Some("Downloading".to_string()));
                    }

                    match client.stream_download(image.info().url(), writer).await {
                        Ok(_) => {}
                        Err(err) => {
                            console.error(format!("    Failed to download image: {err}"));
                            // silently delete the file
                            tokio::fs::remove_file(&img_dl_path)
                                .await
                                .unwrap_or_default();
                        }
                    }
                }
                console.stop_progress(Some("Downloaded".to_string()));
            }

            0
        }
        _ => 1,
    }
}
