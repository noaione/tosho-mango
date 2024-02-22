use std::path::{Path, PathBuf};
use std::sync::Arc;

use color_print::cformat;
use tosho_sjv::{
    models::{AccountSubscription, MangaChapterDetail, MangaDetail, SubscriptionType},
    SJClient, SJPlatform,
};

use crate::{
    cli::ExitCode,
    r#impl::{
        models::{ChapterDetailDump, MangaDetailDump},
        parser::NumberOrString,
    },
    term::ConsoleChoice,
};

use super::common::get_cached_store_data;

#[derive(Clone, Debug, Default)]
pub(crate) struct SJDownloadCliConfig {
    /// Disable all input prompt (a.k.a `autodownload`)
    pub(crate) no_input: bool,

    pub(crate) chapter_ids: Vec<usize>,
    /// The start chapter range.
    ///
    /// Used only when `no_input` is `true`.
    pub(crate) start_from: Option<u32>,
    /// The end chapter range.
    ///
    /// Used only when `no_input` is `true`.
    pub(crate) end_at: Option<u32>,
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

fn create_chapters_info(title: &MangaDetail, chapters: Vec<MangaChapterDetail>) -> MangaDetailDump {
    let mut dumped_chapters: Vec<ChapterDetailDump> = vec![];
    for chapter in chapters {
        dumped_chapters.push(ChapterDetailDump::from(chapter));
    }

    MangaDetailDump::new(
        title.title.clone(),
        title.author.clone().unwrap_or("Unknown Author".to_string()),
        dumped_chapters,
    )
}

fn get_output_directory(
    output_dir: &Path,
    title_id: u32,
    chapter_id: Option<u32>,
    create_folder: bool,
) -> PathBuf {
    let mut pathing = output_dir.to_path_buf();
    pathing.push(format!("SJV_{}", title_id));

    if let Some(chapter_id) = chapter_id {
        pathing.push(chapter_id.to_string());
    }

    if create_folder {
        std::fs::create_dir_all(&pathing).unwrap();
    }

    pathing
}

fn do_chapter_select(
    chapters_entry: Vec<MangaChapterDetail>,
    result: &MangaDetail,
    subs_info: &AccountSubscription,
    console: &mut crate::term::Terminal,
) -> Vec<MangaChapterDetail> {
    console.info("Title information:");
    console.info(&cformat!("  - <bold>ID:</> {}", result.id));
    console.info(&cformat!("  - <bold>Title:</> {}", result.title));
    console.info(&cformat!(
        "  - <bold>Chapters:</> {} chapters",
        chapters_entry.len()
    ));

    let has_subs = match result.subscription_type {
        None => false,
        Some(subs) => match subs {
            SubscriptionType::SJ => subs_info.is_sj_active(),
            SubscriptionType::VM => subs_info.is_vm_active(),
        },
    };

    let select_choices: Vec<ConsoleChoice> = chapters_entry
        .iter()
        .filter(|&ch| {
            // Hide future chapters because we're not time traveler
            if let Some(pub_at) = ch.published_at {
                pub_at.timestamp() <= chrono::Utc::now().timestamp()
            } else {
                true
            }
        })
        .filter_map(|ch| {
            if ch.is_available() || has_subs {
                Some(ConsoleChoice {
                    name: ch.id.to_string(),
                    value: ch.pretty_title(),
                })
            } else {
                None
            }
        })
        .collect();

    let selected_chapters = console.select("Select chapter to download", select_choices);
    match selected_chapters {
        Some(selected) => {
            let selected_chapters: Vec<MangaChapterDetail> = selected
                .iter()
                .map(|x| {
                    let ch_id = x.name.parse::<u32>().unwrap();
                    let ch = chapters_entry
                        .iter()
                        .find(|ch| ch.id == ch_id)
                        .unwrap()
                        .clone();

                    ch
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

pub(crate) async fn sjv_download(
    title_or_slug: NumberOrString,
    dl_config: SJDownloadCliConfig,
    output_dir: PathBuf,
    client: &SJClient,
    console: &mut crate::term::Terminal,
) -> ExitCode {
    if let (Some(start), Some(end)) = (dl_config.start_from, dl_config.end_at) {
        if start > end {
            console.error("Start chapter is greater than end chapter!");
            return 1;
        }
    }

    console.info(&cformat!(
        "Fetching info for <magenta,bold>{}</>...",
        title_or_slug
    ));

    let results = get_cached_store_data(client).await;

    if let Err(e) = results {
        console.error(&format!("Failed to fetch cached store: {}", e));
        return 1;
    }

    let results = results.unwrap();
    let title = results.series.iter().find(|x| {
        if let NumberOrString::Number(n) = title_or_slug {
            x.id == n as u32
        } else {
            x.slug == title_or_slug.to_string()
        }
    });
    if title.is_none() {
        console.warn("No match found");
        return 1;
    }

    console.info("Fetching subscription info...");
    let subs_resp = client.get_entitlements().await;
    if let Err(e) = subs_resp {
        console.error(&format!("Failed to fetch subscription info: {}", e));
        return 1;
    }

    let subs_resp = subs_resp.unwrap();

    let title = title.unwrap();
    console.info(&cformat!(
        "Fetching chapters for <magenta,bold>{}</>...",
        title.title
    ));

    let chapters_resp = client.get_chapters(title.id).await;

    match chapters_resp {
        Ok(chapters) => {
            let chapters: Vec<MangaChapterDetail> = chapters
                .iter()
                .filter_map(|ch| {
                    if ch.chapter.is_some() {
                        Some(ch.clone())
                    } else {
                        None
                    }
                })
                .collect();

            if chapters.is_empty() {
                console.warn("No chapters found");
                return 1;
            }

            let select_chapters = if dl_config.no_input {
                chapters.clone()
            } else {
                do_chapter_select(chapters.clone(), title, &subs_resp.subscriptions, console)
            };

            let has_subs = match title.subscription_type {
                None => false,
                Some(subs) => match subs {
                    SubscriptionType::SJ => subs_resp.subscriptions.is_sj_active(),
                    SubscriptionType::VM => subs_resp.subscriptions.is_vm_active(),
                },
            };

            let mut download_chapters: Vec<&MangaChapterDetail> = select_chapters
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
                .filter(|&ch| ch.is_available() || has_subs)
                .filter(|&ch| {
                    // Hide future chapters because we're not time traveler
                    if let Some(pub_at) = ch.published_at {
                        pub_at.timestamp() <= chrono::Utc::now().timestamp()
                    } else {
                        true
                    }
                })
                .collect();

            if download_chapters.is_empty() {
                console.warn("No chapters after filtered by selected chapter ids");
                return 1;
            }

            download_chapters.sort_by(|&a, &b| a.id.cmp(&b.id));

            let title_dir = get_output_directory(&output_dir, title.id, None, true);
            let dump_info = create_chapters_info(title, chapters);

            let title_dump_path = title_dir.join("_info.json");
            dump_info
                .dump(&title_dump_path)
                .expect("Failed to dump title info");

            for chapter in download_chapters {
                console.info(&cformat!(
                    "  Downloading chapter <m,s>{}</> ({})...",
                    chapter.pretty_title(),
                    chapter.id
                ));

                let image_dir =
                    get_output_directory(&output_dir, title.id, Some(chapter.id), false);
                let image_ext = match client.get_platform() {
                    SJPlatform::Web => "png",
                    _ => "jpg",
                };

                if let Some(count) = check_downloaded_image_count(&image_dir, image_ext) {
                    if count >= chapter.pages as usize {
                        console.warn(&cformat!(
                            "   Chapter <m,s>{}</> (<s>{}</>) has been downloaded, skipping",
                            chapter.pretty_title(),
                            chapter.id
                        ));
                        continue;
                    }
                }

                let view_req = client.verify_chapter(chapter.id).await;
                if let Err(e) = view_req {
                    console.error(&format!("Failed to verify chapter: {}", e));
                    continue;
                }

                let ch_metadata = client.get_chapter_metadata(chapter.id).await;
                if let Err(e) = ch_metadata {
                    console.error(&format!("Failed to fetch chapter metadata: {}", e));
                    continue;
                }

                // create chapter dir
                std::fs::create_dir_all(&image_dir).unwrap();

                let total_image_count = chapter.pages as u64;

                let progress = Arc::new(indicatif::ProgressBar::new(total_image_count));
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

                let tasks: Vec<_> = (0..chapter.pages)
                    .map(|page| {
                        // wrap function in async block
                        let wrap_client = client.clone();
                        let image_dir = image_dir.clone();
                        let cnsl = console.clone();
                        let progress = Arc::clone(&progress);
                        let chapter_id = chapter.id.clone();
                        tokio::spawn(async move {
                            let download_url = wrap_client
                                .get_manga_url(chapter_id, false, Some(page))
                                .await
                                .unwrap();

                            let image_fn = format!("p{:03}.{}", page, image_ext);
                            let img_dl_path = image_dir.join(&image_fn);

                            let writer = tokio::fs::File::create(&img_dl_path)
                                .await
                                .expect("Failed to create image file!");

                            if cnsl.is_debug() {
                                cnsl.log(&cformat!(
                                    "   Downloading image <s>{}</> to <s>{}</>...",
                                    download_url,
                                    image_fn
                                ));
                            }

                            match wrap_client.stream_download(&download_url, writer).await {
                                Ok(_) => {}
                                Err(err) => {
                                    cnsl.error(&format!("    Failed to download image: {}", err));
                                    // silent delete the file
                                    tokio::fs::remove_file(&img_dl_path)
                                        .await
                                        .unwrap_or_default();
                                }
                            }

                            progress.inc(1);
                        })
                    })
                    .collect();

                futures::future::join_all(tasks).await;
                progress.finish_with_message("Downloaded");
            }

            0
        }
        Err(e) => {
            console.error(&format!("Failed to fetch chapters: {}", e));
            1
        }
    }
}
