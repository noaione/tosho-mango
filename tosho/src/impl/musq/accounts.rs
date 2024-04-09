use clap::ValueEnum;
use color_print::cformat;
use num_format::{Locale, ToFormattedString};
use tosho_musq::{proto::SubscriptionStatus, MUClient};

use crate::{
    cli::ExitCode,
    config::{get_all_config, save_config, try_remove_config},
};

use super::config::{Config, DeviceType};

#[derive(Clone)]
pub(crate) enum DeviceKind {
    Android,
    Apple,
}

impl ValueEnum for DeviceKind {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            DeviceKind::Android => Some(clap::builder::PossibleValue::new("android")),
            DeviceKind::Apple => Some(clap::builder::PossibleValue::new("ios")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[DeviceKind::Android, DeviceKind::Apple]
    }

    fn from_str(s: &str, ignore_case: bool) -> Result<Self, String> {
        let s = if ignore_case {
            s.to_lowercase()
        } else {
            s.to_string()
        };
        match s.as_str() {
            "android" => Ok(DeviceKind::Android),
            "ios" => Ok(DeviceKind::Apple),
            _ => Err(format!("Invalid device kind: {}", s)),
        }
    }
}

pub(crate) async fn musq_auth_session(
    session_id: String,
    device_kind: DeviceKind,
    console: &crate::term::Terminal,
) -> ExitCode {
    let r#type = match device_kind {
        DeviceKind::Android => DeviceType::Android,
        DeviceKind::Apple => DeviceType::Apple,
    };

    let all_configs = get_all_config(&crate::r#impl::Implementations::Musq, None);
    let old_config = all_configs.iter().find(|&c| match c {
        crate::config::ConfigImpl::Musq(c) => c.session == session_id && c.r#type == r#type as i32,
        _ => false,
    });

    let mut old_id: Option<String> = None;
    if let Some(old_config) = old_config {
        console.warn("Session ID already authenticated!");
        let abort_it = console.confirm(Some("Do you want to replace it?"));
        if !abort_it {
            console.info("Aborting...");
            return 0;
        }

        match old_config {
            crate::config::ConfigImpl::Musq(c) => {
                old_id = Some(c.id.clone());
            }
            _ => unreachable!(),
        }
    }

    console.info(&cformat!(
        "Authenticating with session ID <m,s>{}</> (<s>{}</>)",
        session_id,
        r#type.to_name()
    ));

    let mut config = Config::from_session(&session_id, r#type);
    if let Some(old_id) = old_id {
        config.apply_id(&old_id);
    }

    let client = crate::r#impl::client::make_musq_client(&config);
    let account = client.get_account().await;

    match account {
        Ok(_) => {
            // save config
            console.info("Authentication successful! Saving config...");
            save_config(crate::config::ConfigImpl::Musq(config), None);
            0
        }
        Err(e) => {
            console.error(&format!("Authentication failed: {}", e));
            1
        }
    }
}

pub(crate) fn musq_accounts(console: &crate::term::Terminal) -> ExitCode {
    let all_configs = get_all_config(&crate::r#impl::Implementations::Musq, None);

    match all_configs.len() {
        0 => {
            console.warn("No accounts found!");

            1
        }
        _ => {
            console.info(&format!("Found {} accounts:", all_configs.len()));
            for (i, c) in all_configs.iter().enumerate() {
                match c {
                    crate::config::ConfigImpl::Musq(c) => {
                        console.info(&format!(
                            "{:02}. {} ({})",
                            i + 1,
                            c.id,
                            c.r#type().to_name()
                        ));
                    }
                    _ => unreachable!(),
                }
            }

            0
        }
    }
}

pub async fn musq_account_info(
    client: &MUClient,
    acc_info: &Config,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(&cformat!(
        "Fetching account info for <magenta,bold>{}</>...",
        acc_info.id
    ));

    let account = client.get_account().await;
    match account {
        Ok(account) => {
            console.info(&cformat!(
                "Account info for <magenta,bold>{}</>:",
                acc_info.id
            ));
            console.info(&cformat!("  <bold>Session:</> {}", acc_info.session));
            console.info(&cformat!(
                "  <bold>Type:</> {}",
                acc_info.r#type().to_name()
            ));
            console.info(&cformat!("  <bold>Registered?</> {}", account.registered()));
            if !account.devices.is_empty() {
                console.info(&cformat!("  <bold>Devices:</>"));
                for device in account.devices {
                    let device_name = device.name;
                    let device_id = device.id;
                    console.info(&cformat!("    - <bold>{}:</> ({})", device_name, device_id));
                }
            }

            0
        }
        Err(e) => {
            console.error(&format!("Failed to fetch account info: {}", e));
            1
        }
    }
}

pub async fn musq_account_balance(
    client: &MUClient,
    acc_info: &Config,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(&cformat!(
        "Checking balance for <magenta,bold>{}</>...",
        acc_info.id
    ));

    let user_shop = client.get_point_shop().await;
    match user_shop {
        Ok(user_shop) => {
            console.info("Your current point balance:");
            let user_point = user_shop.user_point.clone().unwrap_or_default();
            let total_bal = user_point.sum().to_formatted_string(&Locale::en);
            let paid_point = user_point.paid.to_formatted_string(&Locale::en);
            let xp_point = user_point.event.to_formatted_string(&Locale::en);
            let free_point = user_point.free.to_formatted_string(&Locale::en);
            console.info(&cformat!(
                "  - <bold>Total:</> <cyan!,bold><reverse>{}</>c</cyan!,bold>",
                total_bal
            ));
            console.info(&cformat!(
                "  - <bold>Paid point:</> <yellow!,bold><reverse>{}</>c</yellow!,bold>",
                paid_point
            ));
            console.info(&cformat!(
                "  - <bold>Event/XP point:</> <magenta,bold><reverse>{}</>c</magenta,bold>",
                xp_point
            ));
            console.info(&cformat!(
                "  - <bold>Free point:</> <green,bold><reverse>{}</>c</green,bold>",
                free_point
            ));
            let subs_status = if user_shop.subscription_status() == SubscriptionStatus::Subscribed {
                "<green,bold>Subscribed</>"
            } else {
                "<red,bold>Unsubscribed</>"
            };
            console.info(&cformat!("  - <bold>Subscription:</> {}", subs_status));
            0
        }
        Err(e) => {
            console.error(&format!("Failed to fetch account info: {}", e));
            1
        }
    }
}

pub(crate) fn musq_account_revoke(account: &Config, console: &crate::term::Terminal) -> ExitCode {
    let confirm = console.confirm(Some(&cformat!(
        "Are you sure you want to delete <m,s>{}</>?\nThis action is irreversible!",
        account.id
    )));

    if !confirm {
        console.warn("Aborted");
        return 0;
    }

    match try_remove_config(
        account.id.as_str(),
        crate::r#impl::Implementations::Kmkc,
        None,
    ) {
        Ok(_) => {
            console.info(&cformat!(
                "Successfully deleted <magenta,bold>{}</>",
                account.id
            ));
            0
        }
        Err(err) => {
            console.error(&format!("Failed to delete account: {}", err));
            1
        }
    }
}
