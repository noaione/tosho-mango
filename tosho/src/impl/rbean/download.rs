use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use clap::ValueEnum;
use color_print::cformat;
use tosho_macros::EnumName;
use tosho_rbean::{
    models::{Chapter, ChapterPage, Manga, UserAccount},
    RBClient,
};

use crate::{
    cli::ExitCode,
    r#impl::{
        clean_filename,
        models::{ChapterDetailDump, MangaDetailDump},
    },
    term::{ConsoleChoice, Terminal},
};

use super::{common::save_session_config, config::Config};

#[derive(Debug, Clone, Copy, EnumName, Default)]
pub(crate) enum CLIDownloadFormat {
    #[default]
    Jpeg,
    Webp,
}

impl ValueEnum for CLIDownloadFormat {
    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        let input = if ignore_case {
            input.to_lowercase()
        } else {
            input.to_string()
        };
        match input.as_str() {
            "jpeg" | "jpg" => Ok(CLIDownloadFormat::Jpeg),
            "webp" => Ok(CLIDownloadFormat::Webp),
            _ => Err(format!("Invalid download format option: {}", input)),
        }
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            CLIDownloadFormat::Jpeg => Some(clap::builder::PossibleValue::new("jpeg")),
            CLIDownloadFormat::Webp => Some(clap::builder::PossibleValue::new("webp")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[CLIDownloadFormat::Jpeg, CLIDownloadFormat::Webp]
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct RBDownloadConfigCli {
    /// Disable all input prompt (a.k.a `autodownload`)
    pub(crate) no_input: bool,

    /// The UUID of the title to download.
    pub(crate) chapter_ids: Vec<String>,

    /// The format to download the images in.
    pub(crate) format: CLIDownloadFormat,

    /// Parallel download
    pub(crate) parallel: bool,
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

fn create_chapters_info(title: &Manga, chapters: Vec<Chapter>) -> MangaDetailDump {
    let mut dumped_chapters: Vec<ChapterDetailDump> = vec![];
    for chapter in chapters {
        dumped_chapters.push(ChapterDetailDump::from(chapter));
    }

    let creators = title
        .creators
        .iter()
        .map(|cc| cc.name.clone())
        .collect::<Vec<String>>();

    MangaDetailDump::new(title.title.clone(), creators.join(", "), dumped_chapters)
}

fn get_output_directory(
    output_dir: &Path,
    title_id: String,
    chapter_id: Option<String>,
    create_folder: bool,
) -> PathBuf {
    let mut pathing = output_dir.to_path_buf();
    pathing.push(format!("RB_{}", title_id));

    if let Some(chapter_id) = chapter_id {
        pathing.push(clean_filename(&chapter_id));
    }

    if create_folder {
        std::fs::create_dir_all(&pathing).unwrap();
    }

    pathing
}

fn do_chapter_select(
    chapters_entry: Vec<&Chapter>,
    result: &Manga,
    user_info: &UserAccount,
    console: &mut crate::term::Terminal,
) -> Vec<Chapter> {
    console.info("Title information:");
    console.info(&cformat!("  - <bold>ID:</> {}", result.uuid));
    console.info(&cformat!("  - <bold>Title:</> {}", result.title));
    console.info(&cformat!(
        "  - <bold>Chapters:</> {} chapters",
        chapters_entry.len()
    ));

    let select_choices: Vec<ConsoleChoice> = chapters_entry
        .iter()
        .filter(|&&ch| {
            // Hide unavailable chapters
            ch.published.is_some()
        })
        .filter_map(|&ch| {
            // Download chapter if it's free or user is premium
            if ch.free_published.is_some() || user_info.is_premium {
                Some(ConsoleChoice {
                    name: ch.uuid.to_string(),
                    value: ch.formatted_title(),
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
                .filter_map(|x| {
                    let ch = chapters_entry
                        .iter()
                        .find(|&&ch| ch.uuid == x.name)
                        .cloned();

                    ch.cloned()
                })
                .collect();

            if selected_chapters.is_empty() {
                console.warn("No chapters selected, aborting...");
            }

            selected_chapters
        }
        None => {
            console.warn("Aborted!");
            vec![]
        }
    }
}

struct DownloadNode {
    client: RBClient,
    page: ChapterPage,
    idx: usize,
    extension: String,
}

async fn rbean_actual_downloader(
    node: DownloadNode,
    image_dir: PathBuf,
    dl_config: RBDownloadConfigCli,
    console: Terminal,
    progress: Arc<indicatif::ProgressBar>,
) -> anyhow::Result<()> {
    let image_fn = format!("p{:03}.{}", node.idx, node.extension);
    let img_dl_path = image_dir.join(image_fn.clone());

    let mut img_source = match dl_config.format {
        CLIDownloadFormat::Jpeg => node.page.image.jpg.clone(),
        CLIDownloadFormat::Webp => node.page.image.webp.clone(),
    };

    img_source.sort();
    img_source.reverse();

    let download_url = img_source.first().unwrap();

    let writer = tokio::fs::File::create(&img_dl_path).await?;

    if console.is_debug() {
        console.log(&cformat!(
            "   Downloading image <s>{}</> to <s>{}</>...",
            download_url.url,
            image_fn
        ));
    }

    match node.client.stream_download(&download_url.url, writer).await {
        Ok(_) => {}
        Err(err) => {
            console.error(&format!("    Failed to download image: {}", err));
            // silent delete the file
            tokio::fs::remove_file(&img_dl_path).await?;
        }
    }

    progress.inc(1);

    Ok(())
}

pub(crate) async fn rbean_download(
    uuid: &str,
    dl_config: RBDownloadConfigCli,
    output_dir: PathBuf,
    client: &mut RBClient,
    account: &Config,
    console: &mut crate::term::Terminal,
) -> ExitCode {
    console.info(&cformat!(
        "Fetching user information for <magenta,bold>{}</>...",
        account.email
    ));

    let acc_info = client.get_user().await;
    if let Err(e) = acc_info {
        console.error(&format!("Failed to fetch user information: {}", e));
        return 1;
    }

    let acc_info = acc_info.unwrap();
    save_session_config(client, account);

    console.info(&cformat!(
        "Fetching info for ID <magenta,bold>{}</>...",
        uuid
    ));

    let result = client.get_manga(uuid).await;

    if let Err(e) = result {
        console.error(&format!("Failed to fetch manga: {}", e));
        return 1;
    }

    let result = result.unwrap();
    save_session_config(client, account);

    console.info(&cformat!(
        "Fetching chapters for <magenta,bold>{}</>...",
        result.title
    ));

    let chapter_meta = client.get_chapter_list(uuid).await;

    if let Err(e) = chapter_meta {
        console.error(&format!("Failed to fetch chapters: {}", e));
        return 1;
    }

    let chapter_meta = chapter_meta.unwrap();
    save_session_config(client, account);

    let chapters: Vec<&Chapter> = chapter_meta
        .chapters
        .iter()
        .filter(|&ch| ch.published.is_some())
        .collect();

    if chapters.is_empty() {
        console.error("No chapters available to download!");
        return 1;
    }

    let selected_chapters: Vec<Chapter> = if dl_config.no_input {
        chapters.iter().map(|&x| x.clone()).collect()
    } else {
        do_chapter_select(chapters, &result, &acc_info, console)
    };

    let download_chapters: Vec<&Chapter> = selected_chapters
        .iter()
        .filter(|&ch| dl_config.chapter_ids.is_empty() || dl_config.chapter_ids.contains(&ch.uuid))
        .filter(|&ch| ch.published.is_some())
        .filter(|&ch| {
            // Download chapter if it's free or user is premium
            ch.free_published.is_some() || acc_info.is_premium
        })
        .collect();

    if download_chapters.is_empty() {
        console.warn("No chapters after filtered by selected chapter ids");
        return 1;
    }

    let title_dir = get_output_directory(&output_dir, result.uuid.clone(), None, true);
    let dump_info = create_chapters_info(&result, chapter_meta.chapters);

    let title_dump_path = title_dir.join("_info.json");
    dump_info
        .dump(&title_dump_path)
        .expect("Failed to dump title info");

    for chapter in download_chapters {
        console.info(&cformat!(
            "  Downloading chapter <m,s>{}</> ({})...",
            chapter.formatted_title(),
            chapter.uuid
        ));

        let image_dir = get_output_directory(
            &output_dir,
            result.uuid.clone(),
            Some(chapter.formatted_title()),
            false,
        );

        let image_ext = match dl_config.format {
            CLIDownloadFormat::Jpeg => "jpg",
            CLIDownloadFormat::Webp => "webp",
        };

        let view_req = client.get_chapter_viewer(&chapter.uuid).await;

        if let Err(e) = view_req {
            console.error(&cformat!(
                "Failed to fetch viewer for <m,s>{}</>: {}",
                chapter.formatted_title(),
                e
            ));
            continue;
        }

        let view_req = view_req.unwrap();
        save_session_config(client, account);

        if let Some(count) = check_downloaded_image_count(&image_dir, image_ext) {
            if count >= view_req.data.pages.len() {
                console.warn(&cformat!(
                    "   Chapter <m,s>{}</> (<s>{}</>) has been downloaded, skipping",
                    chapter.formatted_title(),
                    chapter.uuid
                ));
                continue;
            }
        }

        // create chapter dir
        std::fs::create_dir_all(&image_dir).unwrap();

        let total_img_count = view_req.data.pages.len() as u64;

        let progress = Arc::new(indicatif::ProgressBar::new(total_img_count));
        progress.enable_steady_tick(std::time::Duration::from_millis(120));
        progress.set_style(
            indicatif::ProgressStyle::with_template(
                "{spinner:.blue} {msg} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len}",
            )
            .unwrap()
            .progress_chars("#>-")
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏", " "]),
        );
        progress.set_message("Downloading");

        let pages_data = view_req.data.pages.clone();

        if dl_config.parallel {
            let tasks: Vec<_> = pages_data
                .iter()
                .enumerate()
                .map(|(idx, page)| {
                    // wrap function in async block
                    let page = page.clone();
                    let wrap_client = client.clone();
                    let image_dir = image_dir.clone();
                    let cnsl = console.clone();
                    let dl_config = dl_config.clone();
                    let progress = Arc::clone(&progress);
                    tokio::spawn(async move {
                        match rbean_actual_downloader(
                            DownloadNode {
                                client: wrap_client,
                                page,
                                idx,
                                extension: image_ext.to_string(),
                            },
                            image_dir,
                            dl_config,
                            cnsl.clone(),
                            progress,
                        )
                        .await
                        {
                            Ok(_) => {}
                            Err(e) => {
                                cnsl.error(&format!("Failed to download image: {}", e));
                            }
                        }
                    })
                })
                .collect();

            futures::future::join_all(tasks).await;
        } else {
            for (idx, page) in pages_data.iter().enumerate() {
                let node = DownloadNode {
                    client: client.clone(),
                    page: page.clone(),
                    idx,
                    extension: image_ext.to_string(),
                };

                match rbean_actual_downloader(
                    node,
                    image_dir.clone(),
                    dl_config.clone(),
                    console.clone(),
                    Arc::clone(&progress),
                )
                .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        console.error(&format!("Failed to download image: {}", e));
                    }
                }
            }
        }
        progress.finish_with_message("Downloaded");
    }

    0
}
