use color_print::cformat;
use tosho_amap::constants::BASE_HOST;
use tosho_rbean::{
    models::{Carousel, MangaNode},
    RBClient,
};

use crate::{cli::ExitCode, linkify, term::ConsoleChoice};

use super::common::{do_print_search_information, save_session_config};

pub(crate) async fn rbean_home_page(
    client: &mut RBClient,
    account: &super::config::Config,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(&cformat!(
        "Getting home page for user <m,s>{}</>",
        account.id
    ));

    let results = client.get_home_page().await;

    match results {
        Err(e) => {
            console.error(&cformat!("Unable to get home page: {}", e));
            1
        }
        Ok(results) => {
            save_session_config(client, account);
            console.info(&cformat!("Home page for <m,s>{}</>", account.id));

            if let Some(hero_manga) = results.hero.manga {
                let manga_url = format!("https://{}/series/{}", *BASE_HOST, hero_manga.slug);
                let linked_url = linkify!(manga_url, &hero_manga.title);
                console.info(&cformat!(">> <m,s>{}</> <<", linked_url));
                console.info(&manga_url);
                if !results.hero.title.is_empty() {
                    console.info(&cformat!("<m,s>{}</>", results.hero.title));
                }
                if !results.hero.subtitle.is_empty() {
                    console.info(&cformat!(" <s>{}</>", results.hero.subtitle));
                }
                if !results.hero.alt_text.is_empty() {
                    console.info(&format!(" {}", results.hero.alt_text));
                }

                println!();
            }

            if !results.featured.is_empty() {
                console.info(&cformat!("<s>Featured manga:</>"));
            }

            for featured in results.featured.iter() {
                let manga_url = format!("https://{}/series/{}", *BASE_HOST, featured.manga.slug);
                let linked_url = linkify!(manga_url, &featured.manga.title);
                console.info(&cformat!(
                    " <m,s>{}</>: <s>{}</>",
                    featured.title,
                    linked_url
                ));
                console.info(&format!("  {}", featured.description));
                console.info(&format!("   {}", manga_url));
            }

            loop {
                let rank_choices = results
                    .carousels
                    .iter()
                    .map(|r| match r {
                        Carousel::ContinueReading(c) => ConsoleChoice {
                            name: c.title.clone(),
                            value: c.title.clone(),
                        },
                        Carousel::MangaList(c) => ConsoleChoice {
                            name: c.title.clone(),
                            value: c.title.clone(),
                        },
                        Carousel::MangaWithChapters(c) => ConsoleChoice {
                            name: c.title.clone(),
                            value: c.title.clone(),
                        },
                    })
                    .collect::<Vec<ConsoleChoice>>();

                let select = console.choice("Select carousels you want to see", rank_choices);

                match select {
                    None => {
                        console.warn("Aborted");
                        break;
                    }
                    Some(select) => {
                        let ranking = results
                            .carousels
                            .iter()
                            .find(|&r| match r {
                                Carousel::ContinueReading(c) => c.title == select.name,
                                Carousel::MangaList(c) => c.title == select.name,
                                Carousel::MangaWithChapters(c) => c.title == select.name,
                            })
                            .unwrap();

                        let title = match ranking {
                            Carousel::ContinueReading(c) => c.title.clone(),
                            Carousel::MangaList(c) => c.title.clone(),
                            Carousel::MangaWithChapters(c) => c.title.clone(),
                        };

                        let manga_list: Vec<MangaNode> = match ranking {
                            Carousel::ContinueReading(c) => {
                                c.items.iter().map(|i| i.manga.clone()).collect()
                            }
                            Carousel::MangaList(c) => c.items.to_vec(),
                            Carousel::MangaWithChapters(c) => c.items.to_vec(),
                        };

                        console.info(&cformat!(
                            "Carousel for <m,s>{}</> ({} titles):",
                            title,
                            manga_list.len()
                        ));

                        do_print_search_information(&manga_list, true, None);
                        println!();
                    }
                }
            }

            0
        }
    }
}
