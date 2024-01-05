use std::path::PathBuf;

use clap::ValueEnum;
use color_print::cformat;
use tosho_kmkc::{KMClient, KMConfig, KMConfigMobile};

use crate::{
    cli::ExitCode,
    config::{get_all_config, save_config},
};

use super::{
    common::make_client,
    config::{Config, MobilePlatform},
};

#[derive(Clone)]
pub(crate) enum DeviceKind {
    /// Website platform.
    Web,
    /// Android platform.
    Android,
    // /// iOS platform.
    // Apple,
}

impl ValueEnum for DeviceKind {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            DeviceKind::Web => Some(clap::builder::PossibleValue::new("web")),
            DeviceKind::Android => Some(clap::builder::PossibleValue::new("android")),
            // DeviceKind::Apple => Some(clap::builder::PossibleValue::new("ios")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[
            DeviceKind::Web,
            DeviceKind::Android,
            // DeviceKind::Apple
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
            // "ios" => Ok(DeviceKind::Apple),
            _ => Err(format!("Invalid device kind: {}", s)),
        }
    }
}

impl PartialEq<MobilePlatform> for DeviceKind {
    fn eq(&self, other: &MobilePlatform) -> bool {
        match self {
            DeviceKind::Android => matches!(other, MobilePlatform::Android),
            // DeviceKind::Apple => matches!(other, MobilePlatform::Apple),
            _ => false,
        }
    }
}

pub(crate) async fn kmkc_authweb_cookies(
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
            console.info(&cformat!("Authenticated as <m,s>{}</>", account.name));
            let old_config = all_configs.iter().find(|&c| match c {
                crate::config::ConfigImpl::Kmkc(c) => match c {
                    super::config::Config::Web(cc) => {
                        cc.account_id == account.id && cc.device_id == account.user_id
                    }
                    _ => false,
                },
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
                    crate::config::ConfigImpl::Kmkc(c) => match c {
                        super::config::Config::Web(cc) => {
                            acc_config = acc_config.with_id(cc.id.clone());
                        }
                        _ => unreachable!(),
                    },
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

pub(crate) async fn kmkc_authmobile(
    user_id: u32,
    hash_key: String,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(&cformat!(
        "Authenticating with <m,s>{}</> and key <m,s>{}</>",
        user_id,
        hash_key
    ));

    let all_configs = get_all_config(crate::r#impl::Implementations::Kmkc, None);

    // find old config
    let old_config = all_configs.iter().find(|&c| match c {
        crate::config::ConfigImpl::Kmkc(c) => match c {
            super::config::Config::Mobile(cc) => cc.device_id == user_id,
            _ => false,
        },
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
            crate::config::ConfigImpl::Kmkc(c) => match c {
                super::config::Config::Mobile(cc) => {
                    old_id = Some(cc.id.clone());
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    let config = KMConfigMobile {
        user_id: user_id.to_string(),
        hash_key,
    };
    let client = make_client(&KMConfig::Mobile(config.clone()));

    let account = client.get_account().await;

    match account {
        Ok(account) => {
            console.info(&cformat!("Authenticated as <m,s>{}</>", account.name));

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
    console.info(&cformat!("Authenticating with email <m,s>{}</>...", email,));

    let all_configs = get_all_config(crate::r#impl::Implementations::Kmkc, None);

    // find old config
    let old_config = all_configs.iter().find(|&c| match c {
        crate::config::ConfigImpl::Kmkc(c) => match c {
            super::config::Config::Mobile(cc) => cc.email == email && platform == cc.platform(),
            _ => false,
        },
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
            crate::config::ConfigImpl::Kmkc(c) => match c {
                super::config::Config::Mobile(cc) => {
                    old_id = Some(cc.id.clone());
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    let config = KMClient::login(&email, &password, !matches!(platform, DeviceKind::Web)).await;

    match config {
        Ok(config) => {
            console.info(&cformat!(
                "Authenticated as <m,s>{}</>",
                config.account.name
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
