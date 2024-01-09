use std::path::PathBuf;

use clap::ValueEnum;

use color_print::cformat;
use tosho_musq::{
    proto::{ChapterV2, MangaDetailV2},
    ImageQuality,
};

use crate::{
    cli::ExitCode,
    r#impl::models::{ChapterDetailDump, MangaDetailDump},
};

use super::common::{common_purchase_select, select_single_account};

#[derive(Debug, Clone)]
pub(crate) enum DownloadImageQuality {
    Normal,
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
    output_dir: &PathBuf,
    title_id: u64,
    chapter_id: Option<u64>,
    create_folder: bool,
) -> PathBuf {
    let mut pathing = output_dir.clone();
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
    chapter_ids: Vec<usize>,
    show_all: bool,
    auto_purchase: bool,
    image_quality: DownloadImageQuality,
    account_id: Option<&str>,
    output_dir: PathBuf,
    console: &mut crate::term::Terminal,
) -> ExitCode {
    let account = select_single_account(account_id);

    if account.is_none() {
        console.warn("Aborted");
        return 1;
    }

    let account = account.unwrap();
    let (results, manga_detail, client, user_bal) =
        common_purchase_select(title_id, &account, true, show_all, console).await;

    match (results, manga_detail, user_bal) {
        (Ok(results), Some(manga_detail), Some(coin_purse)) => {
            let results: Vec<&ChapterV2> = results
                .iter()
                .filter(|&ch| {
                    // allow if chapter_ids is empty or chapter id is in chapter_ids
                    chapter_ids.is_empty() || chapter_ids.contains(&(ch.id as usize))
                })
                .collect();

            if results.is_empty() {
                return 1;
            }

            let mut coin_purse = coin_purse.clone();

            console.info(&format!("Downloading {} chapters...", results.len()));
            let mut download_chapters = vec![];
            for chapter in results {
                if chapter.is_free() {
                    download_chapters.push(chapter);
                    continue;
                }

                let consume = client.calculate_coin(&coin_purse, &chapter);
                if !consume.is_possible() {
                    console.warn(&cformat!(
                        "Chapter <m,s>{}</> (<s>{}</>) is not available for purchase, skipping",
                        chapter.title,
                        chapter.id
                    ));
                    console.warn(&format!(
                        "Need {} free coin, {} XP coin, and {} paid coin",
                        consume.get_free(),
                        consume.get_event(),
                        consume.get_paid()
                    ));
                    continue;
                }

                let mut should_purchase = auto_purchase;
                if !auto_purchase {
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
                        "Purchasing chapter <m,s>{}</> (<s>{}</>) with consumption <s>{:?}</>...",
                        chapter.title,
                        chapter.id,
                        consume
                    ));

                    let purchase_result = client
                        .get_chapter_images(
                            chapter.id,
                            image_quality.clone().into(),
                            Some(consume.clone()),
                        )
                        .await;

                    match purchase_result {
                        Err(err) => {
                            console.error(&format!("Failed to purchase chapter: {}", err));
                            console.error(&format!(
                                "Skipping chapter <m,s>{}</> (<s>{}</>)",
                                chapter.title, chapter.id
                            ));
                        }
                        Ok(ch_view) => {
                            if ch_view.blocks.is_empty() {
                                console.warn(&cformat!(
                                    "Unable to purchase chapter <m,s>{}</> (<s>{}</>) since image block is empty, skipping",
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

            let title_dir = get_output_directory(&output_dir, title_id, None, true);
            let dump_info = create_chapters_info(manga_detail);

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

                let ch_images = client
                    .get_chapter_images(chapter.id, image_quality.clone().into(), None)
                    .await;
                if let Err(err) = ch_images {
                    console.error(&format!("Failed to download chapter: {}", err));
                    console.error(&format!(
                        "   Skipping chapter <m,s>{}</> (<s>{}</>)",
                        chapter.title, chapter.id
                    ));
                    continue;
                }

                let ch_images = ch_images.unwrap();
                if ch_images.blocks.is_empty() {
                    console.warn(&cformat!(
                        "   Unable to download chapter <m,s>{}</> (<s>{}</>) since image block is empty, skipping",
                        chapter.title,
                        chapter.id
                    ));
                    continue;
                }

                if ch_images.blocks.len() > 1 {
                    console.warn(&cformat!(
                        "   Chapter <m,s>{}</> (<s>{}</>) has {} blocks, report to developer!",
                        chapter.title,
                        chapter.id,
                        ch_images.blocks.len()
                    ));
                    continue;
                }

                let image_blocks = ch_images.blocks[0].clone().images;
                if image_blocks.is_empty() {
                    console.warn(&cformat!(
                        "   Unable to download chapter <m,s>{}</> (<s>{}</>) since image block is empty, skipping",
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
