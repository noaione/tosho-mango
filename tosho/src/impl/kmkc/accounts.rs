use std::path::PathBuf;

use clap::ValueEnum;
use color_print::cformat;
use num_format::{Locale, ToFormattedString};
use tosho_kmkc::{KMClient, KMConfig, KMConfigMobile, KMConfigMobilePlatform};
use tosho_macros::EnumName;

use crate::{
    cli::ExitCode,
    config::{get_all_config, save_config, try_remove_config},
    term::ConsoleChoice,
};

use super::{
    common::{make_client, select_single_account},
    config::{Config, ConfigMobile, MobilePlatform},
};

#[derive(Clone, PartialEq, EnumName)]
pub(crate) enum DeviceKind {
    /// Website platform.
    Web,
    /// Android platform.
    Android,
    /// iOS platform.
    Apple,
}

impl ValueEnum for DeviceKind {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            DeviceKind::Web => Some(clap::builder::PossibleValue::new("web")),
            DeviceKind::Android => Some(clap::builder::PossibleValue::new("android")),
            DeviceKind::Apple => Some(clap::builder::PossibleValue::new("ios")),
        }
    }

    #[cfg(not(tarpaulin_include))]
    fn value_variants<'a>() -> &'a [Self] {
        &[DeviceKind::Web, DeviceKind::Android, DeviceKind::Apple]
    }

    fn from_str(s: &str, ignore_case: bool) -> Result<Self, String> {
        let s = if ignore_case {
            s.to_lowercase()
        } else {
            s.to_string()
        };
        match s.as_str() {
            "web" => Ok(DeviceKind::Web),
            "android" => Ok(DeviceKind::Android),
            "ios" => Ok(DeviceKind::Apple),
            _ => Err(format!("Invalid device kind: {}", s)),
        }
    }
}

impl PartialEq<MobilePlatform> for DeviceKind {
    fn eq(&self, other: &MobilePlatform) -> bool {
        match self {
            DeviceKind::Android => matches!(other, MobilePlatform::Android),
            DeviceKind::Apple => matches!(other, MobilePlatform::Apple),
            _ => false,
        }
    }
}

impl TryFrom<DeviceKind> for KMConfigMobilePlatform {
    type Error = String;

    fn try_from(value: DeviceKind) -> Result<Self, Self::Error> {
        match value {
            DeviceKind::Android => Ok(KMConfigMobilePlatform::Android),
            DeviceKind::Apple => Ok(KMConfigMobilePlatform::Apple),
            _ => Err("Invalid device kind!".to_string()),
        }
    }
}

