use clap::ValueEnum;

use crate::{
    cli::ExitCode,
    config::{get_all_config, save_config},
};

use super::config::{Config, DeviceType};

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

    let all_configs = get_all_config(crate::r#impl::Implementations::MUSQ, None);
    let old_config = all_configs.iter().find(|&c| match c {
        crate::config::ConfigImpl::KMKC(_) => false,
        crate::config::ConfigImpl::MUSQ(c) => c.session == session_id && c.r#type == r#type as i32,
    });

    if let Some(_) = old_config {
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
            save_config(crate::config::ConfigImpl::MUSQ(config), None);
            0
        }
        Err(e) => {
            console.error(&format!("Authentication failed: {}", e));
            1
        }
    }
}
