//! # tosho
//!
//! [`tosho-mango`](https://github.com/noaione/tosho-mango) (or `tosho`) is a downloader but can also
//! be considered an almost full-blown replacement for the app/web version, with the exception of
//! currency purchase, as a simple CLI application.
//!
//! Currently we support the following source:
//! - [MU! by SQ](https://crates.io/crates/tosho-musq)
//! - [KM by KC](https://crates.io/crates/tosho-kmkc)
//! - [AM by AP](https://crates.io/crates/tosho-amap)
//! - [SJ/M by V](https://crates.io/crates/tosho-sjv)
//! - [小豆 (Red Bean) by KRKR](https://crates.io/crates/tosho-rbean)
//!
//! ## Installation
//!
//! You can install by cloning the repository then building manually...
//!
//! Or...
//!
//! ```bash
//! $ cargo install tosho
//! ```
//!
//! Or, if you have [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)...
//!
//! ```bash
//! $ cargo binstall tosho
//! ```
//!
//! ## Usage
//!
//! Refer to the [repo](https://github.com/noaione/tosho-mango) on how to authenticate with each source.<br />
//! For a list of available commands, use the `--help` argument.
//!
//! [![asciicast](https://asciinema.org/a/636303.svg)](https://asciinema.org/a/636303)
//!
//! ## Disclaimer
//!
//! This project is designed as an experiment and to create a local copy for personal use.
//! These tools will not circumvent any paywall, and you will need to purchase and own each chapter on each platform
//! with your own account to be able to make your own local copy.
//!
//! We're not responsible if your account got deactivated.
//!
//! ## License
//!
//! This project is licensed with MIT License ([LICENSE](https://github.com/noaione/tosho-mango/blob/master/LICENSE) or <http://opensource.org/licenses/MIT>)

use std::path::PathBuf;

use clap::Parser;
use cli::ToshoCommands;
use r#impl::amap::download::AMDownloadCliConfig;
use r#impl::amap::AMAPCommands;
use r#impl::client::select_single_account;
use r#impl::parser::WeeklyCodeCli;
use r#impl::rbean::download::RBDownloadConfigCli;
use r#impl::rbean::RBeanCommands;
use r#impl::sjv::download::SJDownloadCliConfig;
use r#impl::sjv::SJVCommands;
use r#impl::tools::ToolsCommands;
use r#impl::Implementations;
use r#impl::{kmkc::download::KMDownloadCliConfig, musq::download::MUDownloadCliConfig};
use r#impl::{kmkc::KMKCCommands, musq::MUSQCommands};
use tosho_musq::WeeklyCode;
use updater::check_for_update;

