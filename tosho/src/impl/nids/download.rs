use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use clap::ValueEnum;
use color_print::cformat;
use num_format::{Locale, ToFormattedString};
use tokio::time::Instant;
use tosho_nids::NIClient;

use crate::{cli::ExitCode, r#impl::nids::common::timedelta_to_humantime};

#[derive(Debug, Clone, Default)]
pub(crate) enum DownloadImageQuality {
    /// Desktop quality (full resolution)
    #[default]
    Desktop,
    /// Mobile quality (smaller)
    Mobile,
}

impl ValueEnum for DownloadImageQuality {
    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        let input = if ignore_case {
            input.to_lowercase()
        } else {
            input.to_string()
        };
        match input.as_str() {
            "desktop" | "high" => Ok(Self::Desktop),
            "mobile" | "normal" => Ok(Self::Mobile),
            _ => Err(format!("Invalid image quality: {input}")),
        }
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::Desktop => Some(clap::builder::PossibleValue::new("desktop").alias("high")),
            Self::Mobile => Some(clap::builder::PossibleValue::new("mobile").alias("normal")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Desktop, Self::Mobile]
    }
}

fn get_output_directory(
    output_dir: &Path,
    title_id: u32,
    issue_id: Option<u32>,
    create_folder: bool,
) -> PathBuf {
    let mut pathing = output_dir.to_path_buf();
    pathing.push(format!("NI_{}", title_id));

    if let Some(chapter_id) = issue_id {
        pathing.push(chapter_id.to_string());
    }

    if create_folder {
        std::fs::create_dir_all(&pathing).unwrap();
    }

    pathing
}

struct DownloadNode {
    client: NIClient,
    page_url: String,
    page_name: String,
}

fn extract_extensions_from_url(url: &str) -> Option<String> {
    let parsed_url = reqwest::Url::parse(url).ok()?;
    let mut path_segments = parsed_url.path_segments()?;
    let last_segment = path_segments.next_back()?;
    let last_segment_path = PathBuf::from(last_segment);
    let extension = last_segment_path.extension()?.to_str()?;
    Some(extension.to_string())
}

