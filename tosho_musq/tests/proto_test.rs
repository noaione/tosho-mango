use std::{fs::File, io::Read, path::PathBuf, str::FromStr};

use prost::Message;
use tosho_musq::proto::{
    ChapterViewer, ChapterViewerV2, HomeViewV2, MangaDetail, MangaDetailV2, MyPageView,
    PointHistoryView, PointShopView, Status,
};

fn common_reader(file_name: &str) -> Result<String, std::io::Error> {
    let manifest_dir = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).unwrap();
    let root_dir = manifest_dir.parent().unwrap();

    let assets_dir = root_dir.join("tosho_assets");

    if !assets_dir.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found, skipping test",
        ));
    }

    if !assets_dir.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found, skipping test",
        ));
    }

    let file_path = assets_dir.join("musq").join(file_name);

    if !file_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found, skipping test",
        ));
    }

    let mut data_file = File::open(file_path).unwrap();
    // Check if file exists, if not skip the test (lfs not fetched or something)
    if data_file.metadata().is_err() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found, skipping test",
        ));
    }

    let mut buf = vec![];
    data_file
        .read_to_end(&mut buf)
        .expect("Failed to read file");

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

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut chars = hex.chars();
    while let (Some(a), Some(b)) = (chars.next(), chars.next()) {
        bytes.push(u8::from_str_radix(&format!("{}{}", a, b), 16).unwrap());
    }
    bytes
}

#[test]
fn test_proto_chapter_view() {
    let proto_data = common_reader("chapterview.tmfxture");

    match proto_data {
        Err(err) => {
            assert!(true, "{}", err);
        }
        Ok(proto_hex) => {
            let proto_bytes = hex_to_bytes(&proto_hex);

            let result = ChapterViewer::decode(proto_bytes.as_slice()).unwrap();
            assert_eq!(result.status(), Status::Success);
            let user_point = result.user_point.clone().unwrap();
            assert_eq!(user_point.free, 0);
            assert_eq!(user_point.event, 0);
            assert_eq!(user_point.paid, 480);
            assert_eq!(user_point.sum(), 480);

            assert!(result.previous_chapter.is_none());
            assert!(result.next_chapter.is_some());
            assert_eq!(result.images.len(), 11);

            let image = result.images.get(0).unwrap();
            assert_eq!(image.file_name(), "1.avif");
            assert_eq!(image.file_stem(), "1");
            assert_eq!(image.extension(), "avif");

            let mut image2 = result.images.get(1).unwrap().clone();
            image2.url = "/data/1/noextension".to_owned();
            assert_eq!(image2.file_name(), "noextension");
            assert_eq!(image2.file_stem(), "noextension");
            assert_eq!(image2.extension(), "");
        }
    }
}

#[test]
fn test_proto_chapter_viewv2() {
    let proto_data = common_reader("chapterview_v2.tmfxture");

    match proto_data {
        Err(err) => {
            assert!(true, "{}", err);
        }
        Ok(proto_hex) => {
            let proto_bytes = hex_to_bytes(&proto_hex);

            let result = ChapterViewerV2::decode(proto_bytes.as_slice()).unwrap();
            assert_eq!(result.status(), Status::Success);
            let user_point = result.user_point.clone().unwrap();
            assert_eq!(user_point.free, 40);
            assert_eq!(user_point.event, 0);
            assert_eq!(user_point.paid, 370);
            assert_eq!(user_point.sum(), 410);

            assert_eq!(result.blocks.len(), 1);
            assert!(result.next_chapter.is_some());

            let block = result.blocks.get(0).unwrap();
            assert_eq!(block.title, "Chapter 10.1");
            let image = block.images.get(0).unwrap();
            assert_eq!(image.file_name(), "1.avif");
            assert_eq!(image.file_stem(), "1");
            assert_eq!(image.extension(), "avif");

            let mut image2 = block.images.get(1).unwrap().clone();
            image2.url = "/data/1/noextension".to_owned();
            assert_eq!(image2.file_name(), "noextension");
            assert_eq!(image2.file_stem(), "noextension");
            assert_eq!(image2.extension(), "");
        }
    }
}

#[test]
fn test_proto_coinhistory() {
    let proto_data = common_reader("coinhistory.tmfxture");

    match proto_data {
        Err(err) => {
            assert!(true, "{}", err);
        }
        Ok(proto_hex) => {
            let proto_bytes = hex_to_bytes(&proto_hex);

            let result = PointHistoryView::decode(proto_bytes.as_slice()).unwrap();
            let user_point = result.user_point.clone().unwrap();
            assert_eq!(user_point.free, 0);
            assert_eq!(user_point.event, 0);
            assert_eq!(user_point.paid, 280);
            assert_eq!(user_point.sum(), 280);

            assert!(result.logs.len() > 0);
        }
    }
}

#[test]
fn test_proto_homev2() {
    let proto_data = common_reader("homev2.tmfxture");

    match proto_data {
        Err(err) => {
            assert!(true, "{}", err);
        }
        Ok(proto_hex) => {
            let proto_bytes = hex_to_bytes(&proto_hex);

            let result = HomeViewV2::decode(proto_bytes.as_slice()).unwrap();
            assert!(result.top_banners.len() > 0);
            assert!(result.top_sub_banners.len() > 0);
            assert!(result.tutorial_banner.is_none());
            assert_eq!(result.updated_section_name, "Updates for you");
            assert!(result.updated_titles.len() > 0);
            assert_eq!(result.tags.len(), 8);
            assert!(result.featured.is_some());
            assert_eq!(result.new_section_name, "New Series");
            assert!(result.new_titles.len() > 0);
            assert_eq!(result.ranking_section_name, "Ranking");
            assert_eq!(result.rankings.len(), 4);
            assert_ne!(result.ranking_description, "");
            assert_ne!(result.recommended_banner_image_url, "");
        }
    }
}

