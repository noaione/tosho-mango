use std::path::PathBuf;

use clap::Subcommand;

use crate::r#impl::nids::common::{
    FilterPairInput, FilterScopeInput, SortByInput, SortOrderInput, parse_filter_pairs,
    parse_sort_by,
};

pub(crate) mod accounts;
pub(crate) mod common;
pub(crate) mod config;

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
        issue_id: u64,
        /// Output directory to use
        #[arg(short = 'o', long = "output", default_value = None)]
        output: Option<PathBuf>,
    },
    /// Get single issue information by the ID
    Issue {
        /// Issue ID to get
        issue_id: u64,

        /// Get with marketplace editions
        #[arg(short = 'm', long = "marketplaces", default_value_t = false)]
        with_marketplace: bool,
    },
    /// Get a list of issues depending on the filters
    Issues {
        /// The multiple filter pair to use (ex: key=value)
        #[arg(short = 'f', long = "filter", default_value = None, value_parser = parse_filter_pairs)]
        filters: Option<Vec<FilterPairInput>>,
        /// Maximum number of issues to return
        #[arg(short = 'l', long = "limit", default_value_t = 20)]
        limit: usize,
        /// Page number to return (starts from 1)
        #[arg(short = 'p', long = "page", default_value_t = 1)]
        page: usize,
        /// What field to use for sorting
        ///
        /// Some examples: `id`, `title`, `full_title`, `issue_number`, `book_index`, `release_date`, `publication-date`
        #[arg(short = 's', long = "sort", default_value = "full_title", value_parser = parse_sort_by)]
        sort_by: Option<SortByInput>,
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
    /// Get a list of publishers
    Publishers,
    /// Get specific purchased issues by series ID
    PurchasedIssues {
        /// Series ID to get
        series_run_id: u32,

        /// Maximum number of issues to return
        #[arg(short = 'l', long = "limit", default_value_t = 18)]
        limit: usize,
        /// Page number to return (starts from 1)
        #[arg(short = 'p', long = "page", default_value_t = 1)]
        page: usize,
    },
    /// Get list of purchased series
    PurchasedSeries {
        /// Maximum number of series to return
        #[arg(short = 'l', long = "limit", default_value_t = 18)]
        limit: usize,
        /// Page number to return (starts from 1)
        #[arg(short = 'p', long = "page", default_value_t = 1)]
        page: usize,
    },
    /// Revoke or delete an account
    Revoke,
    /// Get specific series run information by ID
    SeriesRun {
        /// Series run ID to get
        series_run_id: u64,
    },
    /// Get a list of series runs
    SeriesRuns {
        /// The multiple filter pair to use (ex: key=value)
        #[arg(short = 'f', long = "filter", default_value = None, value_parser = parse_filter_pairs)]
        filters: Option<Vec<FilterPairInput>>,
        /// Maximum number of issues to return
        #[arg(short = 'l', long = "limit", default_value_t = 20)]
        limit: usize,
        /// Page number to return (starts from 1)
        #[arg(short = 'p', long = "page", default_value_t = 1)]
        page: usize,
        /// What field to use for sorting
        ///
        /// Some examples: `id`, `title`, `full_title`, `issue_number`, `book_index`, `release_date`, `publication-date`
        #[arg(short = 's', long = "sort", default_value = "title", value_parser = parse_sort_by)]
        sort_by: Option<SortByInput>,
        /// The direction of the sort order
        #[arg(short = 'd', long = "direction", default_value = "asc")]
        direction: SortOrderInput,
    },
}
