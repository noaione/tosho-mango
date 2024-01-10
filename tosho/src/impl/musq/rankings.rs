use color_print::cformat;

use crate::{cli::ExitCode, term::ConsoleChoice};

use super::common::{do_print_search_information, make_client, select_single_account};

pub(crate) async fn musq_home_rankings(
    account_id: Option<&str>,
    console: &crate::term::Terminal,
) -> ExitCode {
    let account = select_single_account(account_id);

    if account.is_none() {
        console.warn("Aborted");
        return 1;
    }

    let account = account.unwrap();
    console.info(&cformat!(
        "Getting rankings list for user <m,s>{}</>",
        account.id
    ));
    let client = make_client(&account);

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
                        name: r.tag_id.to_string(),
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
                            .find(|r| r.tag_id == select.name.parse::<u64>().unwrap())
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
