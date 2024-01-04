use clap::Parser;

mod cli;
pub(crate) mod config;
pub(crate) mod r#impl;
pub(crate) mod term;
use crate::cli::ToshoCli;

#[tokio::main]
async fn main() {
    // For some god know what reason, `clap` + rustc_lint will show this as unreachable code.
    let _cli = ToshoCli::parse();

    let t = term::get_console(_cli.verbose);
}