async fn nids_actual_downloader(
    node: DownloadNode,
    image_dir: PathBuf,
    console: &crate::term::Terminal,
    progress: Arc<indicatif::ProgressBar>,
) -> anyhow::Result<()> {
    // determine image extension from page_url
    let extension = match extract_extensions_from_url(&node.page_url) {
        Some(ext) => ext,
        None => {
            anyhow::bail!("Failed to determine image extension from URL");
        }
    };

    let image_fn = format!("{}.{}", node.page_name, extension);
    let img_dl_path = image_dir.join(&image_fn);

    let writer = tokio::fs::File::create(&img_dl_path).await?;

    if console.is_debug() {
        console.log(cformat!(
            "   Downloading image <s>{}</> to <s>{}</>...",
            &node.page_url,
            image_fn
        ));
    }

    match node.client.stream_download(&node.page_url, writer).await {
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

#[derive(serde::Serialize)]
struct FrameWithPage {
    frame: tosho_nids::models::reader::ReaderFrame,
    filename: String,
}

fn dump_reading_frames(
    output_dir: &Path,
    images: &[tosho_nids::models::reader::ReaderPage],
) -> anyhow::Result<()> {
    let mut frames_with_page = Vec::new();
    let mut has_any = false;
    for (idx, page) in images.iter().enumerate() {
        for frame in page.frames().iter() {
            has_any = true;
            let extension = match extract_extensions_from_url(page.image().url()) {
                Some(ext) => ext,
                None => {
                    anyhow::bail!("Failed to determine image extension from URL");
                }
            };
            frames_with_page.push(FrameWithPage {
                frame: frame.clone(),
                filename: format!("i_{:04}.{}", idx + 1, extension),
            });
        }
    }

    if has_any {
        let output_file = output_dir.join("frames.json");
        match serde_json::to_string_pretty(&frames_with_page) {
            Ok(json_str) => {
                if let Err(err) = std::fs::write(&output_file, json_str) {
                    anyhow::bail!("Failed to write frames.json: {}", err);
                }
            }
            Err(err) => {
                anyhow::bail!("Failed to serialize frames.json: {}", err);
            }
        }
    }

    Ok(())
}

#[derive(Clone, Debug)]
pub(crate) struct NIDownloadCliConfig {
    /// Quality of images to download
    pub(crate) quality: DownloadImageQuality,
    /// Parallel download
    pub(crate) parallel: bool,
    /// Number of threads to use for parallel download
    pub(crate) threads: usize,
    /// Override output directory
    pub(crate) output: Option<PathBuf>,
    /// Do not report page viewing progress to the server
    pub(crate) no_report: bool,
}

impl Default for NIDownloadCliConfig {
    fn default() -> Self {
        Self {
            parallel: false,
            threads: 4,
            output: None,
            no_report: false,
            quality: DownloadImageQuality::Desktop,
        }
    }
}

async fn nids_report_progress(
    issue_uuid: &str,
    pages: u32,
    client: &NIClient,
) -> anyhow::Result<()> {
    // Report progress to the server
    client
        .report_page_view(issue_uuid, pages)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to report progress: {}", e))?;

    Ok(())
}

pub(crate) async fn nids_download(
    issue_id: u32,
    dl_config: NIDownloadCliConfig,
    output_dir: PathBuf,
    client: &NIClient,
    console: &mut crate::term::Terminal,
) -> ExitCode {
    console.info(cformat!(
        "Fetching info for <magenta,bold>{}</>...",
        issue_id
    ));

    let results = match client.get_issue(issue_id).await {
        Ok(res) => res,
        Err(err) => {
            console.error(format!("Failed to fetch issue info: {err}"));
            return 1;
        }
    };

    console.info(cformat!(
        "Fetching reader for issue <m,s>{}</m,s> (<m,s>{}</m,s>)",
        results.full_title(),
        results.id()
    ));

    let pages_meta = match client.get_issue_reader(issue_id).await {
        Ok(pages) => pages,
        Err(err) => {
            console.error(format!("Failed to fetch issue reader info: {err}"));
            return 1;
        }
    };

    let total_pages = pages_meta.total_pages().to_formatted_string(&Locale::en);
    console.info(cformat!(
        "Downloading issue <m,s>{}</m,s> with <m,s>{}</m,s> pages...",
        results.full_title(),
        total_pages
    ));

    // Download cover first
    let default_dir = get_output_directory(
        &output_dir,
        results.series_run().id(),
        Some(results.id()),
        false,
    );
    let output_dir = dl_config.output.clone().unwrap_or(default_dir);

    if !output_dir.exists()
        && let Err(err) = std::fs::create_dir_all(&output_dir)
    {
        console.error(format!(
            "Failed to create output directory <s>{}</s>: {}",
            output_dir.display(),
            err
        ));
        return 1;
    }

    console.info(cformat!(
        "  Using output directory <s>{}</s>",
        output_dir.display()
    ));

    // Try doing dumping reading frames
    if let Err(err) = dump_reading_frames(&output_dir, pages_meta.pages().pages()) {
        console.error(format!("   Failed to dump reading frames: {}", err));
    }

    // Try downloading cover
    let cover_url = match dl_config.quality {
        DownloadImageQuality::Desktop => pages_meta.pages().cover().url(),
        DownloadImageQuality::Mobile => pages_meta.pages().cover().mobile_url(),
    };
    let cover_ext = extract_extensions_from_url(cover_url).unwrap_or("webp".to_string());
    let cover_path = output_dir.join(format!("cover.{}", cover_ext));
    let cover_file = match tokio::fs::File::create(&cover_path).await {
        Ok(f) => f,
        Err(err) => {
            console.error(format!(
                "   Failed to create cover file <s>{}</s>: {}",
                cover_path.display(),
                err
            ));
            return 1;
        }
    };

    if console.is_debug() {
        console.log(cformat!(
            "   Downloading cover <s>{}</> to <s>{}</>...",
            cover_url,
            cover_path.display()
        ));
    }
    match client.stream_download(cover_url, cover_file).await {
        Ok(_) => {}
        Err(err) => {
            console.error(format!("    Failed to download cover: {err}"));
            // silent delete the file
            let _ = tokio::fs::remove_file(&cover_path).await;
        }
    }

    let current_time = chrono::Local::now();
    let progress = console.make_progress_arc(pages_meta.total_pages(), Some("Downloading"));
    if dl_config.parallel {
        let semaphore = Arc::new(tokio::sync::Semaphore::new(dl_config.threads));

        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<usize>();

        // reporter tasks
        let throttle_duration = std::time::Duration::from_millis(1500); // every 1.5s we report
        let report_client = client.clone();
        let issue_uuid = results.uuid().to_string();
        tokio::spawn(async move {
            let mut rx = rx;
            let mut last_run = Instant::now() - throttle_duration;

            while let Some(idx) = rx.recv().await {
                if last_run.elapsed() >= throttle_duration {
                    let pg_num = (idx + 1) as u32;
                    if !dl_config.no_report {
                        // report progress
                        let _ = nids_report_progress(&issue_uuid, pg_num, &report_client).await;
                    }
                    last_run = Instant::now();
                }
            }
        });

        let tasks: Vec<_> = pages_meta
            .pages()
            .pages()
            .iter()
            .enumerate()
            .map(|(idx, page)| {
                let wrap_client = client.clone();
                let image_dir = output_dir.clone();
                let cnsl = console.clone();
                let progress = Arc::clone(&progress);
                let semaphore = Arc::clone(&semaphore);
                let page_url = match dl_config.quality {
                    DownloadImageQuality::Desktop => page.image().url().to_string(),
                    DownloadImageQuality::Mobile => page.image().mobile_url().to_string(),
                };

                let tx = tx.clone();

                tokio::spawn(async move {
                    let _permit = semaphore.acquire().await.unwrap();

                    let node = DownloadNode {
                        client: wrap_client,
                        page_url,
                        page_name: format!("i_{:04}", idx + 1),
                    };

                    if let Err(err) = nids_actual_downloader(node, image_dir, &cnsl, progress).await
                    {
                        cnsl.error(format!("    Failed to download page {}: {}", idx + 1, err));
                    }

                    let _ = tx.send(idx);
                })
            })
            .collect();

        drop(tx); // Close the sender

        futures_util::future::join_all(tasks).await;
    } else {
        for (idx, page) in pages_meta.pages().pages().iter().enumerate() {
            let node = DownloadNode {
                client: client.clone(),
                page_url: page.image().url().to_string(),
                page_name: format!("i_{:04}", idx + 1),
            };

            match nids_actual_downloader(node, output_dir.clone(), console, Arc::clone(&progress))
                .await
            {
                Ok(_) => {
                    // report
                    let pg_num = (idx + 1) as u32;
                    if !dl_config.no_report {
                        let _ = nids_report_progress(results.uuid(), pg_num, client).await;
                    }
                }
                Err(err) => {
                    console.error(format!("    Failed to download page {}: {}", idx + 1, err))
                }
            }
        }
    }

    let end_time = chrono::Local::now();

    let duration = end_time - current_time;
    progress.finish_with_message("Downloaded");
    console.info(cformat!(
        "Downloaded <m,s>{}</m,s> in <m,s>{}</m,s>",
        results.full_title(),
        timedelta_to_humantime(duration)
    ));

    0
}
