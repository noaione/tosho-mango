use directories::BaseDirs;
use prost::Message;
use std::{
    io::{Cursor, Read, Write},
    path::PathBuf,
};

use crate::r#impl::Implementations;

/// Macro expansion to generate functions for reading and getting config files.
macro_rules! config_reader {
    (
        $($name:literal)*,
        $($rimpl:ident)*,
        $($struct:ident)*
    ) => {
        $(
            struct $struct;

            impl $struct {
                #[doc = "Read a single config file for "]
                #[doc = $name]
                #[doc = " source."]
                fn read_config(user_conf: PathBuf) -> Option<$crate::r#impl::$rimpl::config::Config> {
                    if !user_conf.exists() {
                        None
                    } else {
                        let mut file = std::fs::File::open(user_conf).unwrap();
                        let mut buffer = Vec::new();
                        file.read_to_end(&mut buffer).unwrap();
                        drop(file);
                        <$crate::r#impl::$rimpl::config::Config>::decode(&mut Cursor::new(buffer)).ok()
                    }
                }

                #[doc = "Get a single config file for "]
                #[doc = $name]
                #[doc = " source."]
                fn get_config(
                    id: &str,
                    user_path: PathBuf,
                ) -> Option<$crate::r#impl::$rimpl::config::Config> {
                    let mut user_conf = user_path;
                    user_conf.push(format!(
                        "{}.{}.tmconf",
                        crate::r#impl::$rimpl::config::PREFIX,
                        id
                    ));

                    $struct::read_config(user_conf)
                }
            }
        )*
    };
}

