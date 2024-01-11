use std::{ops::RangeInclusive, path::PathBuf};

use clap::Subcommand;

use super::parser::{parse_comma_number, CommaSeparatedNumber, WeeklyCodeCli};

use self::rankings::RankingType;

pub(crate) mod accounts;
pub(super) mod common;
pub(crate) mod config;
pub(crate) mod download;
pub(crate) mod manga;
pub(crate) mod purchases;
pub(crate) mod rankings;

#[derive(Subcommand)]
pub(crate) enum KMKCCommands {
    /// Authenticate tosho with your KM account. (Experimental)
    ///
    /// The following use email/password authentication
    Auth {
        /// Email to use
        email: String,
        /// Password to use
        password: String,
        /// Device kind/type to use
        #[arg(short, long, value_enum, default_value = "android")]
        r#type: crate::r#impl::kmkc::accounts::DeviceKind,
    },
    /// Authenticate tosho with your KM account.
    ///
    /// The following use user ID/hash key to authenticate as mobile.
    AuthMobile {
        /// User ID to use
        user_id: u32,
        /// Hash key to use
        hash_key: String,
        /// Device kind/type to use
        #[arg(short, long, value_enum, default_value = "android")]
        r#type: crate::r#impl::kmkc::accounts::DeviceKind,
    },
    /// Authenticate tosho with your KM account.
    ///
    /// The following use Netscape cookies to authenticate as web.
    AuthWeb {
        /// Path to Netscape cookies file
        cookies: PathBuf,
    },
    /// Adapt web config/account to mobile config/account
    AuthAdapt {
        /// Device kind/type to use
        #[arg(short, long, value_enum, default_value = "android")]
        r#type: crate::r#impl::kmkc::accounts::DeviceKind,
    },
    /// Get an account information
    Account {
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
    /// See all the accounts you have authenticated with
    Accounts,
    /// Automatically/batch download a chapter(s) from a title
    #[command(name = "autodownload")]
    AutoDownload {
        /// Title ID to use
        title_id: i32,
        /// Disable the auto purchase feature and only download free/purchased chapter(s).
        #[arg(short = 'n', long)]
        no_purchase: bool,
        /// Specify the starting chapter ID to download
        #[arg(short = 's', long, default_value = None)]
        start_from: Option<i32>,
        /// Specify the end chapter ID to download
        #[arg(short = 'e', long, default_value = None)]
        end_until: Option<i32>,
        /// Disable both title/premium ticket from being used to purchase chapters
        #[arg(long)]
        no_ticket: bool,
        /// Disable the use of points to purchase chapters
        #[arg(long)]
        no_point: bool,
        /// Output directory to use
        #[arg(short = 'o', long = "output", default_value = None)]
        output: Option<PathBuf>,
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
    /// Get your account point balance
    Balance {
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
    /// Download a chapters from a title
    Download {
        /// Title ID to use
        title_id: i32,
        /// Specify the chapter ID to purchase (ex: 1,2,3,4,5)
        #[arg(short = 'c', long = "chapters", default_value = None, value_parser = parse_comma_number)]
        chapters: Option<CommaSeparatedNumber>,
        /// Show all the chapters available for the title
        #[arg(long = "show-all")]
        show_all: bool,
        /// Automatically purchase chapters if needed
        #[arg(short = 'p', long = "auto-purchase")]
        auto_purchase: bool,
        /// Output directory to use
        #[arg(short = 'o', long = "output", default_value = None)]
        output: Option<PathBuf>,
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
    /// Get a title information
    Info {
        /// Title ID to use
        title_id: i32,
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
        /// Show each chapter detailed information
        #[arg(short = 'c', long = "chapters")]
        show_chapters: bool,
    },
    /// Get magazines list information
    Magazines {
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
    /// Purchases chapters for a title
    Purchase {
        /// Title ID to use
        title_id: i32,
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
    /// See purchased titles for an account
    Purchased {
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
    /// Precalculate the amount of points needed to purchase chapters for a title
    Precalculate {
        /// Title ID to use
        title_id: i32,
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
    /// Get the current title rankings
    Rankings {
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
        /// Which ranking tab to use
        #[arg(short = 'r', long = "ranking", default_value = None)]
        ranking_tab: Option<RankingType>,
        /// Limit the amount of titles to fetch/show
        #[arg(short = 'l', long = "limit", default_value = None, value_parser = kmkc_ranking_limit_range)]
        limit: Option<u32>,
    },
    /// Revoke or delete an account
    Revoke {
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
    /// Search for a title
    Search {
        /// Query to search for
        query: String,
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
    /// Get weekly releases
    Weekly {
        /// Day of the week to get releases for
        #[arg(short = 'd', long = "day", value_enum, default_value = None)]
        weekday: Option<WeeklyCodeCli>,
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
}

const KMKC_RANKING_LIMIT_RANGE: RangeInclusive<usize> = 1..=100;

fn kmkc_ranking_limit_range(s: &str) -> Result<Option<u32>, String> {
    let s: usize = s.parse().map_err(|_| format!("Invalid limit: {}", s))?;

    if KMKC_RANKING_LIMIT_RANGE.contains(&s) {
        Ok(Some(s as u32))
    } else {
        Err(format!(
            "Limit not in range {}-{}",
            KMKC_RANKING_LIMIT_RANGE.start(),
            KMKC_RANKING_LIMIT_RANGE.end()
        ))
    }
}
