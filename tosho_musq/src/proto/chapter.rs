//! A module containing information related to chapter.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

#![allow(clippy::derive_partial_eq_without_eq)]

use tosho_common::ToshoResult;
use tosho_macros::AutoGetter;

use super::enums::{Badge, ConsumptionType, Status};

/// Represents a single chapter.
///
/// The following is ``v1`` implementation of the chapter that used by the API.
///
/// See also: [``ChapterV2``]
#[derive(Clone, PartialEq, AutoGetter, ::prost::Message)]
pub struct Chapter {
    /// The chapter ID.
    #[prost(uint64, tag = "1")]
    id: u64,
    /// The chapter title.
    #[prost(string, tag = "2")]
    title: ::prost::alloc::string::String,
    /// The chapter subtitle, usually the actual chapter title.
    #[prost(string, optional, tag = "3")]
    #[skip_field]
    subtitle: ::core::option::Option<::prost::alloc::string::String>,
    /// The chapter thumbnail URL.
    #[prost(string, tag = "4")]
    thumbnail_url: ::prost::alloc::string::String,
    /// The chapter consumption type.
    #[prost(enumeration = "ConsumptionType", tag = "5")]
    #[skip_field]
    consumption: i32,
    /// The chapter price in coins, check with [``Self::consumption``] to see which type of coins
    /// can be used to read this chapter.
    #[prost(uint64, tag = "6")]
    price: u64,
    /// How much chapter rental period left in seconds.
    ///
    /// If the value is ``0``, the chapter rental period has ended.
    /// If the value is ``None``, the chapter is not yet rented.
    #[prost(uint64, optional, tag = "7")]
    #[skip_field]
    end_of_rental_period: ::core::option::Option<u64>,
    /// How many comments this chapter has.
    #[prost(uint64, optional, tag = "8")]
    #[skip_field]
    comments: ::core::option::Option<u64>,
    /// When this chapter was published.
    #[prost(string, optional, tag = "9")]
    #[skip_field]
    published_at: ::core::option::Option<::prost::alloc::string::String>,
    /// The chapter badge.
    #[prost(enumeration = "Badge", tag = "10")]
    #[skip_field]
    badge: i32,
    /// The first page URL of this chapter.
    #[prost(string, tag = "11")]
    first_page_url: ::prost::alloc::string::String,
}

impl Chapter {
    /// Whether or not this chapter is free.
    pub fn is_free(&self) -> bool {
        self.price == 0
    }

    /// Format the chapter title and subtitle into a single string.
    ///
    /// If the subtitle is [`None`], the title will be returned as is.
    pub fn as_chapter_title(&self) -> String {
        let base_title = self.title.clone();
        if let Some(subtitle) = self.subtitle.clone() {
            format!("{base_title} — {subtitle}")
        } else {
            base_title
        }
    }

    /// Set subtitle field, not recommended to use.
    pub fn set_subtitle(&mut self, subtitle: impl Into<String>) {
        self.subtitle = Some(subtitle.into())
    }
}

/// Represents a single chapter.
///
/// The following is ``v2`` implementation of the chapter that used by the API.
///
/// See also: [``Chapter``]
#[derive(Clone, PartialEq, AutoGetter, ::prost::Message)]
pub struct ChapterV2 {
    /// The chapter ID.
    #[prost(uint64, tag = "1")]
    id: u64,
    /// The chapter title.
    #[prost(string, tag = "2")]
    title: ::prost::alloc::string::String,
    /// The chapter subtitle, usually the actual chapter title.
    #[prost(string, optional, tag = "3")]
    #[skip_field]
    subtitle: ::core::option::Option<::prost::alloc::string::String>,
    /// The chapter thumbnail URL.
    #[prost(string, tag = "4")]
    thumbnail_url: ::prost::alloc::string::String,
    /// The chapter consumption type.
    #[prost(enumeration = "ConsumptionType", tag = "5")]
    #[skip_field]
    consumption: i32,
    /// The chapter price in coins, check with [``Self::consumption``] to see which type of coins
    /// can be used to read this chapter.
    #[prost(uint64, tag = "6")]
    price: u64,
    /// How much chapter rental period left in seconds.
    ///
    /// If the value is ``0``, the chapter rental period has ended.
    /// If the value is ``None``, the chapter is not yet rented.
    #[prost(uint64, optional, tag = "7")]
    #[skip_field]
    end_of_rental_period: ::core::option::Option<u64>,
    /// How many comments this chapter has.
    #[prost(uint64, optional, tag = "8")]
    #[skip_field]
    comments: ::core::option::Option<u64>,
    /// When this chapter was published.
    #[prost(string, optional, tag = "9")]
    #[skip_field]
    published_at: ::core::option::Option<::prost::alloc::string::String>,
    /// The chapter badge.
    #[prost(enumeration = "Badge", tag = "10")]
    #[skip_field]
    badge: i32,
    /// The first page URL of this chapter.
    #[prost(string, tag = "11")]
    first_page_url: ::prost::alloc::string::String,
    /// Whether this is the final chapter or not.
    #[prost(bool, tag = "12")]
    final_chapter: bool,
    /// How many pages this chapter has.
    #[prost(uint64, tag = "13")]
    page_count: u64,
    /// How many times this chapter has been read.
    #[prost(uint64, tag = "14")]
    read_count: u64,
}

