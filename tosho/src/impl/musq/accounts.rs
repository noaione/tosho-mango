use clap::ValueEnum;
use color_print::cformat;
use num_format::{Locale, ToFormattedString};

use crate::{
    cli::ExitCode,
    config::{get_all_config, save_config},
};

use super::{
    common::select_single_account,
    config::{Config, DeviceType},
};

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

    let all_configs = get_all_config(crate::r#impl::Implementations::Musq, None);
    let old_config = all_configs.iter().find(|&c| match c {
        crate::config::ConfigImpl::Kmkc(_) => false,
        crate::config::ConfigImpl::Musq(c) => c.session == session_id && c.r#type == r#type as i32,
    });

    if old_config.is_some() {
        console.warn("Session ID already authenticated!");
        let abort_it = console.confirm(Some("Do you want to replace it?"));
        if !abort_it {
            console.info("Aborting...");
            return 0;
        }
    }

    console.info(&format!(
        "Authenticating with session ID {} ({})",
        session_id,
        r#type.to_name()
    ));

    let config = Config::from_session(&session_id, r#type);

    let client = super::common::make_client(&config);
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
    let all_configs = get_all_config(crate::r#impl::Implementations::Musq, None);

    match all_configs.len() {
        0 => {
            console.warn("No accounts found!");

            1
        }
        _ => {
            console.info(&format!("Found {} accounts:", all_configs.len()));
            for (i, c) in all_configs.iter().enumerate() {
                match c {
                    crate::config::ConfigImpl::Kmkc(_) => {}
                    crate::config::ConfigImpl::Musq(c) => {
                        console.info(&format!(
                            "{:02}. {} ({})",
                            i + 1,
                            c.id,
                            c.r#type().to_name()
                        ));
                    }
                }
            }

            0
        }
    }
}

pub async fn musq_account_info(
    account_id: Option<&str>,
    console: &crate::term::Terminal,
) -> ExitCode {
    let acc_info = select_single_account(account_id);

    match acc_info {
        None => {
            console.warn("Aborted");
            1
        }
        Some(acc_info) => {
            console.info(&cformat!(
                "Fetching account info for <magenta,bold>{}</>...",
                acc_info.id
            ));
            let client = super::common::make_client(&acc_info);

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
                            console.info(&cformat!(
                                "    - <bold>{}:</> ({})",
                                device_name,
                                device_id
                            ));
                        }
                    } else {
                        console.info(&cformat!("  <bold>Devices:</> <red>None</>"));
                    }
                    0
                }
                Err(e) => {
                    console.error(&format!("Failed to fetch account info: {}", e));
                    1
                }
            }
        }
    }
}

pub async fn musq_account_balance(
    account_id: Option<&str>,
    console: &crate::term::Terminal,
) -> ExitCode {
    let acc_info = select_single_account(account_id);

    match acc_info {
        None => {
            console.warn("Aborted");
            1
        }
        Some(acc_info) => {
            console.info(&cformat!(
                "Checking balance for <magenta,bold>{}</>...",
                acc_info.id
            ));
            let client = super::common::make_client(&acc_info);

            let user_bal = client.get_user_point().await;
            match user_bal {
                Ok(user_bal) => {
                    console.info("Your current point balance:");
                    let total_bal = user_bal.sum().to_formatted_string(&Locale::en);
                    let paid_point = user_bal.paid.to_formatted_string(&Locale::en);
                    let xp_point = user_bal.event.to_formatted_string(&Locale::en);
                    let free_point = user_bal.free.to_formatted_string(&Locale::en);
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
                    0
                }
                Err(e) => {
                    console.error(&format!("Failed to fetch account info: {}", e));
                    1
                }
            }
        }
    }
}