use std::path::PathBuf;

use super::parser::{parse_comma_number, CommaSeparatedNumber};
use clap::Subcommand;

pub(crate) mod accounts;
pub(super) mod common;
pub(crate) mod config;
pub(crate) mod download;
pub(crate) mod manga;

#[derive(Subcommand)]
pub(crate) enum AMAPCommands {
    /// Authenticate tosho with your AM account.
    Auth {
        /// Email to use
        email: String,
        /// Password to use
        password: String,
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
        /// Disable the use of paid ticket to purchase chapters
        #[arg(long = "no-paid")]
        no_paid_ticket: bool,
        /// Disable the use of premium ticket to purchase chapters
        #[arg(long = "no-premium")]
        no_premium_ticket: bool,
        /// Output directory to use
        #[arg(short = 'o', long = "output", default_value = None)]
        output: Option<PathBuf>,
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
    /// Get your account ticket balance
    Balance {
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
    /// Get home discovery
    Discovery {
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
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
        title_id: u64,
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
        /// Show each chapter detailed information
        #[arg(short = 'c', long = "chapters")]
        show_chapters: bool,
    },
    /// Search for a title
    Search {
        /// Query to search for
        query: String,
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
}
