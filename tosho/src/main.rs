use std::path::PathBuf;

use clap::Parser;
use cli::ToshoCommands;
use r#impl::amap::download::AMDownloadCliConfig;
use r#impl::amap::AMAPCommands;
use r#impl::client::select_single_account;
use r#impl::parser::WeeklyCodeCli;
use r#impl::tools::ToolsCommands;
use r#impl::Implementations;
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

    let parsed_proxy = match _cli.proxy {
        Some(proxy) => match reqwest::Proxy::all(proxy) {
            Ok(proxy) => Some(proxy),
            Err(e) => {
                t.warn(&format!("Unable to parse proxy: {}", e));
                std::process::exit(1);
            }
        },
        None => None,
    };

    match _cli.command {
        ToshoCommands::Musq {
            account_id,
            subcommand,
        } => {
            let early_exit = match subcommand.clone() {
                MUSQCommands::Auth { session_id, r#type } => {
                    Some(r#impl::musq::accounts::musq_auth_session(session_id, r#type, &t).await)
                }
                MUSQCommands::Accounts => Some(r#impl::musq::accounts::musq_accounts(&t)),
                _ => None,
            };

            // early exit
            if let Some(early_exit) = early_exit {
                std::process::exit(early_exit as i32);
            }

            let config = select_single_account(account_id.as_deref(), Implementations::Musq, &t);
            let config = match config {
                Some(config) => match config {
                    config::ConfigImpl::Musq(c) => c,
                    _ => unreachable!(),
                },
                None => {
                    t.warn("Aborted!");
                    std::process::exit(1);
                }
            };

            let client = r#impl::client::make_musq_client(&config);
            let client = if let Some(proxy) = parsed_proxy {
                client.with_proxy(proxy)
            } else {
                client
            };

            let exit_code = match subcommand {
                MUSQCommands::Auth {
                    session_id: _,
                    r#type: _,
                } => 0,
                MUSQCommands::Account => {
                    r#impl::musq::accounts::musq_account_info(&client, &config, &t).await
                }
                MUSQCommands::Accounts => 0,
                MUSQCommands::AutoDownload {
                    title_id,
                    no_purchase,
                    start_from,
                    end_until,
                    no_paid_coins,
                    no_xp_coins,
                    quality,
                    output,
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
                        output.unwrap_or_else(get_default_download_dir),
                        &client,
                        &mut t_mut,
                    )
                    .await
                }
                MUSQCommands::Balance => {
                    r#impl::musq::accounts::musq_account_balance(&client, &config, &t).await
                }
                MUSQCommands::Download {
                    title_id,
                    chapters,
                    show_all,
                    auto_purchase,
                    quality,
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
                        output.unwrap_or_else(get_default_download_dir),
                        &client,
                        &mut t_mut,
                    )
                    .await
                }
                MUSQCommands::Favorites => {
                    r#impl::musq::favorites::musq_my_favorites(&client, &config, &t).await
                }
                MUSQCommands::History => {
                    r#impl::musq::favorites::musq_my_history(&client, &config, &t).await
                }
                MUSQCommands::Info {
                    title_id,
                    show_chapters,
                    show_related,
                } => {
                    r#impl::musq::manga::musq_title_info(
                        title_id,
                        show_chapters,
                        show_related,
                        &client,
                        &t,
                    )
                    .await
                }
                MUSQCommands::Purchase { title_id } => {
                    r#impl::musq::purchases::musq_purchase(title_id, &client, &mut t_mut).await
                }
                MUSQCommands::Precalculate { title_id } => {
                    r#impl::musq::purchases::musq_purchase_precalculate(title_id, &client, &t).await
                }
                MUSQCommands::Rankings => {
                    r#impl::musq::rankings::musq_home_rankings(&client, &config, &t).await
                }
                MUSQCommands::Revoke => r#impl::musq::accounts::musq_account_revoke(&config, &t),
                MUSQCommands::Search { query } => {
                    r#impl::musq::manga::musq_search(query.as_str(), &client, &t).await
                }
                MUSQCommands::Weekly { weekday } => {
                    let weekday: WeeklyCode = match weekday {
                        Some(week) => week.into(),
                        None => WeeklyCode::today(),
                    };

                    r#impl::musq::manga::musq_search_weekly(weekday, &client, &t).await
                }
            };

            std::process::exit(exit_code as i32)
        }
        ToshoCommands::Kmkc {
            account_id,
            subcommand,
        } => {
            let early_exit = match subcommand.clone() {
                KMKCCommands::Auth {
                    email,
                    password,
                    r#type,
                } => Some(
                    r#impl::kmkc::accounts::kmkc_account_login(email, password, r#type, &t).await,
                ),
                KMKCCommands::AuthMobile {
                    user_id,
                    hash_key,
                    r#type,
                } => Some(
                    r#impl::kmkc::accounts::kmkc_account_login_mobile(
                        user_id, hash_key, r#type, &t,
                    )
                    .await,
                ),
                KMKCCommands::AuthWeb { cookies } => {
                    Some(r#impl::kmkc::accounts::kmkc_account_login_web(cookies, &t).await)
                }
                KMKCCommands::AuthAdapt { r#type } => {
                    Some(r#impl::kmkc::accounts::kmkc_account_login_adapt(r#type, &t).await)
                }
                KMKCCommands::Accounts => Some(r#impl::kmkc::accounts::kmkc_accounts(&t)),
                _ => None,
            };

            // exit early
            if let Some(exit_code) = early_exit {
                std::process::exit(exit_code as i32);
            }

            let config = select_single_account(account_id.as_deref(), Implementations::Kmkc, &t);
            let config = match config {
                Some(config) => match config {
                    config::ConfigImpl::Kmkc(c) => c,
                    _ => unreachable!(),
                },
                None => {
                    t.warn("Aborted!");
                    std::process::exit(1);
                }
            };

            let client = r#impl::client::make_kmkc_client(&config.clone().into());
            let client = if let Some(proxy) = parsed_proxy {
                client.with_proxy(proxy)
            } else {
                client
            };

            let exit_code = match subcommand {
                KMKCCommands::Auth {
                    email: _,
                    password: _,
                    r#type: _,
                } => 0,
                KMKCCommands::AuthMobile {
                    user_id: _,
                    hash_key: _,
                    r#type: _,
                } => 0,
                KMKCCommands::AuthWeb { cookies: _ } => 0,
                KMKCCommands::AuthAdapt { r#type: _ } => 0,
                KMKCCommands::Account => {
                    r#impl::kmkc::accounts::kmkc_account_info(&client, &config, &t).await
                }
                KMKCCommands::Accounts => 0,
                KMKCCommands::AutoDownload {
                    title_id,
                    no_purchase,
                    start_from,
                    end_until,
                    no_ticket,
                    no_point,
                    output,
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
                        output.unwrap_or_else(get_default_download_dir),
                        &client,
                        &config,
                        &mut t_mut,
                    )
                    .await
                }
                KMKCCommands::Balance => {
                    r#impl::kmkc::accounts::kmkc_balance(&client, &config, &t).await
                }
                KMKCCommands::Download {
                    title_id,
                    chapters,
                    show_all,
                    auto_purchase,
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
                        output.unwrap_or_else(get_default_download_dir),
                        &client,
                        &config,
                        &mut t_mut,
                    )
                    .await
                }
                KMKCCommands::Favorites => {
                    r#impl::kmkc::favorites::kmkc_my_favorites(&client, &config, &t).await
                }
                KMKCCommands::Info {
                    title_id,
                    show_chapters,
                } => {
                    r#impl::kmkc::manga::kmkc_title_info(title_id, show_chapters, &client, &t).await
                }
                KMKCCommands::Magazines => {
                    r#impl::kmkc::manga::kmkc_magazines_list(&client, &t).await
                }
                KMKCCommands::Purchase { title_id } => {
                    r#impl::kmkc::purchases::kmkc_purchase(title_id, &client, &config, &mut t_mut)
                        .await
                }
                KMKCCommands::Purchased => {
                    r#impl::kmkc::purchases::kmkc_purchased(&client, &config, &t).await
                }
                KMKCCommands::Precalculate { title_id } => {
                    r#impl::kmkc::purchases::kmkc_purchase_precalculate(
                        title_id, &client, &config, &mut t_mut,
                    )
                    .await
                }
                KMKCCommands::Rankings { ranking_tab, limit } => {
                    r#impl::kmkc::rankings::kmkc_home_rankings(ranking_tab, limit, &client, &t)
                        .await
                }
                KMKCCommands::Revoke => r#impl::kmkc::accounts::kmkc_account_revoke(&config, &t),
                KMKCCommands::Search { query } => {
                    r#impl::kmkc::manga::kmkc_search(query.as_str(), &client, &t).await
                }
                KMKCCommands::Weekly { weekday } => {
                    let weekday: WeeklyCodeCli = match weekday {
                        Some(week) => week,
                        None => WeeklyCode::today().into(),
                    };

                    r#impl::kmkc::manga::kmkc_search_weekly(weekday, &client, &t).await
                }
            };

            std::process::exit(exit_code as i32)
        }
        ToshoCommands::Amap {
            account_id,
            subcommand,
        } => {
            let early_exit = match subcommand.clone() {
                AMAPCommands::Auth { email, password } => {
                    Some(r#impl::amap::accounts::amap_account_login(email, password, &t).await)
                }
                AMAPCommands::Accounts => Some(r#impl::amap::accounts::amap_accounts(&t)),
                _ => None,
            };

            // early exit
            if let Some(early_exit) = early_exit {
                std::process::exit(early_exit as i32);
            }

            let config = select_single_account(account_id.as_deref(), Implementations::Amap, &t);
            let config = match config {
                Some(config) => match config {
                    config::ConfigImpl::Amap(c) => c,
                    _ => unreachable!(),
                },
                None => {
                    t.warn("Aborted!");
                    std::process::exit(1);
                }
            };

            let client = r#impl::client::make_amap_client(&config.clone().into());
            let client = if let Some(proxy) = parsed_proxy {
                client.with_proxy(proxy)
            } else {
                client
            };

            let exit_code = match subcommand {
                AMAPCommands::Auth {
                    email: _,
                    password: _,
                } => 0,
                AMAPCommands::Account => {
                    r#impl::amap::accounts::amap_account_info(&client, &config, &t).await
                }
                AMAPCommands::Accounts => 0,
                AMAPCommands::AutoDownload {
                    title_id,
                    no_purchase,
                    start_from,
                    end_until,
                    no_paid_ticket,
                    no_premium_ticket,
                    output,
                } => {
                    let dl_config = AMDownloadCliConfig {
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
                        dl_config,
                        output.unwrap_or_else(get_default_download_dir),
                        &client,
                        &config,
                        &mut t_mut,
                    )
                    .await
                }
                AMAPCommands::Balance => {
                    r#impl::amap::accounts::amap_account_balance(&client, &config, &t).await
                }
                AMAPCommands::Discovery => {
                    r#impl::amap::rankings::amap_discovery(&client, &config, &t).await
                }
                AMAPCommands::Download {
                    title_id,
                    chapters,
                    show_all,
                    auto_purchase,
                    output,
                } => {
                    let dl_config = AMDownloadCliConfig {
                        auto_purchase,
                        show_all,
                        chapter_ids: chapters.unwrap_or_default(),
                        ..Default::default()
                    };

                    r#impl::amap::download::amap_download(
                        title_id,
                        dl_config,
                        output.unwrap_or_else(get_default_download_dir),
                        &client,
                        &config,
                        &mut t_mut,
                    )
                    .await
                }
                AMAPCommands::Favorites => {
                    r#impl::amap::favorites::amap_my_favorites(&client, &config, &t).await
                }
                AMAPCommands::Info {
                    title_id,
                    show_chapters,
                } => {
                    r#impl::amap::manga::amap_title_info(title_id, show_chapters, &client, &t).await
                }
                AMAPCommands::Purchase { title_id } => {
                    r#impl::amap::purchases::amap_purchase(title_id, &client, &config, &mut t_mut)
                        .await
                }
                AMAPCommands::Precalculate { title_id } => {
                    r#impl::amap::purchases::amap_purchase_precalculate(
                        title_id, &client, &config, &t,
                    )
                    .await
                }
                AMAPCommands::Revoke => r#impl::amap::accounts::amap_account_revoke(&config, &t),
                AMAPCommands::Search { query } => {
                    r#impl::amap::manga::amap_search(query.as_str(), &client, &config, &t).await
                }
            };

            std::process::exit(exit_code as i32);
        }
        ToshoCommands::Tools { subcommand } => {
            let exit_code = match subcommand {
                ToolsCommands::AutoMerge {
                    input_folder,
                    skip_last,
                } => {
                    let config = r#impl::tools::merger::ToolsMergeConfig {
                        skip_last,
                        no_input: true,
                        ignore_manual_info: true,
                    };

                    r#impl::tools::merger::tools_split_merge(&input_folder, config, &mut t_mut)
                        .await
                }
                ToolsCommands::Merge {
                    input_folder,
                    ignore_manual_merge,
                } => {
                    let config = r#impl::tools::merger::ToolsMergeConfig {
                        ignore_manual_info: ignore_manual_merge,
                        ..Default::default()
                    };

                    r#impl::tools::merger::tools_split_merge(&input_folder, config, &mut t_mut)
                        .await
                }
            };
            std::process::exit(exit_code as i32)
        }
    };
}
