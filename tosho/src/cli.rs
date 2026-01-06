use clap::{
    Parser, Subcommand,
    builder::{
        Styles,
        styling::{AnsiColor, Effects},
    },
};

use crate::r#impl::{
    amap::AMAPCommands, kmkc::KMKCCommands, musq::MUSQCommands, sjv::SJVCommands,
    tools::ToolsCommands,
};

#[derive(Parser)]
#[command(name = "tosho")]
#[command(bin_name = "tosho")]
#[command(author, version = app_version(), about, long_about = None, styles = cli_styles())]
#[command(propagate_version = true, disable_help_subcommand = true)]
pub(crate) struct ToshoCli {
    /// Increase message verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub(crate) verbose: u8,
    /// Use proxy for all requests
    ///
    /// Format: `http(s)://<ip>:<port>` or `socks5://<ip>:<port>`.
    ///
    /// You can also add username and password to the URL like this:
    /// `http(s)://<username>:<password>@<ip>:<port>` or `socks5://<username>:<password>@<ip>:<port>`.
    #[arg(long)]
    pub(crate) proxy: Option<String>,

    #[command(subcommand)]
    pub(crate) command: ToshoCommands,
}

#[derive(Subcommand)]
pub(crate) enum ToshoCommands {
    /// Download manga from MU!
    #[command(name = "mu")]
    Musq {
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,

        #[command(subcommand)]
        subcommand: MUSQCommands,
    },
    /// Download manga from KM
    #[command(name = "km")]
    Kmkc {
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,

        #[command(subcommand)]
        subcommand: KMKCCommands,
    },
    /// Download manga from AM
    #[command(name = "am")]
    Amap {
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,

        #[command(subcommand)]
        subcommand: AMAPCommands,
    },
    /// Download manga from SJ/M
    #[command(name = "sj")]
    Sjv {
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,

        #[command(subcommand)]
        subcommand: SJVCommands,
    },
    /// Download manga from 小豆 (Red Bean)
    #[command(name = "rb")]
    Rbean {
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,

        #[command(subcommand)]
        subcommand: crate::r#impl::rbean::RBeanCommands,
    },
    /// Download manga from M+
    #[command(name = "mp")]
    Mplus {
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,

        /// Language to use
        #[arg(short = 'l', long = "language", value_enum, default_value = "en")]
        language: Option<crate::r#impl::mplus::MPlusLanguage>,

        /// Override the app version code used
        #[arg(short = 'p', long = "app-ver", default_value = None)]
        app_version: Option<u32>,

        #[command(subcommand)]
        subcommand: crate::r#impl::mplus::MPlusCommands,
    },
    /// Download manga and comic from NI
    #[command(name = "ni")]
    Nids {
        /// Account ID to use
        #[arg(short = 'a', long = "account", default_value = None)]
        account_id: Option<String>,

        #[command(subcommand)]
        subcommand: crate::r#impl::nids::NIDSCommands,
    },
    /// Additional tools to manage your downloaded manga
    Tools {
        #[command(subcommand)]
        subcommand: ToolsCommands,
    },
    /// Update tosho to the latest version
    #[cfg(feature = "with-updater")]
    Update,
}

fn cli_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default() | Effects::BOLD)
        .usage(AnsiColor::Magenta.on_default() | Effects::BOLD | Effects::UNDERLINE)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::BrightCyan.on_default())
}

fn app_version() -> &'static str {
    let base_ver = env!("CARGO_PKG_VERSION");
    let commit = option_env!("VERSION_WITH_HASH");

    match commit {
        Some(commit) => commit,
        None => base_ver,
    }
}

pub(crate) fn max_threads(prefer_thread: usize) -> usize {
    let max_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(prefer_thread);

    prefer_thread.min(max_threads).max(1)
}
