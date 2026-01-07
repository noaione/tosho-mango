use clap::ValueEnum;
use color_eyre::eyre::Context;
use color_print::cformat;
use tosho_mplus::MPClient;

use crate::config::{get_all_config, save_config, try_remove_config};

use super::config::{Config, DeviceType};

#[derive(Clone)]
pub(crate) enum DeviceKind {
    Android,
}

impl ValueEnum for DeviceKind {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            DeviceKind::Android => Some(clap::builder::PossibleValue::new("android")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[DeviceKind::Android]
    }

    fn from_str(s: &str, ignore_case: bool) -> Result<Self, String> {
        let s = if ignore_case {
            s.to_lowercase()
        } else {
            s.to_string()
        };
        match s.as_str() {
            "android" => Ok(DeviceKind::Android),
            _ => Err(format!("Invalid device kind: {s}")),
        }
    }
}

pub(crate) async fn mplus_auth_session(
    session_id: String,
    device_kind: DeviceKind,
    console: &crate::term::Terminal,
) -> color_eyre::Result<()> {
    let device_type = match device_kind {
        DeviceKind::Android => DeviceType::Android,
    };

    let all_configs = get_all_config(&crate::r#impl::Implementations::Mplus, None)?;
    let old_config = all_configs.iter().find(|&c| match c {
        crate::config::ConfigImpl::Musq(c) => {
            c.session == session_id && c.r#type == device_type as i32
        }
        _ => false,
    });

    let mut old_id: Option<String> = None;
    if let Some(old_config) = old_config {
        console.warn("Session ID already authenticated!");
        let abort_it = console.confirm(Some("Do you want to replace it?"));
        if !abort_it {
            console.info("Aborting...");
            return Err(color_eyre::eyre::eyre!("Aborted by user"));
        }

        match old_config {
            crate::config::ConfigImpl::Musq(c) => {
                old_id = Some(c.id.clone());
            }
            _ => unreachable!(),
        }
    }

    console.info(cformat!(
        "Authenticating with session ID <m,s>{}</> (<s>{}</>)",
        session_id,
        device_type.to_name()
    ));

    let mut config = Config::from_session(&session_id, device_type);
    if let Some(old_id) = old_id {
        config = config.with_id(&old_id);
    }

    let client =
        crate::r#impl::client::make_mplus_client(&config, tosho_mplus::proto::Language::English)
            .context("Failed to create client")?;

    let account_resp = client
        .get_user_profile()
        .await
        .context("Authentication failed")?;

    match account_resp {
        tosho_mplus::APIResponse::Success(account) => {
            let mut final_config = config.clone();

            if !account.user_name().is_empty() {
                final_config = final_config.with_username(account.user_name());
            }

            console.info(cformat!(
                "Authenticated as <m,b>{}</> (<s>{}</>)",
                final_config.username.as_ref().unwrap_or(&final_config.id),
                final_config.r#type().to_name()
            ));

            save_config(crate::config::ConfigImpl::Mplus(final_config), None)?;

            Ok(())
        }
        tosho_mplus::APIResponse::Error(e) => Err(color_eyre::eyre::eyre!(
            "Authentication failed: {}",
            e.as_string()
        )),
    }
}

pub(crate) fn mplus_accounts(console: &crate::term::Terminal) -> color_eyre::Result<()> {
    let all_configs = get_all_config(&crate::r#impl::Implementations::Mplus, None)?;

    match all_configs.len() {
        0 => {
            console.warn("No accounts found!");
            Ok(())
        }
        other => {
            console.info(format!("Found {} accounts:", other));
            for (i, c) in all_configs.iter().enumerate() {
                match c {
                    crate::config::ConfigImpl::Mplus(c) => match &c.username {
                        Some(uname) => {
                            console.info(format!(
                                "{:02}. {} - {} ({})",
                                i + 1,
                                c.id,
                                uname,
                                c.r#type().to_name()
                            ));
                        }
                        None => {
                            console.info(format!(
                                "{:02}. {} ({})",
                                i + 1,
                                c.id,
                                c.r#type().to_name()
                            ));
                        }
                    },
                    _ => unreachable!(),
                }
            }

            Ok(())
        }
    }
}

pub async fn mplus_account_info(
    client: &MPClient,
    acc_info: &Config,
    console: &crate::term::Terminal,
) -> color_eyre::Result<()> {
    console.info(cformat!(
        "Fetching account info for <magenta,bold>{}</>...",
        acc_info.id
    ));

    let account_resp = client
        .get_user_settings()
        .await
        .context("Failed fetching account info")?;

    match account_resp {
        tosho_mplus::APIResponse::Success(account_resp) => {
            console.info(cformat!(
                "Account info for <magenta,bold>{}</> (<s>{}</>):",
                acc_info.id,
                acc_info.r#type().to_name()
            ));

            console.info(cformat!("  <bold>Session:</> {}", acc_info.session));
            console.info(cformat!("  <bold>Type:</> {}", acc_info.r#type().to_name()));

            let mut username = account_resp.user_name();
            if username.is_empty() {
                username = "[No username]";
            }

            console.info(cformat!("  <bold>Username:</> {}", username));
            let subs_info = account_resp.subscription().cloned().unwrap_or_default();
            console.info(cformat!(
                "  <bold>Subscription:</> {}",
                subs_info.plan().to_name()
            ));

            console.info(cformat!(
                "  <bold>Notify news?</> {}",
                if account_resp.news_notification() {
                    "Yes"
                } else {
                    "No"
                }
            ));

            console.info(cformat!(
                "  <bold>Notify chapter update?</> {}",
                if account_resp.chapter_notification() {
                    "Yes"
                } else {
                    "No"
                }
            ));

            Ok(())
        }
        tosho_mplus::APIResponse::Error(e) => {
            console.error(format!("Failed to fetch account info: {}", e.as_string()));
            Err(color_eyre::eyre::eyre!(
                "Failed to fetch account info: {}",
                e.as_string()
            ))
        }
    }
}

pub(crate) fn mplus_account_revoke(
    account: &Config,
    console: &crate::term::Terminal,
) -> color_eyre::Result<()> {
    let confirm = console.confirm(Some(&cformat!(
        "Are you sure you want to delete <m,s>{}</>?\nThis action is irreversible!",
        account.id
    )));

    if !confirm {
        console.warn("Aborted");
        return Ok(());
    }

    try_remove_config(
        account.id.as_str(),
        crate::r#impl::Implementations::Mplus,
        None,
    )
    .context("Failed to delete account")?;

    console.info(cformat!(
        "Successfully deleted <magenta,bold>{}</>",
        account.get_id()
    ));

    Ok(())
}
