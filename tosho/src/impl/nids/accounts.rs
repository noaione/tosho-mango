use clap::ValueEnum;
use color_print::cformat;
use num_format::{Locale, ToFormattedString};

use crate::{
    cli::ExitCode,
    config::{get_all_config, save_config, try_remove_config},
};

use super::config::{Config, DeviceType};

#[derive(Clone)]
pub(crate) enum DeviceKind {
    Web,
}

impl ValueEnum for DeviceKind {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            DeviceKind::Web => Some(clap::builder::PossibleValue::new("web")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[DeviceKind::Web]
    }

    fn from_str(s: &str, ignore_case: bool) -> Result<Self, String> {
        let s = if ignore_case {
            s.to_lowercase()
        } else {
            s.to_string()
        };
        match s.as_str() {
            "web" => Ok(DeviceKind::Web),
            _ => Err(format!("Invalid device kind: {s}")),
        }
    }
}

pub(crate) async fn nids_auth_session(
    session_token: String,
    device_kind: DeviceKind,
    console: &crate::term::Terminal,
) -> ExitCode {
    let r#type = match device_kind {
        DeviceKind::Web => DeviceType::Web,
    };

    let all_configs = get_all_config(&crate::r#impl::Implementations::Nids, None);

    console.info(cformat!(
        "Authenticating with session token <m,s>{}</>...",
        session_token
    ));

    let random_uuid = uuid::Uuid::new_v4().to_string();
    let mut config = Config::from_session(&session_token, r#type);
    config.apply_id(&random_uuid);

    match crate::r#impl::client::make_nids_client(&config) {
        Err(e) => {
            console.error(cformat!("Unable to create client: {}", e));
            1
        }
        Ok(client) => {
            let user_info = client.get_profile().await;

            match user_info {
                Err(e) => {
                    console.error(cformat!("Unable to authenticate: {}", e));
                    1
                }
                Ok(user_info) => {
                    let old_config = all_configs.iter().find(|&c| match c {
                        crate::config::ConfigImpl::Nids(c) => c.id == user_info.id(),
                        _ => false,
                    });

                    if let Some(old_config) = old_config {
                        console.warn("Session ID already authenticated!");
                        let abort_it = console.confirm(Some("Do you want to replace it?"));
                        if !abort_it {
                            console.info("Aborting...");
                            return 0;
                        }

                        match old_config {
                            crate::config::ConfigImpl::Musq(c) => {
                                config.apply_id(&c.id);
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        config.apply_id(user_info.id());
                    }
                    config.email = user_info.email().to_string();
                    config.username = user_info.username().map(|s| s.to_string());

                    console.info(cformat!(
                        "Authenticated as <m,s>{}</> ({})...",
                        user_info.username().unwrap_or("Unknown"),
                        user_info.email()
                    ));

                    console.info(cformat!(
                        "Created session ID <m,s>{}</>, saving config...",
                        config.get_id()
                    ));
                    save_config(crate::config::ConfigImpl::Nids(config), None);
                    0
                }
            }
        }
    }
}

pub(crate) fn nids_accounts(console: &crate::term::Terminal) -> ExitCode {
    let all_configs = get_all_config(&crate::r#impl::Implementations::Nids, None);

    match all_configs.len() {
        0 => {
            console.warn("No accounts found!");

            1
        }
        _ => {
            console.info(format!("Found {} accounts:", all_configs.len()));
            for (i, c) in all_configs.iter().enumerate() {
                match c {
                    crate::config::ConfigImpl::Nids(c) => {
                        let plat_name = c.r#type().to_name();
                        console.info(cformat!(
                            "{:02}. {} — <s>{}</> ({})",
                            i + 1,
                            c.id,
                            c.email,
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

pub(crate) async fn nids_account_info(
    client: &tosho_nids::NIClient,
    account: &Config,
    console: &crate::term::Terminal,
) -> ExitCode {
    match client.get_profile().await {
        Ok(acc_info) => {
            console.info(cformat!(
                "Account info for <magenta,bold>{}</>:",
                account.id
            ));

            console.info(cformat!("  <s>ID</>: {}", acc_info.id()));
            console.info(cformat!("  <s>Email</>: {}", acc_info.email()));
            let username = acc_info.username().unwrap_or("[no username]");
            console.info(cformat!("  <s>Username</>: {}", username));

            let mut user_name = String::new();
            if let Some(first_name) = acc_info.first_name()
                && !first_name.is_empty()
            {
                user_name.push_str(first_name);
                user_name.push(' ');
            }
            if let Some(last_name) = acc_info.last_name()
                && !last_name.is_empty()
            {
                user_name.push_str(last_name);
            }

            user_name = user_name.trim().to_string();
            if !user_name.is_empty() {
                console.info(cformat!("  <s>Name</>: {}", user_name));
            }
            console.info(cformat!("  <s>Roles</>: {}", acc_info.roles()));

            console.info(cformat!(
                "  <s>Balance</>: {}",
                acc_info.balance().to_formatted_string(&Locale::en)
            ));
            if let Some(payment_method) = acc_info.payment_method() {
                // Formatting: {Brand} **** {Last4} ({ExpMonth}/{ExpYear})
                let pm_string = format!(
                    "{} **** {} ({}/{})",
                    payment_method.brand(),
                    payment_method.last4(),
                    payment_method.exp_month(),
                    payment_method.exp_year()
                );
                console.info(cformat!(
                    "  <s>Payment method</>: {} ({})",
                    pm_string,
                    payment_method.pm_id()
                ));
            }

            0
        }
        Err(e) => {
            console.error(cformat!("Unable to fetch account info: {}", e));
            1
        }
    }
}

pub(crate) fn nids_account_revoke(account: &Config, console: &crate::term::Terminal) -> ExitCode {
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
        crate::r#impl::Implementations::Nids,
        None,
    ) {
        Ok(_) => {
            console.info(cformat!(
                "Successfully deleted <magenta,bold>{}</>",
                account.id
            ));
            0
        }
        Err(err) => {
            console.error(format!("Failed to delete account: {err}"));
            1
        }
    }
}
