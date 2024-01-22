use serde::{Deserialize, Serialize};
use tosho_amap::models::{ComicEpisodeInfo, ComicEpisodeInfoNode};
use tosho_kmkc::models::EpisodeNode;
use tosho_musq::proto::ChapterV2;

/// A dump info of a chapter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterDetailDump {
    /// The chapter ID.
    pub id: u64,
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
            id: value.id,
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
            id: value.id as u64,
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
            id: value.id,
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

#[derive(Clone, Default, Deserialize, Serialize, Debug)]
pub struct MangaManualMergeChapterDetail {
    pub(crate) name: String,
    pub(crate) chapters: Vec<u64>,
}

#[derive(Clone, Default, Deserialize, Serialize, Debug)]
pub struct MangaManualMergeDetail {
    pub(crate) title: String,
    pub(crate) chapters: Vec<MangaManualMergeChapterDetail>,
}
