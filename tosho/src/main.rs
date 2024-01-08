use clap::Parser;
use cli::{ToshoCommands, WeeklyCodeCli};
use tosho_musq::WeeklyCode;

mod cli;
pub(crate) mod config;
pub(crate) mod r#impl;
pub(crate) mod term;
pub(crate) mod win_term;
use crate::cli::ToshoCli;

#[tokio::main]
async fn main() {
    // For some god know what reason, `clap` + rustc_lint will show this as unreachable code.
    let _cli = ToshoCli::parse();

    let t = term::get_console(_cli.verbose);
    let mut t_mut = term::get_console(_cli.verbose);

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
            cli::MUSQCommands::Info {
                title_id,
                account_id,
                show_chapters,
                show_related,
            } => {
                r#impl::musq::manga::musq_title_info(
                    title_id,
                    account_id.as_deref(),
                    show_chapters,
                    show_related,
                    &t,
                )
                .await
            }
            cli::MUSQCommands::Purchase {
                title_id,
                account_id,
            } => {
                r#impl::musq::purchases::musq_purchase(title_id, account_id.as_deref(), &mut t_mut)
                    .await
            }
            cli::MUSQCommands::Precalculate {
                title_id,
                account_id,
            } => {
                r#impl::musq::purchases::musq_purchase_precalculate(
                    title_id,
                    account_id.as_deref(),
                    &t,
                )
                .await
            }
            cli::MUSQCommands::Search { query, account_id } => {
                r#impl::musq::manga::musq_search(query.as_str(), account_id.as_deref(), &t).await
            }
            cli::MUSQCommands::Weekly {
                weekday,
                account_id,
            } => {
                let weekday: WeeklyCode = match weekday {
                    Some(week) => week.into(),
                    None => WeeklyCode::today(),
                };

                r#impl::musq::manga::musq_search_weekly(weekday, account_id.as_deref(), &t).await
            }
        },
        ToshoCommands::Kmkc { subcommand } => match subcommand {
            cli::KMKCCommands::Auth {
                email,
                password,
                r#type,
            } => r#impl::kmkc::accounts::kmkc_account_login(email, password, r#type, &t).await,
            cli::KMKCCommands::AuthMobile { user_id, hash_key } => {
                r#impl::kmkc::accounts::kmkc_account_login_mobile(user_id, hash_key, &t).await
            }
            cli::KMKCCommands::AuthWeb { cookies } => {
                r#impl::kmkc::accounts::kmkc_account_login_web(cookies, &t).await
            }
            cli::KMKCCommands::AuthAdapt => {
                r#impl::kmkc::accounts::kmkc_account_login_adapt(&t).await
            }
            cli::KMKCCommands::Account { account_id } => {
                r#impl::kmkc::accounts::kmkc_account_info(account_id.as_deref(), &t).await
            }
            cli::KMKCCommands::Accounts => r#impl::kmkc::accounts::kmkc_accounts(&t),
            cli::KMKCCommands::Balance { account_id } => {
                r#impl::kmkc::accounts::kmkc_balance(account_id.as_deref(), &t).await
            }
            cli::KMKCCommands::Info {
                title_id,
                account_id,
                show_chapters,
            } => {
                r#impl::kmkc::manga::kmkc_title_info(
                    title_id,
                    account_id.as_deref(),
                    show_chapters,
                    &t,
                )
                .await
            }
            cli::KMKCCommands::Search { query, account_id } => {
                r#impl::kmkc::manga::kmkc_search(query.as_str(), account_id.as_deref(), &t).await
            }
            cli::KMKCCommands::Weekly {
                weekday,
                account_id,
            } => {
                let weekday: WeeklyCodeCli = match weekday {
                    Some(week) => week,
                    None => WeeklyCode::today().into(),
                };

                r#impl::kmkc::manga::kmkc_search_weekly(weekday, account_id.as_deref(), &t).await
            }
        },
    };

    std::process::exit(exit_code as i32);
}
