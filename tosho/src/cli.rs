use std::{ops::RangeInclusive, path::PathBuf};

use clap::{
    builder::{
        styling::{AnsiColor, Effects},
        Styles,
    },
    Parser, Subcommand, ValueEnum,
};
use tosho_macros::EnumName;
use tosho_musq::WeeklyCode;

use crate::r#impl::kmkc::rankings::RankingType;

pub(crate) type ExitCode = u32;
pub(crate) type CommaSeparatedNumber = Vec<usize>;
pub(crate) type CommaSeparatedString = Vec<String>;

#[derive(Clone, EnumName)]
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

impl From<WeeklyCode> for WeeklyCodeCli {
    fn from(value: WeeklyCode) -> Self {
        match value {
            WeeklyCode::Sunday => WeeklyCodeCli::Sunday,
            WeeklyCode::Monday => WeeklyCodeCli::Monday,
            WeeklyCode::Tuesday => WeeklyCodeCli::Tuesday,
            WeeklyCode::Wednesday => WeeklyCodeCli::Wednesday,
            WeeklyCode::Thursday => WeeklyCodeCli::Thursday,
            WeeklyCode::Friday => WeeklyCodeCli::Friday,
            WeeklyCode::Saturday => WeeklyCodeCli::Saturday,
        }
    }
}

impl WeeklyCodeCli {
    /// Get the index of the weekday
    pub fn indexed(&self) -> i32 {
        match self {
            WeeklyCodeCli::Monday => 1,
            WeeklyCodeCli::Tuesday => 2,
            WeeklyCodeCli::Wednesday => 3,
            WeeklyCodeCli::Thursday => 4,
            WeeklyCodeCli::Friday => 5,
            WeeklyCodeCli::Saturday => 6,
            WeeklyCodeCli::Sunday => 7,
        }
    }
}

#[derive(Parser)]
#[command(name = "tosho")]
#[command(bin_name = "tosho")]
#[command(author, version, about, long_about = None, styles = cli_styles())]
#[command(propagate_version = true, disable_help_subcommand = true)]
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
    #[command(name = "mu")]
    Musq {
        #[command(subcommand)]
        subcommand: MUSQCommands,
    },
    /// Download manga from KM
    #[command(name = "km")]
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
    /// Download a chapters from a title
    Download {
        /// Title ID to use
        title_id: u64,
        /// Specify the chapter ID to purchase (ex: 1,2,3,4,5)
        #[arg(short = 'c', long = "chapters", default_value = None, value_parser = parse_comma_number)]
        chapters: Option<CommaSeparatedNumber>,
        /// Show all the chapters available for the title
        #[arg(long = "show-all")]
        show_all: bool,
        /// Automatically purchase chapters if needed
        #[arg(short = 'p', long = "auto-purchase")]
        auto_purchase: bool,
        /// Specify the image quality to download
        #[arg(short = 'q', long = "quality", default_value = "high", value_enum)]
        quality: super::r#impl::musq::download::DownloadImageQuality,
        /// Output directory to use
        #[arg(short = 'o', long = "output", default_value = None)]
        output: Option<PathBuf>,
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
    /// Get your account favorites list
    Favorites {
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
    /// Get your account reading history
    History {
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
    /// Get a title information
    Info {
        /// Title ID to use
        title_id: u64,
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
        /// Show each chapter detailed information
        #[arg(short = 'c', long = "chapters")]
        show_chapters: bool,
        /// Show related titles
        #[arg(short = 'r', long = "related")]
        show_related: bool,
    },
    /// Purchases chapters for a title
    Purchase {
        /// Title ID to use
        title_id: u64,
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
    /// Precalculate the amount of points needed to purchase chapters for a title
    Precalculate {
        /// Title ID to use
        title_id: u64,
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
    /// Get the current title rankings
    Rankings {
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
    /// Revoke or delete an account
    Revoke {
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
        /// Device kind/type to use
        #[arg(short, long, value_enum, default_value = "android")]
        r#type: super::r#impl::kmkc::accounts::DeviceKind,
    },
    /// Authenticate tosho with your KM account.
    ///
    /// The following use Netscape cookies to authenticate as web.
    AuthWeb {
        /// Path to Netscape cookies file
        cookies: PathBuf,
    },
    /// Adapt web config/account to mobile config/account
    AuthAdapt {
        /// Device kind/type to use
        #[arg(short, long, value_enum, default_value = "android")]
        r#type: super::r#impl::kmkc::accounts::DeviceKind,
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
    /// Get a title information
    Info {
        /// Title ID to use
        title_id: i32,
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
        /// Show each chapter detailed information
        #[arg(short = 'c', long = "chapters")]
        show_chapters: bool,
    },
    /// Get magazines list information
    Magazines {
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
    /// Purchases chapters for a title
    Purchase {
        /// Title ID to use
        title_id: i32,
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
    /// See purchased titles for an account
    Purchased {
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
    /// Precalculate the amount of points needed to purchase chapters for a title
    Precalculate {
        /// Title ID to use
        title_id: i32,
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
    },
    /// Get the current title rankings
    Rankings {
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,
        /// Which ranking tab to use
        #[arg(short = 'r', long = "ranking", default_value = None)]
        ranking_tab: Option<RankingType>,
        /// Limit the amount of titles to fetch/show
        #[arg(short = 'l', long = "limit", default_value = None, value_parser = kmkc_ranking_limit_range)]
        limit: Option<u32>,
    },
    /// Revoke or delete an account
    Revoke {
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

fn cli_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default() | Effects::BOLD)
        .usage(AnsiColor::Magenta.on_default() | Effects::BOLD | Effects::UNDERLINE)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::BrightCyan.on_default())
}

const KMKC_RANKING_LIMIT_RANGE: RangeInclusive<usize> = 1..=100;

fn kmkc_ranking_limit_range(s: &str) -> Result<Option<u32>, String> {
    let s: usize = s.parse().map_err(|_| format!("Invalid limit: {}", s))?;

    if KMKC_RANKING_LIMIT_RANGE.contains(&s) {
        Ok(Some(s as u32))
    } else {
        Err(format!(
            "Limit not in range {}-{}",
            KMKC_RANKING_LIMIT_RANGE.start(),
            KMKC_RANKING_LIMIT_RANGE.end()
        ))
    }
}

/// Value parser for comma separated numbers
fn parse_comma_number(s: &str) -> Result<CommaSeparatedNumber, String> {
    let mut numbers = Vec::new();

    for number in s.split(',') {
        let number = number.trim();
        let number = number
            .parse()
            .map_err(|_| format!("Invalid number: {}", number))?;

        numbers.push(number);
    }

    Ok(numbers)
}

/// Value parser for comma separated strings
#[allow(dead_code)]
fn parse_comma_string(s: &str) -> Result<CommaSeparatedString, String> {
    let mut strings = Vec::new();

    for string in s.split(',') {
        let string = string.trim();

        // skip if empty
        if string.is_empty() {
            continue;
        }

        strings.push(string.to_string());
    }

    Ok(strings)
}
