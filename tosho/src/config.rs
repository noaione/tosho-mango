use directories::BaseDirs;
use prost::Message;
use std::{
    io::{Cursor, Read, Write},
    path::PathBuf,
};

use crate::r#impl::Implementations;

/// The many type of config files.
pub enum ConfigImpl {
    KMKC(crate::r#impl::kmkc::config::Config),
    MUSQ(crate::r#impl::musq::config::Config),
}

fn get_user_path() -> std::path::PathBuf {
    #[cfg(windows)]
    let user_path = {
        let mut local_appdata: std::path::PathBuf =
            BaseDirs::new().unwrap().config_local_dir().to_path_buf();
        local_appdata.push("ToshoMango");
        local_appdata
    };
    #[cfg(not(windows))]
    let user_path: std::path::PathBuf = {
        let mut home = BaseDirs::new().unwrap().home_dir().to_path_buf();
        home.push(".toshomango");
        home
    };
    user_path
}

//--> Reader <--//
fn read_kmkc_config(user_conf: PathBuf) -> Option<crate::r#impl::kmkc::config::Config> {
    if !user_conf.exists() {
        None
    } else {
        let mut file = std::fs::File::open(user_conf).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        drop(file);
        let conf_temp =
            crate::r#impl::kmkc::config::ConfigBase::decode(&mut Cursor::new(buffer.clone()))
                .unwrap();

        match conf_temp.r#type() {
            crate::r#impl::kmkc::config::DeviceType::Web => {
                let conf = crate::r#impl::kmkc::config::ConfigWeb::decode(&mut Cursor::new(buffer))
                    .unwrap();
                Some(conf.into())
            }
            crate::r#impl::kmkc::config::DeviceType::Mobile => {
                let conf =
                    crate::r#impl::kmkc::config::ConfigMobile::decode(&mut Cursor::new(buffer))
                        .unwrap();
                Some(conf.into())
            }
        }
    }
}

fn get_config_kmkc(id: &str, user_path: PathBuf) -> Option<crate::r#impl::kmkc::config::Config> {
    let mut user_conf = user_path;
    user_conf.push(format!(
        "{}.{}.tmconf",
        crate::r#impl::kmkc::config::PREFIX,
        id
    ));

    read_kmkc_config(user_conf)
}

fn read_musq_config(user_conf: PathBuf) -> Option<crate::r#impl::musq::config::Config> {
    if !user_conf.exists() {
        None
    } else {
        let mut file = std::fs::File::open(user_conf).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        drop(file);
        let conf = crate::r#impl::musq::config::Config::decode(&mut Cursor::new(buffer)).unwrap();
        Some(conf)
    }
}

fn get_config_musq(id: &str, user_path: PathBuf) -> Option<crate::r#impl::musq::config::Config> {
    let mut user_conf = user_path;
    user_conf.push(format!(
        "{}.{}.tmconf",
        crate::r#impl::musq::config::PREFIX,
        id
    ));

    read_musq_config(user_conf)
}

pub fn get_config(
    id: &str,
    r#impl: Implementations,
    user_path: Option<PathBuf>,
) -> Option<ConfigImpl> {
    let user_path = user_path.unwrap_or(get_user_path());

    match r#impl {
        Implementations::KMKC => {
            let conf = get_config_kmkc(id, user_path);
            match conf {
                Some(conf) => Some(ConfigImpl::KMKC(conf)),
                None => None,
            }
        }
        Implementations::MUSQ => {
            let conf = get_config_musq(id, user_path);
            match conf {
                Some(conf) => Some(ConfigImpl::MUSQ(conf)),
                None => None,
            }
        }
    }
}

pub fn get_all_config(r#impl: Implementations, user_path: Option<PathBuf>) -> Vec<ConfigImpl> {
    let user_path = user_path.unwrap_or(get_user_path());

    if !user_path.exists() {
        std::fs::create_dir_all(user_path.clone()).unwrap();
    }

    // glob .tmconf files
    let mut glob_path = user_path.clone();
    let prefix = match r#impl {
        Implementations::KMKC => crate::r#impl::kmkc::config::PREFIX,
        Implementations::MUSQ => crate::r#impl::musq::config::PREFIX,
    };
    glob_path.push(format!("{}.*.tmconf", prefix));

    let mut matched_entries: Vec<ConfigImpl> = Vec::new();
    for entry in glob::glob(glob_path.to_str().unwrap()).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => match r#impl {
                Implementations::KMKC => {
                    let conf = read_kmkc_config(path);
                    match conf {
                        Some(conf) => {
                            matched_entries.push(ConfigImpl::KMKC(conf));
                        }
                        None => {}
                    }
                }
                Implementations::MUSQ => {
                    let conf = read_musq_config(path);
                    match conf {
                        Some(conf) => {
                            matched_entries.push(ConfigImpl::MUSQ(conf));
                        }
                        None => {}
                    }
                }
            },
            Err(_) => {}
        }
    }
    matched_entries
}

pub fn save_config(config: ConfigImpl, user_path: Option<PathBuf>) {
    let user_path = user_path.unwrap_or(get_user_path());

    if !user_path.exists() {
        std::fs::create_dir_all(user_path.clone()).unwrap();
    }

    match config {
        ConfigImpl::KMKC(config) => {
            let mut user_conf = user_path.clone();
            let conf_id = match config.clone() {
                crate::r#impl::kmkc::config::Config::Mobile(config) => config.id,
                crate::r#impl::kmkc::config::Config::Web(config) => config.id,
            };
            user_conf.push(format!(
                "{}.{}.tmconf",
                crate::r#impl::kmkc::config::PREFIX,
                conf_id,
            ));

            let mut file = std::fs::File::create(user_conf).unwrap();
            let mut buffer = Vec::new();

            match config {
                crate::r#impl::kmkc::config::Config::Mobile(config) => {
                    config.encode(&mut buffer).unwrap();
                }
                crate::r#impl::kmkc::config::Config::Web(config) => {
                    config.encode(&mut buffer).unwrap();
                }
            }
            file.write_all(&buffer).unwrap();
            drop(file);
        }
        ConfigImpl::MUSQ(config) => {
            let mut user_conf = user_path.clone();
            user_conf.push(format!(
                "{}.{}.tmconf",
                crate::r#impl::musq::config::PREFIX,
                config.id
            ));

            let mut file = std::fs::File::create(user_conf).unwrap();
            let mut buffer = Vec::new();
            config.encode(&mut buffer).unwrap();
            file.write_all(&buffer).unwrap();
            drop(file);
        }
    }
}