pub(crate) async fn kmkc_account_login_web(
    cookies_path: PathBuf,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info("Authenticating your account...");

    // parse netscape cookies
    let cookie_config = super::common::parse_netscape_cookies(cookies_path);
    let all_configs = get_all_config(crate::r#impl::Implementations::Kmkc, None);

    let client = make_client(&KMConfig::Web(cookie_config.clone()));

    let account = client.get_account().await;

    match account {
        Ok(account) => {
            console.info(&cformat!("Authenticated as <m,s>{}</>", account.email));
            let old_config = all_configs.iter().find(|&c| match c {
                crate::config::ConfigImpl::Kmkc(super::config::Config::Web(cc)) => {
                    cc.account_id == account.id && cc.device_id == account.user_id
                }
                _ => false,
            });

            let mut acc_config =
                super::config::ConfigWeb::from(cookie_config).with_user_account(&account);

            if let Some(old_config) = old_config {
                console.warn("Session ID already exists!");
                let abort_it = console.confirm(Some("Do you want to replace it?"));
                if !abort_it {
                    console.info("Aborting...");
                    return 0;
                }

                match old_config {
                    crate::config::ConfigImpl::Kmkc(super::config::Config::Web(cc)) => {
                        acc_config = acc_config.with_id(cc.id.clone());
                    }
                    _ => unreachable!(),
                }
            }

            console.info("Authentication successful! Saving config...");
            save_config(
                crate::config::ConfigImpl::Kmkc(Config::Web(acc_config)),
                None,
            );
            0
        }
        Err(err) => {
            console.error(&format!("Failed to authenticate your account: {}", err));

            1
        }
    }
}

pub(crate) async fn kmkc_account_login_mobile(
    user_id: u32,
    hash_key: String,
    platform: DeviceKind,
    console: &crate::term::Terminal,
) -> ExitCode {
    if platform == DeviceKind::Web {
        console.warn("Invalid platform!");
        return 1;
    }

    console.info(&cformat!(
        "Authenticating with <m,s>{}</> and key <m,s>{}</> [{}]",
        user_id,
        hash_key,
        platform.to_name()
    ));

    let all_configs = get_all_config(crate::r#impl::Implementations::Kmkc, None);

    // find old config
    let old_config = all_configs.iter().find(|&c| match c {
        crate::config::ConfigImpl::Kmkc(super::config::Config::Mobile(cc)) => {
            cc.device_id == user_id && platform == cc.platform()
        }
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
            crate::config::ConfigImpl::Kmkc(super::config::Config::Mobile(cc)) => {
                old_id = Some(cc.id.clone());
            }
            _ => unreachable!(),
        }
    }

    let config = KMConfigMobile {
        user_id: user_id.to_string(),
        hash_key,
        platform: platform.try_into().unwrap(),
    };
    let client = make_client(&KMConfig::Mobile(config.clone()));

    let account = client.get_account().await;

    match account {
        Ok(account) => {
            console.info(&cformat!("Authenticated as <m,s>{}</>", account.email));

            let mut acc_config =
                super::config::ConfigMobile::from(config).with_user_account(&account);

            if let Some(old_id) = old_id {
                acc_config = acc_config.with_id(old_id);
            }

            console.info("Authentication successful! Saving config...");
            save_config(
                crate::config::ConfigImpl::Kmkc(Config::Mobile(acc_config)),
                None,
            );

            0
        }
        Err(err) => {
            console.error(&format!("Failed to authenticate your account: {}", err));

            1
        }
    }
}

pub async fn kmkc_account_login(
    email: String,
    password: String,
    platform: DeviceKind,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(&cformat!(
        "Authenticating with email <m,s>{}</> and password <m,s>{}</>...",
        email,
        password
    ));

    let all_configs = get_all_config(crate::r#impl::Implementations::Kmkc, None);

    // find old config
    let old_config = all_configs.iter().find(|&c| match c {
        crate::config::ConfigImpl::Kmkc(super::config::Config::Mobile(cc)) => {
            cc.email == email && platform == cc.platform()
        }
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
            crate::config::ConfigImpl::Kmkc(super::config::Config::Mobile(cc)) => {
                old_id = Some(cc.id.clone());
            }
            _ => unreachable!(),
        }
    }

    let mobile_match = match platform {
        DeviceKind::Web => None,
        DeviceKind::Android => Some(KMConfigMobilePlatform::Android),
        DeviceKind::Apple => Some(KMConfigMobilePlatform::Apple),
    };

    let config = KMClient::login(&email, &password, mobile_match).await;

    match config {
        Ok(config) => {
            console.info(&cformat!(
                "Authenticated as <m,s>{}</>",
                config.account.email
            ));

            let acc_config = match super::config::Config::from(config.config) {
                super::config::Config::Mobile(cc) => {
                    Config::Mobile(cc.with_user_account(&config.account).with_id_opt(old_id))
                }
                super::config::Config::Web(cc) => {
                    Config::Web(cc.with_user_account(&config.account).with_id_opt(old_id))
                }
            };

            console.info(&cformat!(
                "Created session ID <m,s>{}</>, saving config...",
                acc_config.get_id()
            ));
            save_config(crate::config::ConfigImpl::Kmkc(acc_config), None);

            0
        }
        Err(err) => {
            console.error(&format!("Failed to authenticate your account: {}", err));

            1
        }
    }
}

pub async fn kmkc_account_login_adapt(
    platform: DeviceKind,
    console: &crate::term::Terminal,
) -> ExitCode {
    if platform == DeviceKind::Web {
        console.warn("Invalid platform!");
        return 1;
    }

    let binding = get_all_config(crate::r#impl::Implementations::Kmkc, None);
    let web_configs = binding
        .iter()
        .filter_map(|c| match c {
            crate::config::ConfigImpl::Kmkc(super::config::Config::Web(cc)) => Some(cc),
            _ => None,
        })
        .collect::<Vec<_>>();

    if web_configs.is_empty() {
        console.warn("There's no available web account to adapt!");
        return 1;
    }

    let web_choices: Vec<ConsoleChoice> = web_configs
        .iter()
        .map(|&c| ConsoleChoice {
            name: c.id.clone(),
            value: format!("{} [{}]", c.id, c.r#type().to_name()),
        })
        .collect();

    let select_acc = console.choice("Select an account:", web_choices);
    match select_acc {
        None => {
            console.warn("Aborted!");
            1
        }
        Some(selected) => {
            let config = web_configs
                .iter()
                .cloned()
                .find(|&c| c.id == selected.name)
                .unwrap();

            let client = make_client(&config.clone().into());
            console.info(&cformat!(
                "Re-Authenticating with email <m,s>{}</>...",
                config.email
            ));

            let account = client.get_account().await;

            match account {
                Ok(account) => {
                    let user_info = client.get_user(account.id).await.unwrap();

                    console.info(&cformat!("Authenticated as <m,s>{}</>", account.email));

                    let mobile_config = KMConfigMobile {
                        user_id: account.id.to_string(),
                        hash_key: user_info.hash_key,
                        platform: platform.try_into().unwrap(),
                    };
                    let into_tosho: ConfigMobile = mobile_config.into();
                    let final_config = into_tosho.with_user_account(&account);

                    console.info(&cformat!(
                        "Created session ID <m,s>{}</>, saving config...",
                        final_config.id.clone()
                    ));

                    save_config(final_config.into(), None);

                    0
                }
                Err(err) => {
                    console.error(&format!("Failed to authenticate your account: {}", err));

                    1
                }
            }
        }
    }
}

pub(crate) fn kmkc_accounts(console: &crate::term::Terminal) -> ExitCode {
    let all_configs = get_all_config(crate::r#impl::Implementations::Kmkc, None);

    match all_configs.len() {
        0 => {
            console.warn("No accounts found!");

            1
        }
        _ => {
            console.info(&format!("Found {} accounts:", all_configs.len()));
            for (i, c) in all_configs.iter().enumerate() {
                match c {
                    crate::config::ConfigImpl::Kmkc(c) => {
                        let mut plat_name = c.get_type().to_name().to_string();
                        if let Config::Mobile(mob) = &c {
                            plat_name = format!("{} - {}", plat_name, mob.platform().to_name());
                        }
                        console.info(&cformat!(
                            "{:02}. {} — <s>{}</> ({})",
                            i + 1,
                            c.get_id(),
                            c.get_username(),
                            plat_name,
                        ));
                    }
                    _ => unreachable!(),
                }
            }

            0
        }
    }
}

pub(crate) async fn kmkc_account_info(
    account_id: Option<&str>,
    console: &crate::term::Terminal,
) -> ExitCode {
    let acc_info = select_single_account(account_id);

    match acc_info {
        None => {
            console.warn("Aborted!");

            1
        }
        Some(acc_info) => {
            let binding = acc_info.clone();
            let acc_id = binding.get_id();
            console.info(&cformat!(
                "Fetching account info for <magenta,bold>{}</>...",
                acc_id
            ));

            let client = make_client(&acc_info.into());
            let account = client.get_account().await;

            match account {
                Ok(account) => {
                    console.info(&cformat!("Account info for <magenta,bold>{}</>:", acc_id));

                    console.info(&cformat!("  <s>ID:</>: {}", account.id));
                    console.info(&cformat!("  <s>User ID:</>: {}", account.user_id));
                    let username = account.name.unwrap_or("Unknown".to_string());
                    console.info(&cformat!("  <s>Username:</>: {}", username));
                    console.info(&cformat!("  <s>Email:</>: {}", account.email));
                    console.info(&cformat!("  <s>Registered?</>: {}", account.registered));

                    if !account.devices.is_empty() {
                        console.info(&cformat!("  <s>Devices:</>"));
                        for device in account.devices {
                            console.info(&cformat!(
                                "    - <s>{}</>: {} [{}]",
                                device.id,
                                device.name,
                                device.platform.to_name()
                            ));
                        }
                    }

                    0
                }
                Err(err) => {
                    console.error(&format!("Failed to fetch account info: {}", err));
                    1
                }
            }
        }
    }
}

pub(crate) async fn kmkc_balance(
    account_id: Option<&str>,
    console: &crate::term::Terminal,
) -> ExitCode {
    let account = select_single_account(account_id);
    if account.is_none() {
        console.warn("Aborted");
        return 1;
    }

    let account = account.unwrap();

    let client = super::common::make_client(&account.clone().into());
    console.info(&cformat!(
        "Checking balance for <magenta,bold>{}</>...",
        account.get_id()
    ));

    let balance = client.get_user_point().await;
    match balance {
        Err(err) => {
            console.error(&format!("Failed to fetch balance: {}", err));
            1
        }
        Ok(balance) => {
            console.info("Your current point balance:");
            let total_bal = balance.point.total_point().to_formatted_string(&Locale::en);
            let paid_point = balance.point.paid_point.to_formatted_string(&Locale::en);
            let free_point = balance.point.free_point.to_formatted_string(&Locale::en);
            let premium_ticket = balance.ticket.total_num.to_formatted_string(&Locale::en);
            console.info(&cformat!(
                "  - <bold>Total:</> <cyan!,bold><reverse>{}</>c</cyan!,bold>",
                total_bal
            ));
            console.info(&cformat!(
                "  - <bold>Paid point:</> <g,bold><reverse>{}</>c</g,bold>",
                paid_point
            ));
            console.info(&cformat!(
                "  - <bold>Free point:</> <cyan,bold><reverse>{}</>c</cyan,bold>",
                free_point
            ));
            console.info(&cformat!(
                "  - <bold>Premium ticket:</> <yellow,bold><reverse>{}</> ticket</yellow,bold>",
                premium_ticket
            ));

            0
        }
    }
}

pub(crate) fn kmkc_account_revoke(
    account_id: Option<&str>,
    console: &crate::term::Terminal,
) -> ExitCode {
    let account = select_single_account(account_id);
    if account.is_none() {
        console.warn("Aborted");
        return 1;
    }

    let account = account.unwrap();
    let confirm = console.confirm(Some(&cformat!(
        "Are you sure you want to delete <m,s>{}</>?\nThis action is irreversible!",
        account.get_id()
    )));

    if !confirm {
        console.warn("Aborted");
        return 0;
    }

    match try_remove_config(account.get_id(), crate::r#impl::Implementations::Kmkc, None) {
        Ok(_) => {
            console.info(&cformat!(
                "Successfully deleted <magenta,bold>{}</>",
                account.get_id()
            ));
            0
        }
        Err(err) => {
            console.error(&format!("Failed to delete account: {}", err));
            1
        }
    }
}
