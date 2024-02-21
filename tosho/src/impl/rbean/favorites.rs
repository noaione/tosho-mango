use color_print::cformat;
use tosho_rbean::{constants::BASE_HOST, RBClient};

use crate::{cli::ExitCode, linkify};

use super::common::{do_print_single_information, save_session_config};

pub(crate) async fn rbean_read_list(
    client: &mut RBClient,
    account: &super::config::Config,
    console: &crate::term::Terminal,
) -> ExitCode {
    console.info(&cformat!(
        "Getting read list for user <m,s>{}</>",
        account.id
    ));

    let results = client.get_reading_list().await;

    match results {
        Err(e) => {
            console.error(&cformat!("Unable to get read list: {}", e));
            1
        }
        Ok(results) => {
            save_session_config(client, account);
            console.info(&cformat!("Reading list for <m,s>{}</>", account.id));

            for result in results.iter() {
                do_print_single_information(&result.manga, 0, false, None);

                if let Some(chapter) = &result.chapter {
                    let linked_ch = format!(
                        "https://{}/series/{}/read/{}",
                        *BASE_HOST, result.manga.slug, chapter.uuid
                    );

                    let linked_url = linkify!(linked_ch, &format!("Chapter {}", chapter.name));

                    console.info(&cformat!("   <s>{}:</> {}", linked_url, linked_ch));
                }
            }

            0
        }
    }
}
