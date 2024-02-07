use clap::ValueEnum;
use color_print::cformat;
use tosho_kmkc::{
    constants::{RankingTab, RANKING_TABS},
    KMClient,
};

use crate::cli::ExitCode;

use super::common::do_print_search_information;

#[derive(Debug, Clone, ValueEnum, Default)]
pub enum RankingType {
    Action = 3,
    Sports = 4,
    Romance = 5,
    Isekai = 6,
    Suspense = 7,
    Outlaws = 8,
    Drama = 9,
    Fantasy = 10,
    Sol = 11,
    #[default]
    All = 12,
    Specials = 13,
}

impl RankingType {
    pub fn get_tab(&self) -> Option<&RankingTab> {
        let find_manual = RANKING_TABS.iter().find(|&t| t.id == self.clone() as u32);
        find_manual
    }
}

pub(crate) async fn kmkc_home_rankings(
    ranking: Option<RankingType>,
    limit: Option<u32>,
    client: &KMClient,
    console: &crate::term::Terminal,
) -> ExitCode {
    let ranking = ranking.unwrap_or_default();

    let rank_tab = match ranking.get_tab() {
        Some(tab) => tab,
        None => {
            console.error(&format!("Invalid ranking type: {:?}", ranking));
            return 1;
        }
    };
    let limit = limit.unwrap_or(25);

    console.info(&cformat!(
        "Getting ranking <magenta,bold>{}</>...",
        rank_tab.name
    ));

    let results = client
        .get_all_rankings(rank_tab.id, Some(limit), Some(0))
        .await;

    match results {
        Err(err) => {
            console.error(&cformat!("Unable to connect to KMKC!: {}", err));
            1
        }
        Ok(results) => {
            if results.titles.is_empty() {
                console.error("There are no rankings available for some reason.");
                return 1;
            }

            console.info(&cformat!(
                "Fetching <m,s>{}</> titles from <m,s>{}</>",
                results.titles.len(),
                rank_tab.name
            ));

            let all_titles = client
                .get_titles(results.titles.iter().map(|t| t.id).collect())
                .await;

            match all_titles {
                Err(err) => {
                    console.error(&cformat!("Failed when fetching title list: {}", err));
                    1
                }
                Ok(titles) => {
                    if titles.is_empty() {
                        console.error("There are no titles available for some reason.");
                        return 1;
                    }

                    console.info(&cformat!(
                        "Ranking <m,s>{}</> (<s>{}</> results)",
                        rank_tab.name,
                        titles.len()
                    ));
                    do_print_search_information(titles, true, None);
                    0
                }
            }
        }
    }
}
