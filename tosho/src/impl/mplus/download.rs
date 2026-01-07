use std::path::{Path, PathBuf};
use std::sync::Arc;

use clap::ValueEnum;
use color_eyre::eyre::{Context, OptionExt};
use color_print::cformat;
use tosho_mplus::proto::{Chapter, ChapterPage, TitleDetail};
use tosho_mplus::{APIResponse, ImageQuality, MPClient};

use crate::r#impl::common::check_downloaded_image_count;
use crate::term::Terminal;
use crate::{
    r#impl::models::{ChapterDetailDump, MangaDetailDump},
    term::ConsoleChoice,
};

#[derive(Debug, Clone, Default)]
pub(crate) enum DownloadImageQuality {
    /// Low quality images
    Low,
    /// Normal quality images
    Normal,
    /// High quality images
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
            "low" => Ok(Self::Low),
            "normal" | "middle" | "standard" => Ok(Self::Normal),
            "super_high" | "high_quality" | "high" => Ok(Self::High),
            _ => Err(format!("Invalid image quality: {input}")),
        }
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::Low => Some(clap::builder::PossibleValue::new("low")),
            Self::Normal => Some(clap::builder::PossibleValue::new("normal")),
            Self::High => Some(clap::builder::PossibleValue::new("high")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Low, Self::Normal, Self::High]
    }
}

