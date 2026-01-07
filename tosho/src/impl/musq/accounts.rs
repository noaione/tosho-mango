use clap::ValueEnum;
use color_eyre::eyre::Context;
use color_print::cformat;
use num_format::{Locale, ToFormattedString};
use tosho_musq::MUClient;

use crate::{
    config::{get_all_config, save_config, try_remove_config},
    r#impl::common::unix_timestamp_to_string,
};

use super::config::{Config, DeviceType};

#[derive(Clone)]
pub(crate) enum DeviceKind {
    Android,
    Apple,
    Web,
}

impl ValueEnum for DeviceKind {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            DeviceKind::Android => Some(clap::builder::PossibleValue::new("android")),
            DeviceKind::Apple => Some(clap::builder::PossibleValue::new("ios")),
            DeviceKind::Web => Some(clap::builder::PossibleValue::new("web")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[DeviceKind::Android, DeviceKind::Apple, DeviceKind::Web]
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
            "web" => Ok(DeviceKind::Web),
            _ => Err(format!("Invalid device kind: {s}")),
        }
    }
}

pub(crate) async fn musq_auth_session(
    session_id: String,
    device_kind: DeviceKind,
    console: &crate::term::Terminal,
) -> color_eyre::Result<()> {
    let r#type = match device_kind {
        DeviceKind::Android => DeviceType::Android,
        DeviceKind::Apple => DeviceType::Apple,
        DeviceKind::Web => DeviceType::Web,
    };

    let all_configs = get_all_config(&crate::r#impl::Implementations::Musq, None)?;
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
        r#type.to_name()
    ));

    let mut config = Config::from_session(&session_id, r#type);
    if let Some(old_id) = old_id {
        config.apply_id(&old_id);
    }

    let client =
        crate::r#impl::client::make_musq_client(&config).context("Failed to create client")?;

    // just for checking
    client
        .get_account()
        .await
        .context("Authentication failed")?;

    // save config
    console.info("Authentication successful! Saving config...");
    save_config(crate::config::ConfigImpl::Musq(config), None)?;

    Ok(())
}

pub(crate) fn musq_accounts(console: &crate::term::Terminal) -> color_eyre::Result<()> {
    let all_configs = get_all_config(&crate::r#impl::Implementations::Musq, None)?;

    match all_configs.len() {
        0 => {
            console.warn("No accounts found!");

            Ok(())
        }
        other => {
            console.info(format!("Found {} accounts:", other));
            for (i, c) in all_configs.iter().enumerate() {
                match c {
                    crate::config::ConfigImpl::Musq(c) => {
                        console.info(format!("{:02}. {} ({})", i + 1, c.id, c.r#type().to_name()));
                    }
                    _ => unreachable!(),
                }
            }

            Ok(())
        }
    }
}

pub async fn musq_account_info(
    client: &MUClient,
    acc_info: &Config,
    console: &crate::term::Terminal,
) -> color_eyre::Result<()> {
    console.info(cformat!(
        "Fetching account info for <magenta,bold>{}</>...",
        acc_info.id
    ));

    let account = client
        .get_account()
        .await
        .context("Failed to fetch account info")?;

    console.info(cformat!(
        "Account info for <magenta,bold>{}</>:",
        acc_info.id
    ));
    console.info(cformat!("  <bold>Session:</> {}", acc_info.session));
    console.info(cformat!(
        "  <bold>Platform:</> {}",
        acc_info.r#type().to_name()
    ));
    console.info(cformat!("  <bold>Registered?</> {}", account.registered()));
    if !account.devices().is_empty() {
        console.info(cformat!("  <bold>Devices:</>"));
        for device in account.devices() {
            let device_name = device.name();
            let device_id = device.id();
            console.info(cformat!("    - <bold>{}:</> ({})", device_name, device_id));
        }
    }

    Ok(())
}

pub async fn musq_account_balance(
    client: &MUClient,
    acc_info: &Config,
    console: &crate::term::Terminal,
) -> color_eyre::Result<()> {
    console.info(cformat!(
        "Checking balance for <magenta,bold>{}</>...",
        acc_info.id
    ));

    let user_shop = client
        .get_point_shop()
        .await
        .context("Failed to fetch account info")?;

    console.info("Your current point balance:");
    let user_point = user_shop.user_point().unwrap_or_default();
    let total_bal = user_point.sum().to_formatted_string(&Locale::en);
    let paid_point = user_point.paid().to_formatted_string(&Locale::en);
    let xp_point = user_point.event().to_formatted_string(&Locale::en);
    let free_point = user_point.free().to_formatted_string(&Locale::en);
    console.info(cformat!(
        "  - <bold>Total:</> <cyan!,bold><reverse>{}</>c</cyan!,bold>",
        total_bal
    ));
    console.info(cformat!(
        "  - <bold>Paid point:</> <yellow!,bold><reverse>{}</>c</yellow!,bold>",
        paid_point
    ));
    console.info(cformat!(
        "  - <bold>Event/XP point:</> <magenta,bold><reverse>{}</>c</magenta,bold>",
        xp_point
    ));
    console.info(cformat!(
        "  - <bold>Free point:</> <green,bold><reverse>{}</>c</green,bold>",
        free_point
    ));
    let subs_status = match user_shop.subscriptions().first() {
        Some(first_subs) => {
            let status = first_subs.status().as_name();
            match unix_timestamp_to_string(first_subs.end()) {
                Some(ends_at) => cformat!(
                    "<bold><blue>{}</blue> Subscription</bold> (Ends at: <s>{}</>)",
                    status,
                    ends_at
                ),
                None => cformat!("<bold><blue>{}</blue> Subscription</bold>", status),
            }
        }
        None => cformat!("<red,bold>Unsubscribed</>"),
    };
    console.info(cformat!("  - <bold>Subscription:</> {}", subs_status));

    Ok(())
}

pub(crate) fn musq_account_revoke(
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
        crate::r#impl::Implementations::Musq,
        None,
    )
    .context("Failed to delete account")?;

    console.info(cformat!(
        "Successfully deleted <magenta,bold>{}</>",
        account.get_id()
    ));

    Ok(())
}
