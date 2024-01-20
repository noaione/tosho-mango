use color_print::cformat;
use tosho_amap::AMClient;

use crate::{
    cli::ExitCode,
    config::{get_all_config, save_config},
};

use super::{
    common::{make_client, select_single_account},
    config::Config,
};

pub async fn amap_account_login(
    email: String,
    password: String,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(&cformat!(
        "Authenticating with email <m,s>{}</> and password <m,s>{}</>...",
        email,
        password
    ));

    let all_configs = get_all_config(crate::r#impl::Implementations::Amap, None);

    let old_config = all_configs.iter().find(|&c| match c {
        crate::config::ConfigImpl::Amap(cc) => cc.email == email,
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

    let result = AMClient::login(&email, &password).await;

    match result {
        Ok(session) => {
            console.info(&cformat!(
                "Authenticated as <m,s>{}</> ({})",
                session.identifier,
                email
            ));

            let client = super::common::make_client(&session);
            let account = client.get_account().await;

            let as_config: Config = session.into();

            match account {
                Ok(account) => {
                    let as_config = as_config
                        .with_email(&email)
                        .with_account_info(&account.info);

                    console.info(&cformat!("Logged in as <m,s>{}</>", account.info.name));

                    let final_config = match old_id {
                        Some(old_id) => as_config.with_id(&old_id),
                        None => as_config,
                    };

                    console.info(&cformat!(
                        "Created session ID <m,s>{}</>, saving config...",
                        final_config.id
                    ));

                    save_config(crate::config::ConfigImpl::Amap(final_config), None);

                    0
                }
                Err(e) => {
                    console.error(&format!("Failed to login: {}", e));
                    1
                }
            }
        }
        Err(e) => {
            console.error(&format!("Failed to authenticate: {}", e));
            1
        }
    }
}

pub(crate) fn amap_accounts(console: &crate::term::Terminal) -> ExitCode {
    let all_configs = get_all_config(crate::r#impl::Implementations::Amap, None);

    match all_configs.len() {
        0 => {
            console.warn("No accounts found!");

            1
        }
        _ => {
            console.info(&format!("Found {} accounts:", all_configs.len()));
            for (i, c) in all_configs.iter().enumerate() {
                match c {
                    crate::config::ConfigImpl::Amap(c) => {
                        let plat_name = c.r#type().to_name();
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

pub(crate) async fn amap_account_info(
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
            console.info(&cformat!(
                "Fetching account info for <magenta,bold>{}</>...",
                acc_info.id
            ));

            let client = make_client(&acc_info.clone().into());
            let account = client.get_account().await;

            match account {
                Ok(account) => {
                    let info = account.info;

                    console.info(&cformat!(
                        "Account info for <magenta,bold>{}</>:",
                        acc_info.id
                    ));

                    console.info(&cformat!("  <s>ID</>: {}", info.id));
                    console.info(&cformat!("  <s>Email</>: {}", acc_info.email));
                    console.info(&cformat!("  <s>Username</>: {}", info.name));

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
