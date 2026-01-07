//! # tosho
//!
//! ![crates.io version](https://img.shields.io/crates/v/tosho) ![CI](https://github.com/noaione/tosho-mango/actions/workflows/ci.yml/badge.svg)
//!
//! [`tosho-mango`](https://github.com/noaione/tosho-mango) (or `tosho`) is a manga downloader and general CLI tools for official licensor platform.
//!
//! Currently we support the following source:
//! - [MU! by SQ](https://crates.io/crates/tosho-musq)
//! - [KM by KC](https://crates.io/crates/tosho-kmkc)
//! - [AM by AP](https://crates.io/crates/tosho-amap)
//! - [SJ/M by V](https://crates.io/crates/tosho-sjv)
//! - [小豆 (Red Bean) by KRKR](https://crates.io/crates/tosho-rbean)
//! - [M+ by S](https://crates.io/crates/tosho-mplus)
//!
//! ## Installation
//!
//! You can install by cloning the repository then building manually...
//!
//! Or...
//!
//! ```bash
//! cargo install --locked tosho
//! ```
//!
//! Or, if you have [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)...
//!
//! ```bash
//! cargo binstall --locked tosho
//! ```
//!
//! We also provide a pre-built binary in two flavours:
//! - **Stable** release in the **[GitHub Releases](https://github.com/noaione/tosho-mango/releases)** tab.
//! - **Nightly** release from any latest successful commits: [Master CI](https://github.com/noaione/tosho-mango/actions/workflows/ci.yml?query=branch%3Amaster) / [nightly.link](https://nightly.link/noaione/tosho-mango/workflows/ci/master?preview).
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

use crate::cli::{ToshoCli, max_threads};
use clap::Parser;
use cli::ToshoCommands;
use color_eyre::eyre::Context;
use r#impl::Implementations;
use r#impl::amap::AMAPCommands;
use r#impl::amap::download::AMDownloadCliConfig;
use r#impl::client::select_single_account;
use r#impl::mplus::MPlusCommands;
use r#impl::mplus::download::MPDownloadCliConfig;
use r#impl::nids::NIDSCommands;
use r#impl::nids::download::NIDownloadCliConfig;
use r#impl::parser::WeeklyCodeCli;
use r#impl::rbean::RBeanCommands;
use r#impl::rbean::download::RBDownloadConfigCli;
use r#impl::sjv::SJVCommands;
use r#impl::sjv::download::SJDownloadCliConfig;
use r#impl::tools::ToolsCommands;
use r#impl::{kmkc::KMKCCommands, musq::MUSQCommands};
use r#impl::{kmkc::download::KMDownloadCliConfig, musq::download::MUDownloadCliConfig};
use tosho_musq::WeeklyCode;

mod cli;
pub(crate) mod config;
pub(crate) mod r#impl;
pub(crate) mod term;
#[cfg(feature = "with-updater")]
pub(crate) mod updater;
pub(crate) mod win_term;
pub(crate) use term::macros::linkify;

fn get_default_download_dir() -> color_eyre::Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    Ok(cwd.join("DOWNLOADS"))
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    // For some god know what reason, `clap` + rustc_lint will show this as unreachable code.
    let _cli = ToshoCli::parse();

    entrypoint(_cli).await?;

    Ok(())
}

