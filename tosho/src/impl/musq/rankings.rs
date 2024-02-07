use color_print::cformat;
use tosho_musq::MUClient;

use crate::{cli::ExitCode, term::ConsoleChoice};

use super::common::do_print_search_information;

pub(crate) async fn musq_home_rankings(
    client: &MUClient,
    account: &super::config::Config,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(&cformat!(
        "Getting rankings list for user <m,s>{}</>",
        account.id
    ));

    let results = client.get_my_home().await;

    match results {
        Err(e) => {
            console.error(&cformat!("Unable to connect to MU!: {}", e));
            1
        }
        Ok(results) => {
            if results.rankings.is_empty() {
                console.error("There are no rankings available for some reason.");
                return 1;
            }

            loop {
                let rank_choices = results
                    .rankings
                    .iter()
                    .map(|r| ConsoleChoice {
                        name: r.name.clone(),
                        value: r.name.clone(),
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
                            .rankings
                            .iter()
                            .find(|r| r.name == select.name)
                            .unwrap();

                        console.info(&cformat!(
                            "Ranking for <m,s>{}</> ({} titles):",
                            ranking.name,
                            ranking.titles.len()
                        ));

                        do_print_search_information(ranking.titles.clone(), true, None);
                        println!();
                    }
                }
            }

            0
        }
    }
}