mod cli;
pub(crate) mod config;
pub(crate) mod r#impl;
pub(crate) mod term;
pub(crate) mod updater;
pub(crate) mod win_term;
use crate::cli::ToshoCli;
pub(crate) use term::macros::linkify;

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

    check_for_update(&t).await.unwrap_or_else(|e| {
        t.warn(&format!("Failed to check for update: {}", e));
    });

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
                    parallel,
                } => {
                    let main_config = KMDownloadCliConfig {
                        auto_purchase: !no_purchase,
                        no_input: true,
                        start_from,
                        end_at: end_until,
                        no_point,
                        no_ticket,
                        parallel,
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
                    parallel,
                } => {
                    let main_config = KMDownloadCliConfig {
                        auto_purchase,
                        show_all,
                        chapter_ids: chapters.unwrap_or_default(),
                        parallel,
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
        ToshoCommands::Sjv {
            account_id,
            subcommand,
        } => {
            let early_exit = match subcommand.clone() {
                SJVCommands::Auth {
                    email,
                    password,
                    mode,
                    platform,
                } => Some(
                    r#impl::sjv::accounts::sjv_account_login(email, password, mode, platform, &t)
                        .await,
                ),
                SJVCommands::Accounts => Some(r#impl::sjv::accounts::sjv_accounts(&t)),
                _ => None,
            };

            // early exit
            if let Some(early_exit) = early_exit {
                std::process::exit(early_exit as i32);
            }

            let config = select_single_account(account_id.as_deref(), Implementations::Sjv, &t);
            let config = match config {
                Some(config) => match config {
                    config::ConfigImpl::Sjv(c) => c,
                    _ => unreachable!(),
                },
                None => {
                    t.warn("Aborted!");
                    std::process::exit(1);
                }
            };

            let client = r#impl::client::make_sjv_client(&config.clone());
            let client = if let Some(proxy) = parsed_proxy {
                client.with_proxy(proxy)
            } else {
                client
            };

            let exit_code = match subcommand {
                SJVCommands::Auth {
                    email: _,
                    password: _,
                    mode: _,
                    platform: _,
                } => 0,
                SJVCommands::Account => r#impl::sjv::accounts::sjv_account_info(&config, &t).await,
                SJVCommands::Accounts => 0,
                SJVCommands::AutoDownload {
                    title_or_slug,
                    start_from,
                    end_until,
                    output,
                    parallel,
                } => {
                    let dl_config = SJDownloadCliConfig {
                        start_from,
                        end_at: end_until,
                        no_input: true,
                        parallel,
                        ..Default::default()
                    };

                    r#impl::sjv::download::sjv_download(
                        title_or_slug,
                        dl_config,
                        output.unwrap_or_else(get_default_download_dir),
                        &client,
                        &mut t_mut,
                    )
                    .await
                }
                SJVCommands::Download {
                    title_or_slug,
                    chapters,
                    output,
                    parallel,
                } => {
                    let dl_config = SJDownloadCliConfig {
                        chapter_ids: chapters.unwrap_or_default(),
                        parallel,
                        ..Default::default()
                    };

                    r#impl::sjv::download::sjv_download(
                        title_or_slug,
                        dl_config,
                        output.unwrap_or_else(get_default_download_dir),
                        &client,
                        &mut t_mut,
                    )
                    .await
                }
                SJVCommands::Info {
                    title_or_slug,
                    show_chapters,
                } => {
                    r#impl::sjv::manga::sjv_title_info(title_or_slug, show_chapters, &client, &t)
                        .await
                }
                SJVCommands::Revoke => r#impl::sjv::accounts::sjv_account_revoke(&config, &t),
                SJVCommands::Search { query } => {
                    r#impl::sjv::manga::sjv_search(query.as_str(), &client, &t).await
                }
                SJVCommands::Subscription => {
                    r#impl::sjv::accounts::sjv_account_subscriptions(&client, &config, &t).await
                }
            };

            std::process::exit(exit_code as i32);
        }
        ToshoCommands::Rbean {
            subcommand,
            account_id,
        } => {
            let early_exit = match subcommand.clone() {
                RBeanCommands::Auth {
                    email,
                    password,
                    platform,
                } => Some(
                    r#impl::rbean::accounts::rbean_account_login(email, password, platform, &t)
                        .await,
                ),
                RBeanCommands::Accounts => Some(r#impl::rbean::accounts::rbean_accounts(&t)),
                _ => None,
            };

            // early exit
            if let Some(early_exit) = early_exit {
                std::process::exit(early_exit as i32);
            }

            let config = select_single_account(account_id.as_deref(), Implementations::Rbean, &t);
            let config = match config {
                Some(config) => match config {
                    config::ConfigImpl::Rbean(c) => c,
                    _ => unreachable!(),
                },
                None => {
                    t.warn("Aborted!");
                    std::process::exit(1);
                }
            };

            let client = r#impl::client::make_rbean_client(&config);
            let mut client = if let Some(proxy) = parsed_proxy {
                client.with_proxy(proxy)
            } else {
                client
            };

            client.set_expiry_at(Some(config.expiry));

            let exit_code = match subcommand {
                RBeanCommands::Auth {
                    email: _,
                    password: _,
                    platform: _,
                } => 0,
                RBeanCommands::Account => {
                    r#impl::rbean::accounts::rbean_account_info(&mut client, &config, &t).await
                }
                RBeanCommands::Accounts => 0,
                RBeanCommands::AutoDownload {
                    uuid,
                    output,
                    format,
                    parallel,
                } => {
                    let dl_config = RBDownloadConfigCli {
                        no_input: true,
                        format,
                        parallel,
                        ..Default::default()
                    };
                    r#impl::rbean::download::rbean_download(
                        &uuid,
                        dl_config,
                        output.unwrap_or_else(get_default_download_dir),
                        &mut client,
                        &config,
                        &mut t_mut,
                    )
                    .await
                }
                RBeanCommands::Download {
                    uuid,
                    chapters,
                    output,
                    format,
                    parallel,
                } => {
                    let dl_config = RBDownloadConfigCli {
                        format,
                        chapter_ids: chapters.unwrap_or_default(),
                        parallel,
                        ..Default::default()
                    };
                    r#impl::rbean::download::rbean_download(
                        &uuid,
                        dl_config,
                        output.unwrap_or_else(get_default_download_dir),
                        &mut client,
                        &config,
                        &mut t_mut,
                    )
                    .await
                }
                RBeanCommands::Homepage => {
                    r#impl::rbean::rankings::rbean_home_page(&mut client, &config, &t).await
                }
                RBeanCommands::Info {
                    uuid,
                    show_chapters,
                } => {
                    r#impl::rbean::manga::rbean_title_info(
                        &uuid,
                        show_chapters,
                        &mut client,
                        &config,
                        &t,
                    )
                    .await
                }
                RBeanCommands::ReadList => {
                    r#impl::rbean::favorites::rbean_read_list(&mut client, &config, &t).await
                }
                RBeanCommands::Revoke => r#impl::rbean::accounts::rbean_account_revoke(&config, &t),
                RBeanCommands::Search { query, limit, sort } => {
                    r#impl::rbean::manga::rbean_search(
                        &query,
                        limit,
                        sort,
                        &mut client,
                        &config,
                        &t,
                    )
                    .await
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
        ToshoCommands::Update => {
            updater::perform_update(&t).await.unwrap_or_else(|e| {
                t.error(&format!("Failed to update: {}", e));
                std::process::exit(1);
            });

            std::process::exit(0)
        }
    };
}
