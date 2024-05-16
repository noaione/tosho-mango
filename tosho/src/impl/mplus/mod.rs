use clap::{Subcommand, ValueEnum};

pub(crate) mod accounts;
pub(super) mod common;
pub(crate) mod config;
pub(crate) mod manga;

#[derive(Subcommand, Clone)]
pub(crate) enum MPlusCommands {
    /// Authenticate tosho with your M+ account
    Auth {
        /// Session ID
        session_id: String,
        /// Device kind/type to use
        #[arg(short, long, value_enum, default_value = "android")]
        r#type: crate::r#impl::mplus::accounts::DeviceKind,
    },
    /// Get an account information
    Account,
    /// See all the accounts you have authenticated with
    Accounts,
    /// Revoke or delete an account
    Revoke,
    /// Search for a title
    Search {
        /// Query to search for
        query: String,
    },
}

#[derive(Clone, Default)]
pub(crate) enum MPlusLanguage {
    #[default]
    English,
    Spanish,
    French,
    Indonesian,
    BrazilianPortuguese,
    Russian,
    Thai,
    German,
    // Italian,
    Vietnamese,
}

impl ValueEnum for MPlusLanguage {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            MPlusLanguage::English => Some(clap::builder::PossibleValue::new("en")),
            MPlusLanguage::Spanish => Some(clap::builder::PossibleValue::new("es")),
            MPlusLanguage::French => Some(clap::builder::PossibleValue::new("fr")),
            MPlusLanguage::Indonesian => Some(clap::builder::PossibleValue::new("id")),
            MPlusLanguage::BrazilianPortuguese => Some(clap::builder::PossibleValue::new("pt-br")),
            MPlusLanguage::Russian => Some(clap::builder::PossibleValue::new("ru")),
            MPlusLanguage::Thai => Some(clap::builder::PossibleValue::new("th")),
            MPlusLanguage::German => Some(clap::builder::PossibleValue::new("de")),
            // MPlusLanguage::Italian => Some(clap::builder::PossibleValue::new("it")),
            MPlusLanguage::Vietnamese => Some(clap::builder::PossibleValue::new("vi")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[
            MPlusLanguage::English,
            MPlusLanguage::Spanish,
            MPlusLanguage::French,
            MPlusLanguage::Indonesian,
            MPlusLanguage::BrazilianPortuguese,
            MPlusLanguage::Russian,
            MPlusLanguage::Thai,
            MPlusLanguage::German,
            // MPlusLanguage::Italian,
            MPlusLanguage::Vietnamese,
        ]
    }

    fn from_str(s: &str, ignore_case: bool) -> Result<Self, String> {
        let s = if ignore_case {
            s.to_lowercase()
        } else {
            s.to_string()
        };
        match s.as_str() {
            "en" => Ok(MPlusLanguage::English),
            "es" => Ok(MPlusLanguage::Spanish),
            "fr" => Ok(MPlusLanguage::French),
            "id" => Ok(MPlusLanguage::Indonesian),
            "pt-br" => Ok(MPlusLanguage::BrazilianPortuguese),
            "ru" => Ok(MPlusLanguage::Russian),
            "th" => Ok(MPlusLanguage::Thai),
            "de" => Ok(MPlusLanguage::German),
            // "it" => Ok(MPlusLanguage::Italian),
            "vi" => Ok(MPlusLanguage::Vietnamese),
            _ => Err(format!("Invalid language: {}", s)),
        }
    }
}

impl From<MPlusLanguage> for tosho_mplus::proto::Language {
    fn from(lang: MPlusLanguage) -> Self {
        match lang {
            MPlusLanguage::English => tosho_mplus::proto::Language::English,
            MPlusLanguage::Spanish => tosho_mplus::proto::Language::Spanish,
            MPlusLanguage::French => tosho_mplus::proto::Language::French,
            MPlusLanguage::Indonesian => tosho_mplus::proto::Language::Indonesian,
            MPlusLanguage::BrazilianPortuguese => tosho_mplus::proto::Language::BrazilianPortuguese,
            MPlusLanguage::Russian => tosho_mplus::proto::Language::Russian,
            MPlusLanguage::Thai => tosho_mplus::proto::Language::Thai,
            MPlusLanguage::German => tosho_mplus::proto::Language::German,
            // MPlusLanguage::Italian => tosho_mplus::proto::Language::Italian,
            MPlusLanguage::Vietnamese => tosho_mplus::proto::Language::Vietnamese,
        }
    }
}
