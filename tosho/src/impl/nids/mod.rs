use std::path::PathBuf;

use clap::Subcommand;

use crate::r#impl::nids::common::{
    FilterPairInput, FilterScopeInput, SortByInput, SortOrderInput, parse_filter_pairs,
    parse_sort_by,
};

pub(crate) mod accounts;
pub(crate) mod common;
pub(crate) mod config;
pub(crate) mod download;
pub(crate) mod issues;
pub(crate) mod publishers;
pub(crate) mod purchases;
pub(crate) mod series;

#[derive(Subcommand, Clone)]
pub(crate) enum NIDSCommands {
    /// Authenticate tosho with your NI account.
    Auth {
        /// Session token to use
        session_token: String,
        /// Device kind/type to use
        #[arg(short, long, value_enum, default_value = "web")]
        r#type: crate::r#impl::nids::accounts::DeviceKind,
    },
    /// Get an account information
    Account,
    /// See all the accounts you have authenticated with
    Accounts,
    /// Download an issue by the ID
    Download {
        /// Issue ID to download
        issue_id: u32,
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
        /// Do not report page viewing progress to the server
        ///
        /// This is not recommended
        #[arg(long = "no-report", default_value_t = false)]
        no_report: bool,
    },
    /// Get single issue information by the ID
    Issue {
        /// Issue ID to get
        issue_id: u32,

        /// Get with marketplace editions
        #[arg(short = 'm', long = "with-marketplace", default_value_t = false)]
        with_marketplace: bool,
    },
    /// Get a list of issues depending on the filters
    Issues {
        /// The multiple filter pair to use (ex: key=value)
        #[arg(short = 'f', long = "filter", default_value = None, value_parser = parse_filter_pairs)]
        filters: Option<Vec<FilterPairInput>>,
        /// Maximum number of issues to return
        #[arg(short = 'l', long = "limit", default_value_t = 18)]
        limit: u32,
        /// What field to use for sorting
        ///
        /// Some examples: `id`, `title`, `full_title`, `issue_number`, `book_index`, `release_date`, `publication-date`
        #[arg(short = 's', long = "sort", default_value = "full_title", value_parser = parse_sort_by)]
        sort_by: SortByInput,
        /// The direction of the sort order
        #[arg(short = 'd', long = "direction", default_value = "asc")]
        direction: SortOrderInput,
        /// The scope of the filter (frontlist/backlist)
        #[arg(short = 'c', long = "scope", default_value = None)]
        scope: Option<FilterScopeInput>,
    },
    /// Get a list of currently sold issues in the marketplace
    Marketplace {
        /// Maximum number of issues to return
        #[arg(short = 'l', long = "limit", default_value_t = 25)]
        limit: usize,
        /// Page number to return (starts from 1)
        #[arg(short = 'p', long = "page", default_value_t = 1)]
        page: usize,
    },
    /// Get a information about a publisher
    Publisher {
        /// Publisher slug to get
        publisher_slug: String,

        /// Get with imprints
        #[arg(short = 'l', long = "imprints", default_value_t = false)]
        with_imprints: bool,
    },
    /// Get a list of publishers
    Publishers,
    /// Get specific purchased issues by series ID
    PurchasedIssues {
        /// Series UUID to get
        series_run_uuid: String,

        /// Maximum number of issues to return
        #[arg(short = 'l', long = "limit", default_value_t = 18)]
        limit: u32,
    },
    /// Get list of purchased series
    PurchasedSeries {
        /// Maximum number of series to return
        #[arg(short = 'l', long = "limit", default_value_t = 18)]
        limit: u32,
    },
    /// Revoke or delete an account
    Revoke,
    /// Get specific series run information by ID
    SeriesRun {
        /// Series run ID to get
        series_run_id: u32,

        /// Get with marketplace editions
        #[arg(short = 'm', long = "with-marketplace", default_value_t = false)]
        with_marketplace: bool,
    },
    /// Get a list of series runs
    SeriesRuns {
        /// The multiple filter pair to use (ex: key=value)
        #[arg(short = 'f', long = "filter", default_value = None, value_parser = parse_filter_pairs)]
        filters: Option<Vec<FilterPairInput>>,
        /// Maximum number of issues to return
        #[arg(short = 'l', long = "limit", default_value_t = 24)]
        limit: u32,
        /// What field to use for sorting
        ///
        /// Some examples: `id`, `title`, `full_title`, `issue_number`, `book_index`, `release_date`, `publication_date`
        #[arg(short = 's', long = "sort", default_value = "title", value_parser = parse_sort_by)]
        sort_by: SortByInput,
        /// The direction of the sort order
        #[arg(short = 'd', long = "direction", default_value = "asc")]
        direction: SortOrderInput,
    },
}
