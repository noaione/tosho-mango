use std::path::PathBuf;

use clap::Parser;
use cli::ToshoCommands;
use r#impl::amap::download::AMDownloadCliConfig;
use r#impl::amap::AMAPCommands;
use r#impl::parser::WeeklyCodeCli;
use r#impl::tools::ToolsCommands;
use r#impl::{kmkc::download::KMDownloadCliConfig, musq::download::MUDownloadCliConfig};
use r#impl::{kmkc::KMKCCommands, musq::MUSQCommands};
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
            MUSQCommands::Auth { session_id, r#type } => {
                r#impl::musq::accounts::musq_auth_session(session_id, r#type, &t).await
            }
            MUSQCommands::Account { account_id } => {
                r#impl::musq::accounts::musq_account_info(account_id.as_deref(), &t).await
            }
            MUSQCommands::Accounts => r#impl::musq::accounts::musq_accounts(&t),
            MUSQCommands::AutoDownload {
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
                let mu_config = MUDownloadCliConfig {
                    auto_purchase: !no_purchase,
                    no_input: true,
                    quality,
                    start_from,
                    end_at: end_until,
                    no_paid_point: no_paid_coins,
                    no_xp_point: no_xp_coins,
                    ..Default::default()
                };

                r#impl::musq::download::musq_download(
                    title_id,
                    mu_config,
                    account_id.as_deref(),
                    output.unwrap_or_else(get_default_download_dir),
                    &mut t_mut,
                )
                .await
            }
            MUSQCommands::Balance { account_id } => {
                r#impl::musq::accounts::musq_account_balance(account_id.as_deref(), &t).await
            }
            MUSQCommands::Download {
                title_id,
                chapters,
                show_all,
                auto_purchase,
                quality,
                account_id,
                output,
            } => {
                let mu_config = MUDownloadCliConfig {
                    auto_purchase,
                    show_all,
                    chapter_ids: chapters.unwrap_or_default(),
                    quality,
                    ..Default::default()
                };

                r#impl::musq::download::musq_download(
                    title_id,
                    mu_config,
                    account_id.as_deref(),
                    output.unwrap_or_else(get_default_download_dir),
                    &mut t_mut,
                )
                .await
            }
            MUSQCommands::Favorites { account_id } => {
                r#impl::musq::favorites::musq_my_favorites(account_id.as_deref(), &t).await
            }
            MUSQCommands::History { account_id } => {
                r#impl::musq::favorites::musq_my_history(account_id.as_deref(), &t).await
            }
            MUSQCommands::Info {
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
            MUSQCommands::Purchase {
                title_id,
                account_id,
            } => {
                r#impl::musq::purchases::musq_purchase(title_id, account_id.as_deref(), &mut t_mut)
                    .await
            }
            MUSQCommands::Precalculate {
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
            MUSQCommands::Rankings { account_id } => {
                r#impl::musq::rankings::musq_home_rankings(account_id.as_deref(), &t).await
            }
            MUSQCommands::Revoke { account_id } => {
                r#impl::musq::accounts::musq_account_revoke(account_id.as_deref(), &t)
            }
            MUSQCommands::Search { query, account_id } => {
                r#impl::musq::manga::musq_search(query.as_str(), account_id.as_deref(), &t).await
            }
            MUSQCommands::Weekly {
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
            KMKCCommands::Auth {
                email,
                password,
                r#type,
            } => r#impl::kmkc::accounts::kmkc_account_login(email, password, r#type, &t).await,
            KMKCCommands::AuthMobile {
                user_id,
                hash_key,
                r#type,
            } => {
                r#impl::kmkc::accounts::kmkc_account_login_mobile(user_id, hash_key, r#type, &t)
                    .await
            }
            KMKCCommands::AuthWeb { cookies } => {
                r#impl::kmkc::accounts::kmkc_account_login_web(cookies, &t).await
            }
            KMKCCommands::AuthAdapt { r#type } => {
                r#impl::kmkc::accounts::kmkc_account_login_adapt(r#type, &t).await
            }
            KMKCCommands::Account { account_id } => {
                r#impl::kmkc::accounts::kmkc_account_info(account_id.as_deref(), &t).await
            }
            KMKCCommands::Accounts => r#impl::kmkc::accounts::kmkc_accounts(&t),
            KMKCCommands::AutoDownload {
                title_id,
                no_purchase,
                start_from,
                end_until,
                no_ticket,
                no_point,
                output,
                account_id,
            } => {
                let main_config = KMDownloadCliConfig {
                    auto_purchase: !no_purchase,
                    no_input: true,
                    start_from,
                    end_at: end_until,
                    no_point,
                    no_ticket,
                    ..Default::default()
                };

                r#impl::kmkc::download::kmkc_download(
                    title_id,
                    main_config,
                    account_id.as_deref(),
                    output.unwrap_or_else(get_default_download_dir),
                    &mut t_mut,
                )
                .await
            }
            KMKCCommands::Balance { account_id } => {
                r#impl::kmkc::accounts::kmkc_balance(account_id.as_deref(), &t).await
            }
            KMKCCommands::Download {
                title_id,
                chapters,
                show_all,
                auto_purchase,
                account_id,
                output,
            } => {
                let main_config = KMDownloadCliConfig {
                    auto_purchase,
                    show_all,
                    chapter_ids: chapters.unwrap_or_default(),
                    ..Default::default()
                };

                r#impl::kmkc::download::kmkc_download(
                    title_id,
                    main_config,
                    account_id.as_deref(),
                    output.unwrap_or_else(get_default_download_dir),
                    &mut t_mut,
                )
                .await
            }
            KMKCCommands::Info {
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
            KMKCCommands::Magazines { account_id } => {
                r#impl::kmkc::manga::kmkc_magazines_list(account_id.as_deref(), &t).await
            }
            KMKCCommands::Purchase {
                title_id,
                account_id,
            } => {
                r#impl::kmkc::purchases::kmkc_purchase(title_id, account_id.as_deref(), &mut t_mut)
                    .await
            }
            KMKCCommands::Purchased { account_id } => {
                r#impl::kmkc::purchases::kmkc_purchased(account_id.as_deref(), &t).await
            }
            KMKCCommands::Precalculate {
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
            KMKCCommands::Rankings {
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
            KMKCCommands::Revoke { account_id } => {
                r#impl::kmkc::accounts::kmkc_account_revoke(account_id.as_deref(), &t)
            }
            KMKCCommands::Search { query, account_id } => {
                r#impl::kmkc::manga::kmkc_search(query.as_str(), account_id.as_deref(), &t).await
            }
            KMKCCommands::Weekly {
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
        ToshoCommands::Amap { subcommand } => match subcommand {
            AMAPCommands::Auth { email, password } => {
                r#impl::amap::accounts::amap_account_login(email, password, &t).await
            }
            AMAPCommands::Account { account_id } => {
                r#impl::amap::accounts::amap_account_info(account_id.as_deref(), &t).await
            }
            AMAPCommands::Accounts => r#impl::amap::accounts::amap_accounts(&t),
            AMAPCommands::AutoDownload {
                title_id,
                no_purchase,
                start_from,
                end_until,
                no_paid_ticket,
                no_premium_ticket,
                output,
                account_id,
            } => {
                let config = AMDownloadCliConfig {
                    auto_purchase: !no_purchase,
                    no_input: true,
                    start_from,
                    end_at: end_until,
                    no_premium: no_paid_ticket,
                    no_purchased: no_premium_ticket,
                    ..Default::default()
                };

                r#impl::amap::download::amap_download(
                    title_id,
                    config,
                    account_id.as_deref(),
                    output.unwrap_or_else(get_default_download_dir),
                    &mut t_mut,
                )
                .await
            }
            AMAPCommands::Balance { account_id } => {
                r#impl::amap::accounts::amap_account_balance(account_id.as_deref(), &t).await
            }
            AMAPCommands::Discovery { account_id } => {
                r#impl::amap::rankings::amap_discovery(account_id.as_deref(), &t).await
            }
            AMAPCommands::Download {
                title_id,
                chapters,
                show_all,
                auto_purchase,
                output,
                account_id,
            } => {
                let config = AMDownloadCliConfig {
                    auto_purchase,
                    show_all,
                    chapter_ids: chapters.unwrap_or_default(),
                    ..Default::default()
                };

                r#impl::amap::download::amap_download(
                    title_id,
                    config,
                    account_id.as_deref(),
                    output.unwrap_or_else(get_default_download_dir),
                    &mut t_mut,
                )
                .await
            }
            AMAPCommands::Info {
                title_id,
                account_id,
                show_chapters,
            } => {
                r#impl::amap::manga::amap_title_info(
                    title_id,
                    account_id.as_deref(),
                    show_chapters,
                    &t,
                )
                .await
            }
            AMAPCommands::Purchase {
                title_id,
                account_id,
            } => {
                r#impl::amap::purchases::amap_purchase(title_id, account_id.as_deref(), &mut t_mut)
                    .await
            }
            AMAPCommands::Precalculate {
                title_id,
                account_id,
            } => {
                r#impl::amap::purchases::amap_purchase_precalculate(
                    title_id,
                    account_id.as_deref(),
                    &t,
                )
                .await
            }
            AMAPCommands::Revoke { account_id } => {
                r#impl::amap::accounts::amap_account_revoke(account_id.as_deref(), &t)
            }
            AMAPCommands::Search { query, account_id } => {
                r#impl::amap::manga::amap_search(query.as_str(), account_id.as_deref(), &t).await
            }
        },
        ToshoCommands::Tools { subcommand } => match subcommand {
            ToolsCommands::AutoMerge {
                input_folder,
                skip_last,
            } => {
                let config = r#impl::tools::merger::ToolsMergeConfig {
                    skip_last,
                    no_input: true,
                    ignore_manual_info: true,
                };

                r#impl::tools::merger::tools_split_merge(&input_folder, config, &mut t_mut).await
            }
            ToolsCommands::Merge {
                input_folder,
                ignore_manual_merge,
            } => {
                let config = r#impl::tools::merger::ToolsMergeConfig {
                    ignore_manual_info: ignore_manual_merge,
                    ..Default::default()
                };

                r#impl::tools::merger::tools_split_merge(&input_folder, config, &mut t_mut).await
            }
        },
    };

    ::std::process::exit(exit_code as i32);
}