impl ChapterV2 {
    /// Whether or not this chapter is free.
    pub fn is_free(&self) -> bool {
        self.price == 0
    }

    /// Format the chapter title and subtitle into a single string.
    ///
    /// If the subtitle is [`None`], the title will be returned as is.
    pub fn as_chapter_title(&self) -> String {
        let base_title = self.title.clone();
        if let Some(subtitle) = self.subtitle.clone() {
            format!("{base_title} — {subtitle}")
        } else {
            base_title
        }
    }

    /// Set subtitle field, not recommended to use.
    pub fn set_subtitle(&mut self, subtitle: impl Into<String>) {
        self.subtitle = Some(subtitle.into())
    }
}

#[cfg(test)]
impl ChapterV2 {
    /// For testing purposes, set the chapter price.
    pub fn set_price(&mut self, value: u64) {
        self.price = value;
    }
}

impl From<ChapterV2> for Chapter {
    fn from(value: ChapterV2) -> Self {
        Self {
            id: value.id,
            title: value.title,
            subtitle: value.subtitle,
            thumbnail_url: value.thumbnail_url,
            consumption: value.consumption,
            price: value.price,
            end_of_rental_period: value.end_of_rental_period,
            comments: value.comments,
            published_at: value.published_at,
            badge: value.badge,
            first_page_url: value.first_page_url,
        }
    }
}

/// Represents a chapter page.
#[derive(Clone, PartialEq, AutoGetter, ::prost::Message)]
pub struct ChapterPage {
    /// The page URL.
    #[prost(string, tag = "1")]
    url: ::prost::alloc::string::String,
    /// The video HLS URL.
    #[prost(string, optional, tag = "2")]
    #[skip_field]
    video_url: ::core::option::Option<::prost::alloc::string::String>,
    /// The chapter page URL intents.
    #[prost(string, optional, tag = "3")]
    #[skip_field]
    intent_url: ::core::option::Option<::prost::alloc::string::String>,
    /// The extra ID, if any.
    #[prost(uint64, optional, tag = "4")]
    #[skip_field]
    extra_id: ::core::option::Option<u64>,
    /// The encryption key for the image if encrypted.
    #[prost(string, optional, tag = "5")]
    #[skip_field]
    key: ::core::option::Option<::prost::alloc::string::String>,
    /// The initialization vector for the image if encrypted.
    #[prost(string, optional, tag = "6")]
    #[skip_field]
    iv: ::core::option::Option<::prost::alloc::string::String>,
}

impl ChapterPage {
    /// The file name of the image.
    ///
    /// When you have the URL of `/path/to/image.avif`, the filename
    /// would become `image.avif` including the extension.
    pub fn file_name(&self) -> String {
        let url = self.url.clone();
        // split at the last slash
        let split: Vec<&str> = url.rsplitn(2, '/').collect();
        // Remove extra URL parameters
        let file_name: Vec<&str> = split[0].split('?').collect();
        file_name[0].to_string()
    }

    /// Check if the image is encrypted or not.
    pub fn is_encrypted(&self) -> bool {
        self.file_name().ends_with(".enc")
    }

    /// The file extension of the image.
    ///
    /// When you have the URL of `/path/to/image.avif`,
    /// the extension would become `avif`, when there
    /// is no extension it would return an empty string.
    pub fn extension(&self) -> String {
        let file_name = if self.is_encrypted() {
            self.file_name().replace(".enc", "")
        } else {
            self.file_name()
        };

        // split at the last dot
        let split: Vec<&str> = file_name.rsplitn(2, '.').collect();

        if split.len() == 2 {
            split[0].to_string()
        } else {
            "".to_string()
        }
    }

    /// The file stem of the image.
    ///
    /// When you have the URL of `/path/to/image.avif`,
    /// the file stem would become `image`.
    pub fn file_stem(&self) -> String {
        let file_name = if self.is_encrypted() {
            self.file_name().replace(".enc", "")
        } else {
            self.file_name()
        };

        // split at the last dot
        let split: Vec<&str> = file_name.rsplitn(2, '.').collect();

        if split.len() == 2 {
            split[1].to_string()
        } else {
            file_name
        }
    }

    /// Set the URL of the image
    ///
    /// This is mostly used for testing, so it's not recommended to be used.
    pub fn set_url(&mut self, url: impl Into<String>) {
        self.url = url.into();
    }

