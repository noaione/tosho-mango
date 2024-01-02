use clap::{Parser, Subcommand};

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
pub(crate) enum MUSQCommands {}

#[derive(Subcommand)]
pub(crate) enum KMKCCommands {}
