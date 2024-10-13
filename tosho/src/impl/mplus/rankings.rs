use clap::ValueEnum;
use color_print::cformat;
use tosho_macros::EnumName;
use tosho_mplus::{helper::RankingType, MPClient};

use crate::cli::ExitCode;

use super::common::do_print_search_information;

#[derive(Debug, Clone, Default, EnumName)]
pub(crate) enum RankingKind {
    /// The current hottest title ranking
    #[default]
    Hottest,
    /// The currently trending title ranking
    Trending,
    /// Completed title ranking
    Completed,
}

impl ValueEnum for RankingKind {
    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        let input = if ignore_case {
            input.to_lowercase()
        } else {
            input.to_string()
        };
        match input.as_str() {
            "hot" | "hottest" => Ok(Self::Hottest),
            "trending" => Ok(Self::Trending),
            "complete" | "completed" => Ok(Self::Completed),
            _ => Err(format!("Invalid ranking kind: {}", input)),
        }
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::Hottest => Some(clap::builder::PossibleValue::new("hot")),
            Self::Trending => Some(clap::builder::PossibleValue::new("trending")),
            Self::Completed => Some(clap::builder::PossibleValue::new("complete")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Hottest, Self::Trending, Self::Completed]
    }
}

impl From<RankingKind> for RankingType {
    fn from(value: RankingKind) -> Self {
        match value {
            RankingKind::Hottest => Self::Hottest,
            RankingKind::Trending => Self::Trending,
            RankingKind::Completed => Self::Completed,
        }
    }
}

pub(crate) async fn mplus_home_rankings(
    kind: RankingKind,
    client: &MPClient,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info("Getting rankings list for M+");

    let results = client.get_title_ranking(Some(kind.clone().into())).await;

    match results {
        Ok(tosho_mplus::APIResponse::Success(results)) => {
            let all_titles: Vec<tosho_mplus::proto::Title> = results
                .titles()
                .iter()
                .map(|title| title.titles().first().unwrap().clone())
                .collect();

            console.info(cformat!(
                "Ranking for <m,s>{}</> ({} titles):",
                kind.to_name(),
                all_titles.len()
            ));
            do_print_search_information(&all_titles, true, None);

            0
        }
        Ok(tosho_mplus::APIResponse::Error(e)) => {
            console.error(format!("Failed to get rankings list: {}", e.as_string()));
            1
        }
        Err(e) => {
            console.error(format!("Unable to connect to M+: {}", e));
            1
        }
    }
}
