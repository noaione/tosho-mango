use clap::{Parser, Subcommand};

use crate::r#impl::musq::accounts::DeviceKind;

pub(crate) type ExitCode = u32;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct ToshoCli {
    /// Increase message verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub(crate) verbose: u8,

    #[command(subcommand)]
    pub(crate) command: ToshoCommands,
}

#[derive(Subcommand)]
pub(crate) enum ToshoCommands {
    /// Download manga from MU!
    MUSQ {
        #[command(subcommand)]
        subcommand: MUSQCommands,
    },
    /// Download manga from KM
    KMKC {
        #[command(subcommand)]
        subcommand: KMKCCommands,
    },
}

#[derive(Subcommand)]
pub(crate) enum MUSQCommands {
    /// Authenticate tosho with your MU! account
    Auth {
        /// Session ID
        session_id: String,
        /// Device kind/type to use
        #[arg(short, long, value_enum, default_value = "android")]
        r#type: DeviceKind,
    },
    Account {
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
    Accounts,
    Balance {
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
}

#[derive(Subcommand)]
pub(crate) enum KMKCCommands {}
