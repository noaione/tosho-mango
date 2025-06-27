use crate::{cli::ExitCode, config::get_user_path};
use color_print::cformat;
use std::path::{Path, PathBuf};

fn glob_cache(prefix: &str, base_path: &Path) -> Vec<PathBuf> {
    let glob_path = base_path.join(format!("{prefix}*"));

    let mut matched_files = vec![];
    for entry in glob::glob(glob_path.to_str().unwrap())
        .expect("Failed to read glob pattern")
        .flatten()
    {
        if entry.is_file() {
            matched_files.push(entry);
        }
    }

    matched_files
}

pub(crate) async fn tools_clear_cache(console: &mut crate::term::Terminal) -> ExitCode {
    let base_path = get_user_path();

    let sjv_caches = glob_cache("sjv_store_cache_", &base_path);
    let mplus_caches = glob_cache("mplus_titles_", &base_path);

    // if both empty, return immediately!
    if sjv_caches.is_empty() && mplus_caches.is_empty() {
        console.warn("No cache files found!");
        return 1;
    }

    console.info(cformat!(
        "Found <magenta,bold>{}</> cache files to delete:",
        sjv_caches.len() + mplus_caches.len()
    ));

    console.info(cformat!(" SJ/M: <bold>{}</bold> files", sjv_caches.len()));
    console.info(cformat!(" M+: <bold>{}</bold> files", mplus_caches.len()));

    let continue_it = console.confirm(Some("Are you sure you want to delete?"));

    if !continue_it {
        console.warn("Aborted!");
    } else {
        println!();
        for entry in sjv_caches {
            let file_name = entry.file_name().unwrap();
            match tokio::fs::remove_file(entry.clone()).await {
                Ok(_) => console.info(cformat!("Deleted: <bold>{:?}</>", file_name)),
                Err(e) => console.error(cformat!(
                    "Failed to delete: <bold>{:?}</>\n  <red,bold>{}</>",
                    file_name,
                    e
                )),
            }
        }
        for entry in mplus_caches {
            let file_name = entry.file_name().unwrap();
            match tokio::fs::remove_file(entry.clone()).await {
                Ok(_) => console.info(cformat!("Deleted: <bold>{:?}</>", file_name)),
                Err(e) => console.error(cformat!(
                    "Failed to delete: <bold>{:?}</>\n  <red,bold>{}</>",
                    file_name,
                    e
                )),
            }
        }
    }

    0
}
