use clap::Parser;

mod cli;
mod r#impl;
mod term;
use crate::cli::ToshoCli;

#[tokio::main]
async fn main() {
    // For some god know what reason, `clap` + rustc_lint will show this as unreachable code.
    let _cli = ToshoCli::parse();

    #[allow(unreachable_code)]
    match _cli.verbose {
        0 => println!("No verbosity"),
        1 => println!("Verbose"),
        _ => println!("Very verbose"),
    }
}
