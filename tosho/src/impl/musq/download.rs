use std::path::{Path, PathBuf};

use clap::ValueEnum;

use color_print::cformat;
use tosho_musq::{
    proto::{ChapterV2, MangaDetailV2},
    ImageQuality, MUClient,
};

use crate::{
    cli::ExitCode,
    r#impl::models::{ChapterDetailDump, MangaDetailDump},
};

use super::common::common_purchase_select;

#[derive(Debug, Clone, Default)]
pub(crate) enum DownloadImageQuality {
    Normal,
    #[default]
    High,
}

impl ValueEnum for DownloadImageQuality {
    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        let input = if ignore_case {
            input.to_lowercase()
        } else {
            input.to_string()
        };
        match input.as_str() {
            "middle" => Ok(Self::Normal),
            "normal" => Ok(Self::Normal),
            "high" => Ok(Self::High),
            _ => Err(format!("Invalid image quality: {}", input)),
        }
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::Normal => Some(clap::builder::PossibleValue::new("normal")),
            Self::High => Some(clap::builder::PossibleValue::new("high")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Normal, Self::High]
    }
}

impl From<DownloadImageQuality> for ImageQuality {
    fn from(value: DownloadImageQuality) -> Self {
        match value {
            DownloadImageQuality::Normal => Self::Normal,
            DownloadImageQuality::High => Self::High,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct MUDownloadCliConfig {
    /// Disable all input prompt (a.k.a `autodownload`)
    pub(crate) no_input: bool,
    pub(crate) auto_purchase: bool,
    pub(crate) show_all: bool,
    pub(crate) quality: DownloadImageQuality,

    pub(crate) chapter_ids: Vec<usize>,
    /// The start chapter range.
    ///
    /// Used only when `no_input` is `true`.
    pub(crate) start_from: Option<u64>,
    /// The end chapter range.
    ///
    /// Used only when `no_input` is `true`.
    pub(crate) end_at: Option<u64>,

    pub(crate) no_paid_point: bool,
    pub(crate) no_xp_point: bool,
}

fn check_downloaded_image_count(image_dir: &PathBuf) -> Option<usize> {
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
        if path.is_file() && path.extension().unwrap() == "avif" {
            count += 1;
        }
    }

    Some(count)
}

fn create_chapters_info(manga_detail: MangaDetailV2) -> MangaDetailDump {
    let mut chapters: Vec<ChapterDetailDump> = vec![];
    for chapter in manga_detail.chapters {
        chapters.push(ChapterDetailDump::from(chapter));
    }

    MangaDetailDump::new(manga_detail.title, manga_detail.authors, chapters)
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

pub(crate) async fn musq_download(
    title_id: u64,
    dl_config: MUDownloadCliConfig,
    output_dir: PathBuf,
    client: &MUClient,
    console: &mut crate::term::Terminal,
) -> ExitCode {
    let (results, manga_detail, user_bal) = common_purchase_select(
        title_id,
        client,
        true,
        dl_config.show_all,
        dl_config.no_input,
        console,
    )
    .await;

    match (results, manga_detail, user_bal) {
        (Ok(results), Some(manga_detail), Some(coin_purse)) => {
            let results: Vec<&ChapterV2> = results
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
                        // allow if chapter_ids is empty or chapter id is in chapter_ids
                        dl_config.chapter_ids.is_empty()
                            || dl_config.chapter_ids.contains(&(ch.id as usize))
                    }
                })
                .collect();

            if results.is_empty() {
                return 1;
            }

            let mut coin_purse = coin_purse.clone();

            if dl_config.no_paid_point {
                coin_purse.paid = 0;
            }
            if dl_config.no_xp_point {
                coin_purse.event = 0;
            }

            console.info(&format!("Downloading {} chapters...", results.len()));
            let mut download_chapters = vec![];
            for chapter in results {
                if chapter.is_free() {
                    download_chapters.push(chapter);
                    continue;
                }

                let consume = client.calculate_coin(&coin_purse, chapter);
                if !consume.is_possible() {
                    if !dl_config.no_input {
                        console.warn(&cformat!(
                            "  Chapter <m,s>{}</> (<s>{}</>) is not available for purchase, skipping",
                            chapter.title,
                            chapter.id
                        ));
                        console.warn(&format!(
                            "   Need {} free coin, {} XP coin, and {} paid coin",
                            consume.get_free(),
                            consume.get_event(),
                            consume.get_paid()
                        ));
                    }

                    continue;
                }

                let mut should_purchase = dl_config.auto_purchase;
                if !dl_config.auto_purchase && !dl_config.no_input {
                    let prompt = cformat!(
                        "Chapter <m,s>{}</> (<s>{}</>) need to be purchased for {:?}, continue?",
                        chapter.title,
                        chapter.id,
                        consume
                    );
                    should_purchase = console.confirm(Some(&prompt));
                }

                if should_purchase {
                    console.info(&cformat!(
                        "  Purchasing chapter <m,s>{}</> (<s>{}</>) with consumption <s>{:?}</>...",
                        chapter.title,
                        chapter.id,
                        consume
                    ));

                    let purchase_result = client
                        .get_chapter_images(
                            chapter.id,
                            dl_config.quality.clone().into(),
                            Some(consume.clone()),
                        )
                        .await;

                    match purchase_result {
                        Err(err) => {
                            console.error(&format!("   Failed to purchase chapter: {}", err));
                            console.error(&cformat!(
                                "    Skipping chapter <m,s>{}</> (<s>{}</>)",
                                chapter.title,
                                chapter.id
                            ));
                        }
                        Ok(ch_view) => {
                            if ch_view.blocks.is_empty() {
                                console.warn(&cformat!(
                                    "   Unable to purchase chapter <m,s>{}</> (<s>{}</>) since image block is empty, skipping",
                                    chapter.title,
                                    chapter.id
                                ));
                            } else {
                                download_chapters.push(chapter);
                                coin_purse.free -= consume.get_free();
                                coin_purse.event -= consume.get_event();
                                coin_purse.paid -= consume.get_paid();
                            }
                        }
                    }
                }
            }

            if download_chapters.is_empty() {
                console.warn("No chapters to be download after filtering, aborting");
                return 1;
            }

            download_chapters.sort_by(|&a, &b| a.id.cmp(&b.id));

            let title_dir = get_output_directory(&output_dir, title_id, None, true);
            let dump_info = create_chapters_info(manga_detail);

            let title_dump_path = title_dir.join("_info.json");
            dump_info
                .dump(&title_dump_path)
                .expect("Failed to dump title info");

            let mut stored_blocks: Vec<tosho_musq::proto::PageBlock> = vec![];
            for chapter in download_chapters {
                console.info(&cformat!(
                    "  Downloading chapter <m,s>{}</> ({})...",
                    chapter.title,
                    chapter.id
                ));

                let image_blocks = match stored_blocks.iter().find(|&b| b.id == chapter.id) {
                    Some(img_blocks) => img_blocks.images.clone(),
                    None => {
                        let ch_viewer = client
                            .get_chapter_images(chapter.id, dl_config.quality.clone().into(), None)
                            .await;
                        if let Err(err) = ch_viewer {
                            console.error(&format!("Failed to download chapter: {}", err));
                            console.error(&cformat!(
                                "   Skipping chapter <m,s>{}</> (<s>{}</>)",
                                chapter.title,
                                chapter.id
                            ));
                            continue;
                        }

                        let ch_images = ch_viewer.unwrap();
                        if ch_images.blocks.is_empty() {
                            console.warn(&cformat!(
                            "   Unable to download chapter <m,s>{}</> (<s>{}</>) since image block is empty, skipping",
                            chapter.title,
                            chapter.id
                        ));
                            continue;
                        }

                        // push to stored blocks
                        ch_images.blocks.iter().for_each(|block| {
                            stored_blocks.push(block.clone());
                        });

                        let img_blocks = ch_images.blocks.iter().find(|&b| b.id == chapter.id);

                        match img_blocks {
                            Some(img_blocks) => img_blocks.images.clone(),
                            None => {
                                console.warn(&cformat!(
                                    "   Unable to download chapter <m,s>{}</> (<s>{}</>) since we can't find this chapter blocks, skipping",
                                    chapter.title,
                                    chapter.id
                                ));
                                continue;
                            }
                        }
                    }
                };

                let image_blocks: Vec<&tosho_musq::proto::ChapterPage> = image_blocks
                    .iter()
                    .filter(|&x| {
                        // only allow url with /page/ or /page_high/ in it
                        x.url.contains("/page/") || x.url.contains("/page_high/")
                    })
                    .collect();

                if image_blocks.is_empty() {
                    console.warn(&cformat!(
                        "   Chapter <m,s>{}</> (<s>{}</>) has no images, skipping",
                        chapter.title,
                        chapter.id
                    ));
                    continue;
                }

                let ch_dir = get_output_directory(&output_dir, title_id, Some(chapter.id), false);
                if let Some(count) = check_downloaded_image_count(&ch_dir) {
                    if count >= image_blocks.len() {
                        console.warn(&cformat!(
                            "   Chapter <m,s>{}</> (<s>{}</>) has been downloaded, skipping",
                            chapter.title,
                            chapter.id
                        ));
                        continue;
                    }
                }

                // create folder
                std::fs::create_dir_all(&ch_dir).unwrap();

                // download images
                let total_image_count = image_blocks.len() as u64;
                for image in image_blocks {
                    let file_number: u64 = image.file_stem().parse().unwrap();
                    let img_fn = format!("p{:03}.{}", file_number, image.extension());
                    let img_dl_path = ch_dir.join(&img_fn);
                    // async download
                    let writer = tokio::fs::File::create(&img_dl_path)
                        .await
                        .expect("Failed to create image file");

                    if console.is_debug() {
                        console.log(&cformat!(
                            "   Downloading image <s>{}</> to <s>{}</>...",
                            image.file_name(),
                            img_fn
                        ));
                    } else {
                        console.progress(total_image_count, 1, Some("Downloading".to_string()));
                    }

                    match client.stream_download(&image.url, writer).await {
                        Ok(_) => {}
                        Err(err) => {
                            console.error(&format!("    Failed to download image: {}", err));
                            // silent delete the file
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
