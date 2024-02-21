use std::str::FromStr;

use serde::{Deserialize, Serialize};
use tosho_amap::models::{ComicEpisodeInfo, ComicEpisodeInfoNode};
use tosho_kmkc::models::EpisodeNode;
use tosho_musq::proto::ChapterV2;
use tosho_rbean::models::Chapter;
use tosho_sjv::models::MangaChapterDetail;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(untagged)]
pub enum IdDump {
    Number(u64),
    Uuid(String),
}

impl From<u64> for IdDump {
    fn from(value: u64) -> Self {
        Self::Number(value)
    }
}

impl From<String> for IdDump {
    fn from(value: String) -> Self {
        if value.parse::<u64>().is_ok() {
            Self::Number(value.parse().unwrap())
        } else {
            Self::Uuid(value)
        }
    }
}

impl FromStr for IdDump {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.parse::<u64>().is_ok() {
            Ok(Self::Number(s.parse().unwrap()))
        } else {
            Ok(Self::Uuid(s.to_string()))
        }
    }
}

impl ToString for IdDump {
    fn to_string(&self) -> String {
        match self {
            Self::Number(n) => n.to_string(),
            Self::Uuid(s) => s.clone(),
        }
    }
}

/// A dump info of a chapter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterDetailDump {
    /// The chapter ID.
    pub id: IdDump,
    /// The main chapter name.
    pub main_name: String,
    /// The timestamp of the chapter release date.
    timestamp: Option<i64>,
    /// The sub chapter name, if any.
    sub_name: Option<String>,
}

/// A dump info of a manga.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaDetailDump {
    pub(crate) title_name: String,
    author_name: String,
    pub(crate) chapters: Vec<ChapterDetailDump>,
}

impl MangaDetailDump {
    pub fn new(title: String, author: String, chapters: Vec<ChapterDetailDump>) -> Self {
        Self {
            title_name: title,
            author_name: author,
            chapters,
        }
    }

    /// Dump the info into `_info.json` format.
    ///
    /// # Arguments
    /// * `save_path` - The path to save the dump.
    pub fn dump(&self, save_path: &std::path::PathBuf) -> std::io::Result<()> {
        let file = std::fs::File::create(save_path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }
}

impl From<ChapterV2> for ChapterDetailDump {
    /// Convert from [`tosho_musq::proto::ChapterV2`] into [`ChapterDetailDump`]
    /// `_info.json` format.
    fn from(value: ChapterV2) -> Self {
        let pub_at = match value.published_at {
            Some(published) => {
                // assume JST
                let published = chrono::NaiveDate::parse_from_str(&published, "%b %d, %Y")
                    .map(|d| d.and_hms_opt(0, 0, 0).unwrap())
                    .map(|d| d.and_local_timezone(chrono::FixedOffset::east_opt(9 * 3600).unwrap()))
                    .unwrap_or_else(|_| {
                        panic!("Failed to parse published date to JST TZ: {}", published)
                    })
                    .unwrap();

                // to timestamp
                Some(published.timestamp())
            }
            None => None,
        };

        Self {
            id: value.id.into(),
            main_name: value.title,
            timestamp: pub_at,
            sub_name: value.subtitle,
        }
    }
}

impl From<EpisodeNode> for ChapterDetailDump {
    /// Convert from [`tosho_kmkc::models::EpisodeNode`] into [`ChapterDetailDump`]
    /// `_info.json` format.
    fn from(value: EpisodeNode) -> Self {
        let start_time_ts = value.start_time.timestamp();

        Self {
            main_name: value.title,
            id: (value.id as u64).into(),
            timestamp: Some(start_time_ts),
            sub_name: None,
        }
    }
}

impl From<ComicEpisodeInfoNode> for ChapterDetailDump {
    /// Convert from [`tosho_amap::models::ComicEpisodeInfoNode`] into [`ChapterDetailDump`]
    /// `_info.json` format.
    fn from(value: ComicEpisodeInfoNode) -> Self {
        Self {
            main_name: value.title,
            id: value.id.into(),
            timestamp: Some(value.update_date as i64),
            sub_name: None,
        }
    }
}

impl From<ComicEpisodeInfo> for ChapterDetailDump {
    /// Convert from [`tosho_amap::models::ComicEpisodeInfo`] into [`ChapterDetailDump`]
    /// `_info.json` format.
    fn from(value: ComicEpisodeInfo) -> Self {
        Self::from(value.info)
    }
}

impl From<MangaChapterDetail> for ChapterDetailDump {
    /// Convert from [`tosho_sjv::models::MangaChapterDetail`] into [`ChapterDetailDump`]
    /// `_info.json` format.
    fn from(value: MangaChapterDetail) -> Self {
        Self {
            main_name: value.pretty_title(),
            id: (value.id as u64).into(),
            timestamp: value.published_at.map(|d| d.timestamp()),
            sub_name: None,
        }
    }
}

impl From<Chapter> for ChapterDetailDump {
    /// Convert from [`tosho_rbean::models::Chapter`] into [`ChapterDetailDump`]
    /// `_info.json` format.
    fn from(value: Chapter) -> Self {
        Self {
            id: value.uuid.clone().into(),
            main_name: value.formatted_title(),
            timestamp: value.published.map(|d| d.timestamp()),
            sub_name: None,
        }
    }
}

#[derive(Clone, Default, Deserialize, Serialize, Debug)]
pub struct MangaManualMergeChapterDetail {
    pub(crate) name: String,
    pub(crate) chapters: Vec<IdDump>,
}

#[derive(Clone, Default, Deserialize, Serialize, Debug)]
pub struct MangaManualMergeDetail {
    pub(crate) title: String,
    pub(crate) chapters: Vec<MangaManualMergeChapterDetail>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_deser_chapter_uuid() {
        let json = r#"{
            "id": "uuid",
            "mainName": "Chapter 1",
            "timestamp": 1620000000,
            "subName": "Sub Chapter"
        }"#;

        let chapter: super::ChapterDetailDump = serde_json::from_str(json).unwrap();

        assert_eq!(chapter.id, super::IdDump::Uuid("uuid".to_string()));
        assert_eq!(chapter.main_name, "Chapter 1");
        assert_eq!(chapter.timestamp, Some(1620000000));
        assert_eq!(chapter.sub_name, Some("Sub Chapter".to_string()));
    }

    #[test]
    fn test_deser_chapter_number() {
        let json = r#"{
            "id": 1,
            "mainName": "Chapter 1",
            "timestamp": 1620000000,
            "subName": "Sub Chapter"
        }"#;

        let chapter: super::ChapterDetailDump = serde_json::from_str(json).unwrap();

        assert_eq!(chapter.id, super::IdDump::Number(1));
        assert_eq!(chapter.main_name, "Chapter 1");
        assert_eq!(chapter.timestamp, Some(1620000000));
        assert_eq!(chapter.sub_name, Some("Sub Chapter".to_string()));
    }
}
