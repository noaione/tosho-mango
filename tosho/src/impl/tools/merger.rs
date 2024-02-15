use crate::{
    cli::ExitCode,
    r#impl::models::{
        ChapterDetailDump, MangaDetailDump, MangaManualMergeChapterDetail, MangaManualMergeDetail,
    },
    term::ConsoleChoice,
};
use color_print::cformat;
use inquire::{required, validator::StringValidator, Text};
use std::{
    collections::{BTreeMap, HashMap},
    path::{Path, PathBuf},
};

lazy_static::lazy_static! {
    static ref TITLE_REGEX: regex::Regex = regex::Regex::new(
        r"(?:[\w\.]+ |#|[\w]+)(?P<base>0?[\d]+)?(?:[\(-\. ][\(-\. ]?)?(?P<split>[\d]+)?(?:[\)])?"
    ).unwrap();
    static ref MODERN_IMAGE_EXT: [&'static str; 5] = ["avif", "jxl", "webp", "heif", "heic"];
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ToolsMergeConfig {
    pub(crate) skip_last: bool,
    pub(crate) no_input: bool,
    /// Ignore _info_manual_merge.json file which
    /// contains all the chapter that are merged manually
    /// to filter out the chapters that are already merged.
    pub(crate) ignore_manual_info: bool,
}

/// Simulate a Regex match object.
#[derive(Clone, Default)]
struct PseudoMatch {
    contents: HashMap<String, String>,
}

impl PseudoMatch {
    fn get(&self, name: &str) -> Option<String> {
        self.contents.get(name).cloned()
    }

    fn set(&mut self, name: &str, value: &str) {
        self.contents.insert(name.to_string(), value.to_string());
    }
}

fn safe_int(input: &str) -> Option<u64> {
    input.parse::<u64>().ok()
}

fn safe_float(input: &str) -> Option<f64> {
    input.parse::<f64>().ok()
}

#[derive(Clone)]
struct NumberValidation;

#[derive(Clone)]
struct IntValidation;

impl StringValidator for NumberValidation {
    fn validate(
        &self,
        input: &str,
    ) -> Result<inquire::validator::Validation, inquire::CustomUserError> {
        if input.contains('.') {
            // try f64
            if safe_float(input).is_some() {
                return Ok(inquire::validator::Validation::Valid);
            }
        }

        // try u64
        if safe_int(input).is_some() {
            return Ok(inquire::validator::Validation::Valid);
        }

        Ok(inquire::validator::Validation::Invalid(
            "Invalid number (not a float or int)".into(),
        ))
    }
}

impl StringValidator for IntValidation {
    fn validate(
        &self,
        input: &str,
    ) -> Result<inquire::validator::Validation, inquire::CustomUserError> {
        // try u64
        if safe_int(input).is_some() {
            return Ok(inquire::validator::Validation::Valid);
        }

        Ok(inquire::validator::Validation::Invalid(
            "Invalid number (not a float or int)".into(),
        ))
    }
}

fn inquire_chapter_number(
    chapter: &ChapterDetailDump,
    last_known_num: u64,
    console: &mut crate::term::Terminal,
) -> Option<PseudoMatch> {
    console.warn(&format!(
        "  Failed to parse chapter title: {}",
        chapter.main_name
    ));

    let ch_number = Text::new("Chapter number")
        .with_help_message("")
        .with_placeholder(&format!("Last known: Chapter {}", last_known_num))
        .with_validator(required!())
        .with_validator(NumberValidation)
        .prompt();

    let ch_number = match ch_number {
        Ok(ch_number) => ch_number,
        Err(err) => match err {
            inquire::error::InquireError::OperationCanceled => {
                console.warn("Aborted.");
                return None;
            }
            inquire::InquireError::OperationInterrupted => {
                console.warn("Aborted.");
                return None;
            }
            _ => {
                console.error("  Failed to get chapter number.");
                return None;
            }
        },
    };

    let mut matching = PseudoMatch::default();
    if ch_number.contains('.') {
        let (base, floaty) = ch_number.split_once('.').unwrap();
        matching.set("base", base);
        matching.set("floaty", floaty);
    } else {
        matching.set("base", ch_number.as_str());
    }

    Some(matching)
}

pub fn auto_chapters_collector(
    mut chapters_dump: Vec<ChapterDetailDump>,
    console: &mut crate::term::Terminal,
) -> BTreeMap<String, Vec<ChapterDetailDump>> {
    chapters_dump.sort_by(|a, b| a.id.cmp(&b.id));

    if chapters_dump.is_empty() {
        console.error("  Empty chapters collection, aborting...");
        return BTreeMap::new();
    }

    let mut last_known_num = 0;
    let mut extra = 0;
    let mut chapters_mapping: BTreeMap<String, Vec<ChapterDetailDump>> = BTreeMap::new();
    for chapter in chapters_dump {
        let matching = TITLE_REGEX.captures(&chapter.main_name);

        let (mut base, _) = match matching {
            Some(matching) => {
                let base = matching.name("base").map_or("", |m| m.as_str());
                let split = matching.name("split").map_or("", |m| m.as_str());

                match (base, split) {
                    ("", "") => {
                        let temp = inquire_chapter_number(&chapter, last_known_num, console);
                        if let Some(temp) = temp {
                            let base_t = temp.get("base").unwrap_or_default();
                            let split_t = temp.get("split").unwrap_or_default();
                            (base_t, split_t)
                        } else {
                            continue;
                        }
                    }
                    _ => (base.to_string(), split.to_string()),
                }
            }
            None => {
                let test_match = inquire_chapter_number(&chapter, last_known_num, console);

                if let Some(test_match) = test_match {
                    let base = test_match.get("base").unwrap_or_default();
                    let split = test_match.get("split").unwrap_or_default();

                    (base, split)
                } else {
                    continue;
                }
            }
        };

        if base.is_empty() {
            last_known_num += 1;
            base = last_known_num.to_string();
        }

        let mut base = base.trim().parse::<u64>().unwrap();
        let mut use_extra = false;
        if last_known_num > base {
            console.warn(&format!(
                "  Chapter {base} is lower than last known chapter {last_known_num}, assuming extra chapter",
            ));
            base = last_known_num;
            use_extra = true;
        } else {
            last_known_num = base;
        }

        if use_extra {
            // name: ex{base:03}.{extra}
            let name = format!("ex{:03}.{}", base, extra);
            extra += 1;
            chapters_mapping.entry(name).or_default().push(chapter);
        } else {
            // name: c{base:03}
            let name = format!("c{:03}", base);
            chapters_mapping.entry(name).or_default().push(chapter);
        }
    }
    chapters_mapping
}

pub fn manual_chapters_collector(
    mut chapters_dump: Vec<ChapterDetailDump>,
    console: &mut crate::term::Terminal,
) -> BTreeMap<String, Vec<ChapterDetailDump>> {
    chapters_dump.sort_by(|a, b| a.id.cmp(&b.id));

    if chapters_dump.is_empty() {
        console.error("  Empty chapters collection, aborting...");
        return BTreeMap::new();
    }

    let mut chapters_mapping: BTreeMap<String, Vec<ChapterDetailDump>> = BTreeMap::new();
    let mut selected_chapters: Vec<u64> = vec![];
    // map chapter id to chapter
    let mut chapter_id_map: HashMap<u64, &ChapterDetailDump> = HashMap::new();
    for chapter in chapters_dump.iter() {
        chapter_id_map.insert(chapter.id, chapter);
    }

    let mut first_warn = true;
    let mut abort_after = false;
    loop {
        let ch_number = Text::new("Chapter number")
            .with_validator(required!())
            .with_validator(IntValidation)
            .prompt();

        let ch_number = match ch_number {
            Ok(ch_number) => ch_number,
            Err(err) => match err {
                inquire::error::InquireError::OperationCanceled => {
                    console.warn("Aborted.");
                    break;
                }
                inquire::InquireError::OperationInterrupted => {
                    console.warn("Aborted.");
                    break;
                }
                _ => {
                    console.error("  Failed to get chapter number.");
                    continue;
                }
            },
        };

        let ch_number = match ch_number.parse::<u64>() {
            Ok(ch_number) => ch_number,
            Err(_) => {
                console.error("  Failed to parse chapter number.");
                continue;
            }
        };

        let merge_choices: Vec<ConsoleChoice> = chapters_dump
            .clone()
            .iter()
            .filter_map(|ch| {
                if selected_chapters.contains(&ch.id) {
                    None
                } else {
                    Some(ConsoleChoice {
                        name: ch.id.to_string(),
                        value: format!("{} ({})", ch.main_name, ch.id),
                    })
                }
            })
            .collect();

        if first_warn {
            console.info(&cformat!(
                "Please make sure you <c!,s>select</> the chapters in <c!,s>correct order!</>"
            ));
            first_warn = false;
        }

        let merge_choice = console.select("Select chapter you want to merge", merge_choices);
        match merge_choice {
            Some(merge_choice) => {
                let chapter_ids = merge_choice
                    .iter()
                    .map(|choice| choice.name.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                if chapter_ids.is_empty() {
                    console.warn("  No chapter selected, skipping...");
                    continue;
                }
                selected_chapters.extend(chapter_ids.clone());
                let name = format!("c{:03}", ch_number);
                let remapped = chapter_ids
                    .iter()
                    .map(|id| chapter_id_map.remove(id).unwrap().clone())
                    .collect::<Vec<ChapterDetailDump>>();
                chapters_mapping.entry(name).or_default().extend(remapped);
            }
            None => {
                console.warn("Aborted.");
                abort_after = true;
                break;
            }
        }

        let is_continue = console.confirm(Some("Do you want to add more?"));
        if !is_continue {
            break;
        }
    }

    if abort_after {
        return BTreeMap::new();
    }

    chapters_mapping
}

fn is_all_folder_exist(base_dir: PathBuf, chapters: &[ChapterDetailDump]) -> bool {
    for chapter in chapters {
        let tp = base_dir.join(&chapter.id.to_string());
        if !tp.exists() {
            return false;
        }
    }
    true
}

async fn get_last_page(target_dir: PathBuf) -> u64 {
    let mut last_page = 0;
    let mut read_dirs = tokio::fs::read_dir(target_dir).await.unwrap();

    loop {
        let file = read_dirs.next_entry().await.unwrap();
        let file = match file {
            Some(file) => file,
            None => break,
        };

        let path = file.path();
        let file_stem = path.file_stem().unwrap().to_str().unwrap();
        if path.is_file() && file_stem.starts_with('p') {
            let file_stem = file_stem.split_once('p').unwrap().1;
            let file_stem = file_stem.parse::<u64>().unwrap() + 1;
            if file_stem > last_page {
                last_page = file_stem;
            }
        }
    }

    last_page
}

fn guess_from_ext(path: &Path) -> Option<String> {
    let suffix = path.extension();
    if let Some(suffix) = suffix {
        let suffix = suffix.to_str().unwrap();
        if MODERN_IMAGE_EXT.contains(&suffix) {
            return Some(suffix.to_string());
        }
    }
    None
}

fn is_image(path: &PathBuf) -> bool {
    let guess = mime_guess::from_path(path);

    match guess.first() {
        Some(mima) => {
            if mima.type_() == mime_guess::mime::IMAGE {
                true
            } else {
                guess_from_ext(path).is_some()
            }
        }
        None => guess_from_ext(path).is_some(),
    }
}

async fn read_manual_info_json(input_folder: &Path) -> MangaManualMergeDetail {
    let info_json = input_folder.join("_info_manual_merge.json");

    if !info_json.exists() {
        return MangaManualMergeDetail::default();
    }

    let info_json = match tokio::fs::read_to_string(info_json).await {
        Ok(info_json) => info_json,
        Err(_) => {
            return MangaManualMergeDetail::default();
        }
    };

    let info_json: MangaManualMergeDetail = match serde_json::from_str(&info_json) {
        Ok(info_json) => info_json,
        Err(_) => MangaManualMergeDetail::default(),
    };

    info_json
}

pub(crate) async fn tools_split_merge(
    input_folder: &Path,
    config: ToolsMergeConfig,
    console: &mut crate::term::Terminal,
) -> ExitCode {
    let info_json = input_folder.join("_info.json");
    console.info(&format!("Reading _info.json file: {}", info_json.display()));

    if !info_json.exists() {
        console.error("The _info.json file is not found in the input folder.");
        return 1;
    }

    let info_json = match tokio::fs::read_to_string(info_json).await {
        Ok(info_json) => info_json,
        Err(err) => {
            console.error(&format!("Failed to read _info.json file: {}", err));
            return 1;
        }
    };

    let info_json: MangaDetailDump = match serde_json::from_str(&info_json) {
        Ok(info_json) => info_json,
        Err(err) => {
            console.error(&format!("Failed to parse _info.json file: {}", err));
            return 1;
        }
    };

    console.info(&format!(
        "Loaded {} chapters from _info.json, collecting...",
        info_json.chapters.len()
    ));

    let mut manual_info_merge = read_manual_info_json(input_folder).await;

    let mut chapters_maps = if config.no_input {
        auto_chapters_collector(info_json.chapters.clone(), console)
    } else {
        let mut current_chapters = info_json.chapters.clone();
        if !config.ignore_manual_info {
            // get all chapters that are not in manual info
            let mut manual_chapters = vec![];
            for chapter in manual_info_merge.chapters.iter() {
                manual_chapters.extend(chapter.chapters.clone());
            }
            current_chapters.retain(|ch| !manual_chapters.contains(&ch.id));
        }
        manual_chapters_collector(current_chapters, console)
    };

    if chapters_maps.is_empty() {
        console.warn("No chapters collected, aborting...");
        return 1;
    }

    console.info(&format!(
        "Collected {} chapters",
        chapters_maps.keys().len()
    ));
    for (name, chapters) in chapters_maps.iter() {
        let ch_ids = chapters
            .iter()
            .map(|ch| ch.id.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        let mut info_log = format!("  {}: has {} chapters", name, chapters.len());
        if console.is_debug() {
            info_log.push_str(&format!(" ({})", ch_ids));
        }
        console.info(&info_log);
        for chapter in chapters {
            console.log(&format!("   - {}", chapter.main_name));
        }
    }

    if config.no_input {
        let total_values = chapters_maps
            .values()
            .map(|chapters| chapters.len())
            .sum::<usize>();
        let is_continue = console.confirm(Some(&format!(
            "Do you want to continue with {} chapters?",
            total_values
        )));
        if !is_continue {
            console.warn("Aborting...");
            return 1;
        }
    }

    if config.skip_last {
        console.warn("Skipping last chapter merge...");
        // pop the last chapter
        let last_key = chapters_maps.keys().last().unwrap().clone();
        chapters_maps.remove(&last_key).unwrap();
    }

    console.info("Starting merge...");
    for (name, chapters) in chapters_maps.iter() {
        console.info(&format!("  Merging {}...", name));

        if !is_all_folder_exist(input_folder.to_path_buf(), chapters) {
            console.warn(&format!(
                "   Not all folders exist for {}, skipping...",
                name
            ));
            continue;
        }

        let target_dir = input_folder.join(name);
        // create
        match tokio::fs::create_dir_all(&target_dir).await {
            Ok(_) => {}
            Err(err) => {
                // check if folder already exist, if yes ignore
                if err.kind() == tokio::io::ErrorKind::AlreadyExists {
                    // ignore
                } else {
                    console.error(&format!("   Failed to create target directory: {}", err));
                    continue;
                }
            }
        }

        let mut last_page = get_last_page(target_dir.clone()).await;
        let mut write_to_json = false;
        for chapter in chapters {
            let source_dir = input_folder.join(&chapter.id.to_string());
            if !source_dir.exists() {
                console.warn(&format!(
                    "   Source directory for chapter {} does not exist, skipping...",
                    chapter.id
                ));
                continue;
            }

            // iterate through the source directory
            let mut read_dirs = match tokio::fs::read_dir(source_dir).await {
                Ok(read_dirs) => read_dirs,
                Err(err) => {
                    console.error(&format!("   Failed to read source directory: {}", err));
                    continue;
                }
            };

            loop {
                let file = match read_dirs.next_entry().await {
                    Ok(file) => file,
                    Err(err) => {
                        console.error(&format!("   Failed to read source directory: {}", err));
                        break;
                    }
                };

                let file = match file {
                    Some(file) => file,
                    None => break,
                };

                let path = &file.path();
                if path.is_file() && is_image(path) {
                    // move the file from "file" to target_dir / p{last_page}.{ext}
                    let file_name = format!("p{:03}", last_page);
                    let file_ext = path.extension().unwrap().to_str().unwrap();
                    let file_name = format!("{}.{}", file_name, file_ext);

                    let target_path = target_dir.join(file_name);
                    match tokio::fs::rename(path, target_path).await {
                        Ok(_) => {
                            write_to_json = true;
                        }
                        Err(err) => {
                            console.error(&format!(
                                "   Failed to move {}: {}",
                                path.display(),
                                err,
                            ));
                        }
                    }
                    last_page += 1;
                }
            }
        }

        console.info(&format!("   Merged {} with {} pages", name, last_page));

        if !config.no_input && write_to_json {
            // manual mode, update the manual info
            manual_info_merge
                .chapters
                .push(MangaManualMergeChapterDetail {
                    name: name.clone(),
                    chapters: chapters.iter().map(|ch| ch.id).collect::<Vec<u64>>(),
                })
        }
    }

    manual_info_merge.title = info_json.title_name;

    if !config.no_input {
        // write the manual info
        let manual_json_content = serde_json::to_string_pretty(&manual_info_merge).unwrap();
        let manual_json_path = input_folder.join("_info_manual_merge.json");
        match tokio::fs::write(manual_json_path, manual_json_content).await {
            Ok(_) => {}
            Err(err) => {
                console.error(&format!("Failed to write _info_manual_merge.json: {}", err));
            }
        }
    }

    0
}
