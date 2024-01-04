use clap::Parser;
use cli::ToshoCommands;

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

    let exit_code = match _cli.command {
        ToshoCommands::Musq { subcommand } => match subcommand {
            cli::MUSQCommands::Auth { session_id, r#type } => {
                r#impl::musq::accounts::musq_auth_session(session_id, r#type, &t).await
            }
            cli::MUSQCommands::Account { account_id } => {
                r#impl::musq::accounts::musq_account_info(account_id.as_deref(), &t).await
            }
            cli::MUSQCommands::Accounts => r#impl::musq::accounts::musq_accounts(&t),
            cli::MUSQCommands::Balance { account_id } => {
                r#impl::musq::accounts::musq_account_balance(account_id.as_deref(), &t).await
            }
        },
        ToshoCommands::Kmkc { subcommand: _ } => {
            t.warn("TODO");
            0
        }
    };

    std::process::exit(exit_code as i32);
}
