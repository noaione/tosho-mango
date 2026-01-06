use clap::ValueEnum;
use color_eyre::eyre::Context;
use color_print::cformat;

use crate::config::{get_all_config, save_config, try_remove_config};

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
) -> color_eyre::Result<()> {
    let r#type = match device_kind {
        DeviceKind::Web => DeviceType::Web,
    };

    let all_configs = get_all_config(&crate::r#impl::Implementations::Nids, None);

    let cut_token = session_token.chars().take(8).collect::<String>();
    console.info(cformat!(
        "Authenticating with session token <m,s>{}********</>...",
        cut_token
    ));

    let random_uuid = uuid::Uuid::new_v4().to_string();
    let mut config = Config::from_session(&session_token, r#type);
    config.apply_id(&random_uuid);

    let client =
        crate::r#impl::client::make_nids_client(&config).context("Unable to create client")?;

    let user_info = client
        .get_profile()
        .await
        .context("Unable to authenticate")?;

    let old_config = all_configs.iter().find(|&c| match c {
        crate::config::ConfigImpl::Nids(c) => c.id == user_info.id(),
        _ => false,
    });

    if let Some(old_config) = old_config {
        console.warn("Session ID already authenticated!");
        let abort_it = console.confirm(Some("Do you want to replace it?"));
        if !abort_it {
            console.info("Aborting...");
            return Err(color_eyre::eyre::eyre!("Aborted by user"));
        }

        match old_config {
            crate::config::ConfigImpl::Nids(c) => {
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

    Ok(())
}

pub(crate) async fn nids_auth_email(
    email: String,
    password: String,
    device_kind: DeviceKind,
    proxy: Option<&reqwest::Proxy>,
    console: &crate::term::Terminal,
) -> color_eyre::Result<()> {
    let r#type = match device_kind {
        DeviceKind::Web => DeviceType::Web,
    };

    console.info(cformat!("Authenticating with email <m,s>{}</>...", &email));

    let all_configs = get_all_config(&crate::r#impl::Implementations::Nids, None);

    let token_results = tosho_nids::NIClient::login(email, password, proxy.cloned())
        .await
        .context("Unable to authenticate")?;

    let token_data = token_results.data();
    let old_config = all_configs.iter().find(|&c| match c {
        crate::config::ConfigImpl::Nids(c) => c.id == token_data.user().id(),
        _ => false,
    });

    let mut config = Config {
        id: token_data.user().id().to_string(),
        session: token_data.tokens().access_token().to_string(),
        refresh_token: Some(token_data.tokens().refresh_token().to_string()),
        email: token_data.user().email().to_string(),
        username: token_data.user().username().map(|s| s.to_string()),
        r#type: r#type as i32,
    };

    if let Some(old_config) = old_config {
        console.warn("Session ID already authenticated!");
        let abort_it = console.confirm(Some("Do you want to replace it?"));
        if !abort_it {
            console.info("Aborting...");
            return Err(color_eyre::eyre::eyre!("Aborted by user"));
        }

        match old_config {
            crate::config::ConfigImpl::Nids(c) => {
                config.apply_id(&c.id);
            }
            _ => unreachable!(),
        }
    }

    console.info(cformat!(
        "Authenticated as <m,s>{}</> ({})...",
        token_data.user().username().unwrap_or("Unknown"),
        token_data.user().email()
    ));

    console.info(cformat!(
        "Created session ID <m,s>{}</>, saving config...",
        config.get_id()
    ));
    save_config(crate::config::ConfigImpl::Nids(config), None);

    Ok(())
}

pub(crate) fn nids_accounts(console: &crate::term::Terminal) -> color_eyre::Result<()> {
    let all_configs = get_all_config(&crate::r#impl::Implementations::Nids, None);

    match all_configs.len() {
        0 => {
            console.warn("No accounts found!");

            Ok(())
        }
        other => {
            console.info(format!("Found {} accounts:", other));
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

            Ok(())
        }
    }
}

pub(crate) async fn nids_account_info(
    client: &tosho_nids::NIClient,
    account: &Config,
    console: &crate::term::Terminal,
) -> color_eyre::Result<()> {
    let acc_info = client
        .get_profile()
        .await
        .context("Failed to fetch account info")?;

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
    if !acc_info.roles().is_empty() {
        console.info(cformat!("  <s>Roles</>: {}", acc_info.roles()));
    }

    console.info(cformat!(
        "  <s>Balance</>: <g,s>$</g,s>{}",
        tosho_nids::format_price(acc_info.balance())
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

    Ok(())
}

pub(crate) async fn nids_account_refresh(
    refresh_token: Option<&str>,
    client: &tosho_nids::NIClient,
    account: &Config,
    console: &crate::term::Terminal,
) -> color_eyre::Result<()> {
    console.info(cformat!(
        "Refreshing tokens for account <m,s>{}</>...",
        account.id
    ));

    let refresh_token = match refresh_token {
        Some(token) => token.to_string(),
        None => {
            let refresh = account.refresh_token();
            if !refresh.is_empty() {
                refresh.to_string()
            } else {
                console.error("No refresh token found for this account!");
                return Err(color_eyre::eyre::eyre!(
                    "No refresh token found for this account"
                ));
            }
        }
    };

    let token_results = client
        .refresh_token(refresh_token)
        .await
        .context("Unable to refresh tokens")?;

    let token_data = token_results.data();

    console.info(cformat!(
        "Successfully refreshed tokens for account <m,s>{}</>.",
        account.id
    ));

    let mut new_account = account.clone();
    new_account.session = token_data.tokens().access_token().to_string();
    new_account.refresh_token = Some(token_data.tokens().refresh_token().to_string());

    console.info(cformat!(
        "Saving updated config for account <m,s>{}</>...",
        account.id
    ));
    save_config(crate::config::ConfigImpl::Nids(new_account), None);

    Ok(())
}

pub(crate) fn nids_account_revoke(
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
        crate::r#impl::Implementations::Nids,
        None,
    )
    .context("Failed to delete account")?;

    console.info(cformat!(
        "Successfully deleted <magenta,bold>{}</>",
        account.get_id()
    ));

    Ok(())
}
