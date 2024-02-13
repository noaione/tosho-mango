use clap::{
    builder::{
        styling::{AnsiColor, Effects},
        Styles,
    },
    Parser, Subcommand,
};

use crate::r#impl::{
    amap::AMAPCommands, kmkc::KMKCCommands, musq::MUSQCommands, sjv::SJVCommands,
    tools::ToolsCommands,
};

pub(crate) type ExitCode = u32;

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
    /// Additional tools to manage your downloaded manga
    Tools {
        #[command(subcommand)]
        subcommand: ToolsCommands,
    },
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
