use color_print::cformat;
use tosho_rbean::{RBClient, RBPlatform};

use crate::{
    cli::ExitCode,
    config::{get_all_config, save_config, try_remove_config},
};

use super::{
    common::save_session_config,
    config::{Config, DeviceType},
};

pub async fn rbean_account_login(
    email: String,
    password: String,
    platform: DeviceType,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(&cformat!(
        "Authenticating with email <m,s>{}</> and password <m,s>{}</>...",
        email,
        password
    ));

    let rb_platform = match platform {
        DeviceType::Android => RBPlatform::Android,
        DeviceType::Apple => RBPlatform::Apple,
        DeviceType::Web => RBPlatform::Web,
    };

    let all_configs = get_all_config(&crate::r#impl::Implementations::Rbean, None);

    let old_config = all_configs.iter().find(|&c| match c {
        crate::config::ConfigImpl::Rbean(cc) => cc.email == email && cc.platform() == platform,
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

    let login_results = RBClient::login(&email, &password, rb_platform).await;

    match login_results {
        Ok(login_result) => {
            let new_config: Config = login_result.into();

            console.info(&cformat!(
                "Authenticated as <m,s>{}</> ({})...",
                new_config.username,
                email
            ));

            let new_config = if let Some(old_id) = old_id {
                new_config.with_id(&old_id)
            } else {
                new_config
            };

            console.info(&cformat!(
                "Created session ID <m,s>{}</>, saving config...",
                new_config.id
            ));

            save_config(new_config.into(), None);

            0
        }
        Err(e) => {
            console.error(&format!("Failed to authenticate: {}", e));
            1
        }
    }
}

pub(crate) fn rbean_accounts(console: &crate::term::Terminal) -> ExitCode {
    let all_configs = get_all_config(&crate::r#impl::Implementations::Rbean, None);

    match all_configs.len() {
        0 => {
            console.warn("No accounts found!");

            1
        }
        _ => {
            console.info(&format!("Found {} accounts:", all_configs.len()));
            for (i, c) in all_configs.iter().enumerate() {
                match c {
                    crate::config::ConfigImpl::Rbean(c) => {
                        let plat_name = c.platform().to_name();
                        console.info(&cformat!(
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

pub(crate) async fn rbean_account_info(
    client: &mut RBClient,
    account: &Config,
    console: &crate::term::Terminal,
) -> ExitCode {
    let acc_info = client.get_user().await;

    match acc_info {
        Ok(acc_info) => {
            save_session_config(client, account);

            console.info(&cformat!(
                "Account info for <magenta,bold>{}</>:",
                account.id
            ));

            console.info(&cformat!("  <s>ID</>: {}", acc_info.uuid));
            console.info(&cformat!("  <s>Email</>: {}", acc_info.email));
            let username = acc_info.username.unwrap_or("[no username]".to_string());
            console.info(&cformat!("  <s>Username</>: {}", username));

            if let Some(date_at) = acc_info.premium_expiration_date {
                console.info(&cformat!("  <s>Premium until</>: {}", date_at));
            }

            0
        }
        Err(e) => {
            console.error(&format!("Failed to get account info: {}", e));
            1
        }
    }
}

pub(crate) fn rbean_account_revoke(account: &Config, console: &crate::term::Terminal) -> ExitCode {
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
        crate::r#impl::Implementations::Rbean,
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
