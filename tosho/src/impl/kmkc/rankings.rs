use clap::ValueEnum;
use color_eyre::eyre::Context;
use color_print::cformat;
use tosho_kmkc::{
    KMClient,
    constants::{RANKING_TABS, RankingTab},
};

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
        RANKING_TABS.iter().find(|&t| t.id == self.clone() as u32)
    }
}

pub(crate) async fn kmkc_home_rankings(
    ranking: Option<RankingType>,
    limit: Option<u32>,
    client: &KMClient,
    console: &crate::term::Terminal,
) -> color_eyre::Result<()> {
    let ranking = ranking.unwrap_or_default();

    let rank_tab = match ranking.get_tab() {
        Some(tab) => tab,
        None => {
            console.error(format!("Invalid ranking type: {ranking:?}"));
            return Err(color_eyre::eyre::eyre!("Invalid ranking type: {ranking:?}"));
        }
    };

    let limit = limit.unwrap_or(25);

    console.info(cformat!(
        "Getting ranking <magenta,bold>{}</>...",
        rank_tab.name
    ));

    let results = client
        .get_all_rankings(rank_tab.id, Some(limit), Some(0))
        .await
        .context("Failed to fetch rankings information")?;

    if results.titles().is_empty() {
        console.error("There are no rankings available for some reason.");
        return Err(color_eyre::eyre::eyre!(
            "There are no rankings available for some reason."
        ));
    }

    console.info(cformat!(
        "Fetching <m,s>{}</> titles from <m,s>{}</>",
        results.titles().len(),
        rank_tab.name
    ));

    let titles = client
        .get_titles(results.titles().iter().map(|t| t.id()).collect())
        .await
        .context("Failed fetching title list of a ranking")?;

    if titles.is_empty() {
        console.error("There are no titles available for some reason.");
        return Err(color_eyre::eyre::eyre!(
            "There are no titles available for some reason."
        ));
    }

    console.info(cformat!(
        "Ranking <m,s>{}</> (<s>{}</> results)",
        rank_tab.name,
        titles.len()
    ));
    do_print_search_information(&titles, true, None);

    Ok(())
}
