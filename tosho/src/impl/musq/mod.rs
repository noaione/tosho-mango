use std::path::PathBuf;

use clap::Subcommand;

use super::parser::{parse_comma_number, CommaSeparatedNumber, WeeklyCodeCli};

pub(crate) mod accounts;
pub(super) mod common;
pub(crate) mod config;
pub(crate) mod download;
pub(crate) mod favorites;
pub(crate) mod manga;
pub(crate) mod purchases;
pub(crate) mod rankings;

#[derive(Subcommand)]
pub(crate) enum MUSQCommands {
    /// Authenticate tosho with your MU! account
    Auth {
        /// Session ID
        session_id: String,
        /// Device kind/type to use
        #[arg(short, long, value_enum, default_value = "android")]
        r#type: crate::r#impl::musq::accounts::DeviceKind,
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
        /// Disable the auto purchase feature and only download free/purchased chapter(s).
        #[arg(short = 'n', long)]
        no_purchase: bool,
        /// Specify the starting chapter ID to download
        #[arg(short = 's', long, default_value = None)]
        start_from: Option<u64>,
        /// Specify the end chapter ID to download
        #[arg(short = 'e', long, default_value = None)]
        end_until: Option<u64>,
        /// Disable the use of paid coins to purchase chapters
        #[arg(long = "no-paid")]
        no_paid_coins: bool,
        /// Disable the use of XP/event coins to purchase chapters
        #[arg(long = "no-xp")]
        no_xp_coins: bool,
        /// Specify the image quality to download
        #[arg(short = 'q', long = "quality", default_value = "high", value_enum)]
        quality: crate::r#impl::musq::download::DownloadImageQuality,
        /// Output directory to use
        #[arg(short = 'o', long = "output", default_value = None)]
        output: Option<PathBuf>,
    },
    /// Get your account point balance
    Balance,
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
        /// Automatically purchase chapters if needed
        #[arg(short = 'p', long = "auto-purchase")]
        auto_purchase: bool,
        /// Specify the image quality to download
        #[arg(short = 'q', long = "quality", default_value = "high", value_enum)]
        quality: crate::r#impl::musq::download::DownloadImageQuality,
        /// Output directory to use
        #[arg(short = 'o', long = "output", default_value = None)]
        output: Option<PathBuf>,
    },
    /// Get your account favorites list
    Favorites,
    /// Get your account reading history
    History,
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
    /// Purchases chapters for a title
    Purchase {
        /// Title ID to use
        title_id: u64,
    },
    /// Precalculate the amount of points needed to purchase chapters for a title
    Precalculate {
        /// Title ID to use
        title_id: u64,
    },
    /// Get the current title rankings
    Rankings,
    /// Revoke or delete an account
    Revoke,
    /// Search for a title
    Search {
        /// Query to search for
        query: String,
    },
    /// Get weekly releases
    Weekly {
        /// Day of the week to get releases for
        #[arg(short = 'd', long = "day", value_enum, default_value = None)]
        weekday: Option<WeeklyCodeCli>,
    },
}
