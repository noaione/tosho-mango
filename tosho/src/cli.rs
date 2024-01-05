use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use tosho_musq::WeeklyCode;

pub(crate) type ExitCode = u32;

#[derive(Clone)]
pub enum WeeklyCodeCli {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl ValueEnum for WeeklyCodeCli {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            WeeklyCodeCli::Sunday => Some(clap::builder::PossibleValue::new("sun")),
            WeeklyCodeCli::Monday => Some(clap::builder::PossibleValue::new("mon")),
            WeeklyCodeCli::Tuesday => Some(clap::builder::PossibleValue::new("tue")),
            WeeklyCodeCli::Wednesday => Some(clap::builder::PossibleValue::new("wed")),
            WeeklyCodeCli::Thursday => Some(clap::builder::PossibleValue::new("thu")),
            WeeklyCodeCli::Friday => Some(clap::builder::PossibleValue::new("fri")),
            WeeklyCodeCli::Saturday => Some(clap::builder::PossibleValue::new("sat")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[
            WeeklyCodeCli::Sunday,
            WeeklyCodeCli::Monday,
            WeeklyCodeCli::Tuesday,
            WeeklyCodeCli::Wednesday,
            WeeklyCodeCli::Thursday,
            WeeklyCodeCli::Friday,
            WeeklyCodeCli::Saturday,
        ]
    }

    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        let s = if ignore_case {
            input.to_lowercase()
        } else {
            input.to_string()
        };

        match s.as_str() {
            "sun" => Ok(WeeklyCodeCli::Sunday),
            "mon" => Ok(WeeklyCodeCli::Monday),
            "tue" => Ok(WeeklyCodeCli::Tuesday),
            "wed" => Ok(WeeklyCodeCli::Wednesday),
            "thu" => Ok(WeeklyCodeCli::Thursday),
            "fri" => Ok(WeeklyCodeCli::Friday),
            "sat" => Ok(WeeklyCodeCli::Saturday),
            _ => Err(format!("Invalid weekly code: {}", input)),
        }
    }
}

impl From<WeeklyCodeCli> for WeeklyCode {
    fn from(value: WeeklyCodeCli) -> Self {
        match value {
            WeeklyCodeCli::Sunday => WeeklyCode::Sunday,
            WeeklyCodeCli::Monday => WeeklyCode::Monday,
            WeeklyCodeCli::Tuesday => WeeklyCode::Tuesday,
            WeeklyCodeCli::Wednesday => WeeklyCode::Wednesday,
            WeeklyCodeCli::Thursday => WeeklyCode::Thursday,
            WeeklyCodeCli::Friday => WeeklyCode::Friday,
            WeeklyCodeCli::Saturday => WeeklyCode::Saturday,
        }
    }
}

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
    Musq {
        #[command(subcommand)]
        subcommand: MUSQCommands,
    },
    /// Download manga from KM
    Kmkc {
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
        r#type: super::r#impl::musq::accounts::DeviceKind,
    },
    /// Get an account information
    Account {
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
    /// See all the accounts you have authenticated with
    Accounts,
    /// Get your account point balance
    Balance {
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
    /// Search for a title
    Search {
        /// Query to search for
        query: String,
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
    /// Get weekly releases
    Weekly {
        /// Day of the week to get releases for
        #[arg(short = 'd', long = "day", value_enum, default_value = None)]
        weekday: Option<WeeklyCodeCli>,
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
}

#[derive(Subcommand)]
pub(crate) enum KMKCCommands {
    /// Authenticate tosho with your KM account. (Experimental)
    ///
    /// The following use email/password authentication
    Auth {
        /// Email to use
        email: String,
        /// Password to use
        password: String,
        /// Device kind/type to use
        #[arg(short, long, value_enum, default_value = "android")]
        r#type: super::r#impl::kmkc::accounts::DeviceKind,
    },
    /// Authenticate tosho with your KM account.
    ///
    /// The following use user ID/hash key to authenticate as mobile.
    AuthMobile {
        /// User ID to use
        user_id: u32,
        /// Hash key to use
        hash_key: String,
    },
    /// Authenticate tosho with your KM account.
    ///
    /// The following use Netscape cookies to authenticate as web.
    AuthWeb {
        /// Path to Netscape cookies file
        cookies: PathBuf,
    },
    /// Get an account information
    Account {
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
    /// See all the accounts you have authenticated with
    Accounts,
    /// Get your account point balance
    Balance {
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
}
