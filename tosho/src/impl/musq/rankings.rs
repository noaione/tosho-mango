use color_eyre::eyre::{Context, OptionExt};
use color_print::cformat;
use tosho_musq::MUClient;

use crate::term::ConsoleChoice;

use super::common::do_print_search_information;

pub(crate) async fn musq_home_rankings(
    client: &MUClient,
    account: &super::config::Config,
    console: &crate::term::Terminal,
) -> color_eyre::Result<()> {
    console.info(cformat!(
        "Getting rankings list for user <m,s>{}</>",
        account.id
    ));

    let results = client
        .get_my_home()
        .await
        .context("Failed to connect to MU!")?;

    if results.rankings().is_empty() {
        console.error("There are no rankings available for some reason.");
        return Err(color_eyre::eyre::eyre!(
            "There are no rankings available for some reason."
        ));
    }

    loop {
        let rank_choices = results
            .rankings()
            .iter()
            .map(|r| ConsoleChoice {
                name: r.name().to_string(),
                value: r.name().to_string(),
            })
            .collect::<Vec<ConsoleChoice>>();

        let select = console.choice("Select ranking you want to see", rank_choices);

        match select {
            None => {
                console.warn("Aborted");
                break;
            }
            Some(select) => {
                let ranking = results
                    .rankings()
                    .iter()
                    .find(|&r| r.name() == select.name)
                    .ok_or_eyre("Failed to match ranking selection")?;

                console.info(cformat!(
                    "Ranking for <m,s>{}</> ({} titles):",
                    ranking.name(),
                    ranking.titles().len()
                ));

                do_print_search_information(ranking.titles(), true, None);
                println!();
            }
        }
    }

    Ok(())
}
