use std::path::PathBuf;

use clap::{Subcommand, ValueEnum};

use super::parser::{CommaSeparatedNumber, parse_comma_number};

pub(crate) mod accounts;
pub(super) mod common;
pub(crate) mod config;
pub(crate) mod download;
pub(crate) mod favorites;
pub(crate) mod manga;
pub(crate) mod rankings;

#[derive(Subcommand, Clone)]
pub(crate) enum MPlusCommands {
    /// Authenticate tosho with your M+ account
    Auth {
        /// Session ID
        session_id: String,
        /// Device kind/type to use
        #[arg(short, long, value_enum, default_value = "android")]
        r#type: crate::r#impl::mplus::accounts::DeviceKind,
    },
    /// Get an account information
    Account,
    /// See all the accounts you have authenticated with
    Accounts,
    /// Automatically/batch download a chapter(s) from a title
    #[command(name = "autodownload")]
    AutoDownload {
        /// Title ID to use
        title_id: u64,
        /// Specify the starting chapter ID to download
        #[arg(short = 's', long, default_value = None)]
        start_from: Option<u64>,
        /// Specify the end chapter ID to download
        #[arg(short = 'e', long, default_value = None)]
        end_until: Option<u64>,
        /// Specify the image quality to download
        #[arg(short = 'q', long = "quality", default_value = "high", value_enum)]
        quality: crate::r#impl::mplus::download::DownloadImageQuality,
        /// Output directory to use
        #[arg(short = 'o', long = "output", default_value = None)]
        output: Option<PathBuf>,
    },
    /// Download a chapters from a title
    Download {
        /// Title ID to use
        title_id: u64,
        /// Specify the chapter ID to purchase (ex: 1,2,3,4,5)
        #[arg(short = 'c', long = "chapters", default_value = None, value_parser = parse_comma_number)]
        chapters: Option<CommaSeparatedNumber>,
        /// Show all the chapters available for the title
        #[arg(long = "show-all")]
        show_all: bool,
        /// Specify the image quality to download
        #[arg(short = 'q', long = "quality", default_value = "high", value_enum)]
        quality: crate::r#impl::mplus::download::DownloadImageQuality,
        /// Output directory to use
        #[arg(short = 'o', long = "output", default_value = None)]
        output: Option<PathBuf>,
    },
    /// Get your account favorites list
    Favorites,
    /// Get a title information
    Info {
        /// Title ID to use
        title_id: u64,
        /// Show each chapter detailed information
        #[arg(short = 'c', long = "chapters")]
        show_chapters: bool,
        /// Show related titles
        #[arg(short = 'r', long = "related")]
        show_related: bool,
    },
    /// Get the current title rankings
    Rankings {
        /// Specify the ranking you want to see
        #[arg(short, long, default_value = "hot", value_enum)]
        kind: crate::r#impl::mplus::rankings::RankingKind,
    },
    /// Revoke or delete an account
    Revoke,
    /// Search for a title
    Search {
        /// Query to search for
        query: String,
    },
}

#[derive(Clone, Default)]
pub(crate) enum MPlusLanguage {
    #[default]
    English,
    Spanish,
    French,
    Indonesian,
    BrazilianPortuguese,
    Russian,
    Thai,
    German,
    // Italian,
    Vietnamese,
}

impl ValueEnum for MPlusLanguage {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            MPlusLanguage::English => Some(clap::builder::PossibleValue::new("en")),
            MPlusLanguage::Spanish => Some(clap::builder::PossibleValue::new("es")),
            MPlusLanguage::French => Some(clap::builder::PossibleValue::new("fr")),
            MPlusLanguage::Indonesian => Some(clap::builder::PossibleValue::new("id")),
            MPlusLanguage::BrazilianPortuguese => Some(clap::builder::PossibleValue::new("pt-br")),
            MPlusLanguage::Russian => Some(clap::builder::PossibleValue::new("ru")),
            MPlusLanguage::Thai => Some(clap::builder::PossibleValue::new("th")),
            MPlusLanguage::German => Some(clap::builder::PossibleValue::new("de")),
            // MPlusLanguage::Italian => Some(clap::builder::PossibleValue::new("it")),
            MPlusLanguage::Vietnamese => Some(clap::builder::PossibleValue::new("vi")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[
            MPlusLanguage::English,
            MPlusLanguage::Spanish,
            MPlusLanguage::French,
            MPlusLanguage::Indonesian,
            MPlusLanguage::BrazilianPortuguese,
            MPlusLanguage::Russian,
            MPlusLanguage::Thai,
            MPlusLanguage::German,
            // MPlusLanguage::Italian,
            MPlusLanguage::Vietnamese,
        ]
    }

    fn from_str(s: &str, ignore_case: bool) -> Result<Self, String> {
        let s = if ignore_case {
            s.to_lowercase()
        } else {
            s.to_string()
        };
        match s.as_str() {
            "en" => Ok(MPlusLanguage::English),
            "es" => Ok(MPlusLanguage::Spanish),
            "fr" => Ok(MPlusLanguage::French),
            "id" => Ok(MPlusLanguage::Indonesian),
            "pt-br" => Ok(MPlusLanguage::BrazilianPortuguese),
            "ru" => Ok(MPlusLanguage::Russian),
            "th" => Ok(MPlusLanguage::Thai),
            "de" => Ok(MPlusLanguage::German),
            // "it" => Ok(MPlusLanguage::Italian),
            "vi" => Ok(MPlusLanguage::Vietnamese),
            _ => Err(format!("Invalid language: {s}")),
        }
    }
}

impl From<MPlusLanguage> for tosho_mplus::proto::Language {
    fn from(lang: MPlusLanguage) -> Self {
        match lang {
            MPlusLanguage::English => tosho_mplus::proto::Language::English,
            MPlusLanguage::Spanish => tosho_mplus::proto::Language::Spanish,
            MPlusLanguage::French => tosho_mplus::proto::Language::French,
            MPlusLanguage::Indonesian => tosho_mplus::proto::Language::Indonesian,
            MPlusLanguage::BrazilianPortuguese => tosho_mplus::proto::Language::BrazilianPortuguese,
            MPlusLanguage::Russian => tosho_mplus::proto::Language::Russian,
            MPlusLanguage::Thai => tosho_mplus::proto::Language::Thai,
            MPlusLanguage::German => tosho_mplus::proto::Language::German,
            // MPlusLanguage::Italian => tosho_mplus::proto::Language::Italian,
            MPlusLanguage::Vietnamese => tosho_mplus::proto::Language::Vietnamese,
        }
    }
}
