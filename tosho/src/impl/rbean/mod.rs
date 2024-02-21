use clap::Subcommand;

pub(crate) mod accounts;
pub(super) mod common;
pub(crate) mod config;

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
    /// Revoke or delete an account
    Revoke,
}