/// Macro expansion to generate functions for saving config files.
///
/// This takes 4 arguments:
/// 1. The user path to save the config file.
/// 2. The config to save.
/// 3. The list of "Config" enum value
/// 4. The list of implementation name.
macro_rules! save_config_impl {
    (
        $user_path:expr_2021, $config:expr_2021,
        $($handlebar:ident)*,
        $($prefix:ident)*
    ) => {
        match $config {
            $(
                ConfigImpl::$handlebar(config) => {
                    let mut user_conf = $user_path.clone();
                    user_conf.push(format!("{}.{}.tmconf", $crate::r#impl::$prefix::config::PREFIX, config.get_id()));

                    let mut file = std::fs::File::create(user_conf).unwrap();
                    let mut buffer = Vec::new();
                    config.encode(&mut buffer).unwrap();
                    file.write_all(&buffer).unwrap();
                    drop(file);
                }
            )*
        }
    };
}

/// Macro expansion to convert each config implementation to this file [`ConfigImpl`] enum.
///
/// This takes 2 arguments:
/// 1. The list of "Config" enum value
/// 2. The list of implementation name (variant).
macro_rules! config_to_configimpl {
    (
        $($config:ident)*,
        $($variant:ident)*
    ) => {
        $(
            impl From<$crate::r#impl::$config::config::Config> for ConfigImpl {
                fn from(config: $crate::r#impl::$config::config::Config) -> Self {
                    ConfigImpl::$variant(config)
                }
            }
        )*
    };
}

macro_rules! config_match_expand {
    // get_config
    (
        $id:expr_2021, $user_path:expr_2021, $base_impl:expr_2021,
        $($handlebar:ident)*,
        $($get_conf:path)*
    ) => {
        match $base_impl {
            $(
                Implementations::$handlebar => {
                    let conf = $get_conf($id, $user_path);
                    conf.map(ConfigImpl::$handlebar)
                }
            )*
        }
    };
    // get_all_config
    (
        $entry:expr_2021, $base_impl:expr_2021,
        $($handlebar:ident)*,
        $($read_conf:path)*
    ) => {
        match $base_impl {
            $(
                Implementations::$handlebar => {
                    let conf = $read_conf($entry);
                    conf.map(ConfigImpl::$handlebar)
                }
            )*
        }
    };
    // prefix_expansion
    (
        $base_impl:expr_2021,
        $($handlebar:ident)*,
        $($prefix:ident)*
    ) => {
        match $base_impl {
            $(
                Implementations::$handlebar => $crate::r#impl::$prefix::config::PREFIX,
            )*
        }
    }
}

/// The many type of config files.
#[derive(Clone)]
pub enum ConfigImpl {
    Kmkc(crate::r#impl::kmkc::config::Config),
    Musq(crate::r#impl::musq::config::Config),
    Amap(crate::r#impl::amap::config::Config),
    Sjv(crate::r#impl::sjv::config::Config),
    Rbean(crate::r#impl::rbean::config::Config),
    Mplus(crate::r#impl::mplus::config::Config),
}

// Adapt web/mobile
impl From<crate::r#impl::kmkc::config::ConfigWeb> for ConfigImpl {
    fn from(config: crate::r#impl::kmkc::config::ConfigWeb) -> Self {
        ConfigImpl::Kmkc(config.into())
    }
}

impl From<crate::r#impl::kmkc::config::ConfigMobile> for ConfigImpl {
    fn from(config: crate::r#impl::kmkc::config::ConfigMobile) -> Self {
        ConfigImpl::Kmkc(config.into())
    }
}

pub(crate) fn get_user_path() -> std::path::PathBuf {
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

// Implement Config -> ConfigImpl
config_to_configimpl!(
    kmkc musq amap sjv rbean mplus,
    Kmkc Musq Amap Sjv Rbean Mplus
);

// Create config reader functions
config_reader!(
    "KM by KC" "MU! by SQ" "AM by AP" "SJ/M by V" "小豆 by KRKR" "M+ by S",
    kmkc musq amap sjv rbean mplus,
    KMConfRead MUConfRead AMConfRead SJVConfRead RBeanConfRead MPlusConfRead
);

pub fn get_config(
    id: &str,
    r#impl: &Implementations,
    user_path: Option<PathBuf>,
) -> Option<ConfigImpl> {
    let user_path = user_path.unwrap_or(get_user_path());

    config_match_expand!(
        id, user_path, r#impl,
        Kmkc Musq Amap Sjv Rbean Mplus,
        KMConfRead::get_config MUConfRead::get_config AMConfRead::get_config SJVConfRead::get_config RBeanConfRead::get_config MPlusConfRead::get_config
    )
}

pub fn get_all_config(r#impl: &Implementations, user_path: Option<PathBuf>) -> Vec<ConfigImpl> {
    let user_path = user_path.unwrap_or(get_user_path());

    if !user_path.exists() {
        std::fs::create_dir_all(user_path.clone()).unwrap();
    }

    // glob .tmconf files
    let mut glob_path = user_path.clone();
    let prefix = config_match_expand!(
        r#impl,
        Kmkc Musq Amap Sjv Rbean Mplus,
        kmkc musq amap sjv rbean mplus
    );
    glob_path.push(format!("{prefix}.*.tmconf"));

    glob::glob(glob_path.to_str().unwrap()).expect("Failed to read glob pattern")
        .flatten()
        .filter_map(|entry| {
            config_match_expand!(
                entry, r#impl,
                Kmkc Musq Amap Sjv Rbean Mplus,
                KMConfRead::read_config MUConfRead::read_config AMConfRead::read_config SJVConfRead::read_config RBeanConfRead::read_config MPlusConfRead::read_config
            )
        })
        .collect()
}

pub fn save_config(config: ConfigImpl, user_path: Option<PathBuf>) {
    let user_path = user_path.unwrap_or(get_user_path());

    if !user_path.exists() {
        std::fs::create_dir_all(user_path.clone()).unwrap();
    }

    save_config_impl!(
        user_path, config,
        Kmkc Musq Amap Sjv Rbean Mplus,
        kmkc musq amap sjv rbean mplus
    )
}

pub fn try_remove_config(
    id: &str,
    r#impl: Implementations,
    user_path: Option<PathBuf>,
) -> std::io::Result<()> {
    let user_path = user_path.unwrap_or(get_user_path());

    let mut user_conf = user_path.clone();
    let prefix = config_match_expand!(
        r#impl,
        Kmkc Musq Amap Sjv Rbean Mplus,
        kmkc musq amap sjv rbean mplus
    );
    user_conf.push(format!("{prefix}.{id}.tmconf"));

    if user_conf.exists() {
        std::fs::remove_file(user_conf)
    } else {
        Ok(())
    }
}
