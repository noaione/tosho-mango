use clap::Subcommand;

pub(crate) mod accounts;
pub(super) mod common;
pub(crate) mod config;

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
    /// Get your account ticket balance
    Balance {
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
}
