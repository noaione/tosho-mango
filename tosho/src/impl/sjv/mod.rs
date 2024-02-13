use clap::Subcommand;

use super::parser::NumberOrString;

pub(crate) mod accounts;
pub(crate) mod common;
pub(crate) mod config;
pub(crate) mod manga;

#[derive(Subcommand, Clone)]
pub(crate) enum SJVCommands {
    /// Authenticate tosho with your AM account.
    Auth {
        /// Email to use
        email: String,
        /// Password to use
        password: String,
        /// Mode to use
        #[arg(short, long, value_enum, default_value = "sj")]
        mode: crate::r#impl::sjv::config::SJDeviceMode,
    },
    /// Get an account information
    Account,
    /// See all the accounts you have authenticated with
    Accounts,
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
