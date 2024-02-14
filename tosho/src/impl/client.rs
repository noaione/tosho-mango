use crate::{
    config::{get_all_config, get_config},
    term::ConsoleChoice,
};

pub(crate) fn select_single_account(
    account_id: Option<&str>,
    implementation: super::Implementations,
    term: &crate::term::Terminal,
) -> Option<crate::config::ConfigImpl> {
    if let Some(account_id) = account_id {
        let config = get_config(account_id, &implementation, None);

        if let Some(config) = config {
            return Some(config.clone());
        }

        term.warn(&format!("Account ID {} not found!", account_id));
    }

    let all_configs = get_all_config(&implementation, None);
    let all_choices: Vec<ConsoleChoice> = all_configs
        .iter()
        .map(|c| match c {
            crate::config::ConfigImpl::Amap(c) => ConsoleChoice {
                name: c.id.clone(),
                value: format!("{} - {} [{}]", c.id, c.email, c.r#type().to_name()),
            },
            crate::config::ConfigImpl::Kmkc(c) => match c {
                super::kmkc::config::Config::Mobile(cc) => ConsoleChoice {
                    name: cc.id.clone(),
                    value: format!(
                        "{} [{} - {}]",
                        cc.id,
                        cc.r#type().to_name(),
                        cc.platform().to_name()
                    ),
                },
                super::kmkc::config::Config::Web(cc) => ConsoleChoice {
                    name: cc.id.clone(),
                    value: format!("{} [{}]", cc.id, cc.r#type().to_name()),
                },
            },
            crate::config::ConfigImpl::Musq(c) => ConsoleChoice {
                name: c.id.clone(),
                value: format!("{} [{}]", c.id, c.r#type().to_name()),
            },
            crate::config::ConfigImpl::Sjv(c) => ConsoleChoice {
                name: c.id.clone(),
                value: format!(
                    "{} [{} - {}]",
                    c.id,
                    c.r#type().to_name(),
                    c.mode().to_name()
                ),
            },
        })
        .collect();

    if all_configs.is_empty() {
        term.warn("No accounts found!");
        return None;
    }

    // only 1? return
    if all_configs.len() == 1 {
        return Some(all_configs[0].clone());
    }

    let selected = term.choice("Select an account:", all_choices);
    match selected {
        Some(selected) => {
            let config = all_configs
                .iter()
                .find(|&c| match c {
                    crate::config::ConfigImpl::Amap(c) => c.id == selected.name,
                    crate::config::ConfigImpl::Kmkc(c) => match c {
                        super::kmkc::config::Config::Mobile(cc) => cc.id == selected.name,
                        super::kmkc::config::Config::Web(cc) => cc.id == selected.name,
                    },
                    crate::config::ConfigImpl::Musq(c) => c.id == selected.name,
                    crate::config::ConfigImpl::Sjv(c) => c.id == selected.name,
                })
                .unwrap();

            Some(config.clone())
        }
        None => None,
    }
}

pub(crate) fn make_musq_client(config: &super::musq::config::Config) -> tosho_musq::MUClient {
    let constants = tosho_musq::constants::get_constants(config.r#type() as u8);

    tosho_musq::MUClient::new(&config.session, constants.clone())
}

pub(crate) fn make_kmkc_client(config: &tosho_kmkc::KMConfig) -> tosho_kmkc::KMClient {
    tosho_kmkc::KMClient::new(config.clone())
}

pub(crate) fn make_amap_client(config: &tosho_amap::AMConfig) -> tosho_amap::AMClient {
    tosho_amap::AMClient::new(config.clone())
}

pub(crate) fn make_sjv_client(config: &super::sjv::config::Config) -> tosho_sjv::SJClient {
    let mode = match config.mode() {
        crate::r#impl::sjv::config::SJDeviceMode::SJ => tosho_sjv::SJMode::SJ,
        crate::r#impl::sjv::config::SJDeviceMode::VM => tosho_sjv::SJMode::VM,
    };
    tosho_sjv::SJClient::new(config.clone().into(), mode)
}
