use std::path::PathBuf;

use clap::Subcommand;

use super::parser::{CommaSeparatedNumber, NumberOrString, parse_comma_number};

pub(crate) mod accounts;
pub(crate) mod common;
pub(crate) mod config;
pub(crate) mod download;
pub(crate) mod manga;

#[derive(Subcommand, Clone)]
pub(crate) enum SJVCommands {
    /// Authenticate tosho with your SJ/M account.
    Auth {
        /// Email to use
        email: String,
        /// Password to use
        password: String,
        /// Mode to use
        #[arg(short, long, value_enum, default_value = "sj")]
        mode: crate::r#impl::sjv::config::SJDeviceMode,
        #[arg(short, long, value_enum, default_value = "web")]
        platform: crate::r#impl::sjv::config::DeviceType,
    },
    /// Get an account information
    Account,
    /// See all the accounts you have authenticated with
    Accounts,
    /// Automatically/batch download a chapter(s) from a title
    #[command(name = "autodownload")]
    AutoDownload {
        /// Title ID or Slug to use
        title_or_slug: NumberOrString,
        /// Specify the starting chapter ID to download
        #[arg(short = 's', long, default_value = None)]
        start_from: Option<u32>,
        /// Specify the end chapter ID to download
        #[arg(short = 'e', long, default_value = None)]
        end_until: Option<u32>,
        /// Output directory to use
        #[arg(short = 'o', long = "output", default_value = None)]
        output: Option<PathBuf>,
        /// Enable parallel download
        #[arg(short = 'p', long = "parallel")]
        parallel: bool,
        /// Number of threads to use for parallel download
        ///
        /// Please note that this would be capped to your system's available CPU threads.
        /// I recommend not using more than 4 to avoid getting rate limited.
        ///
        /// Needs to be used with `--parallel` flag.
        #[arg(short = 't', long = "threads", default_value = "4")]
        threads: usize,
    },
    /// Download a chapters from a title
    Download {
        /// Title ID or Slug to use
        title_or_slug: NumberOrString,
        /// Specify the chapter ID to purchase (ex: 1,2,3,4,5)
        #[arg(short = 'c', long = "chapters", default_value = None, value_parser = parse_comma_number)]
        chapters: Option<CommaSeparatedNumber>,
        /// Output directory to use
        #[arg(short = 'o', long = "output", default_value = None)]
        output: Option<PathBuf>,
        /// Enable parallel download
        #[arg(short = 'p', long = "parallel")]
        parallel: bool,
        /// Number of threads to use for parallel download
        ///
        /// Please note that this would be capped to your system's available CPU threads.
        /// I recommend not using more than 4 to avoid getting rate limited.
        ///
        /// Needs to be used with `--parallel` flag.
        #[arg(short = 't', long = "threads", default_value = "4")]
        threads: usize,
    },
    /// Get a title information
    Info {
        /// Title ID or Slug to use
        title_or_slug: NumberOrString,
        /// Show each chapter detailed information
        #[arg(short = 'c', long = "chapters")]
        show_chapters: bool,
    },
    /// Revoke or delete an account
    Revoke,
    /// Search for a title
    Search {
        /// Query to search for
        query: String,
    },
    /// Get account subscription info
    Subscription,
}
