use color_print::cformat;
use tosho_sjv::{SJClient, SJConfig, SJMode, SJPlatform};

use crate::{
    cli::ExitCode,
    config::{get_all_config, save_config, try_remove_config},
};

use super::{
    common::unix_timestamp_to_string,
    config::{Config, DeviceType, SJDeviceMode},
};

pub async fn sjv_account_login(
    email: String,
    password: String,
    mode: SJDeviceMode,
    platform: DeviceType,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(&cformat!(
        "Authenticating with email <m,s>{}</> and password <m,s>{}</>...",
        email,
        password
    ));

    let sj_platform = match platform {
        DeviceType::Android => SJPlatform::Android,
        DeviceType::Apple => SJPlatform::Apple,
        DeviceType::Web => SJPlatform::Web,
    };

    let all_configs = get_all_config(&crate::r#impl::Implementations::Sjv, None);

    let old_config = all_configs.iter().find(|&c| match c {
        crate::config::ConfigImpl::Sjv(cc) => {
            cc.email == email && cc.mode() == mode && cc.r#type() == platform
        }
        _ => false,
    });

    let mut old_id: Option<String> = None;
    if let Some(old_config) = old_config {
        console.warn("Email already authenticated!");
        let abort_it = console.confirm(Some("Do you want to replace it?"));
        if !abort_it {
            console.info("Aborting...");
            return 0;
        }

        match old_config {
            crate::config::ConfigImpl::Amap(c) => {
                old_id = Some(c.id.clone());
            }
            _ => unreachable!(),
        }
    }

    let act_mode = match mode {
        SJDeviceMode::SJ => SJMode::SJ,
        SJDeviceMode::VM => SJMode::VM,
    };

    let results = SJClient::login(&email, &password, act_mode, sj_platform).await;

    match results {
        Ok((account, instance_id)) => {
            let config: SJConfig =
                SJConfig::from_login_response(&account, instance_id, sj_platform);

            console.info(&cformat!(
                "Authenticated as <m,s>{}</> ({})",
                account.username,
                email,
            ));

            let new_config: Config = config.into();
            let new_config = new_config
                .with_email(&email)
                .with_username(&account.username)
                .with_mode(mode);

            let new_config = if let Some(old_id) = old_id {
                new_config.with_id(&old_id)
            } else {
                new_config
            };

            console.info(&cformat!(
                "Created session ID <m,s>{}</>, saving config...",
                new_config.id
            ));

            save_config(crate::config::ConfigImpl::Sjv(new_config), None);

            0
        }
        Err(e) => {
            console.error(&format!("Failed to authenticate: {}", e));
            1
        }
    }
}

pub(crate) fn sjv_accounts(console: &crate::term::Terminal) -> ExitCode {
    let all_configs = get_all_config(&crate::r#impl::Implementations::Sjv, None);

    match all_configs.len() {
        0 => {
            console.warn("No accounts found!");

            1
        }
        _ => {
            console.info(&format!("Found {} accounts:", all_configs.len()));
            for (i, c) in all_configs.iter().enumerate() {
                match c {
                    crate::config::ConfigImpl::Sjv(c) => {
                        let plat_name = c.r#type().to_name();
                        let mode_name = c.mode().to_name();
                        console.info(&cformat!(
                            "{:02}. {} — <s>{}</> ({} - {})",
                            i + 1,
                            c.id,
                            c.email,
                            plat_name,
                            mode_name,
                        ));
                    }
                    _ => unreachable!(),
                }
            }

            0
        }
    }
}

pub(crate) async fn sjv_account_info(
    account: &Config,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(&cformat!(
        "Account info for <magenta,bold>{}</>:",
        account.id
    ));

    console.info(&cformat!("  <s>ID</>: {}", account.id));
    console.info(&cformat!("  <s>Email</>: {}", account.email));
    console.info(&cformat!("  <s>Username</>: {}", account.username));

    0
}

pub(crate) async fn sjv_account_subscriptions(
    client: &SJClient,
    account: &Config,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(&cformat!(
        "Getting subscriptions for <magenta,bold>{}</>...",
        account.id
    ));
    let subs_resp = client.get_entitlements().await;

    match subs_resp {
        Ok(subs_resp) => {
            console.info(&cformat!(
                "Subscriptions for <magenta,bold>{}</>:",
                account.id
            ));

            let subs_data = &subs_resp.subscriptions;

            let sj_end_date = match subs_data.sj_valid_to {
                Some(date) => {
                    if let Some(unix_parse) = unix_timestamp_to_string(date) {
                        unix_parse
                    } else {
                        date.to_string()
                    }
                }
                None => "N/A".to_string(),
            };
            let vm_end_date = match subs_data.vm_valid_to {
                Some(date) => {
                    if let Some(unix_parse) = unix_timestamp_to_string(date) {
                        unix_parse
                    } else {
                        date.to_string()
                    }
                }
                None => "N/A".to_string(),
            };

            console.info(&cformat!("  <s>SJ</>: {}", sj_end_date));
            console.info(&cformat!("  <s>VM</>: {}", vm_end_date));

            0
        }
        Err(e) => {
            console.error(&format!("Failed to get subscriptions: {}", e));
            1
        }
    }
}

pub(crate) fn sjv_account_revoke(account: &Config, console: &crate::term::Terminal) -> ExitCode {
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
        crate::r#impl::Implementations::Sjv,
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