#[test]
fn test_proto_mangadetail() {
    let proto_data = common_reader("mangadetail.tmfxture");

    match proto_data {
        Err(err) => {
            assert!(true, "{}", err);
        }
        Ok(proto_hex) => {
            let proto_bytes = hex_to_bytes(&proto_hex);

            let result = MangaDetail::decode(proto_bytes.as_slice()).unwrap();
            assert_eq!(result.status(), Status::Success);

            assert_eq!(
                result.title,
                "The Diary of a Middle-Aged Teacher's Carefree Life in Another World"
            );
            assert_eq!(
                result.authors,
                "Kotobuki Yasukiyo, Maneki, Ryu Nishin, Johndee"
            );
            assert!(!result.copyright.is_empty());

            assert!(result.next_update.is_none());
            assert!(result.warning.is_none());
            assert!(!result.description.is_empty());
            assert!(!result.display_description);
            assert_eq!(result.tags.len(), 1);
            assert!(result.video_url.is_none());

            assert!(result.chapters.len() > 0);

            let first_ch = result.chapters.last().unwrap();
            let mut last_ch = result.chapters.first().unwrap().clone();
            assert_eq!(first_ch.title, "Chapter 1.1");
            assert_eq!(last_ch.title, "Chapter 44.2");
            assert!(first_ch.is_free());
            assert!(!last_ch.is_free());
            last_ch.subtitle = Some("Test".to_owned());
            assert_eq!(first_ch.as_chapter_title(), "Chapter 1.1");
            assert_eq!(last_ch.as_chapter_title(), "Chapter 44.2 — Test");
        }
    }
}

#[test]
fn test_proto_mangadetailv2() {
    let proto_data = common_reader("mangadetail_v2.tmfxture");

    match proto_data {
        Err(err) => {
            assert!(true, "{}", err);
        }
        Ok(proto_hex) => {
            let proto_bytes = hex_to_bytes(&proto_hex);

            let result = MangaDetailV2::decode(proto_bytes.as_slice()).unwrap();
            assert_eq!(result.status(), Status::Success);

            assert_eq!(result.title, "The Angel Next Door Spoils Me Rotten");
            assert_eq!(result.authors, "Saekisan, Hanekoto, Wan Shibata, Suzu Yuki");
            assert!(!result.copyright.is_empty());

            assert!(result.next_update.is_none());
            assert!(result.warning.is_none());
            assert!(!result.description.is_empty());
            assert!(!result.display_description);
            assert_eq!(result.tags.len(), 3);
            assert!(result.video_url.is_none());

            assert!(result.chapters.len() > 0);

            let first_ch = result.chapters.last().unwrap();
            let mut last_ch = result.chapters.first().unwrap().clone();
            assert_eq!(first_ch.title, "Chapter 1.1");
            assert_eq!(last_ch.title, "Chapter 10.3");
            assert!(first_ch.is_free());
            assert!(last_ch.is_free());
            last_ch.subtitle = Some("Test".to_owned());
            assert_eq!(first_ch.as_chapter_title(), "Chapter 1.1");
            assert_eq!(last_ch.as_chapter_title(), "Chapter 10.3 — Test");

            assert!(result.hidden_chapters.is_none());
        }
    }
}

#[test]
fn test_proto_mypage() {
    let proto_data = common_reader("mypage");

    match proto_data {
        Err(err) => {
            assert!(true, "{}", err);
        }
        Ok(proto_hex) => {
            let proto_bytes = hex_to_bytes(&proto_hex);

            let result = MyPageView::decode(proto_bytes.as_slice()).unwrap();
            assert!(result.favorites.len() > 0);
            assert!(result.history.len() > 0);
        }
    }
}

#[test]
fn test_proto_pointshopview() {
    let proto_data = common_reader("pointshop.tmfxture");

    match proto_data {
        Err(err) => {
            assert!(true, "{}", err);
        }
        Ok(proto_hex) => {
            let proto_bytes = hex_to_bytes(&proto_hex);

            let result = PointShopView::decode(proto_bytes.as_slice()).unwrap();
            let user_point = result.user_point.clone().unwrap();
            assert_eq!(user_point.free, 0);
            assert_eq!(user_point.event, 0);
            assert_eq!(user_point.paid, 480);

            let point_limit = result.point_limit.clone().unwrap();
            assert_eq!(point_limit.free, 40);
            assert_eq!(point_limit.event, 100000);
            assert_eq!(point_limit.paid, 100000);

            assert_eq!(result.next_recovery, 1674604800);
            assert_eq!(result.subscriptions.len(), 0);
            assert_eq!(result.billings.len(), 9);
            assert_eq!(result.default_select, 0);

            let billing = result.billings.get(8).unwrap();
            assert_eq!(billing.event_point, 8040);
            assert_eq!(billing.paid_point, 26800);
            assert_eq!(billing.total_point(), 34840);
        }
    }
}

#[test]
fn test_common_reader() {
    let proto_data = common_reader("chapterview.tmfxture");

    match proto_data {
        Err(err) => {
            assert!(true, "{}", err);
        }
        Ok(_) => {
            assert!(true)
        }
    }
}

#[test]
fn test_hex_to_bytes() {
    // encoded string: hello
    let hex_str = "68656c6c6f";

    let bytes = hex_to_bytes(hex_str);
    assert_eq!(bytes, vec![104, 101, 108, 108, 111]);
}
