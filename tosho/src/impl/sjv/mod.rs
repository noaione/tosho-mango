use clap::Subcommand;

pub(crate) mod accounts;
pub(crate) mod common;
pub(crate) mod config;

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
    /// Revoke or delete an account
    Revoke,
    /// Get account subscription info
    Subscription,
}
