use clap::Subcommand;

pub(crate) mod accounts;
pub(super) mod common;
pub(crate) mod config;
pub(crate) mod manga;

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
    /// Get a title information
    Info {
        /// UUID of the title
        uuid: String,
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
        /// Limit the number of results
        #[arg(short, long, default_value = "25")]
        limit: Option<u32>,
        /// Sort the results
        #[arg(short, long, value_enum, default_value = "alphabetical")]
        sort: Option<crate::r#impl::rbean::manga::CLISortOption>,
    },
}