async fn entrypoint(cli: ToshoCli) -> color_eyre::Result<()> {
    let t = term::get_console(cli.verbose);
    let mut t_mut = term::get_console(cli.verbose);

    let parsed_proxy = match cli.proxy {
        Some(proxy) => match reqwest::Proxy::all(proxy) {
            Ok(proxy) => Some(proxy),
            Err(e) => {
                t.warn(format!("Unable to parse proxy: {e}"));
                return Ok(());
            }
        },
        None => None,
    };

    #[cfg(feature = "with-updater")]
    {
        crate::updater::check_for_update(&t)
            .await
            .unwrap_or_else(|e| {
                t.warn(format!("Failed to check for update: {e}"));
            });
    }

    let default_dir = get_default_download_dir()?;

    match cli.command {
        ToshoCommands::Musq {
            account_id,
            subcommand,
        } => {
            let early_act = match subcommand.clone() {
                MUSQCommands::Auth { session_id, r#type } => {
                    Some(r#impl::musq::accounts::musq_auth_session(session_id, r#type, &t).await)
                }
                MUSQCommands::Accounts => Some(r#impl::musq::accounts::musq_accounts(&t)),
                _ => None,
            };

            // early exit
            if let Some(early_act) = early_act {
                return early_act;
            }

            let config = select_single_account(account_id.as_deref(), Implementations::Musq, &t);
            let config = match config {
                Some(config) => match config {
                    config::ConfigImpl::Musq(c) => c,
                    _ => unreachable!(),
                },
                None => {
                    t.warn("Aborted!");
                    return Err(color_eyre::eyre::eyre!("Aborted by user"));
                }
            };

            let client = r#impl::client::make_musq_client(&config)?;
            let client = if let Some(proxy) = parsed_proxy {
                client.with_proxy(proxy)?
            } else {
                client
            };

            let early_act = match subcommand {
                MUSQCommands::Auth {
                    session_id: _,
                    r#type: _,
                } => Ok(()),
                MUSQCommands::Account => {
                    r#impl::musq::accounts::musq_account_info(&client, &config, &t).await
                }
                MUSQCommands::Accounts => Ok(()),
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
                        output.unwrap_or(default_dir),
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
                        output.unwrap_or(default_dir),
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

            early_act
        }
        ToshoCommands::Kmkc {
            account_id,
            subcommand,
        } => {
            let exit_stat = match subcommand.clone() {
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
            if let Some(exit_stat) = exit_stat {
                return exit_stat;
            }

            let config = select_single_account(account_id.as_deref(), Implementations::Kmkc, &t);
            let config = match config {
                Some(config) => match config {
                    config::ConfigImpl::Kmkc(c) => c,
                    _ => unreachable!(),
                },
                None => {
                    t.warn("Aborted!");
                    return Err(color_eyre::eyre::eyre!("Aborted by user"));
                }
            };

            let client = r#impl::client::make_kmkc_client(
                &config
                    .clone()
                    .try_into()
                    .context("Failed to convert client config")?,
            )?;
            let client = if let Some(proxy) = parsed_proxy {
                client.with_proxy(proxy)?
            } else {
                client
            };

            let exit_stat = match subcommand {
                KMKCCommands::Auth {
                    email: _,
                    password: _,
                    r#type: _,
                } => Ok(()),
                KMKCCommands::AuthMobile {
                    user_id: _,
                    hash_key: _,
                    r#type: _,
                } => Ok(()),
                KMKCCommands::AuthWeb { cookies: _ } => Ok(()),
                KMKCCommands::AuthAdapt { r#type: _ } => Ok(()),
                KMKCCommands::Account => {
                    r#impl::kmkc::accounts::kmkc_account_info(&client, &config, &t).await
                }
                KMKCCommands::Accounts => Ok(()),
                KMKCCommands::AutoDownload {
                    title_id,
                    no_purchase,
                    start_from,
                    end_until,
                    no_ticket,
                    no_point,
                    output,
                    parallel,
                    threads,
                } => {
                    let main_config = KMDownloadCliConfig {
                        auto_purchase: !no_purchase,
                        no_input: true,
                        start_from,
                        end_at: end_until,
                        no_point,
                        no_ticket,
                        parallel,
                        threads: max_threads(threads),
                        ..Default::default()
                    };

                    r#impl::kmkc::download::kmkc_download(
                        title_id,
                        main_config,
                        output.unwrap_or(default_dir),
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
                    threads,
                } => {
                    let main_config = KMDownloadCliConfig {
                        auto_purchase,
                        show_all,
                        chapter_ids: chapters.unwrap_or_default(),
                        parallel,
                        threads: max_threads(threads),
                        ..Default::default()
                    };

                    r#impl::kmkc::download::kmkc_download(
                        title_id,
                        main_config,
                        output.unwrap_or(default_dir),
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

            exit_stat
        }
        ToshoCommands::Amap {
            account_id,
            subcommand,
        } => {
            let exit_res = match subcommand.clone() {
                AMAPCommands::Auth { email, password } => {
                    Some(r#impl::amap::accounts::amap_account_login(email, password, &t).await)
                }
                AMAPCommands::Accounts => Some(r#impl::amap::accounts::amap_accounts(&t)),
                _ => None,
            };

            // early exit
            if let Some(exit_res) = exit_res {
                return exit_res; // Propagate error
            }

            let config = select_single_account(account_id.as_deref(), Implementations::Amap, &t);
            let config = match config {
                Some(config) => match config {
                    config::ConfigImpl::Amap(c) => c,
                    _ => unreachable!(),
                },
                None => {
                    t.warn("Aborted!");
                    return Err(color_eyre::eyre::eyre!("Aborted by user"));
                }
            };

            let client = r#impl::client::make_amap_client(&config.clone().into())?;
            let client = if let Some(proxy) = parsed_proxy {
                client.with_proxy(proxy)?
            } else {
                client
            };

            let exit_res = match subcommand {
                AMAPCommands::Auth {
                    email: _,
                    password: _,
                } => Ok(()),
                AMAPCommands::Account => {
                    r#impl::amap::accounts::amap_account_info(&client, &config, &t).await
                }
                AMAPCommands::Accounts => Ok(()),
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
                        output.unwrap_or(default_dir),
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
                        output.unwrap_or(default_dir),
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

            exit_res
        }
        ToshoCommands::Sjv {
            account_id,
            subcommand,
        } => {
            let early_act = match subcommand.clone() {
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
            if let Some(early_act) = early_act {
                return early_act;
            }

            let config = select_single_account(account_id.as_deref(), Implementations::Sjv, &t);
            let config = match config {
                Some(config) => match config {
                    config::ConfigImpl::Sjv(c) => c,
                    _ => unreachable!(),
                },
                None => {
                    t.warn("Aborted!");
                    return Err(color_eyre::eyre::eyre!("Aborted by user"));
                }
            };

            let client = r#impl::client::make_sjv_client(&config.clone())?;
            let client = if let Some(proxy) = parsed_proxy {
                client.with_proxy(proxy)?
            } else {
                client
            };

            let exit_act = match subcommand {
                SJVCommands::Auth {
                    email: _,
                    password: _,
                    mode: _,
                    platform: _,
                } => Ok(()),
                SJVCommands::Account => r#impl::sjv::accounts::sjv_account_info(&config, &t).await,
                SJVCommands::Accounts => Ok(()),
                SJVCommands::AutoDownload {
                    title_or_slug,
                    start_from,
                    end_until,
                    output,
                    parallel,
                    threads,
                } => {
                    let dl_config = SJDownloadCliConfig {
                        start_from,
                        end_at: end_until,
                        no_input: true,
                        parallel,
                        threads: max_threads(threads),
                        ..Default::default()
                    };

                    r#impl::sjv::download::sjv_download(
                        title_or_slug,
                        dl_config,
                        output.unwrap_or(default_dir),
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
                    threads,
                } => {
                    let dl_config = SJDownloadCliConfig {
                        chapter_ids: chapters.unwrap_or_default(),
                        parallel,
                        threads: max_threads(threads),
                        ..Default::default()
                    };

                    r#impl::sjv::download::sjv_download(
                        title_or_slug,
                        dl_config,
                        output.unwrap_or(default_dir),
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

            exit_act
        }
        ToshoCommands::Rbean {
            subcommand,
            account_id,
        } => {
            let early_act = match subcommand.clone() {
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
            if let Some(early_act) = early_act {
                return early_act;
            }

            let config = select_single_account(account_id.as_deref(), Implementations::Rbean, &t);
            let config = match config {
                Some(config) => match config {
                    config::ConfigImpl::Rbean(c) => c,
                    _ => unreachable!(),
                },
                None => {
                    t.warn("Aborted!");
                    return Err(color_eyre::eyre::eyre!("Aborted by user"));
                }
            };

            let client = r#impl::client::make_rbean_client(&config)?;
            let mut client = if let Some(proxy) = parsed_proxy {
                client.with_proxy(proxy)?
            } else {
                client
            };

            client.set_expiry_at(Some(config.expiry));

            let exit_act = match subcommand {
                RBeanCommands::Auth {
                    email: _,
                    password: _,
                    platform: _,
                } => Ok(()),
                RBeanCommands::Account => {
                    r#impl::rbean::accounts::rbean_account_info(&mut client, &config, &t).await
                }
                RBeanCommands::Accounts => Ok(()),
                RBeanCommands::AutoDownload {
                    uuid,
                    output,
                    format,
                    quality,
                    parallel,
                    threads,
                } => {
                    let dl_config = RBDownloadConfigCli {
                        no_input: true,
                        format,
                        parallel,
                        quality,
                        threads: max_threads(threads),
                        ..Default::default()
                    };
                    r#impl::rbean::download::rbean_download(
                        &uuid,
                        dl_config,
                        output.unwrap_or(default_dir),
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
                    quality,
                    parallel,
                    threads,
                } => {
                    let dl_config = RBDownloadConfigCli {
                        format,
                        chapter_ids: chapters.unwrap_or_default(),
                        parallel,
                        quality,
                        threads: max_threads(threads),
                        ..Default::default()
                    };
                    r#impl::rbean::download::rbean_download(
                        &uuid,
                        dl_config,
                        output.unwrap_or(default_dir),
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

            exit_act
        }
        ToshoCommands::Mplus {
            account_id,
            language,
            subcommand,
            app_version,
        } => {
            let early_stat = match subcommand.clone() {
                MPlusCommands::Auth { session_id, r#type } => {
                    Some(r#impl::mplus::accounts::mplus_auth_session(session_id, r#type, &t).await)
                }
                MPlusCommands::Accounts => Some(r#impl::mplus::accounts::mplus_accounts(&t)),
                _ => None,
            };

            // early exit
            if let Some(early_stat) = early_stat {
                return early_stat;
            }

            let config = select_single_account(account_id.as_deref(), Implementations::Mplus, &t);
            let config = match config {
                Some(config) => match config {
                    config::ConfigImpl::Mplus(c) => c,
                    _ => unreachable!(),
                },
                None => {
                    t.warn("Aborted!");
                    return Ok(());
                }
            };

            let client =
                r#impl::client::make_mplus_client(&config, language.unwrap_or_default().into())?;
            let client = if let Some(proxy) = parsed_proxy {
                client.with_proxy(proxy)?
            } else {
                client
            }
            .with_app_version(app_version);

            let exit_stat = match subcommand {
                MPlusCommands::Auth {
                    session_id: _,
                    r#type: _,
                } => Ok(()),
                MPlusCommands::Account => {
                    r#impl::mplus::accounts::mplus_account_info(&client, &config, &t).await
                }
                MPlusCommands::Accounts => Ok(()),
                MPlusCommands::AutoDownload {
                    title_id,
                    start_from,
                    end_until,
                    quality,
                    output,
                } => {
                    let mplus_config = MPDownloadCliConfig {
                        no_input: true,
                        start_from,
                        end_at: end_until,
                        quality,
                        ..Default::default()
                    };

                    r#impl::mplus::download::mplus_download(
                        title_id,
                        mplus_config,
                        output.unwrap_or(default_dir),
                        &client,
                        &mut t_mut,
                    )
                    .await
                }
                MPlusCommands::Download {
                    title_id,
                    chapters,
                    show_all,
                    quality,
                    output,
                } => {
                    let mplus_config = MPDownloadCliConfig {
                        show_all,
                        chapter_ids: chapters.unwrap_or_default(),
                        quality,
                        ..Default::default()
                    };

                    r#impl::mplus::download::mplus_download(
                        title_id,
                        mplus_config,
                        output.unwrap_or(default_dir),
                        &client,
                        &mut t_mut,
                    )
                    .await
                }
                MPlusCommands::Favorites => {
                    r#impl::mplus::favorites::mplus_my_favorites(&client, &config, &t).await
                }
                MPlusCommands::Info {
                    title_id,
                    show_chapters,
                    show_related,
                } => {
                    r#impl::mplus::manga::mplus_title_info(
                        title_id,
                        show_chapters,
                        show_related,
                        &client,
                        &t,
                    )
                    .await
                }
                MPlusCommands::Rankings { kind } => {
                    r#impl::mplus::rankings::mplus_home_rankings(kind, &client, &t).await
                }
                MPlusCommands::Revoke => r#impl::mplus::accounts::mplus_account_revoke(&config, &t),
                MPlusCommands::Search { query } => {
                    r#impl::mplus::manga::mplus_search(query.as_str(), &client, &t).await
                }
            };

            exit_stat
        }
        ToshoCommands::Nids {
            account_id,
            subcommand,
        } => {
            let clean_client =
                tosho_nids::NIClient::new(None, tosho_nids::constants::get_constants(1))?;
            let clean_client = if let Some(proxy) = &parsed_proxy {
                clean_client.with_proxy(proxy.clone())?
            } else {
                clean_client
            };
            let early_act = match subcommand.clone() {
                NIDSCommands::Auth {
                    email,
                    password,
                    r#type,
                } => Some(
                    r#impl::nids::accounts::nids_auth_email(
                        email,
                        password,
                        r#type,
                        parsed_proxy.as_ref(),
                        &t,
                    )
                    .await,
                ),
                NIDSCommands::AuthToken {
                    session_token,
                    r#type,
                } => {
                    Some(r#impl::nids::accounts::nids_auth_session(session_token, r#type, &t).await)
                }
                NIDSCommands::Accounts => Some(r#impl::nids::accounts::nids_accounts(&t)),
                NIDSCommands::Issue {
                    issue_id,
                    with_marketplace,
                } => Some(
                    r#impl::nids::issues::nids_get_issue(
                        issue_id,
                        with_marketplace,
                        &clean_client,
                        &t,
                    )
                    .await,
                ),
                NIDSCommands::Issues {
                    filters,
                    limit,
                    sort_by,
                    direction,
                    scope,
                } => {
                    let base_filter = tosho_nids::Filter::new()
                        .with_per_page(limit)
                        .with_order(sort_by, direction.into());
                    let merged_filters = filters
                        .unwrap_or_default()
                        .into_iter()
                        .fold(base_filter, |acc, (filt_type, filt_data)| {
                            acc.add_filter(filt_type, filt_data)
                        });
                    let mut full_filters = match scope {
                        // For on-sale, we don't need to add any date filters
                        Some(r#impl::nids::common::FilterScopeInput::OnSale) => merged_filters,
                        Some(r#impl::nids::common::FilterScopeInput::NewReleases) => merged_filters,
                        Some(r#impl::nids::common::FilterScopeInput::BestSelling) => merged_filters,
                        Some(s) => {
                            let with_scope = merged_filters.with_scope(s.into());
                            // add release_date_start and release_date_end manually
                            let (start_time, end_time) =
                                crate::r#impl::nids::common::get_scope_dates()?;
                            if with_scope.has_filter(&tosho_nids::FilterType::ReleaseDateStart)
                                || with_scope.has_filter(&tosho_nids::FilterType::ReleaseDateEnd)
                            {
                                // if user already specify either one of the date filters, we won't override it
                                with_scope
                            } else {
                                with_scope
                                    .add_filter(
                                        tosho_nids::FilterType::ReleaseDateStart,
                                        start_time,
                                    )
                                    .add_filter(tosho_nids::FilterType::ReleaseDateEnd, end_time)
                            }
                        }
                        None => merged_filters,
                    };

                    Some(
                        r#impl::nids::issues::nids_get_issues(&mut full_filters, &clean_client, &t)
                            .await,
                    )
                }
                NIDSCommands::Marketplace {
                    limit,
                    direction,
                    grouped,
                } => {
                    let base_filter = tosho_nids::Filter::new().with_per_page(limit);
                    let mut base_filter = if grouped {
                        base_filter
                            .with_order(tosho_nids::SortBy::EditionPriceMin, direction.into())
                    } else {
                        base_filter
                            .with_order(tosho_nids::SortBy::MarketplacePrice, direction.into())
                    };

                    let exit_code = if grouped {
                        r#impl::nids::marketplace::nids_get_marketplace_grouped(
                            &mut base_filter,
                            &clean_client,
                            &t,
                        )
                        .await
                    } else {
                        r#impl::nids::marketplace::nids_get_marketplace_ungrouped(
                            &mut base_filter,
                            &clean_client,
                            &t,
                        )
                        .await
                    };
                    Some(exit_code)
                }
                NIDSCommands::Publisher {
                    publisher_slug,
                    with_imprints,
                } => Some(
                    r#impl::nids::publishers::nids_get_publisher(
                        &publisher_slug,
                        with_imprints,
                        &clean_client,
                        &t,
                    )
                    .await,
                ),
                NIDSCommands::Publishers => {
                    Some(r#impl::nids::publishers::nids_get_publishers(&clean_client, &t).await)
                }
                NIDSCommands::SeriesRun {
                    series_run_id,
                    with_marketplace,
                } => Some(
                    r#impl::nids::series::nids_get_series_info(
                        series_run_id,
                        with_marketplace,
                        &clean_client,
                        &t,
                    )
                    .await,
                ),
                NIDSCommands::SeriesRuns {
                    filters,
                    limit,
                    sort_by,
                    direction,
                } => {
                    let base_filter = tosho_nids::Filter::new()
                        .with_per_page(limit)
                        .with_order(sort_by, direction.into());
                    let mut merged_filters = filters
                        .unwrap_or_default()
                        .into_iter()
                        .fold(base_filter, |acc, (filt_type, filt_data)| {
                            acc.add_filter(filt_type, filt_data)
                        });
                    Some(
                        r#impl::nids::series::nids_get_series(
                            &mut merged_filters,
                            &clean_client,
                            &t,
                        )
                        .await,
                    )
                }
                _ => None,
            };

            // early exit
            if let Some(early_act) = early_act {
                drop(clean_client);
                return early_act;
            }

            let config = select_single_account(account_id.as_deref(), Implementations::Nids, &t);
            let config = match config {
                Some(config) => match config {
                    config::ConfigImpl::Nids(c) => c,
                    _ => unreachable!(),
                },
                None => {
                    t.warn("Aborted!");
                    return Err(color_eyre::eyre::eyre!("Aborted by user"));
                }
            };

            let client = r#impl::client::make_nids_client(&config)?;
            let client = if let Some(proxy) = parsed_proxy {
                client.with_proxy(proxy)?
            } else {
                client
            };

            let exit_act = match subcommand {
                NIDSCommands::Auth { .. } => Ok(()),
                NIDSCommands::AuthToken { .. } => Ok(()),
                NIDSCommands::Account => {
                    r#impl::nids::accounts::nids_account_info(&client, &config, &t).await
                }
                NIDSCommands::Accounts => Ok(()),
                NIDSCommands::Download {
                    issue_id,
                    output,
                    parallel,
                    threads,
                    report,
                    quality,
                } => {
                    let dl_config = NIDownloadCliConfig {
                        output,
                        parallel,
                        threads: max_threads(threads),
                        report,
                        quality,
                    };

                    r#impl::nids::download::nids_download(
                        issue_id,
                        dl_config,
                        get_default_download_dir()?,
                        &client,
                        &mut t_mut,
                    )
                    .await
                }
                NIDSCommands::Issue { .. } => Ok(()),
                NIDSCommands::Issues { .. } => Ok(()),
                NIDSCommands::Marketplace { .. } => Ok(()),
                NIDSCommands::Publisher { .. } => Ok(()),
                NIDSCommands::Publishers => Ok(()),
                NIDSCommands::PurchasedIssues {
                    series_run_uuid,
                    limit,
                } => {
                    let mut filters = tosho_nids::Filter::new()
                        .add_filter(
                            tosho_nids::filters::FilterType::SeriesRunId,
                            &series_run_uuid,
                        )
                        .with_order(
                            tosho_nids::filters::SortBy::FullTitle,
                            tosho_nids::filters::SortOrder::ASC,
                        )
                        .with_per_page(limit)
                        .with_page(1);

                    r#impl::nids::purchases::nids_get_purchased_issues(
                        &series_run_uuid,
                        &mut filters,
                        &client,
                        &config,
                        &t,
                    )
                    .await
                }
                NIDSCommands::PurchasedSeries { limit } => {
                    let mut filters = tosho_nids::Filter::new()
                        .with_order(
                            tosho_nids::filters::SortBy::Title,
                            tosho_nids::filters::SortOrder::ASC,
                        )
                        .with_per_page(limit)
                        .with_page(1);

                    r#impl::nids::purchases::nids_get_purchased_series(
                        &mut filters,
                        &client,
                        &config,
                        &t,
                    )
                    .await
                }
                NIDSCommands::Refresh { refresh_token } => {
                    r#impl::nids::accounts::nids_account_refresh(
                        refresh_token.as_deref(),
                        &client,
                        &config,
                        &t,
                    )
                    .await
                }
                NIDSCommands::Revoke => r#impl::nids::accounts::nids_account_revoke(&config, &t),
                NIDSCommands::SeriesRun { .. } => Ok(()),
                NIDSCommands::SeriesRuns { .. } => Ok(()),
            };

            exit_act
        }
        ToshoCommands::Tools { subcommand } => {
            let exit_act = match subcommand {
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
                ToolsCommands::ClearCache => {
                    r#impl::tools::cache::tools_clear_cache(&mut t_mut).await
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

            exit_act
        }
        #[cfg(feature = "with-updater")]
        ToshoCommands::Update => updater::perform_update(&t).await,
    }
}
