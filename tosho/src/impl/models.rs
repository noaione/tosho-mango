use serde::{Deserialize, Serialize};
use tosho_kmkc::models::EpisodeNode;
use tosho_musq::proto::ChapterV2;

/// A dump info of a chapter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterDetailDump {
    /// The chapter ID.
    id: u64,
    /// The main chapter name.
    main_name: String,
    /// The timestamp of the chapter release date.
    timestamp: Option<i64>,
    /// The sub chapter name, if any.
    sub_name: Option<String>,
}

/// A dump info of a manga.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaDetailDump {
    title_name: String,
    author_name: String,
    chapters: Vec<ChapterDetailDump>,
}

impl MangaDetailDump {
    pub fn new(title: String, author: String, chapters: Vec<ChapterDetailDump>) -> Self {
        Self {
            title_name: title,
            author_name: author,
            chapters,
        }
    }
}

impl From<ChapterV2> for ChapterDetailDump {
    /// Convert from [`tosho_musq::proto::ChapterV2`] into [`ChapterDetailDump`]
    /// `_info.json` format.
    fn from(value: ChapterV2) -> Self {
        let pub_at = match value.published_at {
            Some(published) => {
                // assume JST
                let published = chrono::NaiveDateTime::parse_from_str(&published, "%b %d, %Y")
                    .expect("Failed to parse published date");
                let published_jst = published
                    .and_local_timezone(chrono::FixedOffset::east_opt(9 * 3600).unwrap())
                    .unwrap();

                // to timestmap
                Some(published_jst.timestamp())
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
