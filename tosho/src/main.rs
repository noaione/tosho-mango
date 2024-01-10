use std::path::PathBuf;

use clap::Parser;
use cli::{ToshoCommands, WeeklyCodeCli};
use r#impl::{kmkc::download::KMDownloadCliConfig, musq::download::MUDownloadCliConfig};
use tosho_musq::WeeklyCode;

mod cli;
pub(crate) mod config;
pub(crate) mod r#impl;
pub(crate) mod term;
pub(crate) mod win_term;
use crate::cli::ToshoCli;

fn get_default_download_dir() -> PathBuf {
    let cwd = std::env::current_dir().unwrap();
    cwd.join("DOWNLOADS")
}

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
            cli::MUSQCommands::AutoDownload {
                title_id,
                no_purchase,
                start_from,
                end_until,
                no_paid_coins,
                no_xp_coins,
                quality,
                output,
                account_id,
            } => {
                let mut mu_config = MUDownloadCliConfig::default();
                mu_config.auto_purchase = !no_purchase;
                mu_config.quality = quality;
                mu_config.start_from = start_from;
                mu_config.end_at = end_until;
                mu_config.no_paid_point = no_paid_coins;
                mu_config.no_xp_point = no_xp_coins;
                mu_config.no_input = true;

                r#impl::musq::download::musq_download(
                    title_id,
                    mu_config,
                    account_id.as_deref(),
                    output.unwrap_or_else(get_default_download_dir),
                    &mut t_mut,
                )
                .await
            }
            cli::MUSQCommands::Balance { account_id } => {
                r#impl::musq::accounts::musq_account_balance(account_id.as_deref(), &t).await
            }
            cli::MUSQCommands::Download {
                title_id,
                chapters,
                show_all,
                auto_purchase,
                quality,
                account_id,
                output,
            } => {
                let mut mu_config = MUDownloadCliConfig::default();
                mu_config.auto_purchase = auto_purchase;
                mu_config.show_all = show_all;
                mu_config.chapter_ids = chapters.unwrap_or_default();
                mu_config.quality = quality;

                r#impl::musq::download::musq_download(
                    title_id,
                    mu_config,
                    account_id.as_deref(),
                    output.unwrap_or_else(get_default_download_dir),
                    &mut t_mut,
                )
                .await
            }
            cli::MUSQCommands::Favorites { account_id } => {
                r#impl::musq::favorites::musq_my_favorites(account_id.as_deref(), &t).await
            }
            cli::MUSQCommands::History { account_id } => {
                r#impl::musq::favorites::musq_my_history(account_id.as_deref(), &t).await
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
            cli::MUSQCommands::Rankings { account_id } => {
                r#impl::musq::rankings::musq_home_rankings(account_id.as_deref(), &t).await
            }
            cli::MUSQCommands::Revoke { account_id } => {
                r#impl::musq::accounts::musq_account_revoke(account_id.as_deref(), &t)
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
            cli::KMKCCommands::AuthMobile {
                user_id,
                hash_key,
                r#type,
            } => {
                r#impl::kmkc::accounts::kmkc_account_login_mobile(user_id, hash_key, r#type, &t)
                    .await
            }
            cli::KMKCCommands::AuthWeb { cookies } => {
                r#impl::kmkc::accounts::kmkc_account_login_web(cookies, &t).await
            }
            cli::KMKCCommands::AuthAdapt { r#type } => {
                r#impl::kmkc::accounts::kmkc_account_login_adapt(r#type, &t).await
            }
            cli::KMKCCommands::Account { account_id } => {
                r#impl::kmkc::accounts::kmkc_account_info(account_id.as_deref(), &t).await
            }
            cli::KMKCCommands::Accounts => r#impl::kmkc::accounts::kmkc_accounts(&t),
            cli::KMKCCommands::AutoDownload {
                title_id,
                no_purchase,
                start_from,
                end_until,
                no_ticket,
                no_point,
                output,
                account_id,
            } => {
                let mut main_config = KMDownloadCliConfig::default();
                main_config.auto_purchase = !no_purchase;
                main_config.start_from = start_from;
                main_config.end_at = end_until;
                main_config.no_ticket = no_ticket;
                main_config.no_point = no_point;
                main_config.no_input = true;

                r#impl::kmkc::download::kmkc_download(
                    title_id,
                    main_config,
                    account_id.as_deref(),
                    output.unwrap_or_else(get_default_download_dir),
                    &mut t_mut,
                )
                .await
            }
            cli::KMKCCommands::Balance { account_id } => {
                r#impl::kmkc::accounts::kmkc_balance(account_id.as_deref(), &t).await
            }
            cli::KMKCCommands::Download {
                title_id,
                chapters,
                show_all,
                auto_purchase,
                account_id,
                output,
            } => {
                let mut main_config = KMDownloadCliConfig::default();
                main_config.auto_purchase = auto_purchase;
                main_config.show_all = show_all;
                main_config.chapter_ids = chapters.unwrap_or_default();

                r#impl::kmkc::download::kmkc_download(
                    title_id,
                    main_config,
                    account_id.as_deref(),
                    output.unwrap_or_else(get_default_download_dir),
                    &mut t_mut,
                )
                .await
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
            cli::KMKCCommands::Magazines { account_id } => {
                r#impl::kmkc::manga::kmkc_magazines_list(account_id.as_deref(), &t).await
            }
            cli::KMKCCommands::Purchase {
                title_id,
                account_id,
            } => {
                r#impl::kmkc::purchases::kmkc_purchase(title_id, account_id.as_deref(), &mut t_mut)
                    .await
            }
            cli::KMKCCommands::Purchased { account_id } => {
                r#impl::kmkc::purchases::kmkc_purchased(account_id.as_deref(), &t).await
            }
            cli::KMKCCommands::Precalculate {
                title_id,
                account_id,
            } => {
                r#impl::kmkc::purchases::kmkc_purchase_precalculate(
                    title_id,
                    account_id.as_deref(),
                    &mut t_mut,
                )
                .await
            }
            cli::KMKCCommands::Rankings {
                account_id,
                ranking_tab,
                limit,
            } => {
                r#impl::kmkc::rankings::kmkc_home_rankings(
                    ranking_tab,
                    account_id.as_deref(),
                    limit,
                    &t,
                )
                .await
            }
            cli::KMKCCommands::Revoke { account_id } => {
                r#impl::kmkc::accounts::kmkc_account_revoke(account_id.as_deref(), &t)
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

    ::std::process::exit(exit_code as i32);
}