impl From<DownloadImageQuality> for ImageQuality {
    fn from(value: DownloadImageQuality) -> Self {
        match value {
            DownloadImageQuality::Low => Self::Low,
            DownloadImageQuality::Normal => Self::Normal,
            DownloadImageQuality::High => Self::High,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct MPDownloadCliConfig {
    /// Disable all input prompt (a.k.a `autodownload`)
    pub(crate) no_input: bool,
    pub(crate) show_all: bool,
    pub(crate) quality: DownloadImageQuality,

    /// The ID of the title to download.
    pub(crate) chapter_ids: Vec<usize>,

    /// Parallel download
    pub(crate) parallel: bool,
    /// Threads to use for parallel download
    pub(crate) threads: usize,
    /// The start chapter range.
    ///
    /// Used only when `no_input` is `true`.
    pub(crate) start_from: Option<u64>,
    /// The end chapter range.
    ///
    /// Used only when `no_input` is `true`.
    pub(crate) end_at: Option<u64>,
}

fn create_chapters_info(title: &TitleDetail) -> color_eyre::Result<MangaDetailDump> {
    let dumped_chapters: Vec<ChapterDetailDump> = title
        .flat_chapters_group()
        .iter()
        .map(ChapterDetailDump::from)
        .collect();

    let act_title = title.title().ok_or_eyre("Failed to get title data")?;

    Ok(MangaDetailDump::new(
        act_title.title().to_string(),
        act_title.author().to_string(),
        dumped_chapters,
    ))
}

fn get_output_directory(
    output_dir: &Path,
    title_id: u64,
    chapter_id: Option<u64>,
    create_folder: bool,
) -> color_eyre::Result<PathBuf> {
    let mut pathing = output_dir.to_path_buf();
    pathing.push(format!("MP_{title_id}"));

    if let Some(chapter_id) = chapter_id {
        pathing.push(chapter_id.to_string());
    }

    if create_folder {
        std::fs::create_dir_all(&pathing)?;
    }

    Ok(pathing)
}

fn do_chapter_select(
    result: &TitleDetail,
    show_all: bool,
    console: &mut crate::term::Terminal,
) -> color_eyre::Result<Vec<Chapter>> {
    let title_info = result.title().ok_or_eyre("Failed to get title data")?;
    let flat_chapters = result.flat_chapters_group();
    console.info("Title information:");
    console.info(cformat!("  - <bold>ID:</> {}", title_info.id()));
    console.info(cformat!("  - <bold>Title:</> {}", title_info.title()));
    console.info(cformat!(
        "  - <bold>Chapters:</> {} chapters",
        flat_chapters.len()
    ));

    let title_labels_subs = result
        .title_labels()
        .map(|x| x.plan_type())
        .unwrap_or(tosho_mplus::helper::SubscriptionPlan::Basic);
    let user_subs = result
        .user_subscription()
        .map(|x| x.plan())
        .unwrap_or(tosho_mplus::helper::SubscriptionPlan::Basic);

    let select_choices: Vec<ConsoleChoice> = flat_chapters
        .iter()
        .filter_map(|ch| {
            if ch.is_free() || ch.is_ticketed() || user_subs >= title_labels_subs || show_all {
                Some(ConsoleChoice {
                    name: ch.chapter_id().to_string(),
                    value: ch.as_chapter_title(),
                })
            } else {
                None
            }
        })
        .collect();

    let selected_chapters = console.select("Select chapter to download", select_choices);
    match selected_chapters {
        Some(selected) => {
            let selected_chapters: Vec<Chapter> = selected
                .iter()
                .map(|x| {
                    let ch_id = x.name.parse::<u64>().expect("Failed to parse chapter ID");

                    flat_chapters
                        .iter()
                        .find(|&ch| ch.chapter_id() == ch_id)
                        .expect(&format!("Failed to find chapter ID: {}", ch_id))
                        .clone()
                })
                .collect();

            if selected_chapters.is_empty() {
                console.warn("No chapters selected, aborting...");
            }

            Ok(selected_chapters)
        }
        None => {
            console.warn("Aborted!");
            Ok(vec![])
        }
    }
}

struct MPDownloadNode {
    client: MPClient,
    image: ChapterPage,
    idx: usize,
    extension: String,
}

async fn mplus_actual_downloader(
    node: MPDownloadNode,
    image_dir: PathBuf,
    console: Terminal,
    progress: Arc<indicatif::ProgressBar>,
) -> color_eyre::eyre::Result<()> {
    let image_fn = format!("p{:03}.{}", node.idx, node.extension);
    let img_dl_path = image_dir.join(&image_fn);

    let writer = tokio::fs::File::create(&img_dl_path).await?;

    if console.is_debug() {
        console.log(cformat!(
            "   Downloading image <s>{}</> to <s>{}</>...",
            node.image.url(),
            image_fn
        ));
    }

    match node.client.stream_download(node.image.url(), writer).await {
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

pub(crate) async fn mplus_download(
    title_id: u64,
    dl_config: MPDownloadCliConfig,
    output_dir: PathBuf,
    client: &MPClient,
    console: &mut crate::term::Terminal,
) -> color_eyre::Result<()> {
    if let (Some(start), Some(end)) = (dl_config.start_from, dl_config.end_at)
        && start > end
    {
        console.error("Start chapter is greater than end chapter!");
        return Err(color_eyre::eyre::eyre!(
            "Start chapter is greater than end chapter!"
        ));
    }

    console.info(cformat!(
        "Fetching info for ID <magenta,bold>{}</>...",
        title_id
    ));

    let results = client
        .get_title_details(title_id)
        .await
        .context("Unable to connect to M+")?;

    match results {
        tosho_mplus::APIResponse::Success(results) => {
            let select_chapters = if dl_config.no_input {
                results.flat_chapters_group()
            } else {
                do_chapter_select(&results, dl_config.show_all, console)?
            };

            let title_labels_subs = results
                .title_labels()
                .map(|x| x.plan_type())
                .unwrap_or(tosho_mplus::helper::SubscriptionPlan::Basic);
            let user_subs = results
                .user_subscription()
                .map(|x| x.plan())
                .unwrap_or(tosho_mplus::helper::SubscriptionPlan::Basic);

            let mut download_chapters: Vec<&Chapter> = select_chapters
                .iter()
                .filter(|&ch| {
                    if dl_config.no_input {
                        // check if chapter id is in range
                        match (dl_config.start_from, dl_config.end_at) {
                            (Some(start), Some(end)) => {
                                // between start and end
                                ch.chapter_id() >= start && ch.chapter_id() <= end
                            }
                            (Some(start), None) => {
                                ch.chapter_id() >= start // start to end
                            }
                            (None, Some(end)) => {
                                ch.chapter_id() <= end // 0 to end
                            }
                            _ => true,
                        }
                    } else {
                        dl_config.chapter_ids.is_empty()
                            || dl_config.chapter_ids.contains(&(ch.chapter_id() as usize))
                    }
                })
                .filter(|&ch| ch.is_free() || ch.is_ticketed() || user_subs >= title_labels_subs)
                .collect();

            if download_chapters.is_empty() {
                console.warn("No chapters after filtered by selected chapter ids");
                return Err(color_eyre::eyre::eyre!(
                    "No chapters after filtered by selected chapter ids"
                ));
            }

            download_chapters.sort_by(|&a, &b| a.published_at().cmp(&b.published_at()));

            let title_dir = get_output_directory(&output_dir, title_id, None, true)?;
            let dump_info = create_chapters_info(&results)?;

            let title_dump_path = title_dir.join("_info.json");
            dump_info
                .dump(&title_dump_path)
                .expect("Failed to dump title info");

            for chapter in download_chapters {
                console.info(cformat!(
                    "  Downloading chapter <m,s>{}</> ({})...",
                    chapter.as_chapter_title(),
                    chapter.chapter_id()
                ));

                let viewer_resp = client
                    .get_chapter_viewer(chapter, &results, dl_config.quality.clone().into(), true)
                    .await
                    .context("Failed to get viewer info")?;

                let viewer = match viewer_resp {
                    APIResponse::Success(viewer) => viewer,
                    APIResponse::Error(e) => {
                        console.error(format!("Failed to get viewer info: {}", e.as_string()));
                        return Err(color_eyre::eyre::eyre!(
                            "Failed to get viewer info: {}",
                            e.as_string()
                        ));
                    }
                };

                let chapter_images: Vec<tosho_mplus::proto::ChapterPage> = viewer
                    .pages()
                    .iter()
                    .filter_map(|page| page.page().cloned())
                    .collect();

                let image_dir =
                    get_output_directory(&output_dir, title_id, Some(chapter.chapter_id()), false)?;

                if let Some(count) = check_downloaded_image_count(&image_dir, "webp")?
                    && count >= chapter_images.len()
                {
                    console.warn(cformat!(
                        "   Chapter <m,s>{}</> (<s>{}</>) has been downloaded, skipping",
                        chapter.as_chapter_title(),
                        chapter.chapter_id()
                    ));
                    continue;
                }

                // create chapter dir
                std::fs::create_dir_all(&image_dir)?;

                let progress =
                    console.make_progress_arc(chapter_images.len() as u64, Some("Downloading"));

                if dl_config.parallel {
                    let semaphore = Arc::new(tokio::sync::Semaphore::new(dl_config.threads));

                    let tasks: Vec<_> = chapter_images
                        .iter()
                        .enumerate()
                        .map(|(idx, image)| {
                            // wrap function for async block
                            let wrap_client = client.clone();
                            let image_dir = image_dir.clone();
                            let cnsl = console.clone();
                            let image = image.clone();
                            let progress = Arc::clone(&progress);
                            let semaphore = Arc::clone(&semaphore);

                            tokio::spawn(async move {
                                let _permit = semaphore
                                    .acquire()
                                    .await
                                    .expect("Should acquire semaphore lock");

                                match mplus_actual_downloader(
                                    MPDownloadNode {
                                        client: wrap_client,
                                        image,
                                        idx,
                                        extension: "webp".to_string(),
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
                    for (idx, image) in chapter_images.iter().enumerate() {
                        match mplus_actual_downloader(
                            MPDownloadNode {
                                client: client.clone(),
                                image: image.clone(),
                                idx,
                                extension: "webp".to_string(),
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

            Ok(())
        }
        tosho_mplus::APIResponse::Error(e) => {
            console.error(format!("Failed to get title info: {}", e.as_string()));
            Err(color_eyre::eyre::eyre!(
                "Failed to get title info: {}",
                e.as_string()
            ))
        }
    }
}