    /// Internal function to convert the hex data to bytes.
    fn to_hex_data(&self, hex_data: &str, when: &str) -> ToshoResult<Vec<u8>> {
        if hex_data.len() % 2 != 0 {
            tosho_common::bail_on_error!("Invalid {} length, must be even", when)
        }

        let mut key_bytes = Vec::with_capacity(hex_data.len() / 2);
        let characters = hex_data.chars().collect::<Vec<char>>();
        // We loop like this since we want to propagate error.
        for pair in characters.chunks(2) {
            let byte =
                u8::from_str_radix(&format!("{}{}", pair[0], pair[1]), 16).map_err(|_| {
                    tosho_common::make_error!("Invalid hex string in {} for pair {:?}", when, pair)
                })?;

            key_bytes.push(byte);
        }

        Ok(key_bytes)
    }

    /// Convert the key to bytes if possible.
    pub fn key_as_bytes(&self) -> ToshoResult<Option<Vec<u8>>> {
        if let Some(key) = &self.key {
            let key_bytes = self.to_hex_data(key, "key")?;
            Ok(Some(key_bytes))
        } else {
            Ok(None)
        }
    }

    /// Convert the IV to bytes if possible.
    pub fn iv_as_bytes(&self) -> ToshoResult<Option<Vec<u8>>> {
        if let Some(iv) = &self.iv {
            let iv_bytes = self.to_hex_data(iv, "iv")?;
            Ok(Some(iv_bytes))
        } else {
            Ok(None)
        }
    }
}

/// Represents a chapter viewer response.
#[derive(Clone, PartialEq, AutoGetter, ::prost::Message)]
pub struct ChapterViewer {
    /// The status of the request.
    #[prost(enumeration = "Status", tag = "1")]
    #[skip_field]
    status: i32,
    /// The user purse or point.
    #[prost(message, tag = "2")]
    #[copyable]
    user_point: ::core::option::Option<super::UserPoint>,
    /// The chapter images list.
    #[prost(message, repeated, tag = "3")]
    images: ::prost::alloc::vec::Vec<ChapterPage>,
    /// The next chapter, if any.
    #[prost(message, optional, tag = "4")]
    next_chapter: ::core::option::Option<Chapter>,
    /// The previous chapter, if any.
    #[prost(message, optional, tag = "5")]
    previous_chapter: ::core::option::Option<Chapter>,
    /// The chapter page start.
    #[prost(uint64, tag = "6")]
    page_start: u64,
    /// Whether the chapter comment is enabled or not.
    #[prost(bool, tag = "8")]
    is_comment_enabled: bool,
}

/// Represents an SNS/Social Media sharing info.
#[derive(Clone, PartialEq, AutoGetter, ::prost::Message)]
pub struct SNSInfo {
    /// The text body.
    #[prost(string, tag = "1")]
    body: ::prost::alloc::string::String,
    /// The URL/intent url.
    #[prost(string, tag = "2")]
    url: ::prost::alloc::string::String,
}

/// Represents a single page? block
#[derive(Clone, PartialEq, AutoGetter, ::prost::Message)]
pub struct PageBlock {
    /// The chapter ID.
    #[prost(uint64, tag = "1")]
    id: u64,
    /// The chapter title.
    #[prost(string, tag = "2")]
    title: ::prost::alloc::string::String,
    /// The images list for the current block.
    #[prost(message, repeated, tag = "3")]
    images: ::prost::alloc::vec::Vec<ChapterPage>,
    /// Whether this is the last page or not.
    #[prost(bool, tag = "4")]
    last_page: bool,
    /// The chapter page start.
    #[prost(uint64, tag = "5")]
    start_page: u64,
    /// The chapter SNS.
    #[prost(message, tag = "6")]
    sns: ::core::option::Option<SNSInfo>,
    /// The chapter page start.
    #[prost(uint64, tag = "7")]
    page_start: u64,
    /// The chapter page end.
    #[prost(uint64, tag = "8")]
    page_end: u64,
}

/// Represents a chapter viewer response.
///
/// The following is ``v2`` implementation of the chapter viewer response that used by the API.
///
/// See also: [``ChapterViewer``]
#[derive(Clone, PartialEq, AutoGetter, ::prost::Message)]
pub struct ChapterViewerV2 {
    /// The status of the request.
    #[prost(enumeration = "Status", tag = "1")]
    #[skip_field]
    status: i32,
    /// The user purse or point.
    #[prost(message, tag = "2")]
    #[copyable]
    user_point: ::core::option::Option<super::UserPoint>,
    /// The chapter images list.
    #[prost(message, repeated, tag = "3")]
    blocks: ::prost::alloc::vec::Vec<PageBlock>,
    /// The next chapter, if any.
    #[prost(message, optional, tag = "4")]
    next_chapter: ::core::option::Option<ChapterV2>,
    /// Whether the chapter comment is enabled or not.
    #[prost(bool, tag = "5")]
    is_comment_enabled: bool,
    /// Whether the chapter view guide is enabled or not.
    #[prost(bool, tag = "6")]
    enable_guide: bool,
}
