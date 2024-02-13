use std::{fs::File, io::Read};

use base64::{engine::general_purpose, Engine as _};
use tosho_sjv::models::{
    MangaImprint, MangaRating, MangaSeriesResponse, MangaStoreResponse, SubscriptionType,
};

fn common_reader(file_name: &str) -> Result<String, std::io::Error> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut img_file =
        File::open(format!("{}/tests/{}.tmfxture", manifest_dir, file_name)).unwrap();

    // Check if file exists, if not skip the test (lfs not fetched or something)
    if img_file.metadata().is_err() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found, skipping test",
        ));
    }

    let mut buf = vec![];
    img_file.read_to_end(&mut buf).expect("Failed to read file");

    let buf_str = String::from_utf8(buf).unwrap();

    // check if starts witH
    // version https://git-lfs.github.com/spec/v1
    if buf_str.starts_with("version https://git-lfs.github.com/spec/v1") {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "File found but is a LFS file, please fetch all the LFS files",
        ));
    }

    Ok(buf_str.replace(' ', ""))
}

fn decode_b64(b64encoded: &str) -> String {
    String::from_utf8(
        general_purpose::STANDARD
            .decode(b64encoded)
            .expect("Failed to decode base64"),
    )
    .expect("Failed to decode UTF-8")
}

#[test]
fn test_store_cached_models() {
    let json_file = common_reader("store_cached");

    match json_file {
        Err(err) => {
            assert!(true, "{}", err);
        }
        Ok(json_b64) => {
            let json_data = decode_b64(&json_b64);

            let store_cached = serde_json::from_str::<MangaStoreResponse>(&json_data);

            let store_cached = match store_cached {
                Ok(data) => data,
                Err(error) => {
                    let row_line = error.line() - 1;
                    let split_lines = &json_data.split('\n').collect::<Vec<&str>>();
                    let position = error.column();
                    let start_index = position.saturating_sub(25); // Start 5 characters before the error position
                    let end_index = position.saturating_add(25); // End 5 characters after the error position
                    let excerpt = &split_lines[row_line][start_index..end_index];

                    panic!(
                        "Error parsing JSON at line {}, column {}: {}\nExcerpt: '{}'",
                        error.line(),
                        error.column(),
                        error,
                        excerpt
                    );
                }
            };

            let mut has_777 = false;
            let mut has_829_chapter = false;
            for store_data in store_cached.contents {
                match &store_data {
                    tosho_sjv::models::MangaStoreInfo::Chapter(chapter) => {
                        assert!(chapter.pages > 0);

                        if chapter.series_id == 829 {
                            assert_eq!(chapter.series_slug, "aliens-area");
                            assert_eq!(chapter.subscription_type, Some(SubscriptionType::SJ));
                            assert_eq!(chapter.rating, MangaRating::Teen);
                            has_829_chapter = true;
                        }
                    }
                    tosho_sjv::models::MangaStoreInfo::Manga(manga) => {
                        if manga.id == 777 {
                            assert_eq!(manga.title, "Frieren: Beyond Journey’s End");
                            assert_eq!(manga.slug, "frieren-the-journeys-end");
                            assert_eq!(manga.subscription_type, Some(SubscriptionType::VM));
                            assert_eq!(manga.imprint, MangaImprint::Undefined);
                            assert_eq!(manga.rating, MangaRating::TeenPlus);
                            has_777 = true;
                        }
                    }
                    _ => (),
                }
            }

            assert!(has_777, "Cached data does not have any 777 ID");
            assert!(
                has_829_chapter,
                "Cached data does not have any 829 series ID chapter"
            );
        }
    }
}

#[test]
fn test_store_cached_alt_models_loaded() {
    let json_file = common_reader("store_cached_alt");

    match json_file {
        Err(err) => {
            assert!(true, "{}", err);
        }
        Ok(json_b64) => {
            let json_data = decode_b64(&json_b64);

            let store_cached = serde_json::from_str::<MangaStoreResponse>(&json_data);
            let store_cached = match store_cached {
                Ok(data) => data,
                Err(error) => {
                    let row_line = error.line() - 1;
                    let split_lines = &json_data.split('\n').collect::<Vec<&str>>();
                    let position = error.column();
                    let start_index = position.saturating_sub(25); // Start 5 characters before the error position
                    let end_index = position.saturating_add(25); // End 5 characters after the error position
                    let excerpt = &split_lines[row_line][start_index..end_index];

                    panic!(
                        "Error parsing JSON at line {}, column {}: {}\nExcerpt: '{}'",
                        error.line(),
                        error.column(),
                        error,
                        excerpt
                    );
                }
            };
            assert!(store_cached.contents.len() > 0)
        }
    }
}

#[test]
fn test_manga_detail() {
    let json_file = common_reader("manga_detail");

    match json_file {
        Err(err) => {
            assert!(true, "{}", err);
        }
        Ok(json_b64) => {
            let json_data = decode_b64(&json_b64);

            let manga_detail = serde_json::from_str::<MangaSeriesResponse>(&json_data);

            let manga_detail = match manga_detail {
                Ok(data) => data,
                Err(error) => {
                    let row_line = error.line() - 1;
                    let split_lines = &json_data.split('\n').collect::<Vec<&str>>();
                    let position = error.column();
                    let start_index = position.saturating_sub(25); // Start 5 characters before the error position
                    let end_index = position.saturating_add(25); // End 5 characters after the error position
                    let excerpt = &split_lines[row_line][start_index..end_index];

                    panic!(
                        "Error parsing JSON at line {}, column {}: {}\nExcerpt: '{}'",
                        error.line(),
                        error.column(),
                        error,
                        excerpt
                    );
                }
            };

            assert_eq!(manga_detail.notices[0].offset, 88.0);
            let first_ch = &manga_detail.chapters[0].chapter;
            assert_eq!(first_ch.chapter, Some("100.0".to_string()));
        }
    }
}
