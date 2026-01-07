use std::path::PathBuf;

use clap::ValueEnum;
use color_eyre::eyre::{Context, OptionExt};
use color_print::cformat;
use num_format::{Locale, ToFormattedString};
use tosho_common::{ToshoError, make_error};
use tosho_kmkc::{KMClient, KMConfig, KMConfigMobile, KMConfigMobilePlatform};
use tosho_macros::EnumName;

use crate::{
    config::{get_all_config, save_config, try_remove_config},
    r#impl::client::make_kmkc_client,
    term::ConsoleChoice,
};

use super::config::{Config, ConfigMobile, MobilePlatform};

#[derive(Debug, Clone, Copy, PartialEq, EnumName)]
pub(crate) enum DeviceKind {
    /// Website platform.
    Web,
    /// Android platform.
    Android,
    /// Legacy Android platform.
    LegacyAndroid,
    /// iOS platform.
    Apple,
}

impl ValueEnum for DeviceKind {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            DeviceKind::Web => Some(clap::builder::PossibleValue::new("web")),
            DeviceKind::Android => Some(clap::builder::PossibleValue::new("android")),
            DeviceKind::LegacyAndroid => Some(clap::builder::PossibleValue::new("android-legacy")),
            DeviceKind::Apple => Some(clap::builder::PossibleValue::new("ios")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[
            DeviceKind::Web,
            DeviceKind::Android,
            DeviceKind::LegacyAndroid,
            DeviceKind::Apple,
        ]
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
            "android-legacy" | "android_legacy" => Ok(DeviceKind::LegacyAndroid),
            "ios" => Ok(DeviceKind::Apple),
            _ => Err(format!("Invalid device kind: {s}")),
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
    type Error = ToshoError;

    fn try_from(value: DeviceKind) -> Result<Self, Self::Error> {
        match value {
            DeviceKind::Android => Ok(KMConfigMobilePlatform::Android),
            DeviceKind::Apple => Ok(KMConfigMobilePlatform::Apple),
            _ => Err(make_error!(format!(
                "Cannot convert {value:?} to KMConfigMobilePlatform"
            ))),
        }
    }
}

pub(crate) async fn kmkc_account_login_web(
    cookies_path: PathBuf,
    console: &crate::term::Terminal,
) -> color_eyre::Result<()> {
    console.info("Authenticating your account...");

    // parse netscape cookies
    let cookie_config = super::common::parse_netscape_cookies(cookies_path)?;
    let all_configs = get_all_config(&crate::r#impl::Implementations::Kmkc, None)?;

    let client = make_kmkc_client(&KMConfig::Web(cookie_config.clone()))
        .context("Failed to create client")?;

    let account = client
        .get_account()
        .await
        .context("Failed to authenticate your account")?;

    console.info(cformat!("Authenticated as <m,s>{}</>", account.email()));
    let old_config = all_configs.iter().find(|&c| match c {
        crate::config::ConfigImpl::Kmkc(super::config::Config::Web(cc)) => {
            cc.account_id == account.id() && cc.device_id == account.user_id()
        }
        _ => false,
    });

    let mut acc_config = super::config::ConfigWeb::from(cookie_config).with_user_account(&account);

    if let Some(old_config) = old_config {
        console.warn("Session ID already exists!");
        let abort_it = console.confirm(Some("Do you want to replace it?"));
        if !abort_it {
            console.info("Aborting...");
            return Ok(());
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
    )?;

    Ok(())
}

pub(crate) async fn kmkc_account_login_mobile(
    user_id: u32,
    hash_key: String,
    platform: DeviceKind,
    console: &crate::term::Terminal,
) -> color_eyre::Result<()> {
    if platform == DeviceKind::Web {
        console.warn("Invalid platform!");
        return Err(color_eyre::eyre::eyre!("Invalid platform"));
    }

    console.info(cformat!(
        "Authenticating with <m,s>{}</> and key <m,s>{}</> [{}]",
        user_id,
        hash_key,
        platform.to_name()
    ));

    let all_configs = get_all_config(&crate::r#impl::Implementations::Kmkc, None)?;

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
            return Ok(());
        }

        match old_config {
            crate::config::ConfigImpl::Kmkc(super::config::Config::Mobile(cc)) => {
                old_id = Some(cc.id.clone());
            }
            _ => unreachable!(),
        }
    }

    let config = KMConfigMobile::new(user_id.to_string(), &hash_key, platform.try_into()?);
    let client =
        make_kmkc_client(&KMConfig::Mobile(config.clone())).context("Failed to create client")?;

    let account = client
        .get_account()
        .await
        .context("Failed to authenticate your account")?;
    console.info(cformat!("Authenticated as <m,s>{}</>", account.email()));

    let mut acc_config = super::config::ConfigMobile::from(config).with_user_account(&account);

    if let Some(old_id) = old_id {
        acc_config = acc_config.with_id(old_id);
    }

    console.info("Authentication successful! Saving config...");
    save_config(
        crate::config::ConfigImpl::Kmkc(Config::Mobile(acc_config)),
        None,
    )?;

    Ok(())
}

pub async fn kmkc_account_login(
    email: String,
    password: String,
    platform: DeviceKind,
    console: &crate::term::Terminal,
) -> color_eyre::Result<()> {
    console.info(cformat!(
        "Authenticating with email <m,s>{}</> and password <m,s>{}</>...",
        email,
        password
    ));

    let all_configs = get_all_config(&crate::r#impl::Implementations::Kmkc, None)?;

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
            return Ok(());
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
        DeviceKind::LegacyAndroid => Some(KMConfigMobilePlatform::AndroidLegacy),
    };

    let config = KMClient::login(&email, &password, mobile_match)
        .await
        .context("Failed to authenticate your account")?;

    console.info(cformat!(
        "Authenticated as <m,s>{}</>",
        config.account().email()
    ));

    let acc_config = match super::config::Config::from(config.config()) {
        super::config::Config::Mobile(cc) => {
            Config::Mobile(cc.with_user_account(config.account()).with_id_opt(old_id))
        }
        super::config::Config::Web(cc) => {
            Config::Web(cc.with_user_account(config.account()).with_id_opt(old_id))
        }
    };

    console.info(cformat!(
        "Created session ID <m,s>{}</>, saving config...",
        acc_config.get_id()
    ));
    save_config(crate::config::ConfigImpl::Kmkc(acc_config), None)?;

    Ok(())
}

pub async fn kmkc_account_login_adapt(
    platform: DeviceKind,
    console: &crate::term::Terminal,
) -> color_eyre::Result<()> {
    if platform == DeviceKind::Web {
        console.warn("Invalid platform!");
        return Err(color_eyre::eyre::eyre!("Invalid platform"));
    }

    let all_configs = get_all_config(&crate::r#impl::Implementations::Kmkc, None)?;
    let web_configs = all_configs
        .iter()
        .filter_map(|c| match c {
            crate::config::ConfigImpl::Kmkc(super::config::Config::Web(cc)) => Some(cc),
            _ => None,
        })
        .collect::<Vec<_>>();

    if web_configs.is_empty() {
        console.warn("There's no available web account to adapt!");
        return Err(color_eyre::eyre::eyre!("No available web account to adapt"));
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
            Err(color_eyre::eyre::eyre!("Aborted by user"))
        }
        Some(selected) => {
            let config = web_configs
                .iter()
                .cloned()
                .find(|&c| c.id == selected.name)
                .ok_or_eyre(format!("Failed to find account {}", selected.name))?;

            let client = make_kmkc_client(
                &config
                    .clone()
                    .try_into()
                    .context("Failed to convert config")?,
            )
            .context("Failed to create client")?;

            console.info(cformat!(
                "Re-Authenticating with email <m,s>{}</>...",
                config.email
            ));

            let account = client
                .get_account()
                .await
                .context("Failed to authenticate your account")?;

            let user_info = client
                .get_user(account.id())
                .await
                .context("Failed getting your user information")?;

            console.info(cformat!("Authenticated as <m,s>{}</>", account.email()));

            let mobile_config = KMConfigMobile::new(
                account.id().to_string(),
                user_info.hash_key(),
                platform.try_into()?,
            );
            let into_tosho: ConfigMobile = mobile_config.into();
            let final_config = into_tosho.with_user_account(&account);

            console.info(cformat!(
                "Created session ID <m,s>{}</>, saving config...",
                final_config.id.clone()
            ));

            save_config(final_config.into(), None)?;

            Ok(())
        }
    }
}

pub(crate) fn kmkc_accounts(console: &crate::term::Terminal) -> color_eyre::Result<()> {
    let all_configs = get_all_config(&crate::r#impl::Implementations::Kmkc, None)?;

    match all_configs.len() {
        0 => {
            console.warn("No accounts found!");
            Ok(())
        }
        other => {
            console.info(format!("Found {} accounts:", other));
            for (i, c) in all_configs.iter().enumerate() {
                match c {
                    crate::config::ConfigImpl::Kmkc(c) => {
                        let mut plat_name = c.get_type().to_name().to_string();
                        if let Config::Mobile(mob) = &c {
                            plat_name = format!("{} - {}", plat_name, mob.platform().to_name());
                        }
                        console.info(cformat!(
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

            Ok(())
        }
    }
}

pub(crate) async fn kmkc_account_info(
    client: &KMClient,
    acc_info: &Config,
    console: &crate::term::Terminal,
) -> color_eyre::Result<()> {
    console.info(cformat!(
        "Fetching account info for <magenta,bold>{}</>...",
        acc_info.get_id()
    ));

    let account = client
        .get_account()
        .await
        .context("Failed to fetch account info")?;

    console.info(cformat!(
        "Account info for <magenta,bold>{}</>:",
        acc_info.get_id()
    ));

    console.info(cformat!("  <s>ID:</>: {}", account.id()));
    console.info(cformat!("  <s>User ID:</>: {}", account.user_id()));
    let username = account.name().unwrap_or("Unknown");
    console.info(cformat!("  <s>Username:</>: {}", username));
    console.info(cformat!("  <s>Email:</>: {}", account.email()));
    console.info(cformat!("  <s>Registered?</>: {}", account.registered()));

    if !account.devices().is_empty() {
        console.info(cformat!("  <s>Devices:</>"));
        for device in account.devices() {
            console.info(cformat!(
                "    - <s>{}</>: {} [{}]",
                device.id(),
                device.name(),
                device.platform().to_name()
            ));
        }
    }

    Ok(())
}

pub(crate) async fn kmkc_balance(
    client: &KMClient,
    account: &Config,
    console: &crate::term::Terminal,
) -> color_eyre::Result<()> {
    console.info(cformat!(
        "Checking balance for <magenta,bold>{}</>...",
        account.get_id()
    ));

    let balance = client
        .get_user_point()
        .await
        .context("Failed to fetch balance")?;

    console.info("Your current point balance:");
    let point = balance.point();
    let total_bal = point.total_point().to_formatted_string(&Locale::en);
    let paid_point = point.paid_point().to_formatted_string(&Locale::en);
    let free_point = point.free_point().to_formatted_string(&Locale::en);
    let premium_ticket = balance
        .ticket()
        .total_num()
        .to_formatted_string(&Locale::en);
    console.info(cformat!(
        "  - <bold>Total:</> <cyan!,bold><reverse>{}</>c</cyan!,bold>",
        total_bal
    ));
    console.info(cformat!(
        "  - <bold>Paid point:</> <g,bold><reverse>{}</>c</g,bold>",
        paid_point
    ));
    console.info(cformat!(
        "  - <bold>Free point:</> <cyan,bold><reverse>{}</>c</cyan,bold>",
        free_point
    ));
    console.info(cformat!(
        "  - <bold>Premium ticket:</> <yellow,bold><reverse>{}</> ticket</yellow,bold>",
        premium_ticket
    ));

    Ok(())
}

pub(crate) fn kmkc_account_revoke(
    account: &Config,
    console: &crate::term::Terminal,
) -> color_eyre::Result<()> {
    let confirm = console.confirm(Some(&cformat!(
        "Are you sure you want to delete <m,s>{}</>?\nThis action is irreversible!",
        account.get_id()
    )));

    if !confirm {
        console.warn("Aborted");
        return Ok(());
    }

    try_remove_config(account.get_id(), crate::r#impl::Implementations::Kmkc, None)
        .context("Failed to delete account")?;

    console.info(cformat!(
        "Successfully deleted <magenta,bold>{}</>",
        account.get_id()
    ));

    Ok(())
}
