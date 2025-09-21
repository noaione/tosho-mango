use std::path::PathBuf;

use clap::Subcommand;

use super::parser::{CommaSeparatedString, parse_comma_string};

pub(crate) mod accounts;
pub(super) mod common;
pub(crate) mod config;
pub(crate) mod download;
pub(crate) mod favorites;
pub(crate) mod manga;
pub(crate) mod rankings;

#[derive(Subcommand, Clone)]
pub(crate) enum RBeanCommands {
    /// Authenticate tosho with your 小豆 (Red Bean) account.
    Auth {
        /// Email to use
        email: String,
        /// Password to use
        password: String,
        /// Platform to use
        #[arg(short, long, value_enum, default_value = "android")]
        platform: crate::r#impl::rbean::config::DeviceType,
    },
    /// Get an account information
    Account,
    /// See all the accounts you have authenticated with
    Accounts,
    /// Automatically/batch download a chapter(s) from a title
    #[command(name = "autodownload")]
    AutoDownload {
        /// UUID of the title
        uuid: String,
        /// Output directory to use
        #[arg(short = 'o', long = "output", default_value = None)]
        output: Option<PathBuf>,
        /// Format to use
        #[arg(short = 'f', long = "format", default_value = "jpeg")]
        format: crate::r#impl::rbean::download::CLIDownloadFormat,
        /// Quality to use
        #[arg(short = 'q', long = "quality", default_value = "hires")]
        quality: crate::r#impl::rbean::download::CLIDownloadQuality,
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
        /// UUID of the title
        uuid: String,
        /// Specify the chapter UUID to purchase (ex: uuid-1,uuid-2,uuid-3)
        #[arg(short = 'c', long = "chapters", default_value = None, value_parser = parse_comma_string)]
        chapters: Option<CommaSeparatedString>,
        /// Output directory to use
        #[arg(short = 'o', long = "output", default_value = None)]
        output: Option<PathBuf>,
        /// Format to use
        #[arg(short = 'f', long = "format", default_value = "jpeg")]
        format: crate::r#impl::rbean::download::CLIDownloadFormat,
        /// Quality to use
        #[arg(short = 'q', long = "quality", default_value = "hires")]
        quality: crate::r#impl::rbean::download::CLIDownloadQuality,
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
    /// Get the home page of your account
    Homepage,
    /// Get a title information
    Info {
        /// UUID of the title
        uuid: String,
        /// Show each chapter detailed information
        #[arg(short = 'c', long = "chapters")]
        show_chapters: bool,
    },
    /// Get the read list of your account
    #[command(name = "readlist")]
    ReadList,
    /// Revoke or delete an account
    Revoke,
    /// Search for a title
    Search {
        /// Query to search for
        query: String,
        /// Limit the number of results
        #[arg(short, long, default_value = "25")]
        limit: Option<u32>,
        /// Sort the results
        #[arg(short, long, value_enum, default_value = "alphabetical")]
        sort: Option<crate::r#impl::rbean::manga::CLISortOption>,
    },
}
